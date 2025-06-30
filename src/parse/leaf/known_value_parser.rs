use crate::{Error, Pattern, Result, parse::Token};

// This parser is no longer used with the new dcbor-pattern syntax.
// The new syntax is handled directly in the primary parser.
// Keeping for potential legacy support or reference.

#[allow(dead_code)]
pub(crate) fn parse_known_value(
    _lexer: &mut logos::Lexer<Token>,
) -> Result<Pattern> {
    // With the new dcbor-pattern syntax, this function should not be called
    // as 'known' patterns are handled directly in primary_parser.rs
    Err(Error::UnexpectedToken(Box::new(Token::Known), 0..0))
}
