use std::{collections::HashMap, ops::RangeBounds};

use bc_envelope::Envelope;

use crate::{
    Interval, Pattern,
    pattern::{
        Matcher, Path, compile_as_atomic, structure::StructurePattern,
        vm::Instr,
    },
};

/// Pattern for matching node envelopes.
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum NodePattern {
    /// Matches any node.
    Any,
    /// Matches a node with the specified count of assertions.
    AssertionsInterval(Interval),
}

impl NodePattern {
    /// Creates a new `NodePattern` that matches any node.
    pub fn any() -> Self { NodePattern::Any }

    /// Creates a new `NodePattern` that matches a node with the specified count
    /// of assertions.
    pub fn interval(interval: impl RangeBounds<usize>) -> Self {
        NodePattern::AssertionsInterval(Interval::new(interval))
    }
}

impl Matcher for NodePattern {
    fn paths_with_captures(
        &self,
        envelope: &Envelope,
    ) -> (Vec<Path>, HashMap<String, Vec<Path>>) {
        let paths = {
            if !envelope.is_node() {
                return (vec![], HashMap::new());
            }

            let is_hit = match self {
                NodePattern::Any => true,
                NodePattern::AssertionsInterval(range) => {
                    range.contains(envelope.assertions().len())
                }
            };

            if is_hit {
                vec![vec![envelope.clone()]]
            } else {
                vec![]
            }
        };
        (paths, HashMap::new())
    }

    fn compile(
        &self,
        code: &mut Vec<Instr>,
        literals: &mut Vec<Pattern>,
        captures: &mut Vec<String>,
    ) {
        compile_as_atomic(
            &Pattern::Structure(StructurePattern::Node(self.clone())),
            code,
            literals,
            captures,
        );
    }
}

impl std::fmt::Display for NodePattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NodePattern::Any => write!(f, "NODE"),
            NodePattern::AssertionsInterval(range) => {
                write!(f, "NODE({})", range)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_node_pattern_display() {
        let any_pattern = NodePattern::any();
        assert_eq!(any_pattern.to_string(), "NODE");

        let count_pattern = NodePattern::interval(1..=3);
        assert_eq!(count_pattern.to_string(), "NODE({1,3})");
    }
}
