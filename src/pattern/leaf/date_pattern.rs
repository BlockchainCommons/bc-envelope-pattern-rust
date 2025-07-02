use std::{collections::HashMap, ops::RangeInclusive};

use bc_envelope::prelude::*;

use crate::{
    Pattern,
    pattern::{Matcher, Path, compile_as_atomic, leaf::LeafPattern, vm::Instr},
};

/// Pattern for matching dates. This is a wrapper around
/// dcbor_pattern::DatePattern that provides envelope-specific integration.
#[derive(Debug, Clone)]
pub struct DatePattern(dcbor_pattern::DatePattern);

impl PartialEq for DatePattern {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Eq for DatePattern {}

impl std::hash::Hash for DatePattern {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

// Re-export the dcbor-pattern DatePattern methods through associated
// functions
impl DatePattern {
    /// Creates a new `DatePattern` that matches any date.
    pub fn any() -> Self {
        Self(dcbor_pattern::DatePattern::any())
    }

    /// Creates a new `DatePattern` that matches a specific date.
    pub fn value(date: Date) -> Self {
        Self(dcbor_pattern::DatePattern::value(date))
    }

    /// Creates a new `DatePattern` that matches dates within a range
    /// (inclusive).
    pub fn range(range: RangeInclusive<Date>) -> Self {
        Self(dcbor_pattern::DatePattern::range(range))
    }

    /// Creates a new `DatePattern` that matches dates that are on or after the
    /// specified date.
    pub fn earliest(date: Date) -> Self {
        Self(dcbor_pattern::DatePattern::earliest(date))
    }

    /// Creates a new `DatePattern` that matches dates that are on or before the
    /// specified date.
    pub fn latest(date: Date) -> Self {
        Self(dcbor_pattern::DatePattern::latest(date))
    }

    /// Creates a new `DatePattern` that matches a date by its ISO-8601 string
    /// representation.
    pub fn string(iso_string: impl Into<String>) -> Self {
        Self(dcbor_pattern::DatePattern::string(iso_string))
    }

    /// Creates a new `DatePattern` that matches dates whose ISO-8601 string
    /// representation matches the given regex pattern.
    pub fn regex(regex: regex::Regex) -> Self {
        Self(dcbor_pattern::DatePattern::regex(regex))
    }

    /// Creates a new `DatePattern` from a dcbor-pattern DatePattern.
    pub fn from_dcbor_pattern(
        dcbor_pattern: dcbor_pattern::DatePattern,
    ) -> Self {
        Self(dcbor_pattern)
    }
}

impl Matcher for DatePattern {
    fn paths_with_captures(
        &self,
        envelope: &Envelope,
    ) -> (Vec<Path>, HashMap<String, Vec<Path>>) {
        // Try to extract CBOR from the envelope using the existing as_leaf()
        // method
        if let Some(cbor) = envelope.subject().as_leaf() {
            // Delegate to dcbor-pattern for CBOR matching using paths() method
            // DatePattern doesn't support captures, so we only get paths
            let dcbor_paths = dcbor_pattern::Matcher::paths(&self.0, &cbor);

            // For simple leaf patterns, if dcbor-pattern found matches, return
            // the envelope
            if !dcbor_paths.is_empty() {
                let envelope_paths = vec![vec![envelope.clone()]];
                let envelope_captures = HashMap::new(); // No captures for simple date patterns
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
            &Pattern::Leaf(LeafPattern::Date(self.clone())),
            code,
            literals,
            captures,
        );
    }
}

impl std::fmt::Display for DatePattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use dcbor_parse::parse_dcbor_item;

    use super::*;

    #[test]
    fn test_date_pattern_any() {
        // Create a date envelope
        let date = Date::from_ymd(2023, 12, 25);
        let envelope = Envelope::new(date.clone());

        let pattern = DatePattern::any();
        let paths = pattern.paths(&envelope);
        assert_eq!(paths.len(), 1);
        assert_eq!(paths[0], vec![envelope.clone()]);

        // Test with non-date envelope
        let text_envelope = Envelope::new("test");
        let paths = pattern.paths(&text_envelope);
        assert!(paths.is_empty());
    }

    #[test]
    fn test_date_pattern_specific() {
        // Create a date envelope
        let date = Date::from_ymd(2023, 12, 25);
        let envelope = Envelope::new(date.clone());

        // Test matching date
        let pattern = DatePattern::value(date.clone());
        let paths = pattern.paths(&envelope);
        assert_eq!(paths.len(), 1);

        // Test non-matching date
        let different_date = Date::from_ymd(2023, 12, 24);
        let pattern = DatePattern::value(different_date);
        let paths = pattern.paths(&envelope);
        assert!(paths.is_empty());
    }

