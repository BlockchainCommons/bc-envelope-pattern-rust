// Parsers for leaf-level pattern syntax

use super::{Token, utils};
use crate::{Error, Pattern, Result};
use known_values::KnownValue;

pub(crate) fn parse_bool(lexer: &mut logos::Lexer<Token>) -> Result<Pattern> {
    let mut lookahead = lexer.clone();
    match lookahead.next() {
        Some(Ok(Token::ParenOpen)) => {
            lexer.next();

            let value_token = match lexer.next() {
                Some(Ok(tok)) => tok,
                Some(Err(e)) => return Err(e),
                None => return Err(Error::UnexpectedEndOfInput),
            };

            let value = match value_token {
                Token::BoolTrue => true,
                Token::BoolFalse => false,
                t => {
                    return Err(Error::UnexpectedToken(
                        Box::new(t),
                        lexer.span(),
                    ));
                }
            };

            match lexer.next() {
                Some(Ok(Token::ParenClose)) => Ok(Pattern::bool(value)),
                Some(Ok(t)) => {
                    Err(Error::UnexpectedToken(Box::new(t), lexer.span()))
                }
                Some(Err(e)) => Err(e),
                None => Err(Error::ExpectedCloseParen(lexer.span())),
            }
        }
        _ => Ok(Pattern::any_bool()),
    }
}

pub(crate) fn parse_text(lexer: &mut logos::Lexer<Token>) -> Result<Pattern> {
    let mut lookahead = lexer.clone();
    match lookahead.next() {
        Some(Ok(Token::ParenOpen)) => {
            lexer.next();

            let mut la = lexer.clone();
            match la.next() {
                Some(Ok(Token::Regex(_))) => {
                    if let Some(Ok(Token::Regex(res))) = lexer.next() {
                        let regex = regex::Regex::new(&res?)
                            .map_err(|_| Error::InvalidRegex(lexer.span()))?;
                        match lexer.next() {
                            Some(Ok(Token::ParenClose)) => {
                                Ok(Pattern::text_regex(regex))
                            }
                            Some(Ok(t)) => Err(Error::UnexpectedToken(
                                Box::new(t),
                                lexer.span(),
                            )),
                            Some(Err(e)) => Err(e),
                            None => {
                                Err(Error::ExpectedCloseParen(lexer.span()))
                            }
                        }
                    } else {
                        Err(Error::UnexpectedEndOfInput)
                    }
                }
                _ => {
                    let src = lexer.remainder();
                    let (value, consumed) = utils::parse_string_literal(src)?;
                    lexer.bump(consumed);
                    match lexer.next() {
                        Some(Ok(Token::ParenClose)) => Ok(Pattern::text(value)),
                        Some(Ok(t)) => Err(Error::UnexpectedToken(
                            Box::new(t),
                            lexer.span(),
                        )),
                        Some(Err(e)) => Err(e),
                        None => Err(Error::ExpectedCloseParen(lexer.span())),
                    }
                }
            }
        }
        _ => Ok(Pattern::any_text()),
    }
}

pub(crate) fn parse_array(lexer: &mut logos::Lexer<Token>) -> Result<Pattern> {
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
                        Some(Ok(t)) => Err(Error::UnexpectedToken(
                            Box::new(t),
                            lexer.span(),
                        )),
                        Some(Err(e)) => Err(e),
                        None => Err(Error::ExpectedCloseParen(lexer.span())),
                    }
                }
                Some(Ok(t)) => {
                    Err(Error::UnexpectedToken(Box::new(t), lexer.span()))
                }
                Some(Err(e)) => Err(e),
                None => Err(Error::UnexpectedEndOfInput),
            }
        }
        _ => Ok(Pattern::any_array()),
    }
}

