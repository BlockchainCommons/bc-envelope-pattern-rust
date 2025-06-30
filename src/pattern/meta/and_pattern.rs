use std::collections::HashMap;

use bc_envelope::Envelope;

use crate::pattern::{Matcher, Path, Pattern, vm::Instr};

/// A pattern that matches if all contained patterns match.
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct AndPattern(Vec<Pattern>);

impl AndPattern {
    /// Creates a new `AndPattern` with the given patterns.
    pub fn new(patterns: Vec<Pattern>) -> Self { AndPattern(patterns) }

    pub fn patterns(&self) -> &[Pattern] { &self.0 }
}

impl Matcher for AndPattern {
    fn paths_with_captures(
        &self,
        envelope: &Envelope,
    ) -> (Vec<Path>, HashMap<String, Vec<Path>>) {
        let paths = if self
            .patterns()
            .iter()
            .all(|pattern| pattern.matches(envelope))
        {
            vec![vec![envelope.clone()]]
        } else {
            vec![]
        };
        (paths, HashMap::new())
    }

    /// Compile into byte-code (AND = all must match).
    fn compile(
        &self,
        code: &mut Vec<Instr>,
        lits: &mut Vec<Pattern>,
        captures: &mut Vec<String>,
    ) {
        // Each pattern must match at this position
        for pattern in self.patterns() {
            pattern.compile(code, lits, captures);
        }
    }

    fn is_complex(&self) -> bool {
        // The pattern is complex if it contains more than one pattern, or if
        // the one pattern is complex itself.
        self.patterns().len() > 1
            || self.patterns().iter().any(|p| p.is_complex())
    }
}

impl std::fmt::Display for AndPattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.patterns()
                .iter()
                .map(|p| p.to_string())
                .collect::<Vec<_>>()
                .join(" & ")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_and_pattern_display() {
        let pattern1 = Pattern::number_greater_than(5);
        let pattern2 = Pattern::number_less_than(10);
        let and_pattern = AndPattern::new(vec![pattern1, pattern2]);
        assert_eq!(and_pattern.to_string(), ">5 & <10");
    }
}
