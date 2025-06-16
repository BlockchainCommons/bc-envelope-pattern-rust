use bc_envelope::Envelope;

use super::{
    ArrayPattern, BoolPattern, ByteStringPattern, CBORPattern, DatePattern,
    KnownValuePattern, MapPattern, NullPattern, NumberPattern, TaggedPattern,
    TextPattern,
};
use crate::{
    Pattern,
    pattern::{Compilable, Matcher, Path, compile_as_atomic, vm::Instr},
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
    fn paths(&self, envelope: &Envelope) -> Vec<Path> {
        match self {
            LeafPattern::Any => {
                if envelope.is_leaf() {
                    vec![vec![envelope.clone()]]
                } else {
                    vec![]
                }
            }
            LeafPattern::Cbor(pattern) => pattern.paths(envelope),
            LeafPattern::Number(pattern) => pattern.paths(envelope),
            LeafPattern::Text(pattern) => pattern.paths(envelope),
            LeafPattern::ByteString(pattern) => pattern.paths(envelope),
            LeafPattern::Tag(pattern) => pattern.paths(envelope),
            LeafPattern::Array(pattern) => pattern.paths(envelope),
            LeafPattern::Map(pattern) => pattern.paths(envelope),
            LeafPattern::Bool(pattern) => pattern.paths(envelope),
            LeafPattern::Null(pattern) => pattern.paths(envelope),
            LeafPattern::Date(pattern) => pattern.paths(envelope),
            LeafPattern::KnownValue(pattern) => pattern.paths(envelope),
        }
    }
}

impl Compilable for LeafPattern {
    fn compile(&self, code: &mut Vec<Instr>, literals: &mut Vec<Pattern>) {
        match self {
            LeafPattern::Any => {
                compile_as_atomic(
                    &Pattern::Leaf(LeafPattern::Any),
                    code,
                    literals,
                );
            }
            LeafPattern::Cbor(pattern) => {
                pattern.compile(code, literals);
            }
            LeafPattern::Number(pattern) => {
                pattern.compile(code, literals);
            }
            LeafPattern::Text(pattern) => {
                pattern.compile(code, literals);
            }
            LeafPattern::ByteString(pattern) => {
                pattern.compile(code, literals);
            }
            LeafPattern::Tag(pattern) => {
                pattern.compile(code, literals);
            }
            LeafPattern::Array(pattern) => {
                pattern.compile(code, literals);
            }
            LeafPattern::Map(pattern) => {
                pattern.compile(code, literals);
            }
            LeafPattern::Bool(pattern) => {
                pattern.compile(code, literals);
            }
            LeafPattern::Null(pattern) => {
                pattern.compile(code, literals);
            }
            LeafPattern::Date(pattern) => {
                pattern.compile(code, literals);
            }
            LeafPattern::KnownValue(pattern) => {
                pattern.compile(code, literals);
            }
        }
    }
}
