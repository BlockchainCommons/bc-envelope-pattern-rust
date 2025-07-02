use std::{collections::HashMap, ops::RangeBounds};

use bc_envelope::prelude::*;

use crate::{
    DCBORMatcher, Pattern,
    pattern::{Matcher, Path, compile_as_atomic, leaf::LeafPattern, vm::Instr},
};

/// Pattern for matching maps.
/// This delegates directly to dcbor-pattern for map matching.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct MapPattern(dcbor_pattern::MapPattern);

impl MapPattern {
    /// Creates a new `MapPattern` that matches any map.
    pub fn any() -> Self {
        MapPattern(dcbor_pattern::MapPattern::any())
    }

    /// Creates a new `MapPattern` that matches maps with a specific count of
    /// entries.
    pub fn interval(interval: impl RangeBounds<usize>) -> Self {
        MapPattern(dcbor_pattern::MapPattern::with_length_range(interval))
    }

    /// Creates a new `MapPattern` from a dcbor-pattern MapPattern.
    pub fn from_dcbor_pattern(map_pattern: dcbor_pattern::MapPattern) -> Self {
        MapPattern(map_pattern)
    }
}

impl Matcher for MapPattern {
    fn paths_with_captures(
        &self,
        envelope: &Envelope,
    ) -> (Vec<Path>, HashMap<String, Vec<Path>>) {
        let paths = if let Some(cbor_value) = envelope.subject().as_leaf() {
            // Use dcbor-pattern to match against the CBOR value directly
            if self.0.matches(&cbor_value) {
                vec![vec![envelope.clone()]]
            } else {
                vec![]
            }
        } else {
            vec![]
        };

        // For now, we don't support captures through the simple delegation
        // This could be enhanced later if needed
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
        // Hash the string representation since dcbor_pattern::MapPattern
        // doesn't implement Hash
        self.0.to_string().hash(state);
    }
}

impl std::fmt::Display for MapPattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Delegate to dcbor-pattern's Display implementation
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
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
        assert_eq!(pattern.to_string(), "{*}");

        let pattern = MapPattern::interval(2..=2);
        assert_eq!(pattern.to_string(), "{{2}}");

        let pattern = MapPattern::interval(1..=3);
        assert_eq!(pattern.to_string(), "{{1,3}}");

        let pattern = MapPattern::interval(1..);
        assert_eq!(pattern.to_string(), "{{1,}}");
    }
}
