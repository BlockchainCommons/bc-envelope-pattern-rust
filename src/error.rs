use logos::Span;
use thiserror::Error;

use crate::parse::Token;

/// Errors that can occur during parsing of Envelope patterns.
#[derive(Debug, Clone, Error, PartialEq, Default)]
pub enum Error {
    #[error("Empty input")]
    EmptyInput,

    #[error("Unexpected end of input")]
    UnexpectedEndOfInput,

    #[error("Extra data at end of input")]
    ExtraData(Span),

    #[error("Unexpected token {0:?}")]
    UnexpectedToken(Box<Token>, Span),

    #[error("Unrecognized token at position {0:?}")]
    UnrecognizedToken(Span),

    #[error("Invalid regex pattern at {0:?}")]
    InvalidRegex(Span),

    #[error("Unterminated regex pattern at {0:?}")]
    UnterminatedRegex(Span),

    #[error("Invalid range at {0:?}")]
    InvalidRange(Span),

    #[error("Invalid hex string at {0:?}")]
    InvalidHexString(Span),

    #[error("Invalid date format at {0:?}")]
    InvalidDateFormat(Span),

    #[error("Invalid number format at {0:?}")]
    InvalidNumberFormat(Span),

    #[error("Invalid UR: {0} at {1:?}")]
    InvalidUr(String, Span),

    #[error("Expected opening parenthesis")]
    ExpectedOpenParen(Span),

    #[error("Expected closing parenthesis")]
    ExpectedCloseParen(Span),

    #[error("Expected pattern after operator")]
    ExpectedPattern(Span),

    #[error("Unmatched parentheses")]
    UnmatchedParentheses(Span),

    #[error("Unmatched braces")]
    UnmatchedBraces(Span),

    #[error("Invalid capture group name")]
    InvalidCaptureGroupName(String, Span),

    #[error("Unknown error")]
    #[default]
    Unknown,
}

/// A Result type specialized for envelope pattern parsing.
pub type Result<T> = std::result::Result<T, Error>;
