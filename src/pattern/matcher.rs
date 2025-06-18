use crate::pattern::{Pattern, vm::Instr};
use bc_envelope::Envelope;

pub type Path = Vec<Envelope>;

pub trait Matcher: std::fmt::Debug + Clone {
    fn paths(&self, envelope: &Envelope) -> Vec<Path>;

    fn matches(&self, envelope: &Envelope) -> bool {
        !self.paths(envelope).is_empty()
    }

    fn compile(&self, _code: &mut Vec<Instr>, _literals: &mut Vec<Pattern>) {
        unimplemented!("Matcher::compile must be implemented for {:?}", self);
    }
}

/// Helper you can reuse in many impls: push self into `literals` and
/// emit a single MatchPredicate.
pub fn compile_as_atomic(
    pat: &Pattern,
    code: &mut Vec<Instr>,
    lits: &mut Vec<Pattern>,
) {
    let idx = lits.len();
    lits.push(pat.clone());
    code.push(Instr::MatchPredicate(idx));
}
