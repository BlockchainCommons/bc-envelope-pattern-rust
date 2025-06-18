use logos::{Lexer, Logos};

use super::error::{Error, Result};
use crate::{Greediness, RepeatRange};

/// Tokens for the Gordian Envelope pattern syntax.
#[derive(Debug, Clone, Logos, PartialEq)]
#[rustfmt::skip]
#[logos(error = Error)]
#[logos(skip r"[ \t\r\n\f]+")]
pub enum Token {
    // Meta Pattern Operators
    #[token("&")]
    And,

    #[token("|")]
    Or,

    #[token("!")]
    Not,

    #[token(">", priority = 2)]
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
    #[token("ASSERT")]
    Assertion,

    #[token("ASSERTPRED")]
    AssertionPred,

    #[token("ASSERTOBJ")]
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

    #[token(",")]
    Comma,

    #[token("...")]
    Ellipsis,

    #[token(">=")]
    GreaterThanOrEqual,

    #[token("<=")]
    LessThanOrEqual,

    #[token(">", priority = 1)]
    GreaterThan,

    #[token("<")]
    LessThan,

    #[regex(r"[1-9]\d*|0", |lex|
        lex.slice().parse::<usize>().map_err(|_| Error::InvalidNumberFormat(lex.span()))
    )]
    UnsignedInteger(Result<usize>),

    #[regex(r"@[a-zA-Z_][a-zA-Z0-9_]*", |lex|
        lex.slice()[1..].to_string()
    )]
    GroupName(String),

    #[token("/", parse_regex)]
    Regex(Result<String>),

    #[token("{", parse_range)]
    Range(Result<RepeatRange>),
}

/// Callback used by the `Regex` variant above.
fn parse_regex(lex: &mut Lexer<Token>) -> Result<String> {
    let src = lex.remainder(); // everything after the first '/'
    let mut escape = false;

    for (i, ch) in src.char_indices() {
        match (ch, escape) {
            ('\\', false) => escape = true, // start of an escape
            ('/', false) => {
                // Found the closing delimiter ------------------
                lex.bump(i + 1); // +1 to also eat the '/'
                let content = src[..i].to_owned();
                match regex::Regex::new(&content) {
                    Ok(_) => return Ok(content),
                    Err(_) => return Err(Error::InvalidRegex(lex.span())),
                }
            }
            _ => escape = false, // any other char ends an escape
        }
    }

    // Unterminated literal – treat as lexing error
    Err(Error::UnterminatedRegex(lex.span()))
}
fn parse_range(lex: &mut Lexer<Token>) -> Result<RepeatRange> {
    let src = lex.remainder(); // everything after the first '{'

    // Helper to skip whitespace inside the range specification
    fn skip_ws(s: &str, pos: &mut usize) {
        while let Some(ch) = s[*pos..].chars().next() {
            if matches!(ch, ' ' | '\t' | '\n' | '\r' | '\u{0c}') {
                *pos += ch.len_utf8();
            } else {
                break;
            }
        }
    }

    let mut pos = 0;

    // parse minimum value --------------------------------------------------
    skip_ws(src, &mut pos);
    let start = pos;
    while let Some(ch) = src[pos..].chars().next() {
        if ch.is_ascii_digit() {
            pos += ch.len_utf8();
        } else {
            break;
        }
    }
    if start == pos {
        return Err(Error::InvalidRange(lex.span()));
    }
    let min: usize = src[start..pos]
        .parse()
        .map_err(|_| Error::InvalidRange(lex.span()))?;

    skip_ws(src, &mut pos);

    // parse optional comma and maximum value -------------------------------
    let max: Option<usize>;

    match src[pos..].chars().next() {
        Some(',') => {
            pos += 1;
            skip_ws(src, &mut pos);

            // If the next non-space char is '}', the range is open ended
            match src[pos..].chars().next() {
                Some('}') => {
                    pos += 1;
                    max = None;
                }
                Some(ch) if ch.is_ascii_digit() => {
                    let start = pos;
                    while let Some(ch) = src[pos..].chars().next() {
                        if ch.is_ascii_digit() {
                            pos += ch.len_utf8();
                        } else {
                            break;
                        }
                    }
                    if start == pos {
                        return Err(Error::InvalidRange(lex.span()));
                    }
                    let m: usize = src[start..pos]
                        .parse()
                        .map_err(|_| Error::InvalidRange(lex.span()))?;
                    skip_ws(src, &mut pos);
                    if !matches!(src[pos..].chars().next(), Some('}')) {
                        return Err(Error::InvalidRange(lex.span()));
                    }
                    pos += 1;
                    max = Some(m);
                }
                _ => return Err(Error::InvalidRange(lex.span())),
            }
        }
        Some('}') => {
            pos += 1;
            max = Some(min);
        }
        _ => return Err(Error::InvalidRange(lex.span())),
    }

    // determine greediness -------------------------------------------------
    let mode = match src[pos..].chars().next() {
        Some('?') => {
            pos += 1;
            Greediness::Lazy
        }
        Some('+') => {
            pos += 1;
            Greediness::Possessive
        }
        _ => Greediness::Greedy,
    };

    // consume parsed characters (everything after '{')
    lex.bump(pos);

    if let Some(max) = max {
        if min > max {
            return Err(Error::InvalidRange(lex.span()));
        }
        Ok(RepeatRange::new(min..=max, mode))
    } else {
        Ok(RepeatRange::new(min.., mode))
    }
}
