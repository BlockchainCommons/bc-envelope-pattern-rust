use std::collections::HashMap;

use bc_envelope::prelude::*;

use crate::pattern::{
    Matcher, Path, Pattern, structure::StructurePattern, vm::Instr,
};

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum PredicatePattern {
    Any,
    Pattern(Box<Pattern>),
}

impl PredicatePattern {
    pub fn any() -> Self { PredicatePattern::Any }

    pub fn pattern(pattern: Pattern) -> Self {
        PredicatePattern::Pattern(Box::new(pattern))
    }
}

impl Matcher for PredicatePattern {
    fn paths_with_captures(
        &self,
        haystack: &Envelope,
    ) -> (Vec<Path>, HashMap<String, Vec<Path>>) {
        let paths = {
            if let Some(predicate) = haystack.as_predicate() {
                match self {
                    PredicatePattern::Any => {
                        vec![vec![predicate.clone()]]
                    }
                    PredicatePattern::Pattern(pattern) => {
                        if pattern.matches(&predicate) {
                            vec![vec![predicate.clone()]]
                        } else {
                            vec![]
                        }
                    }
                }
            } else {
                vec![]
            }
        };
        (paths, HashMap::new())
    }

    fn compile(
        &self,
        code: &mut Vec<Instr>,
        literals: &mut Vec<Pattern>,
        _captures: &mut Vec<String>,
    ) {
        let idx = literals.len();
        literals.push(Pattern::Structure(StructurePattern::Predicate(
            self.clone(),
        )));
        code.push(Instr::MatchStructure(idx));
    }
}

impl std::fmt::Display for PredicatePattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PredicatePattern::Any => write!(f, "pred"),
            PredicatePattern::Pattern(pattern) => {
                write!(f, "pred({})", pattern)
            }
        }
    }
}
