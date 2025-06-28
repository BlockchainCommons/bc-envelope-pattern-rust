use logos::Logos;

use super::{Token, meta};
use crate::{Error, Pattern, Result};

impl Pattern {
    /// Parse a pattern expression.
    pub fn parse(input: impl AsRef<str>) -> Result<Pattern> {
        let mut lexer = Token::lexer(input.as_ref());

        let pattern = meta::parse_or(&mut lexer)?;

        match lexer.next() {
            None => Ok(pattern),
            Some(Ok(_)) => Err(Error::ExtraData(lexer.span())),
            Some(Err(e)) => {
                // If we get an Unknown error from the lexer, convert it to
                // UnrecognizedToken with span information
                match e {
                    Error::Unknown => {
                        Err(Error::UnrecognizedToken(lexer.span()))
                    }
                    _ => Err(e),
                }
            }
        }
    }
}

impl TryFrom<&str> for Pattern {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self> {
        Self::parse(value)
    }
}
