use bc_envelope::Envelope;
use known_values::{KNOWN_VALUES, KnownValue};

use crate::{
    Pattern,
    pattern::{Matcher, Path, compile_as_atomic, leaf::LeafPattern, vm::Instr},
};

/// Pattern for matching known values.
#[derive(Debug, Clone)]
pub enum KnownValuePattern {
    /// Matches any known value.
    Any,
    /// Matches the specific known value.
    Value(KnownValue),
    /// Matches the name of a known value.
    Named(String),
    /// Matches the regex for a known value name.
    Regex(regex::Regex),
}

impl PartialEq for KnownValuePattern {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (KnownValuePattern::Any, KnownValuePattern::Any) => true,
            (KnownValuePattern::Value(a), KnownValuePattern::Value(b)) => {
                a == b
            }
            (KnownValuePattern::Named(a), KnownValuePattern::Named(b)) => {
                a == b
            }
            (KnownValuePattern::Regex(a), KnownValuePattern::Regex(b)) => {
                a.as_str() == b.as_str()
            }
            _ => false,
        }
    }
}

impl Eq for KnownValuePattern {}

impl std::hash::Hash for KnownValuePattern {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self {
            KnownValuePattern::Any => {
                0u8.hash(state);
            }
            KnownValuePattern::Value(s) => {
                1u8.hash(state);
                s.hash(state);
            }
            KnownValuePattern::Named(name) => {
                2u8.hash(state);
                name.hash(state);
            }
            KnownValuePattern::Regex(regex) => {
                3u8.hash(state);
                // Regex does not implement Hash, so we hash its pattern string.
                regex.as_str().hash(state);
            }
        }
    }
}

impl KnownValuePattern {
    /// Creates a new `KnownValuePattern` that matches any known value.
    pub fn any() -> Self { KnownValuePattern::Any }

    /// Creates a new `KnownValuePattern` that matches a specific known value.
    pub fn value(value: KnownValue) -> Self { KnownValuePattern::Value(value) }

    /// Creates a new `KnownValuePattern` that matches a known value by name.
    pub fn named(name: impl Into<String>) -> Self {
        KnownValuePattern::Named(name.into())
    }

    /// Creates a new `KnownValuePattern` that matches the regex for a known
    /// value name.
    pub fn regex(regex: regex::Regex) -> Self {
        KnownValuePattern::Regex(regex)
    }
}

impl Matcher for KnownValuePattern {
    fn paths(&self, envelope: &Envelope) -> Vec<Path> {
        if let Ok(value) = envelope.extract_subject::<KnownValue>() {
            match self {
                KnownValuePattern::Any => vec![vec![envelope.clone()]],
                KnownValuePattern::Value(expected) => {
                    if value == *expected {
                        vec![vec![envelope.clone()]]
                    } else {
                        vec![]
                    }
                }
                KnownValuePattern::Named(name) => {
                    // Look up the known value by name in the global registry
                    let binding = KNOWN_VALUES.get();
                    if let Some(known_values_store) = binding.as_ref() {
                        if let Some(expected_value) =
                            known_values_store.known_value_named(name)
                        {
                            if value == *expected_value {
                                vec![vec![envelope.clone()]]
                            } else {
                                vec![]
                            }
                        } else {
                            // Name not found in registry, no match
                            vec![]
                        }
                    } else {
                        // Registry not initialized, no match
                        vec![]
                    }
                }
                KnownValuePattern::Regex(regex) => {
                    // Check if the known value's name matches the regex
                    if regex.is_match(&value.name()) {
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
            &Pattern::Leaf(LeafPattern::KnownValue(self.clone())),
            code,
            literals,
        );
    }
}

impl std::fmt::Display for KnownValuePattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            KnownValuePattern::Any => write!(f, "KNOWN"),
            KnownValuePattern::Value(value) => {
                write!(f, "KNOWN('{}')", value.name())
            }
            KnownValuePattern::Named(name) => write!(f, "KNOWN('{}')", name),
            KnownValuePattern::Regex(regex) => {
                write!(f, "KNOWN(/{}/)", regex.as_str())
            }
        }
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
}
