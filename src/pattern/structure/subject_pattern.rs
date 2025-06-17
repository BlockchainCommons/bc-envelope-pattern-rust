use bc_envelope::Envelope;

use crate::pattern::{
    Compilable, Matcher, Path, Pattern, structure::StructurePattern, vm::Instr,
};

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
    fn paths(&self, env: &Envelope) -> Vec<Path> {
        let subject = env.subject();
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
    }
}

impl Compilable for SubjectPattern {
    fn compile(&self, code: &mut Vec<Instr>, literals: &mut Vec<Pattern>) {
        match self {
            SubjectPattern::Any => {
                code.push(Instr::NavigateSubject);
            }
            SubjectPattern::Pattern(pattern) => {
                // Navigate to the subject first so the resulting path
                // includes the starting envelope and its subject.
                code.push(Instr::NavigateSubject);
                // Save the path and run the inner pattern relative to the
                // subject. This mirrors the behaviour of SequencePattern so
                // that any paths produced by `pattern` are appended to the
                // subject path rather than replacing it.
                code.push(Instr::ExtendSequence);
                pattern.compile(code, literals);
                code.push(Instr::CombineSequence);
            }
        }
    }
}

impl std::fmt::Display for SubjectPattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SubjectPattern::Any => write!(f, "SUBJECT"),
            SubjectPattern::Pattern(pattern) => {
                write!(f, "SUBJECT({})", pattern)
            }
        }
    }
}
