use bc_envelope::Envelope;

use crate::pattern::{Matcher, Path, Pattern, vm::Instr};

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct SequencePattern {
    first: Box<Pattern>,
    rest: Option<Box<SequencePattern>>,
}

impl SequencePattern {
    /// Creates a new `SequencePattern` with the given patterns.
    pub fn new(patterns: Vec<Pattern>) -> Self {
        let mut iter = patterns.into_iter();
        let first_pat = iter.next().unwrap_or_else(Pattern::none);
        // Build rest as a recursive SequencePattern if more remain
        let rest_patterns: Vec<Pattern> = iter.collect();
        let rest = if rest_patterns.is_empty() {
            None
        } else {
            Some(Box::new(SequencePattern::new(rest_patterns)))
        };
        SequencePattern { first: Box::new(first_pat), rest }
    }

    pub fn patterns(&self) -> Vec<Pattern> {
        let mut result = vec![*self.first.clone()];
        if let Some(rest) = &self.rest {
            result.extend(rest.patterns());
        }
        result
    }
}

impl Matcher for SequencePattern {
    fn paths(&self, envelope: &Envelope) -> Vec<Path> {
        // Match head first
        let head_paths = self.first.paths(envelope);
        // If there's no further sequence, return head paths
        if let Some(rest_seq) = &self.rest {
            let mut result = Vec::new();
            for path in head_paths {
                if let Some(last_env) = path.last().cloned() {
                    // Recursively match the rest of the sequence
                    for tail_path in rest_seq.paths(&last_env) {
                        let mut combined = path.clone();
                        combined.extend(tail_path);
                        result.push(combined);
                    }
                }
            }
            result
        } else {
            head_paths
        }
    }

    /// Compile into byte-code (sequential).
    fn compile(
        &self,
        code: &mut Vec<Instr>,
        lits: &mut Vec<Pattern>,
        captures: &mut Vec<String>,
    ) {
        // Compile the first pattern
        self.first.compile(code, lits, captures);

        if let Some(rest) = &self.rest {
            // Save the current path and switch to last envelope
            code.push(Instr::ExtendSequence);
            // Compile the rest of the sequence
            rest.compile(code, lits, captures);
            // Combine the paths correctly
            code.push(Instr::CombineSequence);
        }
    }

    fn is_complex(&self) -> bool {
        // A sequence is complex if `first` is complex, or it has more than one
        // pattern
        self.first.is_complex() || self.rest.is_some()
    }
}

impl std::fmt::Display for SequencePattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.patterns()
                .iter()
                .map(|p| p.to_string())
                .collect::<Vec<_>>()
                .join(">")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sequence_pattern_display() {
        let pattern1 = Pattern::wrapped();
        let pattern2 = Pattern::wrapped();
        let sequence_pattern = SequencePattern::new(vec![pattern1, pattern2]);
        assert_eq!(sequence_pattern.to_string(), "WRAPPED>WRAPPED");
    }
}
