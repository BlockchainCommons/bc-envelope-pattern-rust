use super::{super::Token, not_parser::parse_not};
use crate::{Pattern, Result};

pub(crate) fn parse_traverse(
    lexer: &mut logos::Lexer<Token>,
) -> Result<Pattern> {
    let mut patterns = vec![parse_not(lexer)?];

    loop {
        let mut lookahead = lexer.clone();
        match lookahead.next() {
            Some(Ok(Token::Traverse)) => {
                lexer.next();
                patterns.push(parse_not(lexer)?);
            }
            _ => break,
        }
    }

    if patterns.len() == 1 {
        Ok(patterns.remove(0))
    } else {
        Ok(Pattern::traverse(patterns))
    }
}
