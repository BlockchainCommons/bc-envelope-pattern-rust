use super::super::Token;
use super::and_parser::parse_and;
use crate::{Pattern, Result};

pub(crate) fn parse_not(lexer: &mut logos::Lexer<Token>) -> Result<Pattern> {
    let mut lookahead = lexer.clone();
    match lookahead.next() {
        Some(Ok(Token::Not)) => {
            lexer.next();
            let pat = parse_not(lexer)?;
            Ok(Pattern::not_matching(pat))
        }
        _ => parse_and(lexer),
    }
}
