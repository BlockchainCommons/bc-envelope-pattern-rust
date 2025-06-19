use bc_envelope::Envelope;

use crate::{
    Pattern,
    pattern::{
        Matcher, Path,
        structure::StructurePattern,
        vm::{Axis, Instr},
    },
};

/// Represents patterns for matching wrapped envelopes.
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum WrappedPattern {
    /// Matches any wrapped envelope without descending into its content.
    Any,
    /// Matches a wrapped envelope and also matches on its unwrapped content.
    Unwrap(Box<Pattern>),
}

impl WrappedPattern {
    /// Creates a new `WrappedPattern` that matches any wrapped envelope without
    /// descending.
    pub fn new() -> WrappedPattern { WrappedPattern::Any }

    /// Creates a new `WrappedPattern` that matches a wrapped envelope and also
    /// matches on its unwrapped content.
    pub fn unwrap_matching(pattern: Pattern) -> Self {
        WrappedPattern::Unwrap(Box::new(pattern))
    }

    /// Creates a new `WrappedPattern` that matches any wrapped envelope and
    /// descends into it.
    pub fn unwrap() -> Self { Self::unwrap_matching(Pattern::any()) }
}

impl Default for WrappedPattern {
    fn default() -> Self { WrappedPattern::new() }
}

impl Matcher for WrappedPattern {
    fn paths(&self, envelope: &Envelope) -> Vec<Path> {
        let subject = envelope.subject();
        if subject.is_wrapped() {
            match self {
                WrappedPattern::Any => {
                    // Just match the wrapped envelope itself, don't descend
                    vec![vec![envelope.clone()]]
                }
                WrappedPattern::Unwrap(pattern) => {
                    // Match the content of the wrapped envelope
                    if let Ok(unwrapped) = subject.unwrap_envelope() {
                        pattern
                            .paths(&unwrapped)
                            .into_iter()
                            .map(|mut path| {
                                // Add the current envelope to the path
                                path.insert(0, envelope.clone());
                                path
                            })
                            .collect()
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
        lits: &mut Vec<Pattern>,
        captures: &mut Vec<String>,
    ) {
        match self {
            WrappedPattern::Any => {
                // Just match the wrapped envelope itself, don't descend
                let idx = lits.len();
                lits.push(Pattern::Structure(StructurePattern::Wrapped(
                    self.clone(),
                )));
                code.push(Instr::MatchStructure(idx));
            }
            WrappedPattern::Unwrap(pattern) => {
                // First match that it's wrapped
                let idx = lits.len();
                lits.push(Pattern::Structure(StructurePattern::Wrapped(
                    WrappedPattern::Any,
                )));
                code.push(Instr::MatchStructure(idx));

                // Then move into inner envelope
                code.push(Instr::PushAxis(Axis::Wrapped));

                // Then match the pattern
                pattern.compile(code, lits, captures);
            }
        }
    }
}

impl std::fmt::Display for WrappedPattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WrappedPattern::Any => write!(f, "WRAPPED"),
            WrappedPattern::Unwrap(pattern) => {
                if **pattern == Pattern::any() {
                    write!(f, "UNWRAP")
                } else {
                    write!(f, "UNWRAP({})", pattern)
                }
            }
        }
    }
}
