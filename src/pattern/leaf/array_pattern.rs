use std::{collections::HashMap, ops::RangeBounds};

use bc_envelope::Envelope;
use dcbor_pattern::Matcher as DcborMatcher;

use crate::{
    Pattern,
    pattern::{Matcher, Path, compile_as_atomic, leaf::LeafPattern, vm::Instr},
};

/// Pattern for matching arrays.
/// This delegates directly to dcbor-pattern for array matching.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ArrayPattern(dcbor_pattern::ArrayPattern);

impl ArrayPattern {
    /// Creates a new `ArrayPattern` that matches any array.
    pub fn any() -> Self {
        ArrayPattern(dcbor_pattern::ArrayPattern::any())
    }

    /// Creates a new `ArrayPattern` that matches arrays with a count
    /// of elements in the specified range.
    pub fn interval(interval: impl RangeBounds<usize>) -> Self {
        ArrayPattern(dcbor_pattern::ArrayPattern::with_length_range(interval))
    }

    /// Creates a new `ArrayPattern` that matches arrays with exact count.
    pub fn count(n: usize) -> Self {
        ArrayPattern(dcbor_pattern::ArrayPattern::with_length_range(n..=n))
    }

    /// Creates a new `ArrayPattern` from a dcbor-pattern.
    pub fn from_dcbor_pattern(pattern: dcbor_pattern::Pattern) -> Self {
        ArrayPattern(dcbor_pattern::ArrayPattern::with_elements(pattern))
    }

    /// Creates a new `ArrayPattern` from a dcbor-pattern ArrayPattern.
    pub fn from_dcbor_array_pattern(
        array_pattern: dcbor_pattern::ArrayPattern,
    ) -> Self {
        ArrayPattern(array_pattern)
    }
}

impl std::hash::Hash for ArrayPattern {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        // Hash the string representation since dcbor_pattern::ArrayPattern
        // doesn't implement Hash
        self.0.to_string().hash(state);
    }
}

impl Matcher for ArrayPattern {
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
            &Pattern::Leaf(LeafPattern::Array(self.clone())),
            code,
            literals,
            captures,
        );
    }
}

impl std::fmt::Display for ArrayPattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Delegate to dcbor-pattern's Display implementation
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use bc_envelope::Envelope;
    use dcbor::prelude::*;

    use super::*;

    #[test]
    fn test_array_pattern_any() {
        // Create a CBOR array directly
        let cbor_array = vec![1, 2, 3].to_cbor();
        let envelope = Envelope::new(cbor_array);
        let pattern = ArrayPattern::any();
        let paths = pattern.paths(&envelope);
        assert_eq!(paths.len(), 1);
        assert_eq!(paths[0], vec![envelope.clone()]);

        // Test with non-array envelope
        let text_envelope = Envelope::new("test");
        let paths = pattern.paths(&text_envelope);
        assert!(paths.is_empty());
    }

    #[test]
    fn test_array_pattern_count() {
        // Create a CBOR array directly
        let cbor_array = vec![1, 2, 3].to_cbor();
        let envelope = Envelope::new(cbor_array);

        // Test exact count
        let pattern = ArrayPattern::count(3);
        let paths = pattern.paths(&envelope);
        assert_eq!(paths.len(), 1);

        // Test count mismatch
        let pattern = ArrayPattern::count(5);
        let paths = pattern.paths(&envelope);
        assert!(paths.is_empty());
    }

    #[test]
    fn test_array_pattern_range() {
        // Create a CBOR array directly
        let cbor_array = vec![1, 2, 3].to_cbor();
        let envelope = Envelope::new(cbor_array);

        // Test count range
        let pattern = ArrayPattern::interval(2..=4);
        let paths = pattern.paths(&envelope);
        assert_eq!(paths.len(), 1);

        // Test count mismatch
        let pattern = ArrayPattern::interval(5..=10);
        let paths = pattern.paths(&envelope);
        assert!(paths.is_empty());
    }

    #[test]
    fn test_array_pattern_display() {
        assert_eq!(ArrayPattern::any().to_string(), "[*]");
        assert_eq!(ArrayPattern::count(3).to_string(), "[{3}]");
        assert_eq!(ArrayPattern::interval(2..=5).to_string(), "[{2,5}]");
        assert_eq!(ArrayPattern::interval(3..).to_string(), "[{3,}]");
    }
}
