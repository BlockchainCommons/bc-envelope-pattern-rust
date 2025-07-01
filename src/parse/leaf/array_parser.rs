use crate::{Error, Pattern, Result, parse::{Token, utils}};

pub(crate) fn parse_array(lexer: &mut logos::Lexer<Token>) -> Result<Pattern> {
    // We're at the '[' token, now need to parse until ']'
    let src = lexer.remainder();
    let (pattern, consumed) = utils::parse_array_inner(src)?;
    lexer.bump(consumed);
    match lexer.next() {
        Some(Ok(Token::BracketClose)) => Ok(pattern),
        Some(Ok(t)) => Err(Error::UnexpectedToken(
            Box::new(t),
            lexer.span(),
        )),
        Some(Err(e)) => Err(e),
        None => Err(Error::ExpectedCloseBracket(lexer.span())),
    }
}
