use std::collections::HashMap;

use bc_envelope::prelude::*;

use crate::{
    Pattern,
    pattern::{Matcher, Path, compile_as_atomic, leaf::LeafPattern, vm::Instr},
};

/// Pattern for matching byte string values. This is a wrapper around
/// dcbor_pattern::ByteStringPattern that provides envelope-specific
/// integration.
#[derive(Debug, Clone)]
pub struct ByteStringPattern(dcbor_pattern::ByteStringPattern);

// Re-export the dcbor-pattern ByteStringPattern methods through associated
// functions
impl ByteStringPattern {
    /// Creates a new `ByteStringPattern` that matches any byte string.
    pub fn any() -> Self {
        Self(dcbor_pattern::ByteStringPattern::any())
    }

    /// Creates a new `ByteStringPattern` that matches a specific byte string.
    pub fn value(value: impl AsRef<[u8]>) -> Self {
        Self(dcbor_pattern::ByteStringPattern::value(value))
    }

    /// Creates a new `ByteStringPattern` that matches the binary regex for a
    /// byte string.
    pub fn regex(regex: regex::bytes::Regex) -> Self {
        Self(dcbor_pattern::ByteStringPattern::regex(regex))
    }

    /// Creates a new `ByteStringPattern` from a dcbor-pattern ByteStringPattern.
    pub fn from_dcbor_pattern(dcbor_pattern: dcbor_pattern::ByteStringPattern) -> Self {
        Self(dcbor_pattern)
    }
}

impl Matcher for ByteStringPattern {
    fn paths_with_captures(
        &self,
        envelope: &Envelope,
    ) -> (Vec<Path>, HashMap<String, Vec<Path>>) {
        // Try to extract CBOR from the envelope using the existing as_leaf()
        // method
        if let Some(cbor) = envelope.subject().as_leaf() {
            // Delegate to dcbor-pattern for CBOR matching using paths() method
            // ByteStringPattern doesn't support captures, so we only get paths
            let dcbor_paths = dcbor_pattern::Matcher::paths(&self.0, &cbor);

            // For simple leaf patterns, if dcbor-pattern found matches, return
            // the envelope
            if !dcbor_paths.is_empty() {
                let envelope_paths = vec![vec![envelope.clone()]];
                let envelope_captures = HashMap::new(); // No captures for simple byte string patterns
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
            &Pattern::Leaf(LeafPattern::ByteString(self.clone())),
            code,
            literals,
            captures,
        );
    }
}

impl std::fmt::Display for ByteStringPattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl PartialEq for ByteStringPattern {
    fn eq(&self, other: &Self) -> bool { self.0 == other.0 }
}

impl Eq for ByteStringPattern {}

impl std::hash::Hash for ByteStringPattern {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

#[cfg(test)]
mod tests {
    use dcbor_parse::parse_dcbor_item;

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
        assert_eq!(ByteStringPattern::any().to_string(), "bstr");
        assert_eq!(
            ByteStringPattern::value(vec![1, 2, 3]).to_string(),
            r#"h'010203'"#
        );
        let regex = regex::bytes::Regex::new(r"^\d+$").unwrap();
        assert_eq!(
            ByteStringPattern::regex(regex).to_string(),
            r"h'/^\d+$/'"
        );
    }

    #[test]
    fn test_byte_string_pattern_dcbor_integration() {
        // Test that the dcbor-pattern integration works correctly
        let bytes = vec![0x48, 0x65, 0x6c, 0x6c, 0x6f]; // "Hello"
        let bstr_envelope = Envelope::new(CBOR::to_byte_string(bytes.clone()));
        let text_envelope = Envelope::new("Hello");
        let number_envelope = Envelope::new(42);

        // Test any pattern
        let any_pattern = ByteStringPattern::any();
        assert!(any_pattern.matches(&bstr_envelope));
        assert!(!any_pattern.matches(&text_envelope)); // Should not match text
        assert!(!any_pattern.matches(&number_envelope)); // Should not match number

        // Test exact value patterns
        let exact_pattern = ByteStringPattern::value(bytes.clone());
        assert!(exact_pattern.matches(&bstr_envelope));
        assert!(!exact_pattern.matches(&text_envelope));
        assert!(!exact_pattern.matches(&number_envelope));

        let different_pattern = ByteStringPattern::value(vec![1, 2, 3]);
        assert!(!different_pattern.matches(&bstr_envelope));

        // Test regex patterns
        let alpha_regex = regex::bytes::Regex::new(r"^[A-Za-z]+$").unwrap();
        let alpha_pattern = ByteStringPattern::regex(alpha_regex);
        assert!(alpha_pattern.matches(&bstr_envelope));
        assert!(!alpha_pattern.matches(&text_envelope));
        assert!(!alpha_pattern.matches(&number_envelope));

        // Test paths
        let paths = exact_pattern.paths(&bstr_envelope);
        assert_eq!(paths.len(), 1);
        assert_eq!(paths[0], vec![bstr_envelope.clone()]);

        let no_paths = exact_pattern.paths(&text_envelope);
        assert_eq!(no_paths.len(), 0);
    }