pub(crate) fn parse_byte_string(
    lexer: &mut logos::Lexer<Token>,
) -> Result<Pattern> {
    let mut lookahead = lexer.clone();
    match lookahead.next() {
        Some(Ok(Token::ParenOpen)) => {
            lexer.next();
            let src = lexer.remainder();
            let (pattern, consumed) = if src.trim_start().starts_with('/') {
                let (regex, used) = utils::parse_binary_regex(src)?;
                (Pattern::byte_string_binary_regex(regex), used)
            } else {
                let (bytes, used) = utils::parse_hex_string(src)?;
                (Pattern::byte_string(bytes), used)
            };
            lexer.bump(consumed);
            match lexer.next() {
                Some(Ok(Token::ParenClose)) => Ok(pattern),
                Some(Ok(t)) => {
                    Err(Error::UnexpectedToken(Box::new(t), lexer.span()))
                }
                Some(Err(e)) => Err(e),
                None => Err(Error::ExpectedCloseParen(lexer.span())),
            }
        }
        _ => Ok(Pattern::any_byte_string()),
    }
}

pub(crate) fn parse_cbor(lexer: &mut logos::Lexer<Token>) -> Result<Pattern> {
    let mut lookahead = lexer.clone();
    match lookahead.next() {
        Some(Ok(Token::ParenOpen)) => {
            lexer.next();
            let src = lexer.remainder();
            let (pattern, consumed) = utils::parse_cbor_inner(src)?;
            lexer.bump(consumed);
            match lexer.next() {
                Some(Ok(Token::ParenClose)) => Ok(pattern),
                Some(Ok(t)) => Err(Error::UnexpectedToken(Box::new(t), lexer.span())),
                Some(Err(e)) => Err(e),
                None => Err(Error::ExpectedCloseParen(lexer.span())),
            }
        }
        _ => Ok(Pattern::any_cbor()),
    }
}

pub(crate) fn parse_number(lexer: &mut logos::Lexer<Token>) -> Result<Pattern> {
    let mut lookahead = lexer.clone();
    match lookahead.next() {
        Some(Ok(Token::ParenOpen)) => {
            lexer.next();

            let src = lexer.remainder();
            let (pattern, consumed) = utils::parse_number_inner(src)?;
            lexer.bump(consumed);

            match lexer.next() {
                Some(Ok(Token::ParenClose)) => Ok(pattern),
                Some(Ok(t)) => {
                    Err(Error::UnexpectedToken(Box::new(t), lexer.span()))
                }
                Some(Err(e)) => Err(e),
                None => Err(Error::ExpectedCloseParen(lexer.span())),
            }
        }
        _ => Ok(Pattern::any_number()),
    }
}

pub(crate) fn parse_date(lexer: &mut logos::Lexer<Token>) -> Result<Pattern> {
    let mut lookahead = lexer.clone();
    match lookahead.next() {
        Some(Ok(Token::ParenOpen)) => {
            lexer.next();
            let src = lexer.remainder();
            let (pattern, consumed) = utils::parse_date_inner(src)?;
            lexer.bump(consumed);
            match lexer.next() {
                Some(Ok(Token::ParenClose)) => Ok(pattern),
                Some(Ok(t)) => {
                    Err(Error::UnexpectedToken(Box::new(t), lexer.span()))
                }
                Some(Err(e)) => Err(e),
                None => Err(Error::ExpectedCloseParen(lexer.span())),
            }
        }
        _ => Ok(Pattern::any_date()),
    }
}

