use std::collections::HashMap;

use bc_envelope::Envelope;

use super::{
    AssertionsPattern, DigestPattern, NodePattern, ObjectPattern,
    ObscuredPattern, PredicatePattern, SubjectPattern, WrappedPattern,
};
use crate::pattern::{Matcher, Path, Pattern, vm::Instr};

/// Pattern for matching envelope structure elements.
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum StructurePattern {
    /// Matches assertions.
    Assertions(AssertionsPattern),
    /// Matches digests.
    Digest(DigestPattern),
    /// Matches nodes.
    Node(NodePattern),
    /// Matches objects.
    Object(ObjectPattern),
    /// Matches obscured elements.
    Obscured(ObscuredPattern),
    /// Matches predicates.
    Predicate(PredicatePattern),
    /// Matches subjects.
    Subject(SubjectPattern),
    /// Matches wrapped envelopes.
    Wrapped(WrappedPattern),
}

impl Matcher for StructurePattern {
    fn paths_with_captures(
        &self,
        envelope: &Envelope,
    ) -> (Vec<Path>, HashMap<String, Vec<Path>>) {
        match self {
            StructurePattern::Assertions(pattern) => pattern.paths_with_captures(envelope),
            StructurePattern::Digest(pattern) => pattern.paths_with_captures(envelope),
            StructurePattern::Node(pattern) => pattern.paths_with_captures(envelope),
            StructurePattern::Object(pattern) => pattern.paths_with_captures(envelope),
            StructurePattern::Obscured(pattern) => pattern.paths_with_captures(envelope),
            StructurePattern::Predicate(pattern) => pattern.paths_with_captures(envelope),
            StructurePattern::Subject(pattern) => pattern.paths_with_captures(envelope),
            StructurePattern::Wrapped(pattern) => pattern.paths_with_captures(envelope),
        }
    }

    fn compile(
        &self,
        code: &mut Vec<Instr>,
        lits: &mut Vec<Pattern>,
        captures: &mut Vec<String>,
    ) {
        match self {
            StructurePattern::Subject(s) => s.compile(code, lits, captures),
            StructurePattern::Assertions(s) => s.compile(code, lits, captures),
            StructurePattern::Wrapped(s) => s.compile(code, lits, captures),
            StructurePattern::Object(s) => s.compile(code, lits, captures),
            StructurePattern::Digest(s) => s.compile(code, lits, captures),
            StructurePattern::Node(s) => s.compile(code, lits, captures),
            StructurePattern::Obscured(s) => s.compile(code, lits, captures),
            StructurePattern::Predicate(s) => s.compile(code, lits, captures),
        }
    }

    fn is_complex(&self) -> bool {
        match self {
            StructurePattern::Assertions(pattern) => pattern.is_complex(),
            StructurePattern::Digest(pattern) => pattern.is_complex(),
            StructurePattern::Node(pattern) => pattern.is_complex(),
            StructurePattern::Object(pattern) => pattern.is_complex(),
            StructurePattern::Obscured(pattern) => pattern.is_complex(),
            StructurePattern::Predicate(pattern) => pattern.is_complex(),
            StructurePattern::Subject(pattern) => pattern.is_complex(),
            StructurePattern::Wrapped(pattern) => pattern.is_complex(),
        }
    }
}

impl std::fmt::Display for StructurePattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StructurePattern::Assertions(pattern) => write!(f, "{}", pattern),
            StructurePattern::Digest(pattern) => write!(f, "{}", pattern),
            StructurePattern::Node(pattern) => write!(f, "{}", pattern),
            StructurePattern::Object(pattern) => write!(f, "{}", pattern),
            StructurePattern::Obscured(pattern) => write!(f, "{}", pattern),
            StructurePattern::Predicate(pattern) => write!(f, "{}", pattern),
            StructurePattern::Subject(pattern) => write!(f, "{}", pattern),
            StructurePattern::Wrapped(pattern) => write!(f, "{}", pattern),
        }
    }
}
