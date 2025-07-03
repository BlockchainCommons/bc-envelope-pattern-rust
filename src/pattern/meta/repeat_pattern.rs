//! AST node + compiler for `{min,max}` quantifiers.

use std::collections::HashMap;

use crate::{
    Matcher, Path, Quantifier,
    pattern::{Pattern, vm::Instr},
};

use bc_envelope::prelude::*;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct GroupPattern {
    pattern: Box<Pattern>,
    quantifier: Quantifier,
}

impl GroupPattern {
    /// Creates a new `GroupPattern` with the specified sub-pattern and
    /// quantifier.
    pub fn repeat(pattern: Pattern, quantifier: Quantifier) -> Self {
        GroupPattern { pattern: Box::new(pattern), quantifier }
    }

    /// Creates a new `GroupPattern` with a quantifier that matches exactly
    /// once.
    pub fn new(pattern: Pattern) -> Self {
        GroupPattern {
            pattern: Box::new(pattern),
            quantifier: Quantifier::default(),
        }
    }

    /// Returns the sub-pattern of this group pattern.
    pub fn pattern(&self) -> &Pattern {
        &self.pattern
    }

    /// Returns the quantifier of this group pattern.
    pub fn quantifier(&self) -> &Quantifier {
        &self.quantifier
    }
}

impl Matcher for GroupPattern {
    fn paths_with_captures(
        &self,
        _haystack: &Envelope,
    ) -> (Vec<Path>, HashMap<String, Vec<Path>>) {
        panic!(
            "GroupPattern does not support paths_with_captures directly; use compile instead"
        );
    }

    /// Emit a high-level `Repeat` instruction for the VM.
    fn compile(
        &self,
        code: &mut Vec<Instr>,
        lits: &mut Vec<Pattern>,
        _captures: &mut Vec<String>,
    ) {
        let idx = lits.len();
        lits.push((*self.pattern).clone());
        code.push(Instr::Repeat { pat_idx: idx, quantifier: self.quantifier });
    }
}

impl std::fmt::Display for GroupPattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let formatted_range = self.quantifier.to_string();
        write!(f, "({}){}", self.pattern, formatted_range)
    }
}
