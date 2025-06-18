use super::super::Token;
use crate::{Pattern, Result};

pub(crate) fn parse_elided(
    _lexer: &mut logos::Lexer<Token>,
) -> Result<Pattern> {
    Ok(Pattern::elided())
}
