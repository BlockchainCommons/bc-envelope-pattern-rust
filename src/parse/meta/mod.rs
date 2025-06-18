// Parsers for meta-pattern operators

use super::{Token, leaf};
use crate::{Error, Pattern, Result};

pub(crate) fn parse_or(lexer: &mut logos::Lexer<Token>) -> Result<Pattern> {
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

pub(crate) fn parse_sequence(
    lexer: &mut logos::Lexer<Token>,
) -> Result<Pattern> {
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

pub(crate) fn parse_and(lexer: &mut logos::Lexer<Token>) -> Result<Pattern> {
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
        Token::Array => leaf::parse_array(lexer),
        Token::Bool => leaf::parse_bool(lexer),
        Token::ByteString => leaf::parse_byte_string(lexer),
        Token::Date => leaf::parse_date(lexer),
        Token::Tag => leaf::parse_tag(lexer),
        Token::Known => leaf::parse_known_value(lexer),
        Token::Leaf => Ok(Pattern::any_leaf()),
        Token::Cbor => leaf::parse_cbor(lexer),
        Token::Map => leaf::parse_map(lexer),
        Token::None => Ok(Pattern::none()),
        Token::Null => leaf::parse_null(lexer),
        Token::Number => leaf::parse_number(lexer),
        Token::Text => leaf::parse_text(lexer),
        t => Err(Error::UnexpectedToken(Box::new(t), lexer.span())),
    }
}
