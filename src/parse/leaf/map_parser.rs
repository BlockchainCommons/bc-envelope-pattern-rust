use crate::{Error, Pattern, Result, parse::Token};

/// Legacy map parser - no longer used after migration to dcbor-pattern map syntax
#[allow(dead_code)]
#[deprecated(note = "Use dcbor-pattern map syntax instead: {*}, {{n}}, {{n,m}}, {{n,}}, {key: value}")]
pub(crate) fn parse_map(lexer: &mut logos::Lexer<Token>) -> Result<Pattern> {
    let mut lookahead = lexer.clone();
    match lookahead.next() {
        Some(Ok(Token::ParenOpen)) => {
            lexer.next();
            match lexer.next() {
                Some(Ok(Token::UnsignedInteger(res))) => {
                    let count = res?;
                    match lexer.next() {
                        Some(Ok(Token::ParenClose)) => {
                            Ok(Pattern::map_with_count(count))
                        }
                        Some(Ok(t)) => Err(Error::UnexpectedToken(
                            Box::new(t),
                            lexer.span(),
                        )),
                        Some(Err(e)) => Err(e),
                        None => Err(Error::ExpectedCloseParen(lexer.span())),
                    }
                }
                Some(Ok(Token::Integer(res))) => {
                    let count = res?;
                    if count < 0 {
                        return Err(Error::InvalidNumberFormat(lexer.span()));
                    }
                    match lexer.next() {
                        Some(Ok(Token::ParenClose)) => {
                            Ok(Pattern::map_with_count(count as usize))
                        }
                        Some(Ok(t)) => Err(Error::UnexpectedToken(
                            Box::new(t),
                            lexer.span(),
                        )),
                        Some(Err(e)) => Err(e),
                        None => Err(Error::ExpectedCloseParen(lexer.span())),
                    }
                }
                Some(Ok(Token::Range(res))) => {
                    let range = res?;
                    let pat = if let Some(max) = range.max() {
                        Pattern::map_with_range(range.min()..=max)
                    } else {
                        Pattern::map_with_range(range.min()..)
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
        _ => Ok(Pattern::any_map()),
    }
}
