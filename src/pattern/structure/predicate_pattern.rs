use bc_envelope::Envelope;

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
    fn paths(&self, envelope: &Envelope) -> Vec<Path> {
        if let Some(predicate) = envelope.as_predicate() {
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
            PredicatePattern::Any => write!(f, "PRED"),
            PredicatePattern::Pattern(pattern) => {
                write!(f, "PRED({})", pattern)
            }
        }
    }
}
