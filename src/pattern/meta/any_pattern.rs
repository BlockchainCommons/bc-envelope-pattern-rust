use bc_envelope::Envelope;

use crate::pattern::{
    Matcher, Path, Pattern, compile_as_atomic, meta::MetaPattern, vm::Instr,
};

/// A pattern that matches if any contained pattern matches.
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct AnyPattern;

impl AnyPattern {
    /// Creates a new `AnyPattern`.
    pub fn new() -> Self { AnyPattern }
}

impl Default for AnyPattern {
    fn default() -> Self { AnyPattern }
}

impl Matcher for AnyPattern {
    fn paths(&self, envelope: &Envelope) -> Vec<Path> {
        // Always return a path containing the envelope itself.
        vec![vec![envelope.clone()]]
    }

    fn compile(&self, code: &mut Vec<Instr>, literals: &mut Vec<Pattern>) {
        compile_as_atomic(
            &Pattern::Meta(MetaPattern::Any(self.clone())),
            code,
            literals,
        );
    }
}

impl std::fmt::Display for AnyPattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ANY")
    }
}
