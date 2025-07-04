use std::collections::HashMap;

use bc_envelope::prelude::*;

use crate::pattern::{Pattern, vm::Instr};

/// A vector of envelopes that match a pattern, starting from the root of the
/// envelope.
pub type Path = Vec<Envelope>;

#[doc(hidden)]
pub trait Matcher: std::fmt::Debug + std::fmt::Display + Clone {
    /// Return all matching paths along with any named captures.
    fn paths_with_captures(
        &self,
        _haystack: &Envelope,
    ) -> (Vec<Path>, HashMap<String, Vec<Path>>);

    /// Return only the matching paths, discarding any captures.
    fn paths(&self, haystack: &Envelope) -> Vec<Path> {
        self.paths_with_captures(haystack).0
    }

    fn matches(&self, haystack: &Envelope) -> bool {
        !self.paths(haystack).is_empty()
    }

    fn compile(
        &self,
        _code: &mut Vec<Instr>,
        _literals: &mut Vec<Pattern>,
        _captures: &mut Vec<String>,
    ) {
        unimplemented!("Matcher::compile not implemented for {:?}", self);
    }

    /// Should return true if the Display of the matcher is *complex*,
    /// i.e. contains nested patterns or other complex structures
    /// that require its text rendering to be surrounded by grouping
    /// parentheses.
    fn is_complex(&self) -> bool { false }
}

/// Helper you can reuse in many impls: push self into `literals` and
/// emit a single MatchPredicate.
pub fn compile_as_atomic(
    pat: &Pattern,
    code: &mut Vec<Instr>,
    lits: &mut Vec<Pattern>,
    _captures: &mut Vec<String>,
) {
    let _ = _captures;
    let idx = lits.len();
    lits.push(pat.clone());
    code.push(Instr::MatchPredicate(idx));
}
