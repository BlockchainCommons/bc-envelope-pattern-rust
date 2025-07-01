use std::collections::HashMap;

use bc_envelope::Envelope;

use crate::pattern::{Matcher, Path, Pattern, compile_as_atomic, vm::Instr};

/// Pattern for matching leaf envelopes (terminal nodes in the envelope tree).
#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct LeafStructurePattern;

impl LeafStructurePattern {
    /// Creates a new LeafStructurePattern.
    pub fn new() -> Self {
        LeafStructurePattern
    }
}

impl Default for LeafStructurePattern {
    fn default() -> Self {
        Self::new()
    }
}

impl Matcher for LeafStructurePattern {
    fn paths_with_captures(
        &self,
        envelope: &Envelope,
    ) -> (Vec<Path>, HashMap<String, Vec<Path>>) {
        let paths = if envelope.is_leaf() || envelope.is_known_value() {
            vec![vec![envelope.clone()]]
        } else {
            vec![]
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
            &Pattern::Structure(crate::pattern::structure::StructurePattern::Leaf(
                LeafStructurePattern::new(),
            )),
            code,
            literals,
            captures,
        );
    }

    fn is_complex(&self) -> bool {
        false
    }
}

impl std::fmt::Display for LeafStructurePattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "LEAF")
    }
}
