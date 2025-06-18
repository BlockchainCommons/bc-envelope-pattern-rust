use crate::parse::{Token, utils};
use crate::{Error, Pattern, Result};

pub(crate) fn parse_byte_string(
    lexer: &mut logos::Lexer<Token>,
) -> Result<Pattern> {
    let mut lookahead = lexer.clone();
    match lookahead.next() {
        Some(Ok(Token::ParenOpen)) => {
            lexer.next();
            let src = lexer.remainder();
            let (pattern, consumed) = if src.trim_start().starts_with('/') {
                let (regex, used) = utils::parse_binary_regex(src)?;
                (Pattern::byte_string_binary_regex(regex), used)
            } else {
                let (bytes, used) = utils::parse_hex_string(src)?;
                (Pattern::byte_string(bytes), used)
            };
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
        _ => Ok(Pattern::any_byte_string()),
    }
}
