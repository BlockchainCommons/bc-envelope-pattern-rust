use std::collections::HashMap;

use bc_envelope::prelude::*;

use crate::pattern::{
    Matcher, Path, Pattern, structure::StructurePattern, vm::Instr,
};

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum AssertionsPattern {
    /// Matches any assertion.
    Any,
    /// Matches assertions with predicates that match a specific pattern.
    WithPredicate(Box<Pattern>),
    /// Matches assertions with objects that match a specific pattern.
    WithObject(Box<Pattern>),
}

impl AssertionsPattern {
    /// Creates a new `AssertionsPattern` that matches any assertion.
    pub fn any() -> Self { AssertionsPattern::Any }

    /// Creates a new `AssertionsPattern` that matches assertions
    /// with predicates that match a specific pattern.
    pub fn with_predicate(pattern: Pattern) -> Self {
        AssertionsPattern::WithPredicate(Box::new(pattern))
    }

    /// Creates a new `AssertionsPattern` that matches
    /// assertions with objects that match a specific pattern.
    pub fn with_object(pattern: Pattern) -> Self {
        AssertionsPattern::WithObject(Box::new(pattern))
    }
}

impl Matcher for AssertionsPattern {
    fn paths_with_captures(
        &self,
        haystack: &Envelope,
    ) -> (Vec<Path>, HashMap<String, Vec<Path>>) {
        let mut paths = Vec::new();
        for assertion in haystack.assertions() {
            match self {
                AssertionsPattern::Any => {
                    paths.push(vec![assertion.clone()]);
                }
                AssertionsPattern::WithPredicate(pattern) => {
                    if let Some(predicate) = assertion.as_predicate()
                        && pattern.matches(&predicate)
                    {
                        paths.push(vec![assertion.clone()]);
                    }
                }
                AssertionsPattern::WithObject(pattern) => {
                    if let Some(object) = assertion.as_object()
                        && pattern.matches(&object)
                    {
                        paths.push(vec![assertion.clone()]);
                    }
                }
            }
        }
        (paths, HashMap::new())
    }

    fn compile(
        &self,
        code: &mut Vec<Instr>,
        literals: &mut Vec<Pattern>,
        _captures: &mut Vec<String>,
    ) {
        let idx = literals.len();
        literals.push(Pattern::Structure(StructurePattern::Assertions(
            self.clone(),
        )));
        code.push(Instr::MatchStructure(idx));
    }
}

impl std::fmt::Display for AssertionsPattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AssertionsPattern::Any => write!(f, "assert"),
            AssertionsPattern::WithPredicate(pattern) => {
                write!(f, "assertpred({})", pattern)
            }
            AssertionsPattern::WithObject(pattern) => {
                write!(f, "assertobj({})", pattern)
            }
        }
    }
}
