use std::collections::HashMap;

use bc_envelope::Envelope;
use dcbor::prelude::*;
use known_values::KnownValue;

use crate::{
    Pattern,
    pattern::{Matcher, Path, compile_as_atomic, leaf::LeafPattern, vm::Instr},
};

/// Pattern for matching known values. This is a wrapper around
/// dcbor_pattern::KnownValuePattern that provides envelope-specific
/// integration.
#[derive(Debug, Clone)]
pub struct KnownValuePattern {
    inner: dcbor_pattern::KnownValuePattern,
}

// Re-export the dcbor-pattern KnownValuePattern methods through associated
// functions
impl KnownValuePattern {
    /// Creates a new `KnownValuePattern` that matches any known value.
    pub fn any() -> Self {
        Self { inner: dcbor_pattern::KnownValuePattern::any() }
    }

    /// Creates a new `KnownValuePattern` that matches a specific known value.
    pub fn value(value: KnownValue) -> Self {
        Self {
            inner: dcbor_pattern::KnownValuePattern::value(value),
        }
    }

    /// Creates a new `KnownValuePattern` that matches a known value by name.
    pub fn named(name: impl Into<String>) -> Self {
        Self {
            inner: dcbor_pattern::KnownValuePattern::named(name),
        }
    }

    /// Creates a new `KnownValuePattern` that matches the regex for a known
    /// value name.
    pub fn regex(regex: regex::Regex) -> Self {
        Self {
            inner: dcbor_pattern::KnownValuePattern::regex(regex),
        }
    }
}

impl PartialEq for KnownValuePattern {
    fn eq(&self, other: &Self) -> bool { self.inner == other.inner }
}

impl Eq for KnownValuePattern {}

impl std::hash::Hash for KnownValuePattern {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.inner.hash(state);
    }
}

impl Matcher for KnownValuePattern {
    fn paths(&self, envelope: &Envelope) -> Vec<Path> {
        let subject = envelope.subject();

        // Special case for KnownValue - use as_known_value() instead of
        // as_leaf()
        if let Some(known_value) = subject.as_known_value() {
            // Convert the KnownValue to its CBOR representation
            let known_value_cbor = known_value.to_cbor();

            // Delegate to dcbor-pattern for CBOR matching
            let dcbor_paths =
                dcbor_pattern::Matcher::paths(&self.inner, &known_value_cbor);

            // Convert dcbor paths to envelope paths
            // For KnownValue patterns, the dcbor pattern matches directly
            // against the CBOR, so we just need to map successful
            // matches back to the envelope
            dcbor_paths
                .into_iter()
                .map(|_dcbor_path| {
                    // For leaf patterns like KnownValue, we just return the
                    // envelope itself
                    vec![envelope.clone()]
                })
                .collect()
        } else {
            vec![]
        }
    }

    fn paths_with_captures(
        &self,
        envelope: &Envelope,
    ) -> (Vec<Path>, HashMap<String, Vec<Path>>) {
        // For now, delegate to the base implementation
        (self.paths(envelope), HashMap::new())
    }

    fn compile(
        &self,
        code: &mut Vec<Instr>,
        literals: &mut Vec<Pattern>,
        captures: &mut Vec<String>,
    ) {
        compile_as_atomic(
            &Pattern::Leaf(LeafPattern::KnownValue(self.clone())),
            code,
            literals,
            captures,
        );
    }
}

impl std::fmt::Display for KnownValuePattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Delegate to the inner pattern's Display implementation
        self.inner.fmt(f)
    }
}

#[cfg(test)]
mod tests {
    use bc_envelope::Envelope;

    use crate::{Matcher, pattern::leaf::KnownValuePattern};

    #[test]
    fn test_known_value_pattern_any() {
        use known_values::KnownValue;

        let value = KnownValue::new(1);
        let envelope = Envelope::new(value.clone());
        let pattern = KnownValuePattern::any();
        let paths = pattern.paths(&envelope);
        assert_eq!(paths.len(), 1);
        assert_eq!(paths[0], vec![envelope.clone()]);

        // Test with non-known-value envelope
        let text_envelope = Envelope::new("test");
        let paths = pattern.paths(&text_envelope);
        assert!(paths.is_empty());
    }

    #[test]
    fn test_known_value_pattern_specific() {
        let value = known_values::DATE;
        let envelope = Envelope::new(value.clone());
        let pattern = KnownValuePattern::value(value.clone());
        let paths = pattern.paths(&envelope);
        assert_eq!(paths.len(), 1);
        assert_eq!(paths[0], vec![envelope.clone()]);

        // Test with different known value
        let different_value = known_values::LANGUAGE;
        let pattern = KnownValuePattern::value(different_value);
        let paths = pattern.paths(&envelope);
        assert!(paths.is_empty());
    }

