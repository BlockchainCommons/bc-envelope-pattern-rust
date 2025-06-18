// Parsers for structure-level pattern syntax

use super::{meta, Token};
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
        _ => Ok(Pattern::any_node()),
    }
}

pub(crate) fn parse_wrapped(_lexer: &mut logos::Lexer<Token>) -> Result<Pattern> {
    Ok(Pattern::wrapped())
}

pub(crate) fn parse_subject(lexer: &mut logos::Lexer<Token>) -> Result<Pattern> {
    let mut lookahead = lexer.clone();
    match lookahead.next() {
        Some(Ok(Token::ParenOpen)) => {
            lexer.next();
            let pattern = meta::parse_or(lexer)?;
            match lexer.next() {
                Some(Ok(Token::ParenClose)) => Ok(Pattern::subject(pattern)),
                Some(Ok(t)) => Err(Error::UnexpectedToken(Box::new(t), lexer.span())),
                Some(Err(e)) => Err(e),
                None => Err(Error::ExpectedCloseParen(lexer.span())),
            }
        }
        _ => Ok(Pattern::any_subject()),
    }
}

pub(crate) fn parse_assertion(_lexer: &mut logos::Lexer<Token>) -> Result<Pattern> {
    Ok(Pattern::any_assertion())
}

pub(crate) fn parse_assertion_pred(lexer: &mut logos::Lexer<Token>) -> Result<Pattern> {
    match lexer.next() {
        Some(Ok(Token::ParenOpen)) => {
            let pattern = meta::parse_or(lexer)?;
            match lexer.next() {
                Some(Ok(Token::ParenClose)) => {
                    Ok(Pattern::assertion_with_predicate(pattern))
                }
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

pub(crate) fn parse_assertion_obj(lexer: &mut logos::Lexer<Token>) -> Result<Pattern> {
    match lexer.next() {
        Some(Ok(Token::ParenOpen)) => {
            let pattern = meta::parse_or(lexer)?;
            match lexer.next() {
                Some(Ok(Token::ParenClose)) => {
                    Ok(Pattern::assertion_with_object(pattern))
                }
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

pub(crate) fn parse_object(lexer: &mut logos::Lexer<Token>) -> Result<Pattern> {
    let mut lookahead = lexer.clone();
    match lookahead.next() {
        Some(Ok(Token::ParenOpen)) => {
            lexer.next();
            let pat = meta::parse_or(lexer)?;
            match lexer.next() {
                Some(Ok(Token::ParenClose)) => Ok(Pattern::object(pat)),
                Some(Ok(t)) => Err(Error::UnexpectedToken(Box::new(t), lexer.span())),
                Some(Err(e)) => Err(e),
                None => Err(Error::ExpectedCloseParen(lexer.span())),
            }
        }
        _ => Ok(Pattern::any_object()),
    }
}

pub(crate) fn parse_predicate(lexer: &mut logos::Lexer<Token>) -> Result<Pattern> {
    let mut lookahead = lexer.clone();
    match lookahead.next() {
        Some(Ok(Token::ParenOpen)) => {
            lexer.next();
            let pat = meta::parse_or(lexer)?;
            match lexer.next() {
                Some(Ok(Token::ParenClose)) => Ok(Pattern::predicate(pat)),
                Some(Ok(t)) => Err(Error::UnexpectedToken(Box::new(t), lexer.span())),
                Some(Err(e)) => Err(e),
                None => Err(Error::ExpectedCloseParen(lexer.span())),
            }
        }
        _ => Ok(Pattern::any_predicate()),
    }
}

pub(crate) fn parse_obscured(_lexer: &mut logos::Lexer<Token>) -> Result<Pattern> {
    Ok(Pattern::obscured())
}

pub(crate) fn parse_elided(_lexer: &mut logos::Lexer<Token>) -> Result<Pattern> {
    Ok(Pattern::elided())
}

pub(crate) fn parse_encrypted(_lexer: &mut logos::Lexer<Token>) -> Result<Pattern> {
    Ok(Pattern::encrypted())
}

pub(crate) fn parse_compressed(_lexer: &mut logos::Lexer<Token>) -> Result<Pattern> {
    Ok(Pattern::compressed())
}
