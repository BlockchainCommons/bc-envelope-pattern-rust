use std::collections::HashMap;

use bc_envelope::Envelope;

use crate::pattern::{Matcher, Path, Pattern, vm::Instr};

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum SubjectPattern {
    Any,
    Pattern(Box<Pattern>),
}

impl SubjectPattern {
    pub fn any() -> Self { SubjectPattern::Any }

    pub fn pattern(pattern: Pattern) -> Self {
        SubjectPattern::Pattern(Box::new(pattern))
    }
}

impl Matcher for SubjectPattern {
    fn paths_with_captures(
        &self,
        envelope: &Envelope,
    ) -> (Vec<Path>, HashMap<String, Vec<Path>>) {
        let paths = {
            let subject = envelope.subject();
            match self {
                SubjectPattern::Any => {
                    vec![vec![subject.clone()]]
                }
                SubjectPattern::Pattern(pattern) => {
                    if pattern.matches(&subject) {
                        vec![vec![subject.clone()]]
                    } else {
                        vec![]
                    }
                }
            }
        };
        (paths, HashMap::new())
    }

    fn compile(
        &self,
        code: &mut Vec<Instr>,
        literals: &mut Vec<Pattern>,
        captures: &mut Vec<String>,
    ) {
        match self {
            SubjectPattern::Any => {
                code.push(Instr::NavigateSubject);
            }
            SubjectPattern::Pattern(pattern) => {
                // Navigate to the subject first so the resulting path
                // includes the starting envelope and its subject.
                code.push(Instr::NavigateSubject);
                // Save the path and run the inner pattern relative to the
                // subject. This mirrors the behaviour of TraversalPattern so
                // that any paths produced by `pattern` are appended to the
                // subject path rather than replacing it.
                code.push(Instr::ExtendTraversal);
                pattern.compile(code, literals, captures);
                code.push(Instr::CombineTraversal);
            }
        }
    }
}

impl std::fmt::Display for SubjectPattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SubjectPattern::Any => write!(f, "subj"),
            SubjectPattern::Pattern(pattern) => {
                write!(f, "subj({})", pattern)
            }
        }
    }
}
