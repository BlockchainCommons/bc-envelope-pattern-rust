use std::collections::HashMap;

use bc_envelope::prelude::*;

use super::{
    AndPattern, CapturePattern, GroupPattern, NotPattern, OrPattern,
    SearchPattern, TraversePattern,
};
use crate::{
    Pattern,
    pattern::{Matcher, Path, meta::AnyPattern, vm::Instr},
};

/// Pattern for combining and modifying other patterns.
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum MetaPattern {
    /// Always matches.
    Any(AnyPattern),
    /// Matches if all contained patterns match.
    And(AndPattern),
    /// Matches if any contained pattern matches.
    Or(OrPattern),
    /// Matches if the inner pattern does not match.
    Not(NotPattern),
    /// Searches the entire envelope tree for matches.
    Search(SearchPattern),
    /// Matches a traversal order of patterns.
    Traverse(TraversePattern),
    /// Matches with repetition.
    Group(GroupPattern),
    /// Captures a pattern match.
    Capture(CapturePattern),
}

impl Matcher for MetaPattern {
    fn paths_with_captures(
        &self,
        haystack: &Envelope,
    ) -> (Vec<Path>, HashMap<String, Vec<Path>>) {
        match self {
            MetaPattern::Any(pattern) => pattern.paths_with_captures(haystack),
            MetaPattern::And(pattern) => pattern.paths_with_captures(haystack),
            MetaPattern::Or(pattern) => pattern.paths_with_captures(haystack),
            MetaPattern::Not(pattern) => pattern.paths_with_captures(haystack),
            MetaPattern::Search(pattern) => {
                pattern.paths_with_captures(haystack)
            }
            MetaPattern::Traverse(pattern) => {
                pattern.paths_with_captures(haystack)
            }
            MetaPattern::Group(pattern) => {
                pattern.paths_with_captures(haystack)
            }
            MetaPattern::Capture(pattern) => {
                pattern.paths_with_captures(haystack)
            }
        }
    }

    fn compile(
        &self,
        code: &mut Vec<Instr>,
        lits: &mut Vec<Pattern>,
        captures: &mut Vec<String>,
    ) {
        match self {
            MetaPattern::Any(pattern) => pattern.compile(code, lits, captures),
            MetaPattern::And(pattern) => pattern.compile(code, lits, captures),
            MetaPattern::Or(pattern) => pattern.compile(code, lits, captures),
            MetaPattern::Not(pattern) => pattern.compile(code, lits, captures),
            MetaPattern::Search(pattern) => {
                pattern.compile(code, lits, captures)
            }
            MetaPattern::Traverse(pattern) => {
                pattern.compile(code, lits, captures)
            }
            MetaPattern::Group(pattern) => {
                pattern.compile(code, lits, captures)
            }
            MetaPattern::Capture(pattern) => {
                pattern.compile(code, lits, captures)
            }
        }
    }

    fn is_complex(&self) -> bool {
        match self {
            MetaPattern::Any(pattern) => pattern.is_complex(),
            MetaPattern::And(pattern) => pattern.is_complex(),
            MetaPattern::Or(pattern) => pattern.is_complex(),
            MetaPattern::Not(pattern) => pattern.is_complex(),
            MetaPattern::Search(pattern) => pattern.is_complex(),
            MetaPattern::Traverse(pattern) => pattern.is_complex(),
            MetaPattern::Group(pattern) => pattern.is_complex(),
            MetaPattern::Capture(pattern) => pattern.is_complex(),
        }
    }
}

impl std::fmt::Display for MetaPattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MetaPattern::Any(pattern) => write!(f, "{}", pattern),
            MetaPattern::And(pattern) => write!(f, "{}", pattern),
            MetaPattern::Or(pattern) => write!(f, "{}", pattern),
            MetaPattern::Not(pattern) => write!(f, "{}", pattern),
            MetaPattern::Search(pattern) => write!(f, "{}", pattern),
            MetaPattern::Traverse(pattern) => write!(f, "{}", pattern),
            MetaPattern::Group(pattern) => write!(f, "{}", pattern),
            MetaPattern::Capture(pattern) => write!(f, "{}", pattern),
        }
    }
}

impl MetaPattern {
    pub(crate) fn collect_capture_names(&self, out: &mut Vec<String>) {
        match self {
            MetaPattern::Any(_) => {}
            MetaPattern::And(p) => {
                for pat in p.patterns() {
                    pat.collect_capture_names(out);
                }
            }
            MetaPattern::Or(p) => {
                for pat in p.patterns() {
                    pat.collect_capture_names(out);
                }
            }
            MetaPattern::Not(p) => p.pattern().collect_capture_names(out),
            MetaPattern::Search(p) => p.pattern().collect_capture_names(out),
            MetaPattern::Traverse(p) => {
                for pat in p.patterns() {
                    pat.collect_capture_names(out);
                }
            }
            MetaPattern::Group(p) => p.pattern().collect_capture_names(out),
            MetaPattern::Capture(p) => {
                if !out.contains(&p.name().to_string()) {
                    out.push(p.name().to_string());
                }
                p.pattern().collect_capture_names(out);
            }
        }
    }
}
