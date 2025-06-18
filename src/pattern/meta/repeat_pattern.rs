//! AST node + compiler for `{min,max}` quantifiers.

use bc_envelope::Envelope;

use crate::{
    Matcher, Path, RepeatRange,
    pattern::{Pattern, vm::Instr},
};

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct RepeatPattern {
    sub: Box<Pattern>,
    range: RepeatRange,
}

impl RepeatPattern {
    /// Creates a new `RepeatPattern` with the specified sub-pattern and range.
    pub fn new(sub: Pattern, range: RepeatRange) -> Self {
        RepeatPattern { sub: Box::new(sub), range }
    }

    /// Returns the sub-pattern of this repeat pattern.
    pub fn sub(&self) -> &Pattern { &self.sub }

    /// Returns the range of this repeat pattern.
    pub fn range(&self) -> &RepeatRange { &self.range }
}

impl Matcher for RepeatPattern {
    fn paths(&self, _envelope: &Envelope) -> Vec<Path> {
        todo!();
    }

    /// Emit a high-level `Repeat` instruction for the VM.
    fn compile(&self, code: &mut Vec<Instr>, lits: &mut Vec<Pattern>) {
        let idx = lits.len();
        lits.push((*self.sub).clone());
        code.push(Instr::Repeat {
            pat_idx: idx,
            min: self.range.min(),
            max: self.range.max(),
            mode: self.range.mode(),
        });
    }
}

impl std::fmt::Display for RepeatPattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.sub, self.range)
    }
}
