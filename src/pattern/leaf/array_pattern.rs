use std::ops::RangeBounds;

use bc_envelope::Envelope;

use crate::{
    Pattern, Repeat,
    pattern::{
        Matcher, Path, compile_as_atomic, leaf::LeafPattern,
        vm::Instr,
    },
};

/// Pattern for matching arrays.
#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub enum ArrayPattern {
    /// Matches any array.
    Any,
    /// Matches arrays with a specific count of elements.
    Count(Repeat),
}

impl ArrayPattern {
    /// Creates a new `ArrayPattern` that matches any array.
    pub fn any() -> Self { ArrayPattern::Any }

    /// Creates a new `ArrayPattern` that matches arrays with a count
    /// of elements in the specified range.
    pub fn count(range: impl RangeBounds<usize>) -> Self {
        ArrayPattern::Count(Repeat::new(range))
    }
}

impl Matcher for ArrayPattern {
    fn paths(&self, envelope: &Envelope) -> Vec<Path> {
        if let Some(array) = envelope.subject().as_array() {
            match self {
                ArrayPattern::Any => vec![vec![envelope.clone()]],
                ArrayPattern::Count(range) => {
                    if range.contains(array.len()) {
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
            &Pattern::Leaf(LeafPattern::Array(self.clone())),
            code,
            literals,
        );
    }
}

impl std::fmt::Display for ArrayPattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ArrayPattern::Any => write!(f, "ARRAY"),
            ArrayPattern::Count(range) => {
                if range.is_single() {
                    write!(f, "ARRAY({{{}}})", range.min())
                } else {
                    write!(
                        f,
                        "ARRAY({{{}}},{{{}}})",
                        range.min(),
                        range.max().unwrap()
                    )
                }
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
        let pattern = ArrayPattern::count(3..=3);
        let paths = pattern.paths(&envelope);
        assert_eq!(paths.len(), 1);

        // Test count range
        let pattern = ArrayPattern::count(2..=4);
        let paths = pattern.paths(&envelope);
        assert_eq!(paths.len(), 1);

        // Test count mismatch
        let pattern = ArrayPattern::count(5..=5);
        let paths = pattern.paths(&envelope);
        assert!(paths.is_empty());
    }
}
