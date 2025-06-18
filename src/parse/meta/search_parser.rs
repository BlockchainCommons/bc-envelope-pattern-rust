use super::super::Token;
use super::or_parser::parse_or;
use crate::{Error, Pattern, Result};

pub(crate) fn parse_search(lexer: &mut logos::Lexer<Token>) -> Result<Pattern> {
    match lexer.next() {
        Some(Ok(Token::ParenOpen)) => {
            let pat = parse_or(lexer)?;
            match lexer.next() {
                Some(Ok(Token::ParenClose)) => Ok(Pattern::search(pat)),
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
