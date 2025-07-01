use std::collections::HashMap;

use bc_envelope::Envelope;

use crate::{
    Pattern,
    pattern::{Matcher, Path, compile_as_atomic, leaf::LeafPattern, vm::Instr},
};

/// Pattern for matching number values. This is a wrapper around
/// dcbor_pattern::NumberPattern that provides envelope-specific integration.
#[derive(Debug, Clone)]
pub struct NumberPattern {
    inner: dcbor_pattern::NumberPattern,
}

// Re-export the dcbor-pattern NumberPattern methods through associated
// functions
impl NumberPattern {
    /// Creates a new `NumberPattern` that matches any number.
    pub fn any() -> Self {
        Self { inner: dcbor_pattern::NumberPattern::any() }
    }

    /// Creates a new `NumberPattern` that matches the exact number.
    pub fn exact<T>(value: T) -> Self
    where
        T: Into<f64>,
    {
        Self { inner: dcbor_pattern::NumberPattern::value(value) }
    }

    /// Creates a new `NumberPattern` that matches numbers within the specified
    /// range.
    pub fn range<A>(range: std::ops::RangeInclusive<A>) -> Self
    where
        A: Into<f64> + Copy,
    {
        Self { inner: dcbor_pattern::NumberPattern::range(range) }
    }

    /// Creates a new `NumberPattern` that matches numbers greater than the
    /// specified value.
    pub fn greater_than<T>(value: T) -> Self
    where
        T: Into<f64>,
    {
        Self {
            inner: dcbor_pattern::NumberPattern::greater_than(value),
        }
    }

    /// Creates a new `NumberPattern` that matches numbers greater than or
    /// equal to the specified value.
    pub fn greater_than_or_equal<T>(value: T) -> Self
    where
        T: Into<f64>,
    {
        Self {
            inner: dcbor_pattern::NumberPattern::greater_than_or_equal(value),
        }
    }

    /// Creates a new `NumberPattern` that matches numbers less than the
    /// specified value.
    pub fn less_than<T>(value: T) -> Self
    where
        T: Into<f64>,
    {
        Self {
            inner: dcbor_pattern::NumberPattern::less_than(value),
        }
    }

    /// Creates a new `NumberPattern` that matches numbers less than or equal
    /// to the specified value.
    pub fn less_than_or_equal<T>(value: T) -> Self
    where
        T: Into<f64>,
    {
        Self {
            inner: dcbor_pattern::NumberPattern::less_than_or_equal(value),
        }
    }

    /// Creates a new `NumberPattern` that matches NaN values.
    pub fn nan() -> Self {
        Self { inner: dcbor_pattern::NumberPattern::nan() }
    }

    /// Creates a new `NumberPattern` from a dcbor-pattern NumberPattern.
    pub fn from_dcbor_pattern(
        dcbor_pattern: dcbor_pattern::NumberPattern,
    ) -> Self {
        Self { inner: dcbor_pattern }
    }
}

impl PartialEq for NumberPattern {
    fn eq(&self, other: &Self) -> bool {
        self.inner == other.inner
    }
}

impl Eq for NumberPattern {}

impl std::hash::Hash for NumberPattern {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.inner.hash(state);
    }
}

impl Matcher for NumberPattern {
    fn paths_with_captures(
        &self,
        envelope: &Envelope,
    ) -> (Vec<Path>, HashMap<String, Vec<Path>>) {
        // Try to extract CBOR from the envelope using the existing as_leaf()
        // method
        let paths = if let Some(cbor) = envelope.subject().as_leaf() {
            // Delegate to dcbor-pattern for CBOR matching using paths() method
            // NumberPattern doesn't support captures, so we only get paths
            let dcbor_paths = dcbor_pattern::Matcher::paths(&self.inner, &cbor);

            // For simple leaf patterns, if dcbor-pattern found matches, return
            // the envelope
            if !dcbor_paths.is_empty() {
                vec![vec![envelope.clone()]]
            } else {
                vec![]
            }
        } else {
            // Not a leaf envelope, no match
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
            &Pattern::Leaf(LeafPattern::Number(self.clone())),
            code,
            literals,
            captures,
        );
    }
}

impl std::fmt::Display for NumberPattern {
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
    fn test_number_pattern_display() {
        assert_eq!(NumberPattern::any().to_string(), "number");
        assert_eq!(NumberPattern::exact(42.0).to_string(), "42");
        assert_eq!(NumberPattern::range(1.0..=10.0).to_string(), "1...10");
        assert_eq!(NumberPattern::greater_than(5.0).to_string(), ">5");
        assert_eq!(
            NumberPattern::greater_than_or_equal(5.0).to_string(),
            ">=5"
        );
        assert_eq!(NumberPattern::less_than(5.0).to_string(), "<5");
        assert_eq!(NumberPattern::less_than_or_equal(5.0).to_string(), "<=5");
        assert_eq!(NumberPattern::nan().to_string(), "NaN");
    }

