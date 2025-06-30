use std::collections::HashMap;

use bc_envelope::Envelope;

use crate::pattern::{Matcher, Path, Pattern, vm::Instr};

/// A pattern that matches if any contained pattern matches.
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct OrPattern(Vec<Pattern>);

impl OrPattern {
    /// Creates a new `OrPattern` with the given patterns.
    pub fn new(patterns: Vec<Pattern>) -> Self { OrPattern(patterns) }

    pub fn patterns(&self) -> &[Pattern] { &self.0 }
}

impl Matcher for OrPattern {
    fn paths_with_captures(
        &self,
        envelope: &Envelope,
    ) -> (Vec<Path>, HashMap<String, Vec<Path>>) {
        let paths = if self
            .patterns()
            .iter()
            .any(|pattern| pattern.matches(envelope))
        {
            vec![vec![envelope.clone()]]
        } else {
            vec![]
        };
        (paths, HashMap::new())
    }

    /// Compile into byte-code (OR = any can match).
    fn compile(
        &self,
        code: &mut Vec<Instr>,
        lits: &mut Vec<Pattern>,
        captures: &mut Vec<String>,
    ) {
        if self.patterns().is_empty() {
            return;
        }

        // For N patterns: Split(p1, Split(p2, ... Split(pN-1, pN)))
        let mut splits = Vec::new();

        // Generate splits for all but the last pattern
        for _ in 0..self.patterns().len() - 1 {
            splits.push(code.len());
            code.push(Instr::Split { a: 0, b: 0 }); // Placeholder
        }

        // Now fill in the actual split targets
        for (i, pattern) in self.patterns().iter().enumerate() {
            let pattern_start = code.len();

            // Compile this pattern
            pattern.compile(code, lits, captures);

            // This pattern will jump to the end if it matches
            let jump_past_all = code.len();
            code.push(Instr::Jump(0)); // Placeholder

            // If there's a next pattern, update the split to point here
            if i < self.patterns().len() - 1 {
                let next_pattern = code.len();
                code[splits[i]] =
                    Instr::Split { a: pattern_start, b: next_pattern };
            }

            // Will patch this jump once we know where "past all" is
            splits.push(jump_past_all);
        }

        // Now patch all the jumps to point past all the patterns
        let past_all = code.len();
        for &jump in &splits[self.patterns().len() - 1..] {
            code[jump] = Instr::Jump(past_all);
        }
    }

    fn is_complex(&self) -> bool {
        // The pattern is complex if it contains more than one pattern, or if
        // the one pattern is complex itself.
        self.patterns().len() > 1
            || self.patterns().iter().any(|p| p.is_complex())
    }
}

impl std::fmt::Display for OrPattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.patterns()
                .iter()
                .map(|p| p.to_string())
                .collect::<Vec<_>>()
                .join(" | ")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_or_pattern_display() {
        let pattern1 = Pattern::text("Alice");
        let pattern2 = Pattern::text("Bob");
        let or_pattern = OrPattern::new(vec![pattern1, pattern2]);
        assert_eq!(or_pattern.to_string(), r#""Alice" | "Bob""#);
    }
}
