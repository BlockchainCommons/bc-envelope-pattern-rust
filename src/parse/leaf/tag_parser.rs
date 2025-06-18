use crate::{
    Error, Pattern, Result,
    parse::{Token, utils},
};

pub(crate) fn parse_tag(lexer: &mut logos::Lexer<Token>) -> Result<Pattern> {
    let mut lookahead = lexer.clone();
    match lookahead.next() {
        Some(Ok(Token::ParenOpen)) => {
            lexer.next();
            let src = lexer.remainder();
            let (pattern, consumed) = parse_tag_inner(src)?;
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
        _ => Ok(Pattern::any_tag()),
    }
}

fn parse_tag_inner(src: &str) -> Result<(Pattern, usize)> {
    let mut pos = 0;
    utils::skip_ws(src, &mut pos);
    if src[pos..].starts_with('/') {
        let (regex, used) = utils::parse_text_regex(&src[pos..])?;
        pos += used;
        return Ok((Pattern::tagged_with_regex(regex), pos));
    }

    let (word, used) = utils::parse_bare_word(&src[pos..])?;
    pos += used;
    if let Ok(value) = word.parse::<u64>() {
        Ok((Pattern::tagged_with_value(value), pos))
    } else {
        Ok((Pattern::tagged_with_name(word), pos))
    }
}
