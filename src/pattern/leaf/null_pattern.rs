use bc_envelope::Envelope;
use dcbor_pattern::{Matcher as DcborMatcher, NullPattern as DcborNullPattern};

use crate::{
    Pattern,
    pattern::{Matcher, Path, compile_as_atomic, leaf::LeafPattern, vm::Instr},
};

/// Pattern for matching null values.
///
/// This is a wrapper around `dcbor_pattern::NullPattern` that provides
/// envelope-specific functionality while delegating core matching logic
/// to the underlying CBOR pattern matcher.
#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct NullPattern {
    inner: DcborNullPattern,
}

impl NullPattern {
    /// Creates a new `NullPattern` that matches any null value.
    pub fn new() -> Self { NullPattern { inner: DcborNullPattern::new() } }
}

impl Default for NullPattern {
    fn default() -> Self { NullPattern::new() }
}

impl Matcher for NullPattern {
    fn paths(&self, envelope: &Envelope) -> Vec<Path> {
        if let Some(cbor) = envelope.subject().as_leaf() {
            if self.inner.matches(&cbor) {
                vec![vec![envelope.clone()]]
            } else {
                vec![]
            }
        } else {
            vec![]
        }
    }

    fn compile(
        &self,
        code: &mut Vec<Instr>,
        literals: &mut Vec<Pattern>,
        captures: &mut Vec<String>,
    ) {
        compile_as_atomic(
            &Pattern::Leaf(LeafPattern::Null(self.clone())),
            code,
            literals,
            captures,
        );
    }
}

impl std::fmt::Display for NullPattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.inner)
    }
}

#[cfg(test)]
mod tests {
    use bc_envelope::Envelope;
    use dcbor::prelude::*;

    use super::*;

    #[test]
    fn test_null_pattern_any() {
        let null_envelope = Envelope::null();
        let pattern = NullPattern::new();
        let paths = pattern.paths(&null_envelope);
        assert_eq!(paths.len(), 1);
        assert_eq!(paths[0], vec![null_envelope.clone()]);

        // Test with non-null envelope
        let text_envelope = Envelope::new("test");
        let paths = pattern.paths(&text_envelope);
        assert!(paths.is_empty());

        let number_envelope = Envelope::new(42);
        let paths = pattern.paths(&number_envelope);
        assert!(paths.is_empty());

        let bool_envelope = Envelope::new(true);
        let paths = pattern.paths(&bool_envelope);
        assert!(paths.is_empty());
    }

    #[test]
    fn test_null_pattern_display() {
        assert_eq!(NullPattern::new().to_string(), "NULL");
    }

    #[test]
    fn test_null_pattern_with_assertions() {
        // Test null value with assertions
        let null_envelope = Envelope::null().add_assertion("test", "value");
        let pattern = NullPattern::new();
        let paths = pattern.paths(&null_envelope);
        assert_eq!(paths.len(), 1);
        assert_eq!(paths[0], vec![null_envelope.clone()]);
    }

    #[test]
    fn test_null_pattern_delegated_matching() {
        let pattern = NullPattern::new();

        // Test that the pattern correctly delegates to
        // dcbor_pattern::NullPattern
        let null_cbor = CBOR::null();
        assert!(pattern.inner.matches(&null_cbor));

        let text_cbor = "test".to_cbor();
        assert!(!pattern.inner.matches(&text_cbor));

        let number_cbor = 42.to_cbor();
        assert!(!pattern.inner.matches(&number_cbor));

        let bool_cbor = true.to_cbor();
        assert!(!pattern.inner.matches(&bool_cbor));
    }

    #[test]
    fn test_null_pattern_envelope_as_leaf() {
        let pattern = NullPattern::new();

        // Test with envelope containing null value
        let null_envelope = Envelope::null();
        if let Some(cbor) = null_envelope.subject().as_leaf() {
            assert!(pattern.inner.matches(&cbor));
        } else {
            panic!("Expected null envelope to have leaf subject");
        }

        // Test with envelope containing non-null value
        let text_envelope = Envelope::new("test");
        if let Some(cbor) = text_envelope.subject().as_leaf() {
            assert!(!pattern.inner.matches(&cbor));
        } else {
            panic!("Expected text envelope to have leaf subject");
        }
    }
}
