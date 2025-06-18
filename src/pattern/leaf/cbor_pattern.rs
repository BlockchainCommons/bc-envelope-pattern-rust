use bc_envelope::Envelope;
use dcbor::prelude::*;

use crate::{
    Pattern,
    pattern::{Matcher, Path, compile_as_atomic, leaf::LeafPattern, vm::Instr},
};

/// Pattern for matching specific CBOR values.
#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub enum CBORPattern {
    /// Matches any CBOR value.
    Any,
    /// Matches the specific CBOR value.
    Value(CBOR),
}

impl CBORPattern {
    /// Creates a new `CborPattern` that matches any CBOR value.
    pub fn any() -> Self { CBORPattern::Any }

    /// Creates a new `CborPattern` that matches a specific CBOR value.
    pub fn value(cbor: impl CBOREncodable) -> Self {
        CBORPattern::Value(cbor.to_cbor())
    }
}

impl Matcher for CBORPattern {
    fn paths(&self, envelope: &Envelope) -> Vec<Path> {
        let subject = envelope.subject();

        // Special case for KnownValue
        if let Some(known_value) = subject.as_known_value() {
            return match self {
                CBORPattern::Any => vec![vec![envelope.clone()]],
                CBORPattern::Value(expected_cbor) => {
                    // Create CBOR from the KnownValue for comparison
                    let known_value_cbor = known_value.to_cbor();
                    if &known_value_cbor == expected_cbor {
                        vec![vec![envelope.clone()]]
                    } else {
                        vec![]
                    }
                }
            };
        }

        // Standard case for CBOR leaf
        let subject_cbor = match subject.as_leaf() {
            Some(cbor) => cbor,
            None => return vec![],
        };

        match self {
            CBORPattern::Any => vec![vec![envelope.clone()]],
            CBORPattern::Value(expected_cbor) => {
                if subject_cbor == *expected_cbor {
                    vec![vec![envelope.clone()]]
                } else {
                    vec![]
                }
            }
        }
    }

    fn compile(&self, code: &mut Vec<Instr>, literals: &mut Vec<Pattern>) {
        compile_as_atomic(
            &Pattern::Leaf(LeafPattern::Cbor(self.clone())),
            code,
            literals,
        );
    }
}

impl std::fmt::Display for CBORPattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CBORPattern::Any => write!(f, "CBOR"),
            CBORPattern::Value(cbor) => {
                write!(f, "CBOR({})", cbor.diagnostic_flat())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use bc_envelope::Envelope;

    use super::*;

    #[test]
    fn test_cbor_pattern_any() {
        let envelope = Envelope::new("test");
        let pattern = CBORPattern::any();
        let paths = pattern.paths(&envelope);
        assert_eq!(paths.len(), 1);
        assert_eq!(paths[0], vec![envelope.clone()]);
    }

    #[test]
    fn test_cbor_pattern_exact() {
        let value = "test_value";
        let envelope = Envelope::new(value);
        let cbor = envelope.subject().as_leaf().unwrap().clone();
        let pattern = CBORPattern::value(cbor);
        let paths = pattern.paths(&envelope);
        assert_eq!(paths.len(), 1);
        assert_eq!(paths[0], vec![envelope.clone()]);

        // Test with different value
        let different_envelope = Envelope::new("different");
        let paths = pattern.paths(&different_envelope);
        assert!(paths.is_empty());
    }
}
