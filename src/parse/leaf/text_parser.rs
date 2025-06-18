use regex::Regex;

use crate::{
    Error, Pattern, Result,
    parse::{Token, utils},
};

pub(crate) fn parse_text(lexer: &mut logos::Lexer<Token>) -> Result<Pattern> {
    let mut lookahead = lexer.clone();
    match lookahead.next() {
        Some(Ok(Token::ParenOpen)) => {
            lexer.next();

            let mut la = lexer.clone();
            match la.next() {
                Some(Ok(Token::Regex(_))) => {
                    if let Some(Ok(Token::Regex(res))) = lexer.next() {
                        let regex = Regex::new(&res?)
                            .map_err(|_| Error::InvalidRegex(lexer.span()))?;
                        match lexer.next() {
                            Some(Ok(Token::ParenClose)) => {
                                Ok(Pattern::text_regex(regex))
                            }
                            Some(Ok(t)) => Err(Error::UnexpectedToken(
                                Box::new(t),
                                lexer.span(),
                            )),
                            Some(Err(e)) => Err(e),
                            None => {
                                Err(Error::ExpectedCloseParen(lexer.span()))
                            }
                        }
                    } else {
                        Err(Error::UnexpectedEndOfInput)
                    }
                }
                _ => {
                    let src = lexer.remainder();
                    let (value, consumed) = utils::parse_string_literal(src)?;
                    lexer.bump(consumed);
                    match lexer.next() {
                        Some(Ok(Token::ParenClose)) => Ok(Pattern::text(value)),
                        Some(Ok(t)) => Err(Error::UnexpectedToken(
                            Box::new(t),
                            lexer.span(),
                        )),
                        Some(Err(e)) => Err(e),
                        None => Err(Error::ExpectedCloseParen(lexer.span())),
                    }
                }
            }
        }
        _ => Ok(Pattern::any_text()),
    }
}
