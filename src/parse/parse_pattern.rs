use logos::Logos;

use super::Token;
use crate::{Error, Pattern, Result};

/// Parse a pattern expression.
///
/// This handles a small subset of the grammar consisting of the `ANY`, `NONE`
/// and `BOOL` primitives and the meta-pattern operators `&`, `>` and `|`.
pub fn parse_pattern(input: impl AsRef<str>) -> Result<Pattern> {
    let mut lexer = Token::lexer(input.as_ref());

    let pattern = parse_or(&mut lexer)?;

    match lexer.next() {
        None => Ok(pattern),
        Some(Ok(_)) => Err(Error::ExtraData(lexer.span())),
        Some(Err(e)) => Err(e),
    }
}

fn parse_or(lexer: &mut logos::Lexer<Token>) -> Result<Pattern> {
    let mut patterns = vec![parse_sequence(lexer)?];

    loop {
        let mut lookahead = lexer.clone();
        match lookahead.next() {
            Some(Ok(Token::Or)) => {
                // consume the '|'
                lexer.next();
                patterns.push(parse_sequence(lexer)?);
            }
            _ => break,
        }
    }

    if patterns.len() == 1 {
        Ok(patterns.remove(0))
    } else {
        Ok(Pattern::or(patterns))
    }
}

fn parse_sequence(lexer: &mut logos::Lexer<Token>) -> Result<Pattern> {
    let mut patterns = vec![parse_and(lexer)?];

    loop {
        let mut lookahead = lexer.clone();
        match lookahead.next() {
            Some(Ok(Token::Sequence)) => {
                lexer.next();
                patterns.push(parse_and(lexer)?);
            }
            _ => break,
        }
    }

    if patterns.len() == 1 {
        Ok(patterns.remove(0))
    } else {
        Ok(Pattern::sequence(patterns))
    }
}

fn parse_and(lexer: &mut logos::Lexer<Token>) -> Result<Pattern> {
    let mut patterns = vec![parse_primary(lexer)?];

    loop {
        let mut lookahead = lexer.clone();
        match lookahead.next() {
            Some(Ok(Token::And)) => {
                lexer.next();
                patterns.push(parse_primary(lexer)?);
            }
            _ => break,
        }
    }

    if patterns.len() == 1 {
        Ok(patterns.remove(0))
    } else {
        Ok(Pattern::and(patterns))
    }
}

fn parse_primary(lexer: &mut logos::Lexer<Token>) -> Result<Pattern> {
    let token = match lexer.next() {
        Some(Ok(tok)) => tok,
        Some(Err(e)) => return Err(e),
        None => return Err(Error::UnexpectedEndOfInput),
    };

    match token {
        Token::Any => Ok(Pattern::any()),
        Token::None => Ok(Pattern::none()),
        Token::Bool => parse_bool(lexer),
        Token::Text => parse_text(lexer),
        Token::Number => parse_number(lexer),
        Token::Leaf => Ok(Pattern::any_leaf()),
        Token::Array => parse_array(lexer),
        Token::ByteString => parse_byte_string(lexer),
        t => Err(Error::UnexpectedToken(Box::new(t), lexer.span())),
    }
}

fn parse_bool(lexer: &mut logos::Lexer<Token>) -> Result<Pattern> {
    let mut lookahead = lexer.clone();
    match lookahead.next() {
        Some(Ok(Token::ParenOpen)) => {
            // consume '(' from the real lexer
            lexer.next();

            let value_token = match lexer.next() {
                Some(Ok(tok)) => tok,
                Some(Err(e)) => return Err(e),
                None => return Err(Error::UnexpectedEndOfInput),
            };

            let value = match value_token {
                Token::BoolTrue => true,
                Token::BoolFalse => false,
                t => return Err(Error::UnexpectedToken(Box::new(t), lexer.span())),
            };

            match lexer.next() {
                Some(Ok(Token::ParenClose)) => Ok(Pattern::bool(value)),
                Some(Ok(t)) => Err(Error::UnexpectedToken(Box::new(t), lexer.span())),
                Some(Err(e)) => Err(e),
                None => Err(Error::ExpectedCloseParen(lexer.span())),
            }
        }
        _ => Ok(Pattern::any_bool()),
    }
}

