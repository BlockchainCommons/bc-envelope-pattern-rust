use bc_envelope::Envelope;

use crate::pattern::{Compilable, Matcher, Path, Pattern, vm::Instr};

/// A pattern that matches if all contained patterns match.
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct AndPattern {
    pub patterns: Vec<Pattern>,
}

impl AndPattern {
    /// Creates a new `AndPattern` with the given patterns.
    pub fn new(patterns: Vec<Pattern>) -> Self { AndPattern { patterns } }
}

impl Matcher for AndPattern {
    fn paths(&self, envelope: &Envelope) -> Vec<Path> {
        if self
            .patterns
            .iter()
            .all(|pattern| pattern.matches(envelope))
        {
            vec![vec![envelope.clone()]]
        } else {
            vec![]
        }
    }
}

impl Compilable for AndPattern {
    /// Compile into byte-code (AND = all must match).
    fn compile(&self, code: &mut Vec<Instr>, lits: &mut Vec<Pattern>) {
        // Each pattern must match at this position
        for pattern in &self.patterns {
            pattern.compile(code, lits);
        }
    }
}

impl std::fmt::Display for AndPattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.patterns
                .iter()
                .map(|p| p.to_string())
                .collect::<Vec<_>>()
                .join("&")
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
        assert_eq!(and_pattern.to_string(), "NUMBER(>5)&NUMBER(<10)");
    }
}
