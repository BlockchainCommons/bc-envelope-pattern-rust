// Leaf patterns - patterns dealing with CBOR leaf node values

mod array_pattern;
mod bool_pattern;
mod byte_string_pattern;
mod cbor_pattern;
mod date_pattern;
mod known_value_pattern;
mod map_pattern;
mod null_pattern;
mod number_pattern;
mod tagged_pattern;
mod text_pattern;

use std::collections::HashMap;

pub(crate) use array_pattern::ArrayPattern;
use bc_envelope::prelude::*;
pub(crate) use bool_pattern::BoolPattern;
pub(crate) use byte_string_pattern::ByteStringPattern;
pub(crate) use cbor_pattern::CBORPattern;
pub(crate) use date_pattern::DatePattern;
pub(crate) use known_value_pattern::KnownValuePattern;
pub(crate) use map_pattern::MapPattern;
pub(crate) use null_pattern::NullPattern;
pub(crate) use number_pattern::NumberPattern;
pub(crate) use tagged_pattern::TaggedPattern;
pub(crate) use text_pattern::TextPattern;

use crate::{
    Pattern,
    pattern::{Matcher, Path, vm::Instr},
};

/// Pattern for matching leaf values.
#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub enum LeafPattern {
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
        haystack: &Envelope,
    ) -> (Vec<Path>, HashMap<String, Vec<Path>>) {
        match self {
            LeafPattern::Cbor(pattern) => pattern.paths_with_captures(haystack),
            LeafPattern::Number(pattern) => {
                pattern.paths_with_captures(haystack)
            }
            LeafPattern::Text(pattern) => pattern.paths_with_captures(haystack),
            LeafPattern::ByteString(pattern) => {
                pattern.paths_with_captures(haystack)
            }
            LeafPattern::Tag(pattern) => pattern.paths_with_captures(haystack),
            LeafPattern::Array(pattern) => {
                pattern.paths_with_captures(haystack)
            }
            LeafPattern::Map(pattern) => pattern.paths_with_captures(haystack),
            LeafPattern::Bool(pattern) => pattern.paths_with_captures(haystack),
            LeafPattern::Null(pattern) => pattern.paths_with_captures(haystack),
            LeafPattern::Date(pattern) => pattern.paths_with_captures(haystack),
            LeafPattern::KnownValue(pattern) => {
                pattern.paths_with_captures(haystack)
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
