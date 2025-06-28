use std::collections::HashMap;

use bc_envelope::Envelope;

use crate::pattern::{
    Matcher, Path, Pattern, compile_as_atomic, meta::MetaPattern, vm::Instr,
};

/// A pattern that matches if any contained pattern matches.
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct NonePattern;

impl NonePattern {
    /// Creates a new `NonePattern`.
    pub fn new() -> Self { NonePattern }
}

impl Default for NonePattern {
    fn default() -> Self { NonePattern }
}

impl Matcher for NonePattern {
    fn paths_with_captures(
        &self,
        _envelope: &Envelope,
    ) -> (Vec<Path>, HashMap<String, Vec<Path>>) {
        // Never matches any element.
        (Vec::new(), HashMap::new())
    }

    fn compile(
        &self,
        code: &mut Vec<Instr>,
        literals: &mut Vec<Pattern>,
        captures: &mut Vec<String>,
    ) {
        compile_as_atomic(
            &Pattern::Meta(MetaPattern::None(self.clone())),
            code,
            literals,
            captures,
        );
    }
}

impl std::fmt::Display for NonePattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "NONE")
    }
}