    #[test]
    fn test_known_value_pattern_named() {
        let value = known_values::DATE;
        let envelope = Envelope::new(value.clone());

        // Test matching by name
        let pattern = KnownValuePattern::named("date");
        let paths = pattern.paths(&envelope);
        assert_eq!(paths.len(), 1);
        assert_eq!(paths[0], vec![envelope.clone()]);

        // Test with non-matching name
        let pattern = KnownValuePattern::named("language");
        let paths = pattern.paths(&envelope);
        assert!(paths.is_empty());

        // Test with unknown name
        let pattern = KnownValuePattern::named("unknown_name");
        let paths = pattern.paths(&envelope);
        assert!(paths.is_empty());

        // Test with non-known-value envelope
        let text_envelope = Envelope::new("test");
        let pattern = KnownValuePattern::named("date");
        let paths = pattern.paths(&text_envelope);
        assert!(paths.is_empty());
    }

    #[test]
    fn test_known_value_pattern_regex() {
        // Test regex that matches "date"
        let value = known_values::DATE;
        let envelope = Envelope::new(value.clone());
        let regex = regex::Regex::new(r"^da.*").unwrap();
        let pattern = KnownValuePattern::regex(regex);
        let paths = pattern.paths(&envelope);
        assert_eq!(paths.len(), 1);
        assert_eq!(paths[0], vec![envelope.clone()]);

        // Test regex that matches names ending with "te"
        let regex = regex::Regex::new(r".*te$").unwrap();
        let pattern = KnownValuePattern::regex(regex);
        let paths = pattern.paths(&envelope);
        assert_eq!(paths.len(), 1);
        assert_eq!(paths[0], vec![envelope.clone()]);

        // Test regex that doesn't match
        let regex = regex::Regex::new(r"^lang.*").unwrap();
        let pattern = KnownValuePattern::regex(regex);
        let paths = pattern.paths(&envelope);
        assert!(paths.is_empty());

        // Test with non-known-value envelope
        let text_envelope = Envelope::new("test");
        let regex = regex::Regex::new(r".*").unwrap();
        let pattern = KnownValuePattern::regex(regex);
        let paths = pattern.paths(&text_envelope);
        assert!(paths.is_empty());
    }

    #[test]
    fn test_known_value_pattern_display() {
        bc_envelope::register_tags();

        let pattern = KnownValuePattern::any();
        assert_eq!(pattern.to_string(), "KNOWN");
        let pattern = KnownValuePattern::value(known_values::DATE);
        assert_eq!(pattern.to_string(), "KNOWN('date')");
        let pattern = KnownValuePattern::named("date");
        assert_eq!(pattern.to_string(), "KNOWN('date')");
        let regex = regex::Regex::new(r"^da.*").unwrap();
        let pattern = KnownValuePattern::regex(regex);
        assert_eq!(pattern.to_string(), "KNOWN(/^da.*/)");
    }

    #[test]
    fn test_known_value_pattern_dcbor_integration() {
        // Test that the dcbor-pattern integration works correctly
        let date_envelope = Envelope::new(known_values::DATE);
        let language_envelope = Envelope::new(known_values::LANGUAGE);
        let text_envelope = Envelope::new("test");

        // Test any pattern
        let any_pattern = KnownValuePattern::any();
        assert!(any_pattern.matches(&date_envelope));
        assert!(any_pattern.matches(&language_envelope));
        assert!(!any_pattern.matches(&text_envelope)); // Should not match text

        // Test exact value pattern
        let date_pattern = KnownValuePattern::value(known_values::DATE);
        assert!(date_pattern.matches(&date_envelope));
        assert!(!date_pattern.matches(&language_envelope));
        assert!(!date_pattern.matches(&text_envelope));

        // Test named pattern
        let named_date_pattern = KnownValuePattern::named("date");
        assert!(named_date_pattern.matches(&date_envelope));
        assert!(!named_date_pattern.matches(&language_envelope));
        assert!(!named_date_pattern.matches(&text_envelope));

        // Test regex pattern
        let date_regex = regex::Regex::new(r"^da.*").unwrap();
        let regex_pattern = KnownValuePattern::regex(date_regex);
        assert!(regex_pattern.matches(&date_envelope));
        assert!(!regex_pattern.matches(&language_envelope));
        assert!(!regex_pattern.matches(&text_envelope));

        // Test paths
        let paths = date_pattern.paths(&date_envelope);
        assert_eq!(paths.len(), 1);
        assert_eq!(paths[0], vec![date_envelope.clone()]);

        let no_paths = date_pattern.paths(&language_envelope);
        assert_eq!(no_paths.len(), 0);
    }
}
