use std::collections::HashMap;

use bc_envelope::prelude::*;

use crate::pattern::{Matcher, Path, Pattern, vm::Instr};

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct TraversePattern {
    first: Box<Pattern>,
    rest: Option<Box<TraversePattern>>,
}

impl TraversePattern {
    /// Creates a new `TraversePattern` with the given patterns.
    pub fn new(patterns: Vec<Pattern>) -> Self {
        let mut iter = patterns.into_iter();
        let first_pat = iter.next().unwrap_or_else(Pattern::none);
        // Build rest as a recursive TraversePattern if more remain
        let rest_patterns: Vec<Pattern> = iter.collect();
        let rest = if rest_patterns.is_empty() {
            None
        } else {
            Some(Box::new(TraversePattern::new(rest_patterns)))
        };
        TraversePattern { first: Box::new(first_pat), rest }
    }

    pub fn patterns(&self) -> Vec<Pattern> {
        let mut result = vec![*self.first.clone()];
        if let Some(rest) = &self.rest {
            result.extend(rest.patterns());
        }
        result
    }
}

impl Matcher for TraversePattern {
    fn paths_with_captures(
        &self,
        envelope: &Envelope,
    ) -> (Vec<Path>, HashMap<String, Vec<Path>>) {
        let paths = {
            // Match head first
            let head_paths = self.first.paths(envelope);
            // If there's no further traversal, return head paths
            if let Some(rest_seq) = &self.rest {
                let mut result = Vec::new();
                for path in head_paths {
                    if let Some(last_env) = path.last().cloned() {
                        // Recursively match the rest of the traversal
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
        };
        (paths, HashMap::new())
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
            code.push(Instr::ExtendTraversal);
            // Compile the rest of the traversal
            rest.compile(code, lits, captures);
            // Combine the paths correctly
            code.push(Instr::CombineTraversal);
        }
    }

    fn is_complex(&self) -> bool {
        // A traversal is complex if `first` is complex, or it has more than one
        // pattern
        self.first.is_complex() || self.rest.is_some()
    }
}

impl std::fmt::Display for TraversePattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.patterns()
                .iter()
                .map(|p| p.to_string())
                .collect::<Vec<_>>()
                .join(" -> ")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_traversal_pattern_display() {
        let pattern1 = Pattern::wrapped();
        let pattern2 = Pattern::wrapped();
        let traversal_pattern = TraversePattern::new(vec![pattern1, pattern2]);
        assert_eq!(traversal_pattern.to_string(), "wrapped -> wrapped");
    }
}