    #[test]
    fn test_number_pattern_dcbor_integration() {
        // Test that the dcbor-pattern integration works correctly
        let number_42_envelope = Envelope::new(42);
        let number_100_envelope = Envelope::new(100);
        let text_envelope = Envelope::new("hello");
        let float_envelope = Envelope::new(3.2222);
        let nan_envelope = Envelope::new(f64::NAN);

        // Test any pattern
        let any_pattern = NumberPattern::any();
        assert!(any_pattern.matches(&number_42_envelope));
        assert!(any_pattern.matches(&number_100_envelope));
        assert!(any_pattern.matches(&float_envelope));
        assert!(any_pattern.matches(&nan_envelope));
        assert!(!any_pattern.matches(&text_envelope)); // Should not match text

        // Test exact value pattern
        let exact_42_pattern = NumberPattern::exact(42);
        assert!(exact_42_pattern.matches(&number_42_envelope));
        assert!(!exact_42_pattern.matches(&number_100_envelope));
        assert!(!exact_42_pattern.matches(&text_envelope));

        // Test range pattern
        let range_pattern = NumberPattern::range(40..=50);
        assert!(range_pattern.matches(&number_42_envelope));
        assert!(!range_pattern.matches(&number_100_envelope));
        assert!(!range_pattern.matches(&text_envelope));

        // Test greater than pattern
        let gt_pattern = NumberPattern::greater_than(41);
        assert!(gt_pattern.matches(&number_42_envelope));
        assert!(gt_pattern.matches(&number_100_envelope));
        assert!(!gt_pattern.matches(&text_envelope));

        // Test greater than or equal pattern
        let gte_pattern = NumberPattern::greater_than_or_equal(42);
        assert!(gte_pattern.matches(&number_42_envelope));
        assert!(gte_pattern.matches(&number_100_envelope));
        assert!(!gte_pattern.matches(&text_envelope));

        // Test less than pattern
        let lt_pattern = NumberPattern::less_than(50);
        assert!(lt_pattern.matches(&number_42_envelope));
        assert!(!lt_pattern.matches(&number_100_envelope));
        assert!(!lt_pattern.matches(&text_envelope));

        // Test less than or equal pattern
        let lte_pattern = NumberPattern::less_than_or_equal(42);
        assert!(lte_pattern.matches(&number_42_envelope));
        assert!(!lte_pattern.matches(&number_100_envelope));
        assert!(!lte_pattern.matches(&text_envelope));

        // Test NaN pattern
        let nan_pattern = NumberPattern::nan();
        assert!(nan_pattern.matches(&nan_envelope));
        assert!(!nan_pattern.matches(&number_42_envelope));
        assert!(!nan_pattern.matches(&text_envelope));

        // Test paths
        let paths = exact_42_pattern.paths(&number_42_envelope);
        assert_eq!(paths.len(), 1);
        assert_eq!(paths[0], vec![number_42_envelope.clone()]);

        let no_paths = exact_42_pattern.paths(&number_100_envelope);
        assert_eq!(no_paths.len(), 0);
    }

    #[test]
    fn test_number_pattern_paths_with_captures() {
        let number_envelope = Envelope::new(42);
        let pattern = NumberPattern::exact(42);

        let (paths, captures) = pattern.paths_with_captures(&number_envelope);
        assert_eq!(paths.len(), 1);
        assert_eq!(paths[0], vec![number_envelope.clone()]);
        assert_eq!(captures.len(), 0); // No captures for simple number patterns
    }

    #[test]
    fn test_number_pattern_with_non_number_envelope() {
        // Test with envelope that doesn't contain a number
        let envelope = Envelope::new_assertion("key", "value");
        let pattern = NumberPattern::any();

        let paths = pattern.paths(&envelope);
        assert_eq!(paths.len(), 0); // Should not match non-number envelopes
    }

    #[test]
    fn test_number_pattern_with_direct_cbor_values() {
        // Test that our pattern works with various CBOR number types
        let int_cbor = parse_dcbor_item("42").unwrap();
        let float_cbor = parse_dcbor_item("3.14").unwrap();
        let negative_cbor = parse_dcbor_item("-10").unwrap();

        let int_envelope = Envelope::new(int_cbor);
        let float_envelope = Envelope::new(float_cbor);
        let negative_envelope = Envelope::new(negative_cbor);

        let any_pattern = NumberPattern::any();
        assert!(any_pattern.matches(&int_envelope));
        assert!(any_pattern.matches(&float_envelope));
        assert!(any_pattern.matches(&negative_envelope));

        let range_pattern = NumberPattern::range(-20..=50);
        assert!(range_pattern.matches(&int_envelope));
        assert!(range_pattern.matches(&float_envelope)); // 3.14 is within [-20, 50]
        assert!(range_pattern.matches(&negative_envelope));

        // Test a more restrictive range
        let narrow_range_pattern = NumberPattern::range(40..=45);
        assert!(narrow_range_pattern.matches(&int_envelope)); // 42 is in [40, 45]
        assert!(!narrow_range_pattern.matches(&float_envelope)); // 3.14 is not in [40, 45]
        assert!(!narrow_range_pattern.matches(&negative_envelope)); // -10 is not in [40, 45]
    }
}
