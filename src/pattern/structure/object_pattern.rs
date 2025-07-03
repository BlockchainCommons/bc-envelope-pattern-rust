use std::collections::HashMap;

use bc_envelope::prelude::*;

use crate::pattern::{
    Matcher, Path, Pattern, structure::StructurePattern, vm::Instr,
};

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum ObjectPattern {
    Any,
    Pattern(Box<Pattern>),
}

impl ObjectPattern {
    pub fn any() -> Self { ObjectPattern::Any }

    pub fn pattern(pattern: Pattern) -> Self {
        ObjectPattern::Pattern(Box::new(pattern))
    }
}

impl Matcher for ObjectPattern {
    fn paths_with_captures(
        &self,
        haystack: &Envelope,
    ) -> (Vec<Path>, HashMap<String, Vec<Path>>) {
        let paths = {
            if let Some(object) = haystack.as_object() {
                match self {
                    ObjectPattern::Any => {
                        vec![vec![object.clone()]]
                    }
                    ObjectPattern::Pattern(pattern) => {
                        if pattern.matches(&object) {
                            vec![vec![object.clone()]]
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
        literals
            .push(Pattern::Structure(StructurePattern::Object(self.clone())));
        code.push(Instr::MatchStructure(idx));
    }
}

impl std::fmt::Display for ObjectPattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ObjectPattern::Any => write!(f, "obj"),
            ObjectPattern::Pattern(pattern) => write!(f, "obj({})", pattern),
        }
    }
}
