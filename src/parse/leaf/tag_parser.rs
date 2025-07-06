use crate::{
    DCBORPattern, Error, Pattern, Result,
    parse::{Token, utils},
};

pub(crate) fn parse_tag(lexer: &mut logos::Lexer<Token>) -> Result<Pattern> {
    let mut lookahead = lexer.clone();
    match lookahead.next() {
        Some(Ok(Token::ParenOpen)) => {
            // For parenthesized cases, delegate to dcbor-pattern for full
            // parsing
            lexer.next(); // consume opening paren
            let remainder = lexer.remainder();

            // Construct the full tagged pattern expression for dcbor-pattern
            let closing_paren_pos = find_matching_paren(remainder)?;
            let inner_content = &remainder[..closing_paren_pos];
            let tagged_expr = format!("tagged({})", inner_content);

            // Parse with dcbor-pattern
            match DCBORPattern::parse(&tagged_expr) {
                Ok(dcbor_pattern) => {
                    // Skip to the closing paren
                    lexer.bump(closing_paren_pos);

                    match lexer.next() {
                        Some(Ok(Token::ParenClose)) => {
                            // Extract the TaggedPattern from the dcbor_pattern
                            if let DCBORPattern::Structure(
                                dcbor_pattern::StructurePattern::Tagged(
                                    tagged_pattern,
                                ),
                            ) = dcbor_pattern
                            {
                                Ok(Pattern::tagged_from_dcbor_pattern(
                                    tagged_pattern,
                                ))
                            } else {
                                // This shouldn't happen if we constructed the
                                // expression correctly
                                Err(Error::UnexpectedToken(
                                    Box::new(Token::ParenClose),
                                    lexer.span(),
                                ))
                            }
                        }
                        Some(Ok(t)) => Err(Error::UnexpectedToken(
                            Box::new(t),
                            lexer.span(),
                        )),
                        Some(Err(e)) => Err(e),
                        None => Err(Error::ExpectedCloseParen(lexer.span())),
                    }
                }
                Err(_) => {
                    // Fall back to simple parsing
                    let (pattern, consumed) = parse_tag_inner(remainder)?;
                    lexer.bump(consumed);
                    match lexer.next() {
                        Some(Ok(Token::ParenClose)) => Ok(pattern),
                        Some(Ok(t)) => Err(Error::UnexpectedToken(
                            Box::new(t),
                            lexer.span(),
                        )),
                        Some(Err(e)) => Err(e),
                        None => Err(Error::ExpectedCloseParen(lexer.span())),
                    }
                }
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
        return Ok((Pattern::tagged_regex(regex, DCBORPattern::any()), pos));
    }

    let (word, used) = utils::parse_bare_word(&src[pos..])?;
    pos += used;
    if let Ok(value) = word.parse::<u64>() {
        Ok((Pattern::tagged(value, DCBORPattern::any()), pos))
    } else {
        Ok((Pattern::tagged_name(word, DCBORPattern::any()), pos))
    }
}

fn find_matching_paren(src: &str) -> Result<usize> {
    let mut pos = 0;
    let mut paren_depth = 0;
    while pos < src.len() {
        let ch = src.as_bytes()[pos];
        if ch == b'(' {
            paren_depth += 1;
        } else if ch == b')' {
            if paren_depth == 0 {
                return Ok(pos);
            }
            paren_depth -= 1;
        }
        pos += 1;
    }
    Err(Error::ExpectedCloseParen(pos..pos))
}