pub(crate) fn parse_map(lexer: &mut logos::Lexer<Token>) -> Result<Pattern> {
    let mut lookahead = lexer.clone();
    match lookahead.next() {
        Some(Ok(Token::ParenOpen)) => {
            lexer.next();
            match lexer.next() {
                Some(Ok(Token::UnsignedInteger(res))) => {
                    let count = res?;
                    match lexer.next() {
                        Some(Ok(Token::ParenClose)) => {
                            Ok(Pattern::map_with_count(count))
                        }
                        Some(Ok(t)) => Err(Error::UnexpectedToken(
                            Box::new(t),
                            lexer.span(),
                        )),
                        Some(Err(e)) => Err(e),
                        None => Err(Error::ExpectedCloseParen(lexer.span())),
                    }
                }
                Some(Ok(Token::Range(res))) => {
                    let range = res?;
                    let pat = if let Some(max) = range.max() {
                        Pattern::map_with_range(range.min()..=max)
                    } else {
                        Pattern::map_with_range(range.min()..)
                    };
                    match lexer.next() {
                        Some(Ok(Token::ParenClose)) => Ok(pat),
                        Some(Ok(t)) => Err(Error::UnexpectedToken(
                            Box::new(t),
                            lexer.span(),
                        )),
                        Some(Err(e)) => Err(e),
                        None => Err(Error::ExpectedCloseParen(lexer.span())),
                    }
                }
                Some(Ok(t)) => {
                    Err(Error::UnexpectedToken(Box::new(t), lexer.span()))
                }
                Some(Err(e)) => Err(e),
                None => Err(Error::UnexpectedEndOfInput),
            }
        }
        _ => Ok(Pattern::any_map()),
    }
}

pub(crate) fn parse_tag(lexer: &mut logos::Lexer<Token>) -> Result<Pattern> {
    let mut lookahead = lexer.clone();
    match lookahead.next() {
        Some(Ok(Token::ParenOpen)) => {
            lexer.next();
            let src = lexer.remainder();
            let (pattern, consumed) = parse_tag_inner(src)?;
            lexer.bump(consumed);
            match lexer.next() {
                Some(Ok(Token::ParenClose)) => Ok(pattern),
                Some(Ok(t)) => Err(Error::UnexpectedToken(Box::new(t), lexer.span())),
                Some(Err(e)) => Err(e),
                None => Err(Error::ExpectedCloseParen(lexer.span())),
            }
        }
        _ => Ok(Pattern::any_tag()),
    }
}

fn parse_tag_inner(src: &str) -> Result<(Pattern, usize)> {
    let mut pos = 0;
    utils::skip_ws(src, &mut pos);
    if src[pos..].starts_with('/') {
        let (regex, used) = utils::parse_text_regex(&src[pos..])?;
        pos += used;
        return Ok((Pattern::tagged_with_regex(regex), pos));
    }

    let (word, used) = utils::parse_bare_word(&src[pos..])?;
    pos += used;
    if let Ok(value) = word.parse::<u64>() {
        Ok((Pattern::tagged_with_value(value), pos))
    } else {
        Ok((Pattern::tagged_with_name(word), pos))
    }
}

pub(crate) fn parse_known_value(lexer: &mut logos::Lexer<Token>) -> Result<Pattern> {
    let mut lookahead = lexer.clone();
    match lookahead.next() {
        Some(Ok(Token::ParenOpen)) => {
            lexer.next();
            let src = lexer.remainder();
            let (pattern, consumed) = parse_known_value_inner(src)?;
            lexer.bump(consumed);
            match lexer.next() {
                Some(Ok(Token::ParenClose)) => Ok(pattern),
                Some(Ok(t)) => Err(Error::UnexpectedToken(Box::new(t), lexer.span())),
                Some(Err(e)) => Err(e),
                None => Err(Error::ExpectedCloseParen(lexer.span())),
            }
        }
        _ => Ok(Pattern::any_known_value()),
    }
}

fn parse_known_value_inner(src: &str) -> Result<(Pattern, usize)> {
    let mut pos = 0;
    utils::skip_ws(src, &mut pos);
    if src[pos..].starts_with('/') {
        let (regex, used) = utils::parse_text_regex(&src[pos..])?;
        pos += used;
        return Ok((Pattern::known_value_regex(regex), pos));
    }

    let (inner, used) = utils::parse_single_quoted(&src[pos..])?;
    pos += used;
    if let Ok(value) = inner.parse::<u64>() {
        Ok((Pattern::known_value(KnownValue::new(value)), pos))
    } else {
        Ok((Pattern::known_value_named(inner), pos))
    }
}

pub(crate) fn parse_null(_lexer: &mut logos::Lexer<Token>) -> Result<Pattern> {
    Ok(Pattern::null())
}
