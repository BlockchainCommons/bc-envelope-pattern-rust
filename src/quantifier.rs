use std::ops::RangeBounds;

use crate::{Interval, Reluctance};

/// Defines how many times a pattern may or must match, with an interval and a
/// reluctance.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct Quantifier {
    interval: Interval,
    reluctance: Reluctance,
}

impl Quantifier {
    pub fn new(
        interval: impl RangeBounds<usize>,
        reluctance: Reluctance,
    ) -> Self {
        let repeat = Interval::new(interval);
        Self { interval: repeat, reluctance }
    }

    pub fn min(&self) -> usize { self.interval.min() }

    pub fn max(&self) -> Option<usize> { self.interval.max() }

    pub fn reluctance(&self) -> Reluctance { self.reluctance }

    pub fn contains(&self, count: usize) -> bool {
        self.interval.contains(count)
    }

    pub fn is_unbounded(&self) -> bool { self.interval.is_unbounded() }
}

impl Default for Quantifier {
    fn default() -> Self { Quantifier::new(1..=1, Reluctance::Greedy) }
}

impl std::fmt::Display for Quantifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}",
            self.interval.shorthand_notation(),
            self.reluctance.suffix()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[rustfmt::skip]
    fn test_quantifier_display() {
        assert_eq!(format!("{}", Quantifier::new(1..=5, Reluctance::Greedy)), "{1,5}");
        assert_eq!(format!("{}", Quantifier::new(3..=3, Reluctance::Lazy)), "{3}?");
        assert_eq!(format!("{}", Quantifier::new(2.., Reluctance::Possessive)), "{2,}+");
        assert_eq!(format!("{}", Quantifier::new(0.., Reluctance::Greedy)), "*");
        assert_eq!(format!("{}", Quantifier::new(0.., Reluctance::Lazy)), "*?");
        assert_eq!(format!("{}", Quantifier::new(0.., Reluctance::Possessive)), "*+");
        assert_eq!(format!("{}", Quantifier::new(1.., Reluctance::Greedy)), "+");
        assert_eq!(format!("{}", Quantifier::new(1.., Reluctance::Lazy)), "+?");
        assert_eq!(format!("{}", Quantifier::new(1.., Reluctance::Possessive)), "++");
        assert_eq!(format!("{}", Quantifier::new(0..=1, Reluctance::Greedy)), "?");
        assert_eq!(format!("{}", Quantifier::new(0..=1, Reluctance::Lazy)), "??");
        assert_eq!(format!("{}", Quantifier::new(0..=1, Reluctance::Possessive)), "?+");
    }
}
