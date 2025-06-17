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
}

impl std::fmt::Display for Repeat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match (self.min, self.max) {
            (min, Some(max)) if min == max => write!(f, "{{{}}}", min),
            (min, Some(max)) => write!(f, "{{{},{}}}", min, max),
            (min, None) => write!(f, "{{{},}}", min),
        }
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
        write!(f, "{}{}", self.repeat, self.mode.modifier())
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
    }

    #[test]
    fn test_repeat_range_display() {
        let repeat_range = RepeatRange::new(1..=5, Greediness::Greedy);
        assert_eq!(format!("{}", repeat_range), "{1,5}");

        let repeat_range = RepeatRange::new(3..=3, Greediness::Lazy);
        assert_eq!(format!("{}", repeat_range), "{3}?");

        let repeat_range = RepeatRange::new(2.., Greediness::Possessive);
        assert_eq!(format!("{}", repeat_range), "{2,}+");
    }
}
