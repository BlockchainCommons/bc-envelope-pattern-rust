// Utility parsing helpers shared across pattern parsers

use dcbor_parse::parse_dcbor_item_partial;

use crate::{Error, Pattern, Result};

pub(crate) fn skip_ws(src: &str, pos: &mut usize) {
    while let Some(ch) = src[*pos..].chars().next() {
        if matches!(ch, ' ' | '\t' | '\n' | '\r' | '\u{0c}') {
            *pos += ch.len_utf8();
        } else {
            break;
        }
    }
}

pub(crate) fn parse_text_regex(src: &str) -> Result<(regex::Regex, usize)> {
    let mut pos = 0;
    skip_ws(src, &mut pos);
    if pos >= src.len() || src.as_bytes()[pos] != b'/' {
        return Err(Error::UnterminatedRegex(pos..pos));
    }
    pos += 1;
    let start = pos;
    let mut escape = false;
    while pos < src.len() {
        let b = src.as_bytes()[pos];
        pos += 1;
        if escape {
            escape = false;
            continue;
        }
        if b == b'\\' {
            escape = true;
            continue;
        }
        if b == b'/' {
            let inner = &src[start..pos - 1];
            let regex = regex::Regex::new(inner)
                .map_err(|_| Error::InvalidRegex(pos..pos))?;
            skip_ws(src, &mut pos);
            return Ok((regex, pos));
        }
    }
    Err(Error::UnterminatedRegex(pos..pos))
}

pub(crate) fn parse_cbor_inner(src: &str) -> Result<(Pattern, usize)> {
    let mut pos = 0;
    skip_ws(src, &mut pos);

    // Check if this is a dcbor-pattern expression (/patex/)
    if src[pos..].starts_with('/') {
        pos += 1; // skip opening '/'
        let start = pos;
        let mut escape = false;

        // Find the closing '/'
        while pos < src.len() {
            let b = src.as_bytes()[pos];
            pos += 1;
            if escape {
                escape = false;
                continue;
            }
            if b == b'\\' {
                escape = true;
                continue;
            }
            if b == b'/' {
                let pattern_str = &src[start..pos - 1];

                // Parse the dcbor-pattern expression
                let dcbor_pattern = dcbor_pattern::Pattern::parse(pattern_str)
                    .map_err(|_| Error::InvalidPattern(start..pos - 1))?;

                skip_ws(src, &mut pos);
                return Ok((Pattern::cbor_pattern(dcbor_pattern), pos));
            }
        }
        return Err(Error::UnterminatedRegex(start - 1..pos));
    }

    // Check if this is a UR (ur:type/value)
    if src[pos..].starts_with("ur:") {
        // Parse as UR and convert to CBOR
        let (cbor_v20, consumed) = parse_dcbor_item_partial(&src[pos..])
            .map_err(|_| Error::Unknown)?;
        let bytes = cbor_v20.to_cbor_data();
        let cbor =
            dcbor::CBOR::try_from_data(bytes).map_err(|_| Error::Unknown)?;
        return Ok((Pattern::cbor(cbor), pos + consumed));
    }

    // Default: parse as CBOR diagnostic notation
    let (cbor_v20, consumed) =
        parse_dcbor_item_partial(&src[pos..]).map_err(|_| Error::Unknown)?;
    let bytes = cbor_v20.to_cbor_data();
    let cbor = dcbor::CBOR::try_from_data(bytes).map_err(|_| Error::Unknown)?;
    Ok((Pattern::cbor(cbor), pos + consumed))
}

pub(crate) fn parse_array_inner(src: &str) -> Result<(Pattern, usize)> {
    let mut pos = 0;
    skip_ws(src, &mut pos);

    // Check for the simple "*" pattern first - matches any array
    if src[pos..].starts_with('*') {
        pos += 1;
        skip_ws(src, &mut pos);
        return Ok((Pattern::any_array(), pos));
    }

    // Check for length patterns like {n}, {n,m}, {n,}
    if src[pos..].starts_with('{') {
        pos += 1;
        skip_ws(src, &mut pos);

        // Parse the first number
        let start_pos = pos;
        while pos < src.len() && src[pos..].chars().next().unwrap().is_ascii_digit() {
            pos += 1;
        }
        if pos == start_pos {
            return Err(Error::InvalidRange(pos..pos));
        }

        let first_num: usize = src[start_pos..pos].parse()
            .map_err(|_| Error::InvalidNumberFormat(start_pos..pos))?;

        skip_ws(src, &mut pos);

        if pos >= src.len() {
            return Err(Error::UnexpectedEndOfInput);
        }

        let ch = src[pos..].chars().next().unwrap();
        match ch {
            '}' => {
                // {n} - exact count
                pos += 1;
                skip_ws(src, &mut pos);
                return Ok((Pattern::array_with_count(first_num), pos));
            }
            ',' => {
                pos += 1;
                skip_ws(src, &mut pos);

                if pos >= src.len() {
                    return Err(Error::UnexpectedEndOfInput);
                }

                let ch = src[pos..].chars().next().unwrap();
                if ch == '}' {
                    // {n,} - at least n
                    pos += 1;
                    skip_ws(src, &mut pos);
                    return Ok((Pattern::array_with_range(first_num..), pos));
                } else if ch.is_ascii_digit() {
                    // {n,m} - range
                    let start_pos = pos;
                    while pos < src.len() && src[pos..].chars().next().unwrap().is_ascii_digit() {
                        pos += 1;
                    }
                    let second_num: usize = src[start_pos..pos].parse()
                        .map_err(|_| Error::InvalidNumberFormat(start_pos..pos))?;

                    skip_ws(src, &mut pos);
                    if pos >= src.len() || !src[pos..].starts_with('}') {
                        return Err(Error::UnexpectedEndOfInput);
                    }
                    pos += 1;
                    skip_ws(src, &mut pos);
                    return Ok((Pattern::array_with_range(first_num..=second_num), pos));
                } else {
                    return Err(Error::InvalidRange(pos..pos));
                }
            }
            _ => return Err(Error::InvalidRange(pos..pos)),
        }
    }

    // For any other pattern content, delegate to dcbor-pattern
    // This is the key proxy functionality - just pass the content through
    let pattern_str = format!("[{}]", &src[pos..]);
    match dcbor_pattern::Pattern::parse(&pattern_str) {
        Ok(dcbor_pattern) => {
            // Create an array pattern that wraps the dcbor-pattern
            let consumed = src.len() - pos; // Consume all remaining content
            Ok((Pattern::array_from_dcbor_pattern(dcbor_pattern), consumed))
        }
        Err(_) => Err(Error::InvalidPattern(pos..src.len())),
    }
}

pub(crate) fn parse_bare_word(src: &str) -> Result<(String, usize)> {
    let mut pos = 0;
    skip_ws(src, &mut pos);
    let start = pos;
    while pos < src.len() {
        let ch = src[pos..].chars().next().unwrap();
        if matches!(ch, ' ' | '\t' | '\n' | '\r' | '\u{0c}' | ')') {
            break;
        }
        pos += ch.len_utf8();
    }
    if start == pos {
        return Err(Error::UnexpectedEndOfInput);
    }
    let word = src[start..pos].to_string();
    skip_ws(src, &mut pos);
    Ok((word, pos))
}
