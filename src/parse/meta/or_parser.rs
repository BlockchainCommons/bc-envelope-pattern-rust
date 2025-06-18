use super::super::Token;
use super::sequence_parser::parse_sequence;
use crate::{Error, Pattern, Result};

pub(crate) fn parse_or(lexer: &mut logos::Lexer<Token>) -> Result<Pattern> {
    let mut patterns = vec![parse_sequence(lexer)?];

    loop {
        let mut lookahead = lexer.clone();
        match lookahead.next() {
            Some(Ok(Token::Or)) => {
                lexer.next();
                patterns.push(parse_sequence(lexer)?);
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
