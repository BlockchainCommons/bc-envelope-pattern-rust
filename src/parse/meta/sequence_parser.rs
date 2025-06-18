use super::super::Token;
use super::not_parser::parse_not;
use crate::{Pattern, Result};

pub(crate) fn parse_sequence(
    lexer: &mut logos::Lexer<Token>,
) -> Result<Pattern> {
    let mut patterns = vec![parse_not(lexer)?];

    loop {
        let mut lookahead = lexer.clone();
        match lookahead.next() {
            Some(Ok(Token::Sequence)) => {
                lexer.next();
                patterns.push(parse_not(lexer)?);
            }
            _ => break,
        }
    }

    if patterns.len() == 1 {
        Ok(patterns.remove(0))
    } else {
        Ok(Pattern::sequence(patterns))
    }
}
