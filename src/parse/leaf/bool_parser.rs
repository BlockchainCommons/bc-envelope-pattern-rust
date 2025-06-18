use crate::parse::Token;
use crate::{Error, Pattern, Result};

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
