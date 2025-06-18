use super::super::Token;
use super::primary_parser::parse_primary;
use crate::{Pattern, Result};

pub(crate) fn parse_and(lexer: &mut logos::Lexer<Token>) -> Result<Pattern> {
    let mut patterns = vec![parse_primary(lexer)?];

    loop {
        let mut lookahead = lexer.clone();
        match lookahead.next() {
            Some(Ok(Token::And)) => {
                lexer.next();
                patterns.push(parse_primary(lexer)?);
            }
            _ => break,
        }
    }

    if patterns.len() == 1 {
        Ok(patterns.remove(0))
    } else {
        Ok(Pattern::and(patterns))
    }
}
