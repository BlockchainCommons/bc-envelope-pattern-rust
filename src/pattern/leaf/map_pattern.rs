use std::{collections::HashMap, ops::RangeBounds};

use bc_envelope::Envelope;

use crate::{
    Interval, Pattern,
    pattern::{Matcher, Path, compile_as_atomic, leaf::LeafPattern, vm::Instr},
};

/// Pattern for matching maps.
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum MapPattern {
    /// Matches any map.
    Any,
    /// Matches maps with a specific count of entries.
    Interval(Interval),
    /// Matches maps using dcbor-pattern's map syntax.
    Content(dcbor_pattern::Pattern),
}

impl MapPattern {
    /// Creates a new `MapPattern` that matches any map.
    pub fn any() -> Self { MapPattern::Any }

    /// Creates a new `MapPattern` that matches maps with a specific count of
    /// entries.
    pub fn interval(interval: impl RangeBounds<usize>) -> Self {
        MapPattern::Interval(Interval::new(interval))
    }

    /// Creates a new `MapPattern` from a dcbor-pattern MapPattern.
    pub fn from_dcbor_pattern(map_pattern: dcbor_pattern::MapPattern) -> Self {
        MapPattern::Content(dcbor_pattern::Pattern::Structure(
            dcbor_pattern::StructurePattern::Map(map_pattern)
        ))
    }
}

impl Matcher for MapPattern {
    fn paths_with_captures(
        &self,
        envelope: &Envelope,
    ) -> (Vec<Path>, HashMap<String, Vec<Path>>) {
        let paths = if let Some(map) = envelope.subject().as_map() {
            match self {
                MapPattern::Any => vec![vec![envelope.clone()]],
                MapPattern::Interval(range) => {
                    if range.contains(map.len()) {
                        vec![vec![envelope.clone()]]
                    } else {
                        vec![]
                    }
                }
                MapPattern::Content(dcbor_pattern) => {
                    // Delegate to dcbor-pattern for content matching
                    if let Some(cbor) = envelope.subject().as_leaf() {
                        let dcbor_paths = dcbor_pattern::Matcher::paths(dcbor_pattern, &cbor);
                        if !dcbor_paths.is_empty() {
                            vec![vec![envelope.clone()]]
                        } else {
                            vec![]
                        }
                    } else {
                        vec![]
                    }
                }
            }
        } else {
            vec![]
        };
        (paths, HashMap::new())
    }

    fn compile(
        &self,
        code: &mut Vec<Instr>,
        literals: &mut Vec<Pattern>,
        captures: &mut Vec<String>,
    ) {
        compile_as_atomic(
            &Pattern::Leaf(LeafPattern::Map(self.clone())),
            code,
            literals,
            captures,
        );
    }
}

impl std::hash::Hash for MapPattern {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self {
            MapPattern::Any => {
                0u8.hash(state);
            }
            MapPattern::Interval(range) => {
                1u8.hash(state);
                range.hash(state);
            }
            MapPattern::Content(pattern) => {
                2u8.hash(state);
                // Hash the string representation since dcbor_pattern::Pattern
                // doesn't implement Hash
                pattern.to_string().hash(state);
            }
        }
    }
}

impl std::fmt::Display for MapPattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MapPattern::Any => write!(f, "MAP"),
            MapPattern::Interval(range) => write!(f, "MAP({})", range),
            MapPattern::Content(pattern) => {
                // For Content variants (dcbor-pattern integration), display the pattern directly
                write!(f, "{}", pattern)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use bc_envelope::Envelope;
    use dcbor::prelude::*;

    use super::*;

    #[test]
    fn test_map_pattern_any() {
        // Create a CBOR map directly
        let mut cbor_map = Map::new();
        cbor_map.insert("key1", "value1");
        cbor_map.insert("key2", "value2");
        let envelope = Envelope::new(cbor_map);

        let pattern = MapPattern::any();
        let paths = pattern.paths(&envelope);
        assert_eq!(paths.len(), 1);
        assert_eq!(paths[0], vec![envelope.clone()]);

        // Test with non-map envelope
        let text_envelope = Envelope::new("test");
        let paths = pattern.paths(&text_envelope);
        assert!(paths.is_empty());
    }

    #[test]
    fn test_map_pattern_count() {
        // Create a CBOR map directly
        let mut cbor_map = Map::new();
        cbor_map.insert("key1", "value1");
        cbor_map.insert("key2", "value2");
        let envelope = Envelope::new(cbor_map);

        // Test exact count
        let pattern = MapPattern::interval(2..=2);
        let paths = pattern.paths(&envelope);
        assert_eq!(paths.len(), 1);

        // Test count range
        let pattern = MapPattern::interval(1..=3);
        let paths = pattern.paths(&envelope);
        assert_eq!(paths.len(), 1);

        // Test count mismatch
        let pattern = MapPattern::interval(5..=5);
        let paths = pattern.paths(&envelope);
        assert!(paths.is_empty());
    }

    #[test]
    fn test_map_pattern_display() {
        let pattern = MapPattern::any();
        assert_eq!(pattern.to_string(), "MAP");

        let pattern = MapPattern::interval(2..=2);
        assert_eq!(pattern.to_string(), "MAP({2})");

        let pattern = MapPattern::interval(1..=3);
        assert_eq!(pattern.to_string(), "MAP({1,3})");

        let pattern = MapPattern::interval(1..);
        assert_eq!(pattern.to_string(), "MAP({1,})");
    }
}
