use logos::{Lexer, Logos};

use crate::{Error, Quantifier, Reluctance, Result};

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

    #[token("->", priority = 2)]
    Traverse,

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

    #[token("UNWRAP")]
    Unwrap,

    #[token("SEARCH")]
    Search,

    // Leaf Pattern Keywords
    #[token("ARRAY")]
    Array,

    #[token("bstr")]
    ByteString,

    #[token("LEAF")]
    Leaf,

    #[token("CBOR")]
    Cbor,

    #[token("DATE")]
    Date,

    #[token("KNOWN")]
    Known,

    #[token("MAP")]
    Map,

    #[token("null")]
    Null,

    #[token("NUMBER")]
    Number,

    #[token("TAG")]
    Tag,

    // Meta Pattern Keywords
    #[token("NONE")]
    None,

    // Special literals
    #[token("bool")]
    BoolKeyword,

    #[token("true")]
    BoolTrue,

    #[token("false")]
    BoolFalse,

    #[token("text")]
    TextKeyword,

    #[token("NaN")]
    NaN,

    #[token("\"", parse_string_literal_token)]
    StringLiteral(Result<String>),

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

    #[token("h'", parse_hex_pattern)]
    HexPattern(Result<Vec<u8>>),

    #[token("h'/", parse_hex_binary_regex)]
    HexBinaryRegex(Result<String>),

    #[token("{", parse_range)]
    Range(Result<Quantifier>),
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

/// Callback used by the `HexPattern` variant above.
fn parse_hex_pattern(lex: &mut Lexer<Token>) -> Result<Vec<u8>> {
    let src = lex.remainder(); // everything after the first h'

    // Parse hex digits until we find the closing '
    for (i, ch) in src.char_indices() {
        if ch == '\'' {
            // Found the closing delimiter
            let hex_str = &src[..i];
            lex.bump(i + 1); // +1 to also eat the '
            return hex::decode(hex_str)
                .map_err(|_| Error::InvalidHexString(lex.span()));
        }
        if !ch.is_ascii_hexdigit() {
            return Err(Error::InvalidHexString(lex.span()));
        }
    }

    // Unterminated hex literal
    Err(Error::InvalidHexString(lex.span()))
}

/// Callback used by the `HexBinaryRegex` variant above.
fn parse_hex_binary_regex(lex: &mut Lexer<Token>) -> Result<String> {
    let src = lex.remainder(); // everything after the first h'/
    let mut escape = false;

    for (i, ch) in src.char_indices() {
        match (ch, escape) {
            ('\\', false) => escape = true, // start of an escape
            ('/', false) => {
                // Found the closing delimiter
                lex.bump(i + 1); // +1 to also eat the '/'
                if i + 1 < src.len() && src.chars().nth(i + 1) == Some('\'') {
                    lex.bump(1); // eat the closing '
                }
                let regex_str = &src[..i];
                match regex::bytes::Regex::new(regex_str) {
                    Ok(_) => return Ok(regex_str.to_string()),
                    Err(_) => return Err(Error::InvalidRegex(lex.span())),
                }
            }
            _ => escape = false, // any other char ends an escape
        }
    }

    // Unterminated regex literal
    Err(Error::UnterminatedRegex(lex.span()))
}

fn parse_range(lex: &mut Lexer<Token>) -> Result<Quantifier> {
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
            Reluctance::Lazy
        }
        Some('+') => {
            pos += 1;
            Reluctance::Possessive
        }
        _ => Reluctance::Greedy,
    };

    // consume parsed characters (everything after '{')
    lex.bump(pos);

    if let Some(max) = max {
        if min > max {
            return Err(Error::InvalidRange(lex.span()));
        }
        Ok(Quantifier::new(min..=max, mode))
    } else {
        Ok(Quantifier::new(min.., mode))
    }
}

