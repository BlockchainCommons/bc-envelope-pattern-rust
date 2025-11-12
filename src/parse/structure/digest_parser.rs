use bc_components::Digest;
use bc_envelope::prelude::*;

use super::super::Token;
use crate::{Error, Pattern, Result};

pub(crate) fn parse_digest(lexer: &mut logos::Lexer<Token>) -> Result<Pattern> {
    match lexer.next() {
        Some(Ok(Token::ParenOpen)) => {
            let src = lexer.remainder();
            let (pattern, consumed) = parse_digest_inner(src)?;
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
        Some(Ok(t)) => Err(Error::UnexpectedToken(Box::new(t), lexer.span())),
        Some(Err(e)) => Err(e),
        None => Err(Error::UnexpectedEndOfInput),
    }
}

fn parse_digest_inner(src: &str) -> Result<(Pattern, usize)> {
    let mut pos = 0;
    crate::parse::utils::skip_ws(src, &mut pos);
    if src[pos..].starts_with("ur:") {
        let start = pos;
        while let Some(ch) = src[pos..].chars().next() {
            if ch == ')' {
                break;
            }
            pos += ch.len_utf8();
        }
        let ur = src[start..pos].trim_end();
        let digest = Digest::from_ur_string(ur)
            .map_err(|_| Error::InvalidUr(ur.to_string(), pos..pos))?;
        crate::parse::utils::skip_ws(src, &mut pos);
        Ok((Pattern::digest(digest), pos))
    } else {
        let start = pos;
        while let Some(ch) = src[pos..].chars().next() {
            if ch.is_ascii_hexdigit() {
                pos += ch.len_utf8();
            } else {
                break;
            }
        }
        if start == pos {
            return Err(Error::InvalidHexString(pos..pos));
        }
        let hex_str = &src[start..pos];
        if !hex_str.len().is_multiple_of(2) {
            return Err(Error::InvalidHexString(pos..pos));
        }
        let bytes = hex::decode(hex_str)
            .map_err(|_| Error::InvalidHexString(pos..pos))?;
        if bytes.len() > Digest::DIGEST_SIZE {
            return Err(Error::InvalidHexString(pos..pos));
        }
        crate::parse::utils::skip_ws(src, &mut pos);
        Ok((Pattern::digest_prefix(bytes), pos))
    }
}
