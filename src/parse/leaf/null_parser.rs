use crate::{Pattern, Result};

pub(crate) fn parse_null(
    _lexer: &mut logos::Lexer<crate::parse::Token>,
) -> Result<Pattern> {
    Ok(Pattern::null())
}