/// Callback used by the `StringLiteral` variant above.
fn parse_string_literal_token(lex: &mut Lexer<Token>) -> Result<String> {
    let src = lex.remainder(); // everything after the first '"'
    let mut escape = false;
    let mut content = String::new();

    for (i, b) in src.bytes().enumerate() {
        let consumed = i + 1;
        match b {
            b'"' if !escape => {
                // End of string
                lex.bump(consumed);
                return Ok(content);
            }
            b'\\' if !escape => {
                escape = true;
            }
            b'n' if escape => {
                content.push('\n');
                escape = false;
            }
            b't' if escape => {
                content.push('\t');
                escape = false;
            }
            b'r' if escape => {
                content.push('\r');
                escape = false;
            }
            b'\\' if escape => {
                content.push('\\');
                escape = false;
            }
            b'"' if escape => {
                content.push('"');
                escape = false;
            }
            c => {
                if escape {
                    // Invalid escape sequence, but we'll be lenient
                    content.push('\\');
                    escape = false;
                }
                content.push(c as char);
            }
        }
    }

    // Unterminated string literal
    Err(Error::UnexpectedEndOfInput)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_basic_tokens() {
        // Test meta pattern operators
        assert_eq!(Token::lexer("&").next(), Some(Ok(Token::And)));
        assert_eq!(Token::lexer("|").next(), Some(Ok(Token::Or)));
        assert_eq!(Token::lexer("!").next(), Some(Ok(Token::Not)));
        assert_eq!(Token::lexer("->").next(), Some(Ok(Token::Traverse)));
        assert_eq!(Token::lexer("*").next(), Some(Ok(Token::RepeatZeroOrMore)));
        assert_eq!(Token::lexer("+").next(), Some(Ok(Token::RepeatOneOrMore)));
        assert_eq!(Token::lexer("?").next(), Some(Ok(Token::RepeatZeroOrOne)));

        // Test structure pattern keywords
        assert_eq!(Token::lexer("ASSERT").next(), Some(Ok(Token::Assertion)));
        assert_eq!(Token::lexer("NODE").next(), Some(Ok(Token::Node)));
        assert_eq!(Token::lexer("SUBJECT").next(), Some(Ok(Token::Subject)));
        assert_eq!(Token::lexer("WRAPPED").next(), Some(Ok(Token::Wrapped)));
        assert_eq!(Token::lexer("UNWRAP").next(), Some(Ok(Token::Unwrap)));

        // Test leaf pattern keywords
        assert_eq!(Token::lexer("ARRAY").next(), Some(Ok(Token::Array)));
        assert_eq!(Token::lexer("NUMBER").next(), Some(Ok(Token::Number)));

        // Test literals
        assert_eq!(Token::lexer("bool").next(), Some(Ok(Token::BoolKeyword)));
        assert_eq!(Token::lexer("true").next(), Some(Ok(Token::BoolTrue)));
        assert_eq!(Token::lexer("false").next(), Some(Ok(Token::BoolFalse)));
        assert_eq!(Token::lexer("NaN").next(), Some(Ok(Token::NaN)));
    }

    #[test]
    fn test_complex_tokens() {
        // Group name
        let mut lexer = Token::lexer("@name");
        if let Some(Ok(Token::GroupName(name))) = lexer.next() {
            assert_eq!(name, "name");
        } else {
            panic!("Failed to parse group name");
        }

        // Test regex
        let mut lexer = Token::lexer("/[a-z]+/");
        if let Some(Ok(Token::Regex(Ok(regex)))) = lexer.next() {
            assert_eq!(regex, "[a-z]+");
        } else {
            panic!("Failed to parse regex");
        }

        let mut lx = Token::lexer(r"/abc\/def/  / /  //  /a\//");
        assert_eq!(
            lx.next(),
            Some(Ok(Token::Regex(Ok("abc\\/def".to_string()))))
        );
        assert_eq!(lx.next(), Some(Ok(Token::Regex(Ok(" ".to_string())))));
        assert_eq!(lx.next(), Some(Ok(Token::Regex(Ok("".to_string())))));
        assert_eq!(lx.next(), Some(Ok(Token::Regex(Ok("a\\/".to_string())))));
        assert_eq!(lx.next(), None);
    }

    #[test]
    fn test_unsigned_integer() {
        let mut lexer = Token::lexer("42");
        if let Some(Ok(Token::UnsignedInteger(Ok(42)))) = lexer.next() {
            // Successfully parsed integer
        } else {
            panic!("Failed to parse integer literal");
        }

        // Test unsigned integer
        let mut lexer = Token::lexer("0");
        if let Some(Ok(Token::UnsignedInteger(Ok(0)))) = lexer.next() {
            // Successfully parsed zero
        } else {
            panic!("Failed to parse zero literal");
        }
    }

    #[test]
    fn test_range() {
        struct RangeTestCase {
            input: &'static str,
            expected: Quantifier,
        }
        let test_cases = vec![
            RangeTestCase {
                input: "{1, 5}",
                expected: Quantifier::new(1..=5, Reluctance::default()),
            },
            RangeTestCase {
                input: "{ 3 , }",
                expected: Quantifier::new(3.., Reluctance::default()),
            },
            RangeTestCase {
                input: "{ 5 }",
                expected: Quantifier::new(5..=5, Reluctance::default()),
            },
            RangeTestCase {
                input: "{1, 5 }?",
                expected: Quantifier::new(1..=5, Reluctance::Lazy),
            },
            RangeTestCase {
                input: "{ 3 , }?",
                expected: Quantifier::new(3.., Reluctance::Lazy),
            },
            RangeTestCase {
                input: "{5}?",
                expected: Quantifier::new(5..=5, Reluctance::Lazy),
            },
            RangeTestCase {
                input: "{ 1,5}+",
                expected: Quantifier::new(1..=5, Reluctance::Possessive),
            },
            RangeTestCase {
                input: "{ 3 , }+",
                expected: Quantifier::new(3.., Reluctance::Possessive),
            },
            RangeTestCase {
                input: "{5}+",
                expected: Quantifier::new(5..=5, Reluctance::Possessive),
            },
        ];

        let mut failed_cases = vec![];

        for test_case in test_cases {
            let mut lexer = Token::lexer(test_case.input);
            if let Some(Ok(Token::Range(Ok(range)))) = lexer.next() {
                assert_eq!(range, test_case.expected);
            } else {
                failed_cases.push(test_case.input);
            }
        }

        if !failed_cases.is_empty() {
            panic!("Failed to parse ranges: {:?}", failed_cases);
        }
    }
}
