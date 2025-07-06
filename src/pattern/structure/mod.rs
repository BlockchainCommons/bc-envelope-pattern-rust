// Structure patterns - patterns dealing with envelope structure

mod assertions_pattern;
mod digest_pattern;
mod leaf_structure_pattern;
mod node_pattern;
mod object_pattern;
mod obscured_pattern;
mod predicate_pattern;
mod subject_pattern;
mod wrapped_pattern;

pub(crate) use assertions_pattern::AssertionsPattern;
pub(crate) use digest_pattern::DigestPattern;
pub(crate) use leaf_structure_pattern::LeafStructurePattern;
pub(crate) use node_pattern::NodePattern;
pub(crate) use object_pattern::ObjectPattern;
pub(crate) use obscured_pattern::ObscuredPattern;
pub(crate) use predicate_pattern::PredicatePattern;
pub(crate) use subject_pattern::SubjectPattern;
pub(crate) use wrapped_pattern::WrappedPattern;

use std::collections::HashMap;

use bc_envelope::prelude::*;

use crate::pattern::{Matcher, Path, Pattern, vm::Instr};

/// Pattern for matching envelope structure elements.
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum StructurePattern {
    /// Matches assertions.
    Assertions(AssertionsPattern),
    /// Matches digests.
    Digest(DigestPattern),
    /// Matches leaf envelopes.
    Leaf(LeafStructurePattern),
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
        haystack: &Envelope,
    ) -> (Vec<Path>, HashMap<String, Vec<Path>>) {
        match self {
            StructurePattern::Assertions(pattern) => pattern.paths_with_captures(haystack),
            StructurePattern::Digest(pattern) => pattern.paths_with_captures(haystack),
            StructurePattern::Leaf(pattern) => pattern.paths_with_captures(haystack),
            StructurePattern::Node(pattern) => pattern.paths_with_captures(haystack),
            StructurePattern::Object(pattern) => pattern.paths_with_captures(haystack),
            StructurePattern::Obscured(pattern) => pattern.paths_with_captures(haystack),
            StructurePattern::Predicate(pattern) => pattern.paths_with_captures(haystack),
            StructurePattern::Subject(pattern) => pattern.paths_with_captures(haystack),
            StructurePattern::Wrapped(pattern) => pattern.paths_with_captures(haystack),
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
            StructurePattern::Leaf(s) => s.compile(code, lits, captures),
            StructurePattern::Node(s) => s.compile(code, lits, captures),
            StructurePattern::Obscured(s) => s.compile(code, lits, captures),
            StructurePattern::Predicate(s) => s.compile(code, lits, captures),
        }
    }

    fn is_complex(&self) -> bool {
        match self {
            StructurePattern::Assertions(pattern) => pattern.is_complex(),
            StructurePattern::Digest(pattern) => pattern.is_complex(),
            StructurePattern::Leaf(pattern) => pattern.is_complex(),
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
            StructurePattern::Leaf(pattern) => write!(f, "{}", pattern),
            StructurePattern::Node(pattern) => write!(f, "{}", pattern),
            StructurePattern::Object(pattern) => write!(f, "{}", pattern),
            StructurePattern::Obscured(pattern) => write!(f, "{}", pattern),
            StructurePattern::Predicate(pattern) => write!(f, "{}", pattern),
            StructurePattern::Subject(pattern) => write!(f, "{}", pattern),
            StructurePattern::Wrapped(pattern) => write!(f, "{}", pattern),
        }
    }
}
