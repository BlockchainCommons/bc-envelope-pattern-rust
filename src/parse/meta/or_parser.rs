use super::{super::Token, traverse_parser::parse_traverse};
use crate::{Pattern, Result};

pub(crate) fn parse_or(lexer: &mut logos::Lexer<Token>) -> Result<Pattern> {
    let mut patterns = vec![parse_traverse(lexer)?];

    loop {
        let mut lookahead = lexer.clone();
        match lookahead.next() {
            Some(Ok(Token::Or)) => {
                lexer.next();
                patterns.push(parse_traverse(lexer)?);
            }
            _ => break,
        }
    }

    if patterns.len() == 1 {
        Ok(patterns.remove(0))
    } else {
        Ok(Pattern::or(patterns))
    }
}