fn parse_text(lexer: &mut logos::Lexer<Token>) -> Result<Pattern> {
    let mut lookahead = lexer.clone();
    match lookahead.next() {
        Some(Ok(Token::ParenOpen)) => {
            lexer.next();

            let mut la = lexer.clone();
            match la.next() {
                Some(Ok(Token::Regex(_))) => {
                    if let Some(Ok(Token::Regex(res))) = lexer.next() {
                        let regex = regex::Regex::new(&res?).map_err(|_| Error::InvalidRegex(lexer.span()))?;
                        match lexer.next() {
                            Some(Ok(Token::ParenClose)) => Ok(Pattern::text_regex(regex)),
                            Some(Ok(t)) => Err(Error::UnexpectedToken(Box::new(t), lexer.span())),
                            Some(Err(e)) => Err(e),
                            None => Err(Error::ExpectedCloseParen(lexer.span())),
                        }
                    } else {
                        Err(Error::UnexpectedEndOfInput)
                    }
                }
                _ => {
                    let src = lexer.remainder();
                    let (value, consumed) = parse_string_literal(src)?;
                    lexer.bump(consumed);
                    match lexer.next() {
                        Some(Ok(Token::ParenClose)) => Ok(Pattern::text(value)),
                        Some(Ok(t)) => Err(Error::UnexpectedToken(Box::new(t), lexer.span())),
                        Some(Err(e)) => Err(e),
                        None => Err(Error::ExpectedCloseParen(lexer.span())),
                    }
                }
            }
        }
        _ => Ok(Pattern::any_text()),
    }
}

fn parse_string_literal(src: &str) -> Result<(String, usize)> {
    let mut pos = 0;
    while let Some(ch) = src[pos..].chars().next() {
        if matches!(ch, ' ' | '\t' | '\n' | '\r' | '\u{0c}') {
            pos += ch.len_utf8();
        } else {
            break;
        }
    }

    let bytes = src.as_bytes();
    if pos >= bytes.len() || bytes[pos] != b'"' {
        return Err(Error::UnexpectedEndOfInput);
    }
    pos += 1;
    let start = pos;
    let mut escape = false;
    while pos < bytes.len() {
        let b = bytes[pos];
        pos += 1;
        if escape {
            escape = false;
            continue;
        }
        if b == b'\\' {
            escape = true;
            continue;
        }
        if b == b'"' {
            let inner = &src[start..pos - 1];
            let value = inner.replace("\\\"", "\"").replace("\\\\", "\\");
            while let Some(ch) = src[pos..].chars().next() {
                if matches!(ch, ' ' | '\t' | '\n' | '\r' | '\u{0c}') {
                    pos += ch.len_utf8();
                } else {
                    break;
                }
            }
            return Ok((value, pos));
        }
    }
    Err(Error::UnexpectedEndOfInput)
}

fn parse_array(lexer: &mut logos::Lexer<Token>) -> Result<Pattern> {
    let mut lookahead = lexer.clone();
    match lookahead.next() {
        Some(Ok(Token::ParenOpen)) => {
            lexer.next();
            match lexer.next() {
                Some(Ok(Token::Range(res))) => {
                    let range = res?;
                    let pat = if let Some(max) = range.max() {
                        Pattern::array_with_range(range.min()..=max)
                    } else {
                        Pattern::array_with_range(range.min()..)
                    };
                    match lexer.next() {
                        Some(Ok(Token::ParenClose)) => Ok(pat),
                        Some(Ok(t)) => Err(Error::UnexpectedToken(Box::new(t), lexer.span())),
                        Some(Err(e)) => Err(e),
                        None => Err(Error::ExpectedCloseParen(lexer.span())),
                    }
                }
                Some(Ok(t)) => Err(Error::UnexpectedToken(Box::new(t), lexer.span())),
                Some(Err(e)) => Err(e),
                None => Err(Error::UnexpectedEndOfInput),
            }
        }
        _ => Ok(Pattern::any_array()),
    }
}

fn parse_hex_string(src: &str) -> Result<(Vec<u8>, usize)> {
    let mut pos = 0;
    skip_ws(src, &mut pos);
    if !src[pos..].starts_with("h'") {
        return Err(Error::InvalidHexString(pos..pos));
    }
    pos += 2;
    let start = pos;
    while pos < src.len() {
        let b = src.as_bytes()[pos];
        if b == b'\'' {
            let inner = &src[start..pos];
            let bytes = hex::decode(inner).map_err(|_| Error::InvalidHexString(pos..pos))?;
            pos += 1;
            skip_ws(src, &mut pos);
            return Ok((bytes, pos));
        }
        if !(b as char).is_ascii_hexdigit() {
            return Err(Error::InvalidHexString(pos..pos));
        }
        pos += 1;
    }
    Err(Error::InvalidHexString(pos..pos))
}

fn parse_binary_regex(src: &str) -> Result<(regex::bytes::Regex, usize)> {
    let mut pos = 0;
    skip_ws(src, &mut pos);
    if pos >= src.len() || src.as_bytes()[pos] != b'/' {
        return Err(Error::UnterminatedRegex(pos..pos));
    }
    pos += 1;
    let start = pos;
    let mut escape = false;
    while pos < src.len() {
        let b = src.as_bytes()[pos];
        pos += 1;
        if escape {
            escape = false;
            continue;
        }
        if b == b'\\' {
            escape = true;
            continue;
        }
        if b == b'/' {
            let inner = &src[start..pos - 1];
            let regex = regex::bytes::Regex::new(inner)
                .map_err(|_| Error::InvalidRegex(pos..pos))?;
            skip_ws(src, &mut pos);
            return Ok((regex, pos));
        }
    }
    Err(Error::UnterminatedRegex(pos..pos))
}

