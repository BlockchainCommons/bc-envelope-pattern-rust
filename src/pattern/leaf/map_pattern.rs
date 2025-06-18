use std::ops::RangeBounds;

use bc_envelope::Envelope;

use crate::{
    Interval, Pattern,
    pattern::{Matcher, Path, compile_as_atomic, leaf::LeafPattern, vm::Instr},
};

/// Pattern for matching maps.
#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub enum MapPattern {
    /// Matches any map.
    Any,
    /// Matches maps with a specific count of entries.
    Interval(Interval),
}

impl MapPattern {
    /// Creates a new `MapPattern` that matches any map.
    pub fn any() -> Self { MapPattern::Any }

    /// Creates a new `MapPattern` that matches maps with a specific count of
    /// entries.
    pub fn interval(interval: impl RangeBounds<usize>) -> Self {
        MapPattern::Interval(Interval::new(interval))
    }
}

impl Matcher for MapPattern {
    fn paths(&self, envelope: &Envelope) -> Vec<Path> {
        if let Some(map) = envelope.subject().as_map() {
            match self {
                MapPattern::Any => vec![vec![envelope.clone()]],
                MapPattern::Interval(range) => {
                    if range.contains(map.len()) {
                        vec![vec![envelope.clone()]]
                    } else {
                        vec![]
                    }
                }
            }
        } else {
            vec![]
        }
    }

    fn compile(&self, code: &mut Vec<Instr>, literals: &mut Vec<Pattern>) {
        compile_as_atomic(
            &Pattern::Leaf(LeafPattern::Map(self.clone())),
            code,
            literals,
        );
    }
}

impl std::fmt::Display for MapPattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MapPattern::Any => write!(f, "MAP"),
            MapPattern::Interval(range) => write!(f, "MAP({})", range),
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
