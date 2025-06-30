use std::collections::HashMap;

use bc_envelope::Envelope;

use crate::{
    Pattern,
    pattern::{Matcher, Path, compile_as_atomic, leaf::LeafPattern, vm::Instr},
};

/// Pattern for matching text values. This is a wrapper around
/// dcbor_pattern::TextPattern that provides envelope-specific integration.
#[derive(Debug, Clone)]
pub struct TextPattern {
    inner: dcbor_pattern::TextPattern,
}

// Re-export the dcbor-pattern TextPattern enum variants through associated
// functions
impl TextPattern {
    /// Creates a new `TextPattern` that matches any text.
    pub fn any() -> Self { Self { inner: dcbor_pattern::TextPattern::any() } }

    /// Creates a new `TextPattern` that matches the specific text.
    pub fn value<T: Into<String>>(value: T) -> Self {
        Self { inner: dcbor_pattern::TextPattern::value(value) }
    }

    /// Creates a new `TextPattern` that matches the regex for a text.
    pub fn regex(regex: regex::Regex) -> Self {
        Self { inner: dcbor_pattern::TextPattern::regex(regex) }
    }
}

impl PartialEq for TextPattern {
    fn eq(&self, other: &Self) -> bool { self.inner == other.inner }
}

impl Eq for TextPattern {}

impl std::hash::Hash for TextPattern {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.inner.hash(state);
    }
}

impl Matcher for TextPattern {
    fn paths_with_captures(
        &self,
        envelope: &Envelope,
    ) -> (Vec<Path>, HashMap<String, Vec<Path>>) {
        // Try to extract CBOR from the envelope using the existing as_leaf()
        // method
        if let Some(cbor) = envelope.subject().as_leaf() {
            // Delegate to dcbor-pattern for CBOR matching using paths() method
            // TextPattern doesn't support captures, so we only get paths
            let dcbor_paths = dcbor_pattern::Matcher::paths(&self.inner, &cbor);

            // For simple leaf patterns, if dcbor-pattern found matches, return
            // the envelope
            if !dcbor_paths.is_empty() {
                let envelope_paths = vec![vec![envelope.clone()]];
                let envelope_captures = HashMap::new(); // No captures for simple text patterns
                (envelope_paths, envelope_captures)
            } else {
                (vec![], HashMap::new())
            }
        } else {
            // Not a leaf envelope, no match
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
            &Pattern::Leaf(LeafPattern::Text(self.clone())),
            code,
            literals,
            captures,
        );
    }
}

impl std::fmt::Display for TextPattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.inner)
    }
}

#[cfg(test)]
mod tests {
    use bc_envelope::Envelope;

    use super::*;

    #[test]
    fn test_text_pattern_display() {
        assert_eq!(TextPattern::any().to_string(), "text");
        assert_eq!(TextPattern::value("Hello").to_string(), r#""Hello""#);
        assert_eq!(
            TextPattern::regex(regex::Regex::new(r"^\d+$").unwrap())
                .to_string(),
            r#"/^\d+$/"#
        );
    }

    #[test]
    fn test_text_pattern_dcbor_integration() {
        // Test that the dcbor-pattern integration works correctly
        let hello_envelope = Envelope::new("Hello");
        let world_envelope = Envelope::new("World");
        let number_envelope = Envelope::new(42);

        // Test any pattern
        let any_pattern = TextPattern::any();
        assert!(any_pattern.matches(&hello_envelope));
        assert!(any_pattern.matches(&world_envelope));
        assert!(!any_pattern.matches(&number_envelope)); // Should not match number

        // Test exact value pattern
        let hello_pattern = TextPattern::value("Hello");
        assert!(hello_pattern.matches(&hello_envelope));
        assert!(!hello_pattern.matches(&world_envelope));
        assert!(!hello_pattern.matches(&number_envelope));

        // Test regex pattern
        let word_regex = regex::Regex::new(r"^[A-Za-z]+$").unwrap();
        let word_pattern = TextPattern::regex(word_regex);
        assert!(word_pattern.matches(&hello_envelope));
        assert!(word_pattern.matches(&world_envelope));
        assert!(!word_pattern.matches(&number_envelope));

        // Test paths
        let paths = hello_pattern.paths(&hello_envelope);
        assert_eq!(paths.len(), 1);
        assert_eq!(paths[0], vec![hello_envelope.clone()]);

        let no_paths = hello_pattern.paths(&world_envelope);
        assert_eq!(no_paths.len(), 0);
    }

    #[test]
    fn test_text_pattern_paths_with_captures() {
        let hello_envelope = Envelope::new("Hello");
        let pattern = TextPattern::value("Hello");

        let (paths, captures) = pattern.paths_with_captures(&hello_envelope);
        assert_eq!(paths.len(), 1);
        assert_eq!(paths[0], vec![hello_envelope.clone()]);
        assert_eq!(captures.len(), 0); // No captures for simple text patterns
    }

    #[test]
    fn test_text_pattern_with_non_text_envelope() {
        // Test with envelope that doesn't contain text
        let envelope = Envelope::new_assertion("key", "value");
        let pattern = TextPattern::any();

        let paths = pattern.paths(&envelope);
        assert_eq!(paths.len(), 0); // Should not match non-text envelopes
    }
}
