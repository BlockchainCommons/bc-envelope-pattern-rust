// Parsers for structure-level pattern syntax

use bc_components::Digest;
use bc_envelope::prelude::URDecodable;

use super::{Token, meta};
use crate::{Error, Pattern, Result};

pub(crate) fn parse_node(lexer: &mut logos::Lexer<Token>) -> Result<Pattern> {
    let mut lookahead = lexer.clone();
    match lookahead.next() {
        Some(Ok(Token::ParenOpen)) => {
            lexer.next();
            match lexer.next() {
                Some(Ok(Token::Range(res))) => {
                    let range = res?;
                    let pat = if let Some(max) = range.max() {
                        Pattern::node_with_assertions_range(range.min()..=max)
                    } else {
                        Pattern::node_with_assertions_range(range.min()..)
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
        _ => Ok(Pattern::any_node()),
    }
}

pub(crate) fn parse_wrapped(
    _lexer: &mut logos::Lexer<Token>,
) -> Result<Pattern> {
    Ok(Pattern::wrapped())
}

pub(crate) fn parse_subject(
    lexer: &mut logos::Lexer<Token>,
) -> Result<Pattern> {
    let mut lookahead = lexer.clone();
    match lookahead.next() {
        Some(Ok(Token::ParenOpen)) => {
            lexer.next();
            let pattern = meta::parse_or(lexer)?;
            match lexer.next() {
                Some(Ok(Token::ParenClose)) => Ok(Pattern::subject(pattern)),
                Some(Ok(t)) => {
                    Err(Error::UnexpectedToken(Box::new(t), lexer.span()))
                }
                Some(Err(e)) => Err(e),
                None => Err(Error::ExpectedCloseParen(lexer.span())),
            }
        }
        _ => Ok(Pattern::any_subject()),
    }
}

pub(crate) fn parse_assertion(
    _lexer: &mut logos::Lexer<Token>,
) -> Result<Pattern> {
    Ok(Pattern::any_assertion())
}

pub(crate) fn parse_assertion_pred(
    lexer: &mut logos::Lexer<Token>,
) -> Result<Pattern> {
    match lexer.next() {
        Some(Ok(Token::ParenOpen)) => {
            let pattern = meta::parse_or(lexer)?;
            match lexer.next() {
                Some(Ok(Token::ParenClose)) => {
                    Ok(Pattern::assertion_with_predicate(pattern))
                }
                Some(Ok(t)) => {
                    Err(Error::UnexpectedToken(Box::new(t), lexer.span()))
                }
                Some(Err(e)) => Err(e),
                None => Err(Error::ExpectedCloseParen(lexer.span())),
            }
        }
        Some(Ok(t)) => Err(Error::UnexpectedToken(Box::new(t), lexer.span())),
        Some(Err(e)) => Err(e),
        None => Err(Error::UnexpectedEndOfInput),
    }
}

pub(crate) fn parse_assertion_obj(
    lexer: &mut logos::Lexer<Token>,
) -> Result<Pattern> {
    match lexer.next() {
        Some(Ok(Token::ParenOpen)) => {
            let pattern = meta::parse_or(lexer)?;
            match lexer.next() {
                Some(Ok(Token::ParenClose)) => {
                    Ok(Pattern::assertion_with_object(pattern))
                }
                Some(Ok(t)) => {
                    Err(Error::UnexpectedToken(Box::new(t), lexer.span()))
                }
                Some(Err(e)) => Err(e),
                None => Err(Error::ExpectedCloseParen(lexer.span())),
            }
        }
        Some(Ok(t)) => Err(Error::UnexpectedToken(Box::new(t), lexer.span())),
        Some(Err(e)) => Err(e),
        None => Err(Error::UnexpectedEndOfInput),
    }
}

pub(crate) fn parse_object(lexer: &mut logos::Lexer<Token>) -> Result<Pattern> {
    let mut lookahead = lexer.clone();
    match lookahead.next() {
        Some(Ok(Token::ParenOpen)) => {
            lexer.next();
            let pat = meta::parse_or(lexer)?;
            match lexer.next() {
                Some(Ok(Token::ParenClose)) => Ok(Pattern::object(pat)),
                Some(Ok(t)) => {
                    Err(Error::UnexpectedToken(Box::new(t), lexer.span()))
                }
                Some(Err(e)) => Err(e),
                None => Err(Error::ExpectedCloseParen(lexer.span())),
            }
        }
        _ => Ok(Pattern::any_object()),
    }
}

pub(crate) fn parse_digest(lexer: &mut logos::Lexer<Token>) -> Result<Pattern> {
    match lexer.next() {
        Some(Ok(Token::ParenOpen)) => {
            let src = lexer.remainder();
            let (pattern, consumed) = parse_digest_inner(src)?;
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
        Some(Ok(t)) => Err(Error::UnexpectedToken(Box::new(t), lexer.span())),
        Some(Err(e)) => Err(e),
        None => Err(Error::UnexpectedEndOfInput),
    }
}

fn parse_digest_inner(src: &str) -> Result<(Pattern, usize)> {
    let mut pos = 0;
    super::utils::skip_ws(src, &mut pos);
    if src[pos..].starts_with("ur:") {
        let start = pos;
        while let Some(ch) = src[pos..].chars().next() {
            if ch == ')' {
                break;
            }
            pos += ch.len_utf8();
        }
        let ur = src[start..pos].trim_end();
        let digest = Digest::from_ur_string(ur)
            .map_err(|_| Error::InvalidUr(ur.to_string(), pos..pos))?;
        super::utils::skip_ws(src, &mut pos);
        Ok((Pattern::digest(digest), pos))
    } else {
        let start = pos;
        while let Some(ch) = src[pos..].chars().next() {
            if ch.is_ascii_hexdigit() {
                pos += ch.len_utf8();
            } else {
                break;
            }
        }
        if start == pos {
            return Err(Error::InvalidHexString(pos..pos));
        }
        let hex_str = &src[start..pos];
        if hex_str.len() % 2 != 0 {
            return Err(Error::InvalidHexString(pos..pos));
        }
        let bytes = hex::decode(hex_str)
            .map_err(|_| Error::InvalidHexString(pos..pos))?;
        if bytes.len() > Digest::DIGEST_SIZE {
            return Err(Error::InvalidHexString(pos..pos));
        }
        super::utils::skip_ws(src, &mut pos);
        Ok((Pattern::digest_prefix(bytes), pos))
    }
}

pub(crate) fn parse_predicate(
    lexer: &mut logos::Lexer<Token>,
) -> Result<Pattern> {
    let mut lookahead = lexer.clone();
    match lookahead.next() {
        Some(Ok(Token::ParenOpen)) => {
            lexer.next();
            let pat = meta::parse_or(lexer)?;
            match lexer.next() {
                Some(Ok(Token::ParenClose)) => Ok(Pattern::predicate(pat)),
                Some(Ok(t)) => {
                    Err(Error::UnexpectedToken(Box::new(t), lexer.span()))
                }
                Some(Err(e)) => Err(e),
                None => Err(Error::ExpectedCloseParen(lexer.span())),
            }
        }
        _ => Ok(Pattern::any_predicate()),
    }
}

pub(crate) fn parse_obscured(
    _lexer: &mut logos::Lexer<Token>,
) -> Result<Pattern> {
    Ok(Pattern::obscured())
}

pub(crate) fn parse_elided(
    _lexer: &mut logos::Lexer<Token>,
) -> Result<Pattern> {
    Ok(Pattern::elided())
}

pub(crate) fn parse_encrypted(
    _lexer: &mut logos::Lexer<Token>,
) -> Result<Pattern> {
    Ok(Pattern::encrypted())
}

pub(crate) fn parse_compressed(
    _lexer: &mut logos::Lexer<Token>,
) -> Result<Pattern> {
    Ok(Pattern::compressed())
}
