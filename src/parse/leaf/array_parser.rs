use crate::{Error, Pattern, Result, parse::Token};

pub(crate) fn parse_array(lexer: &mut logos::Lexer<Token>) -> Result<Pattern> {
    let mut lookahead = lexer.clone();
    match lookahead.next() {
        Some(Ok(Token::ParenOpen)) => {
            lexer.next();
            match lexer.next() {
                Some(Ok(Token::Range(res))) => {
                    let range = res?;
                    let pat = if let Some(max) = range.max() {
                        Pattern::array_with_range(range.min()..=max)
                    } else {
                        Pattern::array_with_range(range.min()..)
                    };
                    match lexer.next() {
                        Some(Ok(Token::ParenClose)) => Ok(pat),
                        Some(Ok(t)) => Err(Error::UnexpectedToken(
                            Box::new(t),
                            lexer.span(),
                        )),
                        Some(Err(e)) => Err(e),
                        None => Err(Error::ExpectedCloseParen(lexer.span())),
                    }
                }
                Some(Ok(t)) => {
                    Err(Error::UnexpectedToken(Box::new(t), lexer.span()))
                }
                Some(Err(e)) => Err(e),
                None => Err(Error::UnexpectedEndOfInput),
            }
        }
        _ => Ok(Pattern::any_array()),
    }
}
