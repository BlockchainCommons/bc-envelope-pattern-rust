use crate::parse::{Token, utils};
use crate::{Error, Pattern, Result};
use known_values::KnownValue;

pub(crate) fn parse_known_value(
    lexer: &mut logos::Lexer<Token>,
) -> Result<Pattern> {
    let mut lookahead = lexer.clone();
    match lookahead.next() {
        Some(Ok(Token::ParenOpen)) => {
            lexer.next();
            let src = lexer.remainder();
            let (pattern, consumed) = parse_known_value_inner(src)?;
            lexer.bump(consumed);
            match lexer.next() {
                Some(Ok(Token::ParenClose)) => Ok(pattern),
                Some(Ok(t)) => {
                    Err(Error::UnexpectedToken(Box::new(t), lexer.span()))
                }
                Some(Err(e)) => Err(e),
                None => Err(Error::ExpectedCloseParen(lexer.span())),
            }
        }
        _ => Ok(Pattern::any_known_value()),
    }
}

fn parse_known_value_inner(src: &str) -> Result<(Pattern, usize)> {
    let mut pos = 0;
    utils::skip_ws(src, &mut pos);
    if src[pos..].starts_with('/') {
        let (regex, used) = utils::parse_text_regex(&src[pos..])?;
        pos += used;
        return Ok((Pattern::known_value_regex(regex), pos));
    }

    let (inner, used) = utils::parse_single_quoted(&src[pos..])?;
    pos += used;
    if let Ok(value) = inner.parse::<u64>() {
        Ok((Pattern::known_value(KnownValue::new(value)), pos))
    } else {
        Ok((Pattern::known_value_named(inner), pos))
    }
}
