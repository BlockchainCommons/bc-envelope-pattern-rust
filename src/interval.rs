use std::ops::{Bound, RangeBounds};

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct Interval {
    min: usize,
    max: Option<usize>, // None == unbounded
}

impl Interval {
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

    pub fn min(&self) -> usize { self.min }

    pub fn max(&self) -> Option<usize> { self.max }

    pub fn contains(&self, count: usize) -> bool {
        count >= self.min && (self.max.is_none() || count <= self.max.unwrap())
    }

    pub fn is_single(&self) -> bool { Some(self.min) == self.max }

    pub fn is_unbounded(&self) -> bool { self.max.is_none() }

    pub fn range_notation(&self) -> String {
        match (self.min, self.max) {
            (min, Some(max)) if min == max => format!("{{{}}}", min),
            (min, Some(max)) => format!("{{{},{}}}", min, max),
            (min, None) => format!("{{{},}}", min),
        }
    }

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

impl RangeBounds<usize> for Interval {
    fn start_bound(&self) -> Bound<&usize> {
        if self.min == 0 {
            Bound::Unbounded
        } else {
            Bound::Included(&self.min)
        }
    }

    fn end_bound(&self) -> Bound<&usize> {
        match self.max {
            Some(ref max) => Bound::Included(max),
            None => Bound::Unbounded,
        }
    }
}

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
