// Parsers for meta-pattern operators

use super::{Token, leaf, structure};
use crate::{Error, Pattern, Result, Reluctance};

pub(crate) fn parse_or(lexer: &mut logos::Lexer<Token>) -> Result<Pattern> {
    let mut patterns = vec![parse_sequence(lexer)?];

    loop {
        let mut lookahead = lexer.clone();
        match lookahead.next() {
            Some(Ok(Token::Or)) => {
                // consume the '|'
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

fn parse_primary(lexer: &mut logos::Lexer<Token>) -> Result<Pattern> {
    let token = match lexer.next() {
        Some(Ok(tok)) => tok,
        Some(Err(e)) => return Err(e),
        None => return Err(Error::UnexpectedEndOfInput),
    };

    match token {
        Token::Any => Ok(Pattern::any()),
        Token::Array => leaf::parse_array(lexer),
        Token::Bool => leaf::parse_bool(lexer),
        Token::ByteString => leaf::parse_byte_string(lexer),
        Token::Date => leaf::parse_date(lexer),
        Token::Tag => leaf::parse_tag(lexer),
        Token::Known => leaf::parse_known_value(lexer),
        Token::Leaf => Ok(Pattern::any_leaf()),
        Token::Cbor => leaf::parse_cbor(lexer),
        Token::Map => leaf::parse_map(lexer),
        Token::ParenOpen => parse_group(lexer),
        Token::Search => parse_search(lexer),
        Token::Node => structure::parse_node(lexer),
        Token::Assertion => structure::parse_assertion(lexer),
        Token::AssertionPred => structure::parse_assertion_pred(lexer),
        Token::AssertionObj => structure::parse_assertion_obj(lexer),
        Token::Digest => structure::parse_digest(lexer),
        Token::Obj => structure::parse_object(lexer),
        Token::Obscured => structure::parse_obscured(lexer),
        Token::Elided => structure::parse_elided(lexer),
        Token::Encrypted => structure::parse_encrypted(lexer),
        Token::Compressed => structure::parse_compressed(lexer),
        Token::Pred => structure::parse_predicate(lexer),
        Token::Wrapped => structure::parse_wrapped(lexer),
        Token::Subject => structure::parse_subject(lexer),
        Token::None => Ok(Pattern::none()),
        Token::Null => leaf::parse_null(lexer),
        Token::Number => leaf::parse_number(lexer),
        Token::Text => leaf::parse_text(lexer),
        t => Err(Error::UnexpectedToken(Box::new(t), lexer.span())),
    }
}

fn parse_group(lexer: &mut logos::Lexer<Token>) -> Result<Pattern> {
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

fn parse_search(lexer: &mut logos::Lexer<Token>) -> Result<Pattern> {
    match lexer.next() {
        Some(Ok(Token::ParenOpen)) => {
            let pat = parse_or(lexer)?;
            match lexer.next() {
                Some(Ok(Token::ParenClose)) => Ok(Pattern::search(pat)),
                Some(Ok(t)) => Err(Error::UnexpectedToken(Box::new(t), lexer.span())),
                Some(Err(e)) => Err(e),
                None => Err(Error::ExpectedCloseParen(lexer.span())),
            }
        }
        Some(Ok(t)) => Err(Error::UnexpectedToken(Box::new(t), lexer.span())),
        Some(Err(e)) => Err(e),
        None => Err(Error::UnexpectedEndOfInput),
    }
}
