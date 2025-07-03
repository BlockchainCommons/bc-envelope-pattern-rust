use std::collections::HashMap;

use bc_envelope::prelude::*;

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
    fn paths_with_captures(
        &self,
        haystack: &Envelope,
    ) -> (Vec<Path>, HashMap<String, Vec<Path>>) {
        // Always return a path containing the envelope itself.
        (vec![vec![haystack.clone()]], HashMap::new())
    }

    fn compile(
        &self,
        code: &mut Vec<Instr>,
        literals: &mut Vec<Pattern>,
        captures: &mut Vec<String>,
    ) {
        compile_as_atomic(
            &Pattern::Meta(MetaPattern::Any(self.clone())),
            code,
            literals,
            captures,
        );
    }
}

impl std::fmt::Display for AnyPattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "*")
    }
}