    #[test]
    fn test_date_pattern_range() {
        let date = Date::from_ymd(2023, 12, 25);
        let envelope = Envelope::new(date.clone());

        // Test date within range
        let start = Date::from_ymd(2023, 12, 20);
        let end = Date::from_ymd(2023, 12, 30);
        let pattern = DatePattern::range(start..=end);
        let paths = pattern.paths(&envelope);
        assert_eq!(paths.len(), 1);

        // Test date outside range
        let start = Date::from_ymd(2023, 12, 26);
        let end = Date::from_ymd(2023, 12, 30);
        let pattern = DatePattern::range(start..=end);
        let paths = pattern.paths(&envelope);
        assert!(paths.is_empty());

        // Test range boundaries (inclusive)
        let start = Date::from_ymd(2023, 12, 25);
        let end = Date::from_ymd(2023, 12, 25);
        let pattern = DatePattern::range(start..=end);
        let paths = pattern.paths(&envelope);
        assert_eq!(paths.len(), 1);
    }

    #[test]
    fn test_date_pattern_iso8601() {
        // Test date-only string (midnight)
        let date = Date::from_ymd(2023, 12, 25);
        let envelope = Envelope::new(date.clone());

        let pattern = DatePattern::string("2023-12-25");
        let paths = pattern.paths(&envelope);
        assert_eq!(paths.len(), 1);

        // Test non-matching string
        let pattern = DatePattern::string("2023-12-24");
        let paths = pattern.paths(&envelope);
        assert!(paths.is_empty());

        // Test date with time
        let date_with_time = Date::from_ymd_hms(2023, 12, 25, 15, 30, 45);
        let envelope_with_time = Envelope::new(date_with_time.clone());

        let pattern = DatePattern::string("2023-12-25T15:30:45Z");
        let paths = pattern.paths(&envelope_with_time);
        assert_eq!(paths.len(), 1);
    }

    #[test]
    fn test_date_pattern_regex() {
        let date = Date::from_ymd(2023, 12, 25);
        let envelope = Envelope::new(date.clone());

        // Test regex that matches year 2023
        let regex = regex::Regex::new(r"^2023-.*").unwrap();
        let pattern = DatePattern::regex(regex);
        let paths = pattern.paths(&envelope);
        assert_eq!(paths.len(), 1);

        // Test regex that matches December
        let regex = regex::Regex::new(r".*-12-.*").unwrap();
        let pattern = DatePattern::regex(regex);
        let paths = pattern.paths(&envelope);
        assert_eq!(paths.len(), 1);

        // Test regex that doesn't match
        let regex = regex::Regex::new(r"^2024-.*").unwrap();
        let pattern = DatePattern::regex(regex);
        let paths = pattern.paths(&envelope);
        assert!(paths.is_empty());

        // Test regex with date-time
        let date_with_time = Date::from_ymd_hms(2023, 12, 25, 15, 30, 45);
        let envelope_with_time = Envelope::new(date_with_time.clone());

        let regex = regex::Regex::new(r".*T15:30:45Z$").unwrap();
        let pattern = DatePattern::regex(regex);
        let paths = pattern.paths(&envelope_with_time);
        assert_eq!(paths.len(), 1);
    }

    #[test]
    fn test_date_pattern_with_non_date_tagged_cbor() {
        // Create a non-date tagged CBOR value (e.g., tag 100)
        let tagged_cbor = CBOR::to_tagged_value(100, "not a date");
        let envelope = Envelope::new(tagged_cbor);

        // Should not match any date pattern
        let pattern = DatePattern::any();
        let paths = pattern.paths(&envelope);
        assert!(paths.is_empty());
    }

    #[test]
    fn test_date_pattern_display() {
        let pattern = DatePattern::any();
        assert_eq!(pattern.to_string(), "date");

        let pattern = DatePattern::value(Date::from_ymd(2023, 12, 25));
        assert_eq!(pattern.to_string(), "date'2023-12-25'");

        let pattern = DatePattern::range(
            Date::from_ymd(2023, 12, 20)..=Date::from_ymd(2023, 12, 30),
        );
        assert_eq!(pattern.to_string(), "date'2023-12-20...2023-12-30'");

        let pattern = DatePattern::earliest(Date::from_ymd(2023, 12, 25));
        assert_eq!(pattern.to_string(), "date'2023-12-25...'");

        let pattern = DatePattern::latest(Date::from_ymd(2023, 12, 25));
        assert_eq!(pattern.to_string(), "date'...2023-12-25'");

        let pattern = DatePattern::string("2023-12-25");
        assert_eq!(pattern.to_string(), "date'2023-12-25'");

        let pattern =
            DatePattern::regex(regex::Regex::new(r"^2023-.*").unwrap());
        assert_eq!(pattern.to_string(), "date'/^2023-.*/'");
    }

