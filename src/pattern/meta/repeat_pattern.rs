//! AST node + compiler for `{min,max}` quantifiers.

use bc_envelope::Envelope;

use crate::{
    Matcher, Path, Quantifier,
    pattern::{Pattern, vm::Instr},
};

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct RepeatPattern {
    pattern: Box<Pattern>,
    quantifier: Quantifier,
}

impl RepeatPattern {
    /// Creates a new `RepeatPattern` with the specified sub-pattern and range.
    pub fn new(sub: Pattern, quantifier: Quantifier) -> Self {
        RepeatPattern { pattern: Box::new(sub), quantifier }
    }

    /// Returns the sub-pattern of this repeat pattern.
    pub fn pattern(&self) -> &Pattern { &self.pattern }

    /// Returns the quantifier of this repeat pattern.
    pub fn quantifier(&self) -> &Quantifier { &self.quantifier }
}

impl Matcher for RepeatPattern {
    fn paths(&self, _envelope: &Envelope) -> Vec<Path> {
        todo!();
    }

    /// Emit a high-level `Repeat` instruction for the VM.
    fn compile(&self, code: &mut Vec<Instr>, lits: &mut Vec<Pattern>) {
        let idx = lits.len();
        lits.push((*self.pattern).clone());
        code.push(Instr::Repeat { pat_idx: idx, quantifier: self.quantifier });
    }
}

impl std::fmt::Display for RepeatPattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let formatted_range = self.quantifier.to_string();
        if self.pattern.is_complex() && !formatted_range.is_empty() {
            write!(f, "({}){}", self.pattern, formatted_range)
        } else {
            write!(f, "{}{}", self.pattern, formatted_range)
        }
    }
}
