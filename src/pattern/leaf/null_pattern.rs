use std::collections::HashMap;

use bc_envelope::Envelope;

use crate::{
    DCBORMatcher, Pattern,
    pattern::{Matcher, Path, compile_as_atomic, leaf::LeafPattern, vm::Instr},
};

/// Pattern for matching null values.
///
/// This is a wrapper around `dcbor_pattern::NullPattern` that provides
/// envelope-specific functionality while delegating core matching logic
/// to the underlying CBOR pattern matcher.
#[derive(Debug, Clone, Hash, Eq, PartialEq, Default)]
pub struct NullPattern;

impl Matcher for NullPattern {
    fn paths_with_captures(
        &self,
        envelope: &Envelope,
    ) -> (Vec<Path>, HashMap<String, Vec<Path>>) {
        if let Some(cbor) = envelope.subject().as_leaf() {
            if dcbor_pattern::NullPattern.matches(&cbor) {
                (vec![vec![envelope.clone()]], HashMap::new())
            } else {
                (vec![], HashMap::new())
            }
        } else {
            (vec![], HashMap::new())
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
        write!(f, "{}", dcbor_pattern::NullPattern)
    }
}

#[cfg(test)]
mod tests {
    use bc_envelope::prelude::*;

    use super::*;

    #[test]
    fn test_null_pattern_any() {
        let null_envelope = Envelope::null();
        let pattern = NullPattern;
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
        assert_eq!(NullPattern.to_string(), "null");
    }

    #[test]
    fn test_null_pattern_with_assertions() {
        // Test null value with assertions
        let null_envelope = Envelope::null().add_assertion("test", "value");
        let pattern = NullPattern;
        let paths = pattern.paths(&null_envelope);
        assert_eq!(paths.len(), 1);
        assert_eq!(paths[0], vec![null_envelope.clone()]);
    }

    #[test]
    fn test_null_pattern_delegated_matching() {
        let pattern = NullPattern;

        // Test that the pattern correctly delegates to
        // dcbor_pattern::NullPattern
        let null_envelope = Envelope::null();
        assert!(pattern.matches(&null_envelope));

        let text_envelope = "test".to_envelope();
        assert!(!pattern.matches(&text_envelope));

        let number_envelope = 42.to_envelope();
        assert!(!pattern.matches(&number_envelope));

        let bool_envelope = true.to_envelope();
        assert!(!pattern.matches(&bool_envelope));
    }
}
