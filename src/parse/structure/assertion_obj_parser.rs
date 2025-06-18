use super::super::{Token, meta};
use crate::{Error, Pattern, Result};

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
