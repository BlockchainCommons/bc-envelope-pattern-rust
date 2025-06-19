use super::super::{Token, meta};
use crate::{Error, Pattern, Result};

pub(crate) fn parse_wrapped(
    _lexer: &mut logos::Lexer<Token>,
) -> Result<Pattern> {
    // Simple WRAPPED pattern without arguments
    Ok(Pattern::wrapped())
}

pub(crate) fn parse_unwrap(lexer: &mut logos::Lexer<Token>) -> Result<Pattern> {
    let mut lookahead = lexer.clone();
    match lookahead.next() {
        Some(Ok(Token::ParenOpen)) => {
            // UNWRAP ( pattern )
            lexer.next();
            let pat = meta::parse_or(lexer)?;
            match lexer.next() {
                Some(Ok(Token::ParenClose)) => {
                    Ok(Pattern::unwrap_matching(pat))
                }
                Some(Ok(t)) => {
                    Err(Error::UnexpectedToken(Box::new(t), lexer.span()))
                }
                Some(Err(e)) => Err(e),
                None => Err(Error::ExpectedCloseParen(lexer.span())),
            }
        }
        _ => {
            // Simple UNWRAP pattern without arguments
            Ok(Pattern::unwrap())
        }
    }
}