    #[test]
    fn test_date_pattern_dcbor_integration() {
        // Test that the dcbor-pattern integration works correctly
        let date = Date::from_ymd(2023, 12, 25);
        let date_envelope = Envelope::new(date.clone());
        let text_envelope = Envelope::new("2023-12-25");
        let number_envelope = Envelope::new(42);

        // Test any pattern
        let any_pattern = DatePattern::any();
        assert!(any_pattern.matches(&date_envelope));
        assert!(!any_pattern.matches(&text_envelope)); // Should not match text
        assert!(!any_pattern.matches(&number_envelope)); // Should not match number

        // Test exact value patterns
        let exact_pattern = DatePattern::value(date.clone());
        assert!(exact_pattern.matches(&date_envelope));
        assert!(!exact_pattern.matches(&text_envelope));
        assert!(!exact_pattern.matches(&number_envelope));

        let different_pattern =
            DatePattern::value(Date::from_ymd(2023, 12, 24));
        assert!(!different_pattern.matches(&date_envelope));

        // Test range patterns
        let start = Date::from_ymd(2023, 12, 20);
        let end = Date::from_ymd(2023, 12, 30);
        let range_pattern = DatePattern::range(start..=end);
        assert!(range_pattern.matches(&date_envelope));
        assert!(!range_pattern.matches(&text_envelope));
        assert!(!range_pattern.matches(&number_envelope));

        // Test ISO8601 patterns
        let iso_pattern = DatePattern::string("2023-12-25");
        assert!(iso_pattern.matches(&date_envelope));
        assert!(!iso_pattern.matches(&text_envelope));

        // Test regex patterns
        let regex_pattern =
            DatePattern::regex(regex::Regex::new(r"^2023-.*").unwrap());
        assert!(regex_pattern.matches(&date_envelope));
        assert!(!regex_pattern.matches(&text_envelope));

        // Test paths
        let paths = exact_pattern.paths(&date_envelope);
        assert_eq!(paths.len(), 1);
        assert_eq!(paths[0], vec![date_envelope.clone()]);

        let no_paths = exact_pattern.paths(&text_envelope);
        assert_eq!(no_paths.len(), 0);
    }

    #[test]
    fn test_date_pattern_paths_with_captures() {
        let date = Date::from_ymd(2023, 12, 25);
        let date_envelope = Envelope::new(date.clone());
        let pattern = DatePattern::value(date);

        let (paths, captures) = pattern.paths_with_captures(&date_envelope);
        assert_eq!(paths.len(), 1);
        assert_eq!(paths[0], vec![date_envelope.clone()]);
        assert_eq!(captures.len(), 0); // No captures for simple date patterns
    }

    #[test]
    fn test_date_pattern_with_non_date_envelope() {
        // Test with envelope that doesn't contain a date
        let envelope = Envelope::new_assertion("key", "value");
        let pattern = DatePattern::any();

        let paths = pattern.paths(&envelope);
        assert_eq!(paths.len(), 0); // Should not match non-date envelopes
    }

    #[test]
    fn test_date_pattern_with_direct_cbor_values() {
        // Test that our pattern works with CBOR date values
        let date_cbor = parse_dcbor_item("1(1640995200)").unwrap(); // Unix timestamp as date
        let text_cbor = parse_dcbor_item("\"2023-12-25\"").unwrap();

        let date_envelope = Envelope::new(date_cbor);
        let text_envelope = Envelope::new(text_cbor);

        let any_pattern = DatePattern::any();
        assert!(any_pattern.matches(&date_envelope));
        assert!(!any_pattern.matches(&text_envelope));

        // Test with a specific date conversion
        if let Ok(parsed_date) =
            Date::try_from(parse_dcbor_item("1(1640995200)").unwrap())
        {
            let specific_pattern = DatePattern::value(parsed_date);
            assert!(specific_pattern.matches(&date_envelope));
            assert!(!specific_pattern.matches(&text_envelope));
        }
    }

    #[test]
    fn test_date_pattern_earliest_latest() {
        let date = Date::from_ymd(2023, 12, 25);
        let envelope = Envelope::new(date.clone());

        // Test earliest pattern
        let earlier_date = Date::from_ymd(2023, 12, 20);
        let earliest_pattern = DatePattern::earliest(earlier_date);
        assert!(earliest_pattern.matches(&envelope));

        let later_date = Date::from_ymd(2023, 12, 30);
        let earliest_pattern = DatePattern::earliest(later_date);
        assert!(!earliest_pattern.matches(&envelope));

        // Test latest pattern
        let later_date = Date::from_ymd(2023, 12, 30);
        let latest_pattern = DatePattern::latest(later_date);
        assert!(latest_pattern.matches(&envelope));

        let earlier_date = Date::from_ymd(2023, 12, 20);
        let latest_pattern = DatePattern::latest(earlier_date);
        assert!(!latest_pattern.matches(&envelope));
    }
}
