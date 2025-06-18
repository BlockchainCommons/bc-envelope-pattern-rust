use super::super::Token;
use crate::{Pattern, Result};

pub(crate) fn parse_encrypted(
    _lexer: &mut logos::Lexer<Token>,
) -> Result<Pattern> {
    Ok(Pattern::encrypted())
}
