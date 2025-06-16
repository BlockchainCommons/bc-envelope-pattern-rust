use std::ops::Range;

use bc_ur::prelude::*;
use logos::Logos;
use regex::Regex;

/// Errors that can occur during parsing of Envelope patterns.
#[derive(Debug, Clone, thiserror::Error, PartialEq)]
pub enum Error {
    #[error("Invalid regex pattern at {0:?}")]
    InvalidRegex(Range<usize>),

    #[error("Invalid range at {0:?}")]
    InvalidRange(Range<usize>),

    #[error("Invalid hex string at {0:?}")]
    InvalidHexString(Range<usize>),

    #[error("Invalid date format at {0:?}")]
    InvalidDateFormat(Range<usize>),

    #[error("Invalid number format at {0:?}")]
    InvalidNumberFormat(Range<usize>),

    #[error("Invalid UR: {0} at {1:?}")]
    InvalidUr(String, Range<usize>),

    #[error("Unknown error")]
    Unknown,
}

pub type Result<T> = std::result::Result<T, Error>;

/// Tokens for the Gordian Envelope pattern syntax.
#[derive(Debug, Clone, Logos, PartialEq)]
#[rustfmt::skip]
#[logos(error = Error)]
#[logos(skip r"(?:[ \t\r\n\f]|//[^\n]*)+")]
pub enum Token {
    // Meta Pattern Operators
    #[token("&")]
    And,

    #[token("|")]
    Or,

    #[token("!")]
    Not,

    #[token(">")]
    Sequence,

    #[token("*")]
    RepeatZeroOrMore,

    #[token("*?")]
    RepeatZeroOrMoreLazy,

    #[token("*+")]
    RepeatZeroOrMorePossessive,

    #[token("+")]
    RepeatOneOrMore,

    #[token("+?")]
    RepeatOneOrMoreLazy,

    #[token("++")]
    RepeatOneOrMorePossessive,

    #[token("?")]
    RepeatZeroOrOne,

    #[token("??")]
    RepeatZeroOrOneLazy,

    #[token("?+")]
    RepeatZeroOrOnePossessive,

    // Structure Pattern Keywords
    #[token("ASSERTION")]
    Assertion,

    #[token("ASSERTION-PRED")]
    AssertionPred,

    #[token("ASSERTION-OBJ")]
    AssertionObj,

    #[token("DIGEST")]
    Digest,

    #[token("NODE")]
    Node,

    #[token("OBJ")]
    Obj,

    #[token("OBSCURED")]
    Obscured,

    #[token("ELIDED")]
    Elided,

    #[token("ENCRYPTED")]
    Encrypted,

    #[token("COMPRESSED")]
    Compressed,

    #[token("PRED")]
    Pred,

    #[token("SUBJECT")]
    Subject,

    #[token("WRAPPED")]
    Wrapped,

    #[token("SEARCH")]
    Search,

    // Leaf Pattern Keywords
    #[token("ARRAY")]
    Array,

    #[token("BOOL")]
    Bool,

    #[token("BSTR")]
    ByteString,

    #[token("CBOR")]
    Cbor,

    #[token("DATE")]
    Date,

    #[token("KNOWN")]
    Known,

    #[token("MAP")]
    Map,

    #[token("NULL")]
    Null,

    #[token("NUMBER")]
    Number,

    #[token("TAG")]
    Tag,

    #[token("TEXT")]
    Text,

    // Meta Pattern Keywords
    #[token("ANY")]
    Any,

    #[token("NONE")]
    None,

    // Special literals
    #[token("true")]
    BoolTrue,

    #[token("false")]
    BoolFalse,

    #[token("NaN")]
    NaN,

    // Grouping and Range delimiters
    #[token("(")]
    ParenOpen,

    #[token(")")]
    ParenClose,

    #[token("{")]
    BraceOpen,

    #[token("}")]
    BraceClose,

    #[token(",")]
    Comma,

    #[token("...")]
    Ellipsis,

    #[token(">=")]
    GreaterThanOrEqual,

    #[token("<=")]
    LessThanOrEqual,

    #[token(">")]
    GreaterThan,

    #[token("<")]
    LessThan,

