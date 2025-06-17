use std::ops::{Bound, RangeBounds};

use crate::Greediness;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Repeat {
    min: usize,
    max: Option<usize>, // None == unbounded
}

impl Repeat {
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

impl std::fmt::Display for Repeat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.range_notation())
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct RepeatRange {
    pub repeat: Repeat,
    pub mode: Greediness,
}

impl RepeatRange {
    pub fn new(range: impl RangeBounds<usize>, mode: Greediness) -> Self {
        let repeat = Repeat::new(range);
        Self { repeat, mode }
    }

    pub fn is_unbounded(&self) -> bool { self.repeat.is_unbounded() }
}

impl std::fmt::Display for RepeatRange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.repeat.shorthand_notation(), self.mode.modifier())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_repeat_display() {
        let repeat = Repeat::new(1..=5);
        assert_eq!(format!("{}", repeat), "{1,5}");

        let repeat = Repeat::new(3..=3);
        assert_eq!(format!("{}", repeat), "{3}");

        let repeat = Repeat::new(2..);
        assert_eq!(format!("{}", repeat), "{2,}");

        let repeat = Repeat::new(0..);
        assert_eq!(format!("{}", repeat), "{0,}");

        let repeat = Repeat::new(1..);
        assert_eq!(format!("{}", repeat), "{1,}");

        let repeat = Repeat::new(0..=1);
        assert_eq!(format!("{}", repeat), "{0,1}");
    }

    #[test]
    fn test_repeat_range_display() {
        let repeat = RepeatRange::new(1..=5, Greediness::Greedy);
        assert_eq!(format!("{}", repeat), "{1,5}");

        let repeat = RepeatRange::new(3..=3, Greediness::Lazy);
        assert_eq!(format!("{}", repeat), "{3}?");

        let repeat = RepeatRange::new(2.., Greediness::Possessive);
        assert_eq!(format!("{}", repeat), "{2,}+");

        let repeat = RepeatRange::new(0.., Greediness::Greedy);
        assert_eq!(format!("{}", repeat), "*");

        let repeat = RepeatRange::new(0.., Greediness::Lazy);
        assert_eq!(format!("{}", repeat), "*?");

        let repeat = RepeatRange::new(0.., Greediness::Possessive);
        assert_eq!(format!("{}", repeat), "*+");

        let repeat = RepeatRange::new(1.., Greediness::Greedy);
        assert_eq!(format!("{}", repeat), "+");

        let repeat = RepeatRange::new(1.., Greediness::Lazy);
        assert_eq!(format!("{}", repeat), "+?");

        let repeat = RepeatRange::new(1.., Greediness::Possessive);
        assert_eq!(format!("{}", repeat), "++");

        let repeat = RepeatRange::new(0..=1, Greediness::Greedy);
        assert_eq!(format!("{}", repeat), "?");

        let repeat = RepeatRange::new(0..=1, Greediness::Lazy);
        assert_eq!(format!("{}", repeat), "??");

        let repeat = RepeatRange::new(0..=1, Greediness::Possessive);
        assert_eq!(format!("{}", repeat), "?+");
    }
}
