use bc_envelope::Envelope;

use crate::{
    Pattern,
    pattern::{
        Matcher, Path,
        structure::StructurePattern,
        vm::{Axis, Instr},
    },
};

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct WrappedPattern;

impl WrappedPattern {
    /// Creates a new `WrappedPattern` that matches any wrapped envelope.
    pub fn new() -> WrappedPattern { WrappedPattern }
}

impl Default for WrappedPattern {
    fn default() -> Self { WrappedPattern::new() }
}

impl Matcher for WrappedPattern {
    fn paths(&self, envelope: &Envelope) -> Vec<Path> {
        // println!("Matching WrappedPattern: {:?}", self);
        let subject = envelope.subject();
        if subject.is_wrapped() {
            vec![vec![envelope.clone()]]
        } else {
            vec![]
        }
    }

    /// Emit predicate + descent so the VM makes progress.
    fn compile(
        &self,
        code: &mut Vec<Instr>,
        lits: &mut Vec<Pattern>,
        _captures: &mut Vec<String>,
    ) {
        // println!("Compiling WrappedPattern: {:?}", self);
        // 1) atomic predicate “is wrapped”
        let idx = lits.len();
        lits.push(Pattern::Structure(StructurePattern::Wrapped(self.clone())));
        code.push(Instr::MatchStructure(idx));

        // 2) then move into inner envelope
        code.push(Instr::PushAxis(Axis::Wrapped));
    }
}

impl std::fmt::Display for WrappedPattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "WRAPPED")
    }
}
