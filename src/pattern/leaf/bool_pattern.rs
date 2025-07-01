use std::collections::HashMap;

use bc_envelope::Envelope;

use crate::{
    Pattern,
    pattern::{Matcher, Path, compile_as_atomic, leaf::LeafPattern, vm::Instr},
};

/// Pattern for matching boolean values. This is a wrapper around
/// dcbor_pattern::BoolPattern that provides envelope-specific integration.
#[derive(Debug, Clone)]
pub struct BoolPattern {
    inner: dcbor_pattern::BoolPattern,
}

// Re-export the dcbor-pattern BoolPattern methods through associated
// functions
impl BoolPattern {
    /// Creates a new `BoolPattern` that matches any boolean value.
    pub fn any() -> Self { Self { inner: dcbor_pattern::BoolPattern::any() } }

    /// Creates a new `BoolPattern` that matches the specific boolean value.
    pub fn value(value: bool) -> Self {
        Self { inner: dcbor_pattern::BoolPattern::value(value) }
    }

    /// Creates a new `BoolPattern` from a dcbor-pattern BoolPattern.
    pub fn from_dcbor_pattern(dcbor_pattern: dcbor_pattern::BoolPattern) -> Self {
        Self { inner: dcbor_pattern }
    }
}

impl PartialEq for BoolPattern {
    fn eq(&self, other: &Self) -> bool { self.inner == other.inner }
}

impl Eq for BoolPattern {}

impl std::hash::Hash for BoolPattern {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.inner.hash(state);
    }
}

impl Matcher for BoolPattern {
    fn paths_with_captures(
        &self,
        envelope: &Envelope,
    ) -> (Vec<Path>, HashMap<String, Vec<Path>>) {
        // Try to extract CBOR from the envelope using the existing as_leaf()
        // method
        if let Some(cbor) = envelope.subject().as_leaf() {
            // Delegate to dcbor-pattern for CBOR matching using paths() method
            // BoolPattern doesn't support captures, so we only get paths
            let dcbor_paths = dcbor_pattern::Matcher::paths(&self.inner, &cbor);

            // For simple leaf patterns, if dcbor-pattern found matches, return
            // the envelope
            if !dcbor_paths.is_empty() {
                let envelope_paths = vec![vec![envelope.clone()]];
                let envelope_captures = HashMap::new(); // No captures for simple bool patterns
                (envelope_paths, envelope_captures)
            } else {
                (vec![], HashMap::new())
            }
        } else {
            // Not a leaf envelope, no match
            (vec![], HashMap::new())
        }
    }

    fn paths(&self, envelope: &Envelope) -> Vec<Path> {
        self.paths_with_captures(envelope).0
    }

    fn compile(
        &self,
        code: &mut Vec<Instr>,
        literals: &mut Vec<Pattern>,
        captures: &mut Vec<String>,
    ) {
        compile_as_atomic(
            &Pattern::Leaf(LeafPattern::Bool(self.clone())),
            code,
            literals,
            captures,
        );
    }
}

impl std::fmt::Display for BoolPattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.inner)
    }
}

#[cfg(test)]
mod tests {
    use bc_envelope::Envelope;
    use dcbor_parse::parse_dcbor_item;

    use super::*;

    #[test]
    fn test_bool_pattern_display() {
        assert_eq!(BoolPattern::any().to_string(), "bool");
        assert_eq!(BoolPattern::value(true).to_string(), "true");
        assert_eq!(BoolPattern::value(false).to_string(), "false");
    }

    #[test]
    fn test_bool_pattern_dcbor_integration() {
        // Test that the dcbor-pattern integration works correctly
        let true_envelope = Envelope::new(true);
        let false_envelope = Envelope::new(false);
        let number_envelope = Envelope::new(42);
        let text_envelope = Envelope::new("hello");

        // Test any pattern
        let any_pattern = BoolPattern::any();
        assert!(any_pattern.matches(&true_envelope));
        assert!(any_pattern.matches(&false_envelope));
        assert!(!any_pattern.matches(&number_envelope)); // Should not match number
        assert!(!any_pattern.matches(&text_envelope)); // Should not match text

        // Test exact value patterns
        let true_pattern = BoolPattern::value(true);
        assert!(true_pattern.matches(&true_envelope));
        assert!(!true_pattern.matches(&false_envelope));
        assert!(!true_pattern.matches(&number_envelope));
        assert!(!true_pattern.matches(&text_envelope));

        let false_pattern = BoolPattern::value(false);
        assert!(!false_pattern.matches(&true_envelope));
        assert!(false_pattern.matches(&false_envelope));
        assert!(!false_pattern.matches(&number_envelope));
        assert!(!false_pattern.matches(&text_envelope));

        // Test paths
        let paths = true_pattern.paths(&true_envelope);
        assert_eq!(paths.len(), 1);
        assert_eq!(paths[0], vec![true_envelope.clone()]);

        let no_paths = true_pattern.paths(&false_envelope);
        assert_eq!(no_paths.len(), 0);
    }

    #[test]
    fn test_bool_pattern_paths_with_captures() {
        let bool_envelope = Envelope::new(true);
        let pattern = BoolPattern::value(true);

        let (paths, captures) = pattern.paths_with_captures(&bool_envelope);
        assert_eq!(paths.len(), 1);
        assert_eq!(paths[0], vec![bool_envelope.clone()]);
        assert_eq!(captures.len(), 0); // No captures for simple bool patterns
    }

    #[test]
    fn test_bool_pattern_with_non_bool_envelope() {
        // Test with envelope that doesn't contain a boolean
        let envelope = Envelope::new_assertion("key", "value");
        let pattern = BoolPattern::any();

        let paths = pattern.paths(&envelope);
        assert_eq!(paths.len(), 0); // Should not match non-bool envelopes
    }

    #[test]
    fn test_bool_pattern_with_direct_cbor_values() {
        // Test that our pattern works with CBOR boolean values
        let true_cbor = parse_dcbor_item("true").unwrap();
        let false_cbor = parse_dcbor_item("false").unwrap();

        let true_envelope = Envelope::new(true_cbor);
        let false_envelope = Envelope::new(false_cbor);

        let any_pattern = BoolPattern::any();
        assert!(any_pattern.matches(&true_envelope));
        assert!(any_pattern.matches(&false_envelope));

        let true_pattern = BoolPattern::value(true);
        assert!(true_pattern.matches(&true_envelope));
        assert!(!true_pattern.matches(&false_envelope));

        let false_pattern = BoolPattern::value(false);
        assert!(!false_pattern.matches(&true_envelope));
        assert!(false_pattern.matches(&false_envelope));
    }
}
