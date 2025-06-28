use std::collections::HashMap;

use bc_envelope::Envelope;

use super::{
    ArrayPattern, BoolPattern, ByteStringPattern, CBORPattern, DatePattern,
    KnownValuePattern, MapPattern, NullPattern, NumberPattern, TaggedPattern,
    TextPattern,
};
use crate::{
    Pattern,
    pattern::{Matcher, Path, compile_as_atomic, vm::Instr},
};

/// Pattern for matching leaf values.
#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub enum LeafPattern {
    /// Matches any leaf.
    Any,
    /// Matches the specific CBOR.
    Cbor(CBORPattern),
    /// Matches a numeric value.
    Number(NumberPattern),
    /// Matches a text value.
    Text(TextPattern),
    /// Matches a byte string value.
    ByteString(ByteStringPattern),
    /// Matches a tag value.
    Tag(TaggedPattern),
    /// Matches an array.
    Array(ArrayPattern),
    /// Matches a map.
    Map(MapPattern),
    /// Matches a boolean value.
    Bool(BoolPattern),
    /// Matches the null value.
    Null(NullPattern),
    /// Matches a date value.
    Date(DatePattern),
    /// Matches a known value.
    KnownValue(KnownValuePattern),
}

impl Matcher for LeafPattern {
    fn paths_with_captures(
        &self,
        envelope: &Envelope,
    ) -> (Vec<Path>, HashMap<String, Vec<Path>>) {
        match self {
            LeafPattern::Any => {
                let paths = if envelope.is_leaf() || envelope.is_known_value() {
                    vec![vec![envelope.clone()]]
                } else {
                    vec![]
                };
                (paths, HashMap::new())
            }
            LeafPattern::Cbor(pattern) => pattern.paths_with_captures(envelope),
            LeafPattern::Number(pattern) => {
                pattern.paths_with_captures(envelope)
            }
            LeafPattern::Text(pattern) => pattern.paths_with_captures(envelope),
            LeafPattern::ByteString(pattern) => {
                pattern.paths_with_captures(envelope)
            }
            LeafPattern::Tag(pattern) => pattern.paths_with_captures(envelope),
            LeafPattern::Array(pattern) => {
                pattern.paths_with_captures(envelope)
            }
            LeafPattern::Map(pattern) => pattern.paths_with_captures(envelope),
            LeafPattern::Bool(pattern) => pattern.paths_with_captures(envelope),
            LeafPattern::Null(pattern) => pattern.paths_with_captures(envelope),
            LeafPattern::Date(pattern) => pattern.paths_with_captures(envelope),
            LeafPattern::KnownValue(pattern) => {
                pattern.paths_with_captures(envelope)
            }
        }
    }

    fn compile(
        &self,
        code: &mut Vec<Instr>,
        literals: &mut Vec<Pattern>,
        captures: &mut Vec<String>,
    ) {
        match self {
            LeafPattern::Any => {
                compile_as_atomic(
                    &Pattern::Leaf(LeafPattern::Any),
                    code,
                    literals,
                    captures,
                );
            }
            LeafPattern::Cbor(pattern) => {
                pattern.compile(code, literals, captures);
            }
            LeafPattern::Number(pattern) => {
                pattern.compile(code, literals, captures);
            }
            LeafPattern::Text(pattern) => {
                pattern.compile(code, literals, captures);
            }
            LeafPattern::ByteString(pattern) => {
                pattern.compile(code, literals, captures);
            }
            LeafPattern::Tag(pattern) => {
                pattern.compile(code, literals, captures);
            }
            LeafPattern::Array(pattern) => {
                pattern.compile(code, literals, captures);
            }
            LeafPattern::Map(pattern) => {
                pattern.compile(code, literals, captures);
            }
            LeafPattern::Bool(pattern) => {
                pattern.compile(code, literals, captures);
            }
            LeafPattern::Null(pattern) => {
                pattern.compile(code, literals, captures);
            }
            LeafPattern::Date(pattern) => {
                pattern.compile(code, literals, captures);
            }
            LeafPattern::KnownValue(pattern) => {
                pattern.compile(code, literals, captures);
            }
        }
    }

    fn is_complex(&self) -> bool {
        match self {
            LeafPattern::Any => false,
            LeafPattern::Cbor(pattern) => pattern.is_complex(),
            LeafPattern::Number(pattern) => pattern.is_complex(),
            LeafPattern::Text(pattern) => pattern.is_complex(),
            LeafPattern::ByteString(pattern) => pattern.is_complex(),
            LeafPattern::Tag(pattern) => pattern.is_complex(),
            LeafPattern::Array(pattern) => pattern.is_complex(),
            LeafPattern::Map(pattern) => pattern.is_complex(),
            LeafPattern::Bool(pattern) => pattern.is_complex(),
            LeafPattern::Null(pattern) => pattern.is_complex(),
            LeafPattern::Date(pattern) => pattern.is_complex(),
            LeafPattern::KnownValue(pattern) => pattern.is_complex(),
        }
    }
}

impl std::fmt::Display for LeafPattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LeafPattern::Any => write!(f, "LEAF"),
            LeafPattern::Cbor(pattern) => write!(f, "{}", pattern),
            LeafPattern::Number(pattern) => write!(f, "{}", pattern),
            LeafPattern::Text(pattern) => write!(f, "{}", pattern),
            LeafPattern::ByteString(pattern) => write!(f, "{}", pattern),
            LeafPattern::Tag(pattern) => write!(f, "{}", pattern),
            LeafPattern::Array(pattern) => write!(f, "{}", pattern),
            LeafPattern::Map(pattern) => write!(f, "{}", pattern),
            LeafPattern::Bool(pattern) => write!(f, "{}", pattern),
            LeafPattern::Null(pattern) => write!(f, "{}", pattern),
            LeafPattern::Date(pattern) => write!(f, "{}", pattern),
            LeafPattern::KnownValue(pattern) => write!(f, "{}", pattern),
        }
    }
}