    #[test]
    fn test_byte_string_pattern_paths_with_captures() {
        let bytes = vec![0x48, 0x65, 0x6c, 0x6c, 0x6f]; // "Hello"
        let bstr_envelope = Envelope::new(CBOR::to_byte_string(bytes.clone()));
        let pattern = ByteStringPattern::value(bytes);

        let (paths, captures) = pattern.paths_with_captures(&bstr_envelope);
        assert_eq!(paths.len(), 1);
        assert_eq!(paths[0], vec![bstr_envelope.clone()]);
        assert_eq!(captures.len(), 0); // No captures for simple byte string patterns
    }

    #[test]
    fn test_byte_string_pattern_with_non_bstr_envelope() {
        // Test with envelope that doesn't contain a byte string
        let envelope = Envelope::new_assertion("key", "value");
        let pattern = ByteStringPattern::any();

        let paths = pattern.paths(&envelope);
        assert_eq!(paths.len(), 0); // Should not match non-byte-string envelopes
    }

    #[test]
    fn test_byte_string_pattern_with_direct_cbor_values() {
        // Test that our pattern works with CBOR byte string values
        let bytes_cbor = parse_dcbor_item("h'48656c6c6f'").unwrap(); // "Hello" in hex
        let text_cbor = parse_dcbor_item("\"Hello\"").unwrap();

        let bytes_envelope = Envelope::new(bytes_cbor);
        let text_envelope = Envelope::new(text_cbor);

        let any_pattern = ByteStringPattern::any();
        assert!(any_pattern.matches(&bytes_envelope));
        assert!(!any_pattern.matches(&text_envelope));

        let hello_pattern = ByteStringPattern::value(b"Hello");
        assert!(hello_pattern.matches(&bytes_envelope));
        assert!(!hello_pattern.matches(&text_envelope));

        let regex_pattern = ByteStringPattern::regex(
            regex::bytes::Regex::new(r"^He.*o$").unwrap(),
        );
        assert!(regex_pattern.matches(&bytes_envelope));
        assert!(!regex_pattern.matches(&text_envelope));
    }

    #[test]
    fn test_byte_string_pattern_binary_data() {
        // Test with actual binary data (not text)
        let binary_data = vec![0x00, 0x01, 0x02, 0x03, 0xFF, 0xFE, 0xFD];
        let binary_envelope =
            Envelope::new(CBOR::to_byte_string(binary_data.clone()));

        let any_pattern = ByteStringPattern::any();
        assert!(any_pattern.matches(&binary_envelope));

        let exact_pattern = ByteStringPattern::value(binary_data.clone());
        assert!(exact_pattern.matches(&binary_envelope));

        let different_pattern =
            ByteStringPattern::value(vec![0x00, 0x01, 0x02]);
        assert!(!different_pattern.matches(&binary_envelope));

        // Test regex that matches any bytes starting with 0x00
        let starts_with_zero_regex =
            regex::bytes::Regex::new(r"^\x00").unwrap();
        let starts_with_zero_pattern =
            ByteStringPattern::regex(starts_with_zero_regex);
        assert!(starts_with_zero_pattern.matches(&binary_envelope));

        // Test regex that doesn't match
        let starts_with_one_regex = regex::bytes::Regex::new(r"^\x01").unwrap();
        let starts_with_one_pattern =
            ByteStringPattern::regex(starts_with_one_regex);
        assert!(!starts_with_one_pattern.matches(&binary_envelope));
    }
}
