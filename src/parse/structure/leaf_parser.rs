use super::super::Token;
use crate::{Pattern, Result};

/// Parses LEAF pattern.
pub(crate) fn parse_leaf(
    _lexer: &mut logos::Lexer<Token>,
) -> Result<Pattern> {
    Ok(Pattern::leaf())
}
