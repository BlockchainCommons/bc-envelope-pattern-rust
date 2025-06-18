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
