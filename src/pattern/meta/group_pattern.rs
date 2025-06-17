//! Simple group wrapper.  For now we only emit SAVE instructions;
//! future work can acquire captures and named captures.

use bc_envelope::Envelope;

use crate::{
    Matcher, Path,
    pattern::{Compilable, Pattern, vm::Instr},
};

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct GroupPattern {
    pub name: String,
    pub inner: Box<Pattern>,
}

impl Matcher for GroupPattern {
    fn paths(&self, _envelope: &Envelope) -> Vec<Path> {
        todo!();
    }
}

impl Compilable for GroupPattern {
    fn compile(&self, code: &mut Vec<Instr>, lits: &mut Vec<Pattern>) {
        code.push(Instr::Save); // start
        self.inner.compile(code, lits);
        code.push(Instr::Save); // end
    }
}

impl std::fmt::Display for GroupPattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({})", self.inner)
    }
}
