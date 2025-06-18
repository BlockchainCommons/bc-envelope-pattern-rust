/// Provides an `Interval` type representing a range of usize values with a
/// minimum and optional maximum.
///
/// This module is used in the context of pattern matching for Gordian
/// Envelopes to represent cardinality specifications like `{n}`, `{n,m}`,
/// or `{n,}` in pattern expressions.
use std::ops::{Bound, RangeBounds};

/// Represents an inclusive interval with a minimum value and an optional
/// maximum value.
///
/// When the maximum is `None`, the interval is considered unbounded above.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct Interval {
    min: usize,
    max: Option<usize>, // None == unbounded
}

impl Interval {
    /// Creates a new `Interval` from any type that implements
    /// `RangeBounds<usize>`.
    ///
    /// This allows creating intervals from Rust's range expressions like
    /// `1..5`, `0..=10`, or `2..`.
    pub fn new(range: impl RangeBounds<usize>) -> Self {
        let min = match range.start_bound() {
            Bound::Included(&start) => start,
            Bound::Excluded(&start) => start + 1,
            Bound::Unbounded => 0,
        };
        let max = match range.end_bound() {
            Bound::Included(&end) => Some(end),
            Bound::Excluded(&end) => Some(end - 1),
            Bound::Unbounded => None,
        };
        Self { min, max }
    }

    /// Returns the minimum value of the interval.
    pub fn min(&self) -> usize { self.min }

    /// Returns the maximum value of the interval, or `None` if the interval is
    /// unbounded.
    pub fn max(&self) -> Option<usize> { self.max }

    /// Checks if the given count falls within this interval.
    pub fn contains(&self, count: usize) -> bool {
        count >= self.min && (self.max.is_none() || count <= self.max.unwrap())
    }

    /// Checks if the interval represents a single value (i.e., min equals max).
    pub fn is_single(&self) -> bool { Some(self.min) == self.max }

    /// Checks if the interval is unbounded (i.e., has no maximum value).
    pub fn is_unbounded(&self) -> bool { self.max.is_none() }

    /// Returns a string representation of the interval using standard range
    /// notation.
    ///
    /// Examples:
    /// - `{3}` for the single value 3
    /// - `{1,5}` for the range 1 to 5 inclusive
    /// - `{2,}` for 2 or more
    pub fn range_notation(&self) -> String {
        match (self.min, self.max) {
            (min, Some(max)) if min == max => format!("{{{}}}", min),
            (min, Some(max)) => format!("{{{},{}}}", min, max),
            (min, None) => format!("{{{},}}", min),
        }
    }

    /// Returns a string representation of the interval using shorthand notation
    /// where applicable.
    ///
    /// Examples:
    /// - `?` for `{0,1}` (optional)
    /// - `*` for `{0,}` (zero or more)
    /// - `+` for `{1,}` (one or more)
    pub fn shorthand_notation(&self) -> String {
        match (self.min, self.max) {
            (0, Some(1)) => "?".to_string(),
            (min, Some(max)) if min == max => format!("{{{}}}", min),
            (min, Some(max)) => format!("{{{},{}}}", min, max),
            (0, None) => "*".to_string(),
            (1, None) => "+".to_string(),
            (min, None) => format!("{{{},}}", min),
        }
    }
}

/// Implementation of `RangeBounds<usize>` for `Interval`, allowing it to be
/// used in contexts that expect range bounds, such as slice indexing.
impl RangeBounds<usize> for Interval {
    /// Returns the start bound of the interval.
    ///
    /// - Returns `Bound::Unbounded` if `min` is 0
    /// - Returns `Bound::Included(&self.min)` otherwise
    fn start_bound(&self) -> Bound<&usize> {
        if self.min == 0 {
            Bound::Unbounded
        } else {
            Bound::Included(&self.min)
        }
    }

    /// Returns the end bound of the interval.
    ///
    /// - Returns `Bound::Included(max)` if `max` is `Some`
    /// - Returns `Bound::Unbounded` if `max` is `None`
    fn end_bound(&self) -> Bound<&usize> {
        match self.max {
            Some(ref max) => Bound::Included(max),
            None => Bound::Unbounded,
        }
    }
}

/// Implements the `Display` trait for `Interval`, using the `range_notation`
/// format.
impl std::fmt::Display for Interval {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.range_notation())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[rustfmt::skip]
    fn test_interval_display() {
        assert_eq!(format!("{}", Interval::new(1..=5)), "{1,5}");
        assert_eq!(format!("{}", Interval::new(3..=3)), "{3}");
        assert_eq!(format!("{}", Interval::new(2..)), "{2,}");
        assert_eq!(format!("{}", Interval::new(0..)), "{0,}");
        assert_eq!(format!("{}", Interval::new(1..)), "{1,}");
        assert_eq!(format!("{}", Interval::new(0..=1)), "{0,1}");
    }
}