    // Capture Group
    #[regex(r"@[a-zA-Z_][a-zA-Z0-9_]*", |lex|
        lex.slice()[1..].to_string()
    )]
    CaptureGroup(String),

    // Regular expressions
    #[regex(r"/([^/\\]|\\.)*?/", |lex| {
        let regex_str = &lex.slice()[1..lex.slice().len() - 1];
        match Regex::new(regex_str) {
            Ok(_) => Ok(regex_str.to_string()),
            Err(_) => Err(Error::InvalidRegex(lex.span()))
        }
    })]
    Regex(Result<String>),

    // Range pattern {n,m}
    #[regex(r"{\s*\d+\s*,\s*\d+\s*}", |lex| {
        let range_str = lex.slice();
        let content = &range_str[1..range_str.len()-1];
        let parts: Vec<&str> = content.split(',').map(|s| s.trim()).collect();
        if parts.len() == 2 {
            let start = parts[0].parse::<usize>();
            let end = parts[1].parse::<usize>();
            if let (Ok(s), Ok(e)) = (start, end) {
                Ok((s, e))
            } else {
                Err(Error::InvalidRange(lex.span()))
            }
        } else {
            Err(Error::InvalidRange(lex.span()))
        }
    })]
    Range(Result<(usize, usize)>),

    // Range pattern {n,m}? (lazy)
    #[regex(r"{\s*\d+\s*,\s*\d+\s*}\?", |lex| {
        let range_str = &lex.slice()[..lex.slice().len()-1];
        let content = &range_str[1..range_str.len()-1];
        let parts: Vec<&str> = content.split(',').map(|s| s.trim()).collect();
        if parts.len() == 2 {
            let start = parts[0].parse::<usize>();
            let end = parts[1].parse::<usize>();
            if let (Ok(s), Ok(e)) = (start, end) {
                Ok((s, e))
            } else {
                Err(Error::InvalidRange(lex.span()))
            }
        } else {
            Err(Error::InvalidRange(lex.span()))
        }
    })]
    RangeLazy(Result<(usize, usize)>),

    // Range pattern {n,m}+ (possessive)
    #[regex(r"{\s*\d+\s*,\s*\d+\s*}\+", |lex| {
        let range_str = &lex.slice()[..lex.slice().len()-1];
        let content = &range_str[1..range_str.len()-1];
        let parts: Vec<&str> = content.split(',').map(|s| s.trim()).collect();
        if parts.len() == 2 {
            let start = parts[0].parse::<usize>();
            let end = parts[1].parse::<usize>();
            if let (Ok(s), Ok(e)) = (start, end) {
                Ok((s, e))
            } else {
                Err(Error::InvalidRange(lex.span()))
            }
        } else {
            Err(Error::InvalidRange(lex.span()))
        }
    })]
    RangePossessive(Result<(usize, usize)>),

    // Numbers for pattern matching
    #[regex(r"-?(?:0|[1-9]\d*)(?:\.\d+)?(?:[eE][+-]?\d+)?", |lex|
        lex.slice().parse::<f64>().map_err(|_|
            Error::InvalidNumberFormat(lex.span())
        )
    )]
    NumberLiteral(Result<f64>),

    // String literals for pattern matching
    #[regex(r#""([^"\\]|\\.)*""#, |lex|
        // Remove surrounding quotes
        lex.slice()[1..lex.slice().len()-1].to_string()
    )]
    StringLiteral(String),

    // HexString for byte strings
    #[regex(r"h'[0-9a-fA-F]*'", |lex| {
        let hex = lex.slice();
        let raw_hex = &hex[2..hex.len() - 1];
        if raw_hex.len() % 2 != 0 {
            return Err(Error::InvalidHexString(lex.span()));
        }
        hex::decode(raw_hex)
            .map_err(|_|
                Error::InvalidHexString(lex.span())
            )
    })]
    HexString(Result<Vec<u8>>),

    // ISO-8601 Date
    #[regex(r"\d{4}-\d{2}-\d{2}(T\d{2}:\d{2}:\d{2}(.\d+)?(Z|[+-]\d{2}:?\d{2})?)?", |lex|
        lex.slice().to_string()
    )]
    DateISO8601(String),

    // URI values (including UR)
    #[regex(r#"ur:([a-zA-Z0-9][a-zA-Z0-9-]*)/([a-zA-Z0-9]{8,})"#, |lex|
        let s = lex.slice();
        let ur = UR::from_ur_string(s);
        ur.map_err(|e| {
            Error::InvalidUr(e.to_string(), lex.span())
        })
    )]
    UR(Result<UR>),

    // Identifiers (for tag names, known value names, etc.)
    #[regex(r"[a-zA-Z_][a-zA-Z0-9_-]*", |lex|
        lex.slice().to_string()
    )]
    Identifier(String),
}