fn parse_byte_string(lexer: &mut logos::Lexer<Token>) -> Result<Pattern> {
    let mut lookahead = lexer.clone();
    match lookahead.next() {
        Some(Ok(Token::ParenOpen)) => {
            lexer.next();
            let src = lexer.remainder();
            let (pattern, consumed) = if src.trim_start().starts_with('/') {
                let (regex, used) = parse_binary_regex(src)?;
                (Pattern::byte_string_binary_regex(regex), used)
            } else {
                let (bytes, used) = parse_hex_string(src)?;
                (Pattern::byte_string(bytes), used)
            };
            lexer.bump(consumed);
            match lexer.next() {
                Some(Ok(Token::ParenClose)) => Ok(pattern),
                Some(Ok(t)) => Err(Error::UnexpectedToken(Box::new(t), lexer.span())),
                Some(Err(e)) => Err(e),
                None => Err(Error::ExpectedCloseParen(lexer.span())),
            }
        }
        _ => Ok(Pattern::any_byte_string()),
    }
}

fn parse_number(lexer: &mut logos::Lexer<Token>) -> Result<Pattern> {
    let mut lookahead = lexer.clone();
    match lookahead.next() {
        Some(Ok(Token::ParenOpen)) => {
            lexer.next();

            let src = lexer.remainder();
            let (pattern, consumed) = parse_number_inner(src)?;
            lexer.bump(consumed);

            match lexer.next() {
                Some(Ok(Token::ParenClose)) => Ok(pattern),
                Some(Ok(t)) => Err(Error::UnexpectedToken(Box::new(t), lexer.span())),
                Some(Err(e)) => Err(e),
                None => Err(Error::ExpectedCloseParen(lexer.span())),
            }
        }
        _ => Ok(Pattern::any_number()),
    }
}

fn parse_uint_digits(src: &str, pos: &mut usize) -> Result<f64> {
    let start = *pos;
    while let Some(ch) = src[*pos..].chars().next() {
        if ch.is_ascii_digit() {
            *pos += ch.len_utf8();
        } else {
            break;
        }
    }
    if start == *pos {
        return Err(Error::InvalidNumberFormat(0..0));
    }
    src[start..*pos].parse::<f64>().map_err(|_| Error::InvalidNumberFormat(0..0))
}

fn skip_ws(src: &str, pos: &mut usize) {
    while let Some(ch) = src[*pos..].chars().next() {
        if matches!(ch, ' ' | '\t' | '\n' | '\r' | '\u{0c}') {
            *pos += ch.len_utf8();
        } else {
            break;
        }
    }
}

fn parse_number_inner(src: &str) -> Result<(Pattern, usize)> {
    let mut pos = 0;
    skip_ws(src, &mut pos);

    if src[pos..].starts_with("NaN") {
        pos += 3;
        skip_ws(src, &mut pos);
        return Ok((Pattern::number_nan(), pos));
    }

    let cmp = if src[pos..].starts_with(">=") {
        pos += 2;
        Some("ge")
    } else if src[pos..].starts_with("<=") {
        pos += 2;
        Some("le")
    } else if src[pos..].starts_with('>') {
        pos += 1;
        Some("gt")
    } else if src[pos..].starts_with('<') {
        pos += 1;
        Some("lt")
    } else {
        None
    };

    if let Some(tag) = cmp {
        skip_ws(src, &mut pos);
        let val = parse_uint_digits(src, &mut pos)?;
        skip_ws(src, &mut pos);
        let pattern = match tag {
            "ge" => Pattern::number_greater_than_or_equal(val),
            "le" => Pattern::number_less_than_or_equal(val),
            "gt" => Pattern::number_greater_than(val),
            "lt" => Pattern::number_less_than(val),
            _ => unreachable!(),
        };
        return Ok((pattern, pos));
    }

    let first = parse_uint_digits(src, &mut pos)?;
    skip_ws(src, &mut pos);

    if src[pos..].starts_with("...") {
        pos += 3;
        skip_ws(src, &mut pos);
        let second = parse_uint_digits(src, &mut pos)?;
        skip_ws(src, &mut pos);
        return Ok((Pattern::number_range(first..=second), pos));
    }

    Ok((Pattern::number(first), pos))
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_parse_pattern() {
//         let input = "ANY";
//         let pattern = parse_pattern(input).unwrap();
//         assert_eq!(pattern.to_string(), "ANY");
//     }
// }
