use logos::Logos;

use super::{meta, Token};
use crate::{Error, Pattern, Result};

/// Parse a pattern expression.
///
/// Currently only a subset of the full grammar is supported.
pub fn parse_pattern(input: impl AsRef<str>) -> Result<Pattern> {
    let mut lexer = Token::lexer(input.as_ref());

    let pattern = meta::parse_or(&mut lexer)?;

    match lexer.next() {
        None => Ok(pattern),
        Some(Ok(_)) => Err(Error::ExtraData(lexer.span())),
        Some(Err(e)) => Err(e),
    }
}
