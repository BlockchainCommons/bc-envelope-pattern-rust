use std::{collections::HashMap, ops::RangeBounds};

use bc_envelope::Envelope;
use dcbor_pattern::Matcher as DcborMatcher;

use crate::{
    Interval, Pattern,
    pattern::{Matcher, Path, compile_as_atomic, leaf::LeafPattern, vm::Instr},
};

/// Pattern for matching arrays.
/// This is now a proxy that delegates to dcbor-pattern for array matching.
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ArrayPattern {
    /// Matches any array using dcbor-pattern's [*] syntax.
    Any,
    /// Matches arrays with a specific count of elements using dcbor-pattern's
    /// [{n}] syntax.
    Count(usize),
    /// Matches arrays with a count range using dcbor-pattern's [{n,m}] or
    /// [{n,}] syntax.
    Range(Interval),
    /// Matches arrays with content using dcbor-pattern's [pattern] syntax.
    Content(dcbor_pattern::Pattern),
}

impl ArrayPattern {
    /// Creates a new `ArrayPattern` that matches any array.
    pub fn any() -> Self { ArrayPattern::Any }

    /// Creates a new `ArrayPattern` that matches arrays with a count
    /// of elements in the specified range.
    pub fn interval(interval: impl RangeBounds<usize>) -> Self {
        ArrayPattern::Range(Interval::new(interval))
    }

    /// Creates a new `ArrayPattern` that matches arrays with exact count.
    pub fn count(n: usize) -> Self { ArrayPattern::Count(n) }

    /// Creates a new `ArrayPattern` from a dcbor-pattern.
    pub fn from_dcbor_pattern(pattern: dcbor_pattern::Pattern) -> Self {
        ArrayPattern::Content(pattern)
    }

    /// Creates a new `ArrayPattern` from a dcbor-pattern ArrayPattern.
    pub fn from_dcbor_array_pattern(array_pattern: dcbor_pattern::ArrayPattern) -> Self {
        ArrayPattern::Content(dcbor_pattern::Pattern::Structure(
            dcbor_pattern::StructurePattern::Array(array_pattern)
        ))
    }
}

impl std::hash::Hash for ArrayPattern {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self {
            ArrayPattern::Any => {
                0u8.hash(state);
            }
            ArrayPattern::Count(n) => {
                1u8.hash(state);
                n.hash(state);
            }
            ArrayPattern::Range(range) => {
                2u8.hash(state);
                range.hash(state);
            }
            ArrayPattern::Content(pattern) => {
                3u8.hash(state);
                // Hash the string representation since dcbor_pattern::Pattern
                // doesn't implement Hash
                pattern.to_string().hash(state);
            }
        }
    }
}

impl Matcher for ArrayPattern {
    fn paths_with_captures(
        &self,
        envelope: &Envelope,
    ) -> (Vec<Path>, HashMap<String, Vec<Path>>) {
        let paths = if let Some(cbor_value) = envelope.subject().as_leaf() {
            // Convert the envelope's CBOR value to dcbor format for pattern
            // matching
            let dcbor_pattern = match self {
                ArrayPattern::Any => {
                    // Use dcbor-pattern's [*] syntax
                    match dcbor_pattern::Pattern::parse("[*]") {
                        Ok(pattern) => pattern,
                        Err(_) => return (vec![], HashMap::new()),
                    }
                }
                ArrayPattern::Count(n) => {
                    // Use dcbor-pattern's [{n}] syntax
                    let pattern_str = format!("[{{{}}}]", n);
                    match dcbor_pattern::Pattern::parse(&pattern_str) {
                        Ok(pattern) => pattern,
                        Err(_) => return (vec![], HashMap::new()),
                    }
                }
                ArrayPattern::Range(range) => {
                    // Use dcbor-pattern's [{n,m}] or [{n,}] syntax
                    let pattern_str = if let Some(max) = range.max() {
                        format!("[{{{},{}}}]", range.min(), max)
                    } else {
                        format!("[{{{},}}]", range.min())
                    };
                    match dcbor_pattern::Pattern::parse(&pattern_str) {
                        Ok(pattern) => pattern,
                        Err(_) => return (vec![], HashMap::new()),
                    }
                }
                ArrayPattern::Content(pattern) => pattern.clone(),
            };

            // Use dcbor-pattern to match against the CBOR value
            if dcbor_pattern.matches(&cbor_value) {
                vec![vec![envelope.clone()]]
            } else {
                vec![]
            }
        } else {
            vec![]
        };
        (paths, HashMap::new())
    }

    fn compile(
        &self,
        code: &mut Vec<Instr>,
        literals: &mut Vec<Pattern>,
        captures: &mut Vec<String>,
    ) {
        compile_as_atomic(
            &Pattern::Leaf(LeafPattern::Array(self.clone())),
            code,
            literals,
            captures,
        );
    }
}

impl std::fmt::Display for ArrayPattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ArrayPattern::Any => write!(f, "[*]"),
            ArrayPattern::Count(n) => write!(f, "[{{{}}}]", n),
            ArrayPattern::Range(range) => {
                if let Some(max) = range.max() {
                    write!(f, "[{{{},{}}}]", range.min(), max)
                } else {
                    write!(f, "[{{{},}}]", range.min())
                }
            }
            ArrayPattern::Content(pattern) => {
                // Extract the inner content from the dcbor pattern
                let pattern_str = pattern.to_string();
                if pattern_str.starts_with('[') && pattern_str.ends_with(']') {
                    // Use the inner content without the brackets
                    let inner = &pattern_str[1..pattern_str.len() - 1];
                    write!(f, "[{}]", inner)
                } else {
                    // Fallback: wrap the pattern in brackets
                    write!(f, "[{}]", pattern_str)
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
        let pattern = ArrayPattern::count(3);
        let paths = pattern.paths(&envelope);
        assert_eq!(paths.len(), 1);

        // Test count mismatch
        let pattern = ArrayPattern::count(5);
        let paths = pattern.paths(&envelope);
        assert!(paths.is_empty());
    }

    #[test]
    fn test_array_pattern_range() {
        // Create a CBOR array directly
        let cbor_array = vec![1, 2, 3].to_cbor();
        let envelope = Envelope::new(cbor_array);

        // Test count range
        let pattern = ArrayPattern::interval(2..=4);
        let paths = pattern.paths(&envelope);
        assert_eq!(paths.len(), 1);

        // Test count mismatch
        let pattern = ArrayPattern::interval(5..=10);
        let paths = pattern.paths(&envelope);
        assert!(paths.is_empty());
    }

    #[test]
    fn test_array_pattern_display() {
        assert_eq!(ArrayPattern::any().to_string(), "[*]");
        assert_eq!(ArrayPattern::count(3).to_string(), "[{3}]");
        assert_eq!(ArrayPattern::interval(2..=5).to_string(), "[{2,5}]");
        assert_eq!(ArrayPattern::interval(3..).to_string(), "[{3,}]");
    }
}
