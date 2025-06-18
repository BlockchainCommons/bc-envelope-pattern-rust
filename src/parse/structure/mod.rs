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
