use super::{super::Token, or_parser::parse_or};
use crate::{Error, Pattern, Reluctance, Result};

pub(crate) fn parse_group(lexer: &mut logos::Lexer<Token>) -> Result<Pattern> {
    let pat = parse_or(lexer)?;
    match lexer.next() {
        Some(Ok(Token::ParenClose)) => {
            let mut lookahead = lexer.clone();
            match lookahead.next() {
                Some(Ok(tok)) => match tok {
                    Token::RepeatZeroOrMore => {
                        lexer.next();
                        Ok(Pattern::repeat(pat, 0.., Reluctance::Greedy))
                    }
                    Token::RepeatZeroOrMoreLazy => {
                        lexer.next();
                        Ok(Pattern::repeat(pat, 0.., Reluctance::Lazy))
                    }
                    Token::RepeatZeroOrMorePossessive => {
                        lexer.next();
                        Ok(Pattern::repeat(pat, 0.., Reluctance::Possessive))
                    }
                    Token::RepeatOneOrMore => {
                        lexer.next();
                        Ok(Pattern::repeat(pat, 1.., Reluctance::Greedy))
                    }
                    Token::RepeatOneOrMoreLazy => {
                        lexer.next();
                        Ok(Pattern::repeat(pat, 1.., Reluctance::Lazy))
                    }
                    Token::RepeatOneOrMorePossessive => {
                        lexer.next();
                        Ok(Pattern::repeat(pat, 1.., Reluctance::Possessive))
                    }
                    Token::RepeatZeroOrOne => {
                        lexer.next();
                        Ok(Pattern::repeat(pat, 0..=1, Reluctance::Greedy))
                    }
                    Token::RepeatZeroOrOneLazy => {
                        lexer.next();
                        Ok(Pattern::repeat(pat, 0..=1, Reluctance::Lazy))
                    }
                    Token::RepeatZeroOrOnePossessive => {
                        lexer.next();
                        Ok(Pattern::repeat(pat, 0..=1, Reluctance::Possessive))
                    }
                    Token::Range(res) => {
                        lexer.next();
                        let q = res?;
                        let pat = if let Some(max) = q.max() {
                            Pattern::repeat(pat, q.min()..=max, q.reluctance())
                        } else {
                            Pattern::repeat(pat, q.min().., q.reluctance())
                        };
                        Ok(pat)
                    }
                    _ => Ok(pat),
                },
                _ => Ok(pat),
            }
        }
        Some(Ok(t)) => Err(Error::UnexpectedToken(Box::new(t), lexer.span())),
        Some(Err(e)) => Err(e),
        None => Err(Error::ExpectedCloseParen(lexer.span())),
    }
}
