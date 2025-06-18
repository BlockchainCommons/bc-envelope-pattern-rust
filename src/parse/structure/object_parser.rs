use super::super::{Token, meta};
use crate::{Error, Pattern, Result};

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
