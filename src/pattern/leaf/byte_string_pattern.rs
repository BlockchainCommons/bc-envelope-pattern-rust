use bc_envelope::Envelope;

use crate::{
    Pattern,
    pattern::{Matcher, Path, compile_as_atomic, leaf::LeafPattern, vm::Instr},
};

/// Pattern for matching byte string values.
#[derive(Debug, Clone)]
pub enum ByteStringPattern {
    /// Matches any byte string.
    Any,
    /// Matches the specific byte string.
    Value(Vec<u8>),
    /// Matches the binary regular expression for a byte string.
    Regex(regex::bytes::Regex),
}

impl PartialEq for ByteStringPattern {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (ByteStringPattern::Any, ByteStringPattern::Any) => true,
            (ByteStringPattern::Value(a), ByteStringPattern::Value(b)) => {
                a == b
            }
            (ByteStringPattern::Regex(a), ByteStringPattern::Regex(b)) => {
                a.as_str() == b.as_str()
            }
            _ => false,
        }
    }
}

impl Eq for ByteStringPattern {}

impl std::hash::Hash for ByteStringPattern {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self {
            ByteStringPattern::Any => {
                0u8.hash(state);
            }
            ByteStringPattern::Value(s) => {
                1u8.hash(state);
                s.hash(state);
            }
            ByteStringPattern::Regex(regex) => {
                2u8.hash(state);
                // Regex does not implement Hash, so we hash its pattern string.
                regex.as_str().hash(state);
            }
        }
    }
}

impl ByteStringPattern {
    /// Creates a new `ByteStringPattern` that matches any byte string.
    pub fn any() -> Self { ByteStringPattern::Any }

    /// Creates a new `ByteStringPattern` that matches a specific byte string.
    pub fn value(value: impl AsRef<[u8]>) -> Self {
        ByteStringPattern::Value(value.as_ref().to_vec())
    }

    /// Creates a new `ByteStringPattern` that matches the binary regex for a
    /// byte string.
    pub fn regex(regex: regex::bytes::Regex) -> Self {
        ByteStringPattern::Regex(regex)
    }
}

impl Matcher for ByteStringPattern {
    fn paths(&self, envelope: &Envelope) -> Vec<Path> {
        if let Some(bytes) = envelope.subject().as_byte_string() {
            match self {
                ByteStringPattern::Any => vec![vec![envelope.clone()]],
                ByteStringPattern::Value(value) => {
                    if &bytes == value {
                        vec![vec![envelope.clone()]]
                    } else {
                        vec![]
                    }
                }
                ByteStringPattern::Regex(regex) => {
                    if regex.is_match(&bytes) {
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

    fn compile(
        &self,
        code: &mut Vec<Instr>,
        literals: &mut Vec<Pattern>,
        captures: &mut Vec<String>,
    ) {
        compile_as_atomic(
            &Pattern::Leaf(LeafPattern::ByteString(self.clone())),
            code,
            literals,
            captures,
        );
    }
}

impl std::fmt::Display for ByteStringPattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ByteStringPattern::Any => write!(f, "BSTR"),
            ByteStringPattern::Value(value) => {
                write!(f, "BSTR(h'{}')", hex::encode(value))
            }
            ByteStringPattern::Regex(regex) => {
                write!(f, "BSTR(/{}/)", regex.as_str())
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
    fn test_byte_string_pattern_any() {
        let bytes = vec![1, 2, 3, 4];
        let envelope = Envelope::new(CBOR::to_byte_string(bytes));
        let pattern = ByteStringPattern::any();
        let paths = pattern.paths(&envelope);
        assert_eq!(paths.len(), 1);
        assert_eq!(paths[0], vec![envelope.clone()]);

        // Test with non-byte-string envelope
        let text_envelope = Envelope::new("test");
        let paths = pattern.paths(&text_envelope);
        assert!(paths.is_empty());
    }

    #[test]
    fn test_byte_string_pattern_exact() {
        let bytes = vec![1, 2, 3, 4];
        let envelope = Envelope::new(CBOR::to_byte_string(bytes.clone()));
        let pattern = ByteStringPattern::value(bytes.clone());
        let paths = pattern.paths(&envelope);
        assert_eq!(paths.len(), 1);
        assert_eq!(paths[0], vec![envelope.clone()]);

        // Test with different byte string
        let different_bytes = vec![5, 6, 7, 8];
        let pattern = ByteStringPattern::value(different_bytes);
        let paths = pattern.paths(&envelope);
        assert!(paths.is_empty());
    }

    #[test]
    fn test_byte_string_pattern_binary_regex() {
        let bytes = vec![0x48, 0x65, 0x6c, 0x6c, 0x6f]; // "Hello"
        let envelope = Envelope::new(CBOR::to_byte_string(bytes.clone()));

        // Test matching regex
        let regex = regex::bytes::Regex::new(r"^He.*o$").unwrap();
        let pattern = ByteStringPattern::regex(regex);
        let paths = pattern.paths(&envelope);
        assert_eq!(paths.len(), 1);
        assert_eq!(paths[0], vec![envelope.clone()]);

        // Test non-matching regex
        let regex = regex::bytes::Regex::new(r"^World").unwrap();
        let pattern = ByteStringPattern::regex(regex);
        let paths = pattern.paths(&envelope);
        assert!(paths.is_empty());

        // Test with non-byte-string envelope
        let text_envelope = Envelope::new("test");
        let regex = regex::bytes::Regex::new(r".*").unwrap();
        let pattern = ByteStringPattern::regex(regex);
        let paths = pattern.paths(&text_envelope);
        assert!(paths.is_empty());
    }

    #[test]
    fn test_byte_string_pattern_display() {
        assert_eq!(ByteStringPattern::any().to_string(), "BSTR");
        assert_eq!(
            ByteStringPattern::value(vec![1, 2, 3]).to_string(),
            r#"BSTR(h'010203')"#
        );
        let regex = regex::bytes::Regex::new(r"^\d+$").unwrap();
        assert_eq!(
            ByteStringPattern::regex(regex).to_string(),
            r"BSTR(/^\d+$/)"
        );
    }
}
