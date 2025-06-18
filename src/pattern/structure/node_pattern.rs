use std::ops::RangeBounds;

use bc_envelope::Envelope;

use crate::{
    pattern::{
        compile_as_atomic, structure::StructurePattern, vm::Instr, Matcher, Path
    }, Pattern, Repeat
};

/// Pattern for matching node envelopes.
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum NodePattern {
    /// Matches any node.
    Any,
    /// Matches a node with the specified count of assertions.
    AssertionsCount(Repeat),
}

impl NodePattern {
    /// Creates a new `NodePattern` that matches any node.
    pub fn any() -> Self { NodePattern::Any }

    /// Creates a new `NodePattern` that matches a node with the specified count
    /// of assertions.
    pub fn count(range: impl RangeBounds<usize>) -> Self {
        NodePattern::AssertionsCount(Repeat::new(range))
    }
}

impl Matcher for NodePattern {
    fn paths(&self, envelope: &Envelope) -> Vec<Path> {
        if !envelope.is_node() {
            return vec![];
        }

        let is_hit = match self {
            NodePattern::Any => true,
            NodePattern::AssertionsCount(range) => {
                range.contains(envelope.assertions().len())
            }
        };

        if is_hit {
            vec![vec![envelope.clone()]]
        } else {
            vec![]
        }
    }

    fn compile(&self, code: &mut Vec<Instr>, literals: &mut Vec<Pattern>) {
        compile_as_atomic(
            &Pattern::Structure(StructurePattern::Node(self.clone())),
            code,
            literals,
        );
    }
}

impl std::fmt::Display for NodePattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NodePattern::Any => write!(f, "NODE"),
            NodePattern::AssertionsCount(range) => write!(f, "NODE({})", range),
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

        let count_pattern = NodePattern::count(1..=3);
        assert_eq!(count_pattern.to_string(), "NODE({1,3})");
    }
}
