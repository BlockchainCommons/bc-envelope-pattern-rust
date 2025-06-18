// Utility parsing helpers shared across pattern parsers

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

pub(crate) fn parse_string_literal(src: &str) -> Result<(String, usize)> {
    let mut pos = 0;
    skip_ws(src, &mut pos);

    let bytes = src.as_bytes();
    if pos >= bytes.len() || bytes[pos] != b'"' {
        return Err(Error::UnexpectedEndOfInput);
    }
    pos += 1;
    let start = pos;
    let mut escape = false;
    while pos < bytes.len() {
        let b = bytes[pos];
        pos += 1;
        if escape {
            escape = false;
            continue;
        }
        if b == b'\\' {
            escape = true;
            continue;
        }
        if b == b'"' {
            let inner = &src[start..pos - 1];
            let value = inner.replace("\\\"", "\"").replace("\\\\", "\\");
            skip_ws(src, &mut pos);
            return Ok((value, pos));
        }
    }
    Err(Error::UnexpectedEndOfInput)
}

pub(crate) fn parse_hex_string(src: &str) -> Result<(Vec<u8>, usize)> {
    let mut pos = 0;
    skip_ws(src, &mut pos);
    if !src[pos..].starts_with("h'") {
        return Err(Error::InvalidHexString(pos..pos));
    }
    pos += 2;
    let start = pos;
    while pos < src.len() {
        let b = src.as_bytes()[pos];
        if b == b'\'' {
            let inner = &src[start..pos];
            let bytes = hex::decode(inner).map_err(|_| Error::InvalidHexString(pos..pos))?;
            pos += 1;
            skip_ws(src, &mut pos);
            return Ok((bytes, pos));
        }
        if !(b as char).is_ascii_hexdigit() {
            return Err(Error::InvalidHexString(pos..pos));
        }
        pos += 1;
    }
    Err(Error::InvalidHexString(pos..pos))
}

pub(crate) fn parse_binary_regex(src: &str) -> Result<(regex::bytes::Regex, usize)> {
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
            let regex =
                regex::bytes::Regex::new(inner).map_err(|_| Error::InvalidRegex(pos..pos))?;
            skip_ws(src, &mut pos);
            return Ok((regex, pos));
        }
    }
    Err(Error::UnterminatedRegex(pos..pos))
}

pub(crate) fn parse_uint_digits(src: &str, pos: &mut usize) -> Result<f64> {
    let start = *pos;
    while let Some(ch) = src[*pos..].chars().next() {
        if ch.is_ascii_digit() {
            *pos += ch.len_utf8();
        } else {
            break;
        }
    }
    if start == *pos {
        return Err(Error::InvalidNumberFormat(0..0));
    }
    src[start..*pos]
        .parse::<f64>()
        .map_err(|_| Error::InvalidNumberFormat(0..0))
}

pub(crate) fn parse_number_inner(src: &str) -> Result<(Pattern, usize)> {
    let mut pos = 0;
    skip_ws(src, &mut pos);

    if src[pos..].starts_with("NaN") {
        pos += 3;
        skip_ws(src, &mut pos);
        return Ok((Pattern::number_nan(), pos));
    }

    let cmp = if src[pos..].starts_with(">=") {
        pos += 2;
        Some("ge")
    } else if src[pos..].starts_with("<=") {
        pos += 2;
        Some("le")
    } else if src[pos..].starts_with('>') {
        pos += 1;
        Some("gt")
    } else if src[pos..].starts_with('<') {
        pos += 1;
        Some("lt")
    } else {
        None
    };

    if let Some(tag) = cmp {
        skip_ws(src, &mut pos);
        let val = parse_uint_digits(src, &mut pos)?;
        skip_ws(src, &mut pos);
        let pattern = match tag {
            "ge" => Pattern::number_greater_than_or_equal(val),
            "le" => Pattern::number_less_than_or_equal(val),
            "gt" => Pattern::number_greater_than(val),
            "lt" => Pattern::number_less_than(val),
            _ => unreachable!(),
        };
        return Ok((pattern, pos));
    }

    let first = parse_uint_digits(src, &mut pos)?;
    skip_ws(src, &mut pos);

    if src[pos..].starts_with("...") {
        pos += 3;
        skip_ws(src, &mut pos);
        let second = parse_uint_digits(src, &mut pos)?;
        skip_ws(src, &mut pos);
        return Ok((Pattern::number_range(first..=second), pos));
    }

    Ok((Pattern::number(first), pos))
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
            let regex = regex::Regex::new(inner).map_err(|_| Error::InvalidRegex(pos..pos))?;
            skip_ws(src, &mut pos);
            return Ok((regex, pos));
        }
    }
    Err(Error::UnterminatedRegex(pos..pos))
}

fn parse_iso8601(src: &str, pos: &mut usize) -> Result<dcbor::Date> {
    skip_ws(src, pos);
    let start = *pos;
    while *pos < src.len() {
        if src[*pos..].starts_with("...") || src.as_bytes()[*pos] == b')' {
            break;
        }
        let ch = src[*pos..].chars().next().unwrap();
        if matches!(ch, ' ' | '\t' | '\n' | '\r' | '\u{0c}') {
            break;
        }
        *pos += ch.len_utf8();
    }
    if start == *pos {
        return Err(Error::InvalidDateFormat(0..0));
    }
    let iso = &src[start..*pos];
    let date = dcbor::Date::from_string(iso).map_err(|_| Error::InvalidDateFormat(0..0))?;
    skip_ws(src, pos);
    Ok(date)
}

pub(crate) fn parse_date_inner(src: &str) -> Result<(Pattern, usize)> {
    let mut pos = 0;
    skip_ws(src, &mut pos);

    if src[pos..].starts_with('/') {
        let (regex, used) = parse_text_regex(&src[pos..])?;
        pos += used;
        return Ok((Pattern::date_regex(regex), pos));
    }

    if src[pos..].starts_with("...") {
        pos += 3;
        let date = parse_iso8601(src, &mut pos)?;
        return Ok((Pattern::date_latest(date), pos));
    }

    let first = parse_iso8601(src, &mut pos)?;

    if src[pos..].starts_with("...") {
        pos += 3;
        if let Ok(second) = parse_iso8601(src, &mut pos) {
            return Ok((Pattern::date_range(first..=second), pos));
        } else {
            return Ok((Pattern::date_earliest(first), pos));
        }
    }

    Ok((Pattern::date(first), pos))
}
