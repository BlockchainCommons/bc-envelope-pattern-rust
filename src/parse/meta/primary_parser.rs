use known_values::KnownValue;

use super::{
    super::{Token, leaf, structure},
    capture_parser::parse_capture,
    group_parser::parse_group,
    search_parser::parse_search,
};
use crate::{Error, Pattern, Result};

pub(crate) fn parse_primary(
    lexer: &mut logos::Lexer<Token>,
) -> Result<Pattern> {
    let token = match lexer.next() {
        Some(Ok(tok)) => tok,
        Some(Err(e)) => {
            // Convert Unknown errors to UnrecognizedToken with span information
            match e {
                Error::Unknown => {
                    return Err(Error::UnrecognizedToken(lexer.span()));
                }
                _ => return Err(e),
            }
        }
        None => return Err(Error::UnexpectedEndOfInput),
    };

    match token {
        // Envelope-specific patterns first (these take precedence)
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
        Token::Unwrap => structure::parse_unwrap(lexer),
        Token::Subject => structure::parse_subject(lexer),
        Token::GroupName(name) => parse_capture(lexer, name),
        Token::ParenOpen => parse_group(lexer),
        Token::Leaf => Ok(Pattern::any_leaf()),
        Token::None => Ok(Pattern::none()),

        // Map patterns might have envelope-specific extensions
        Token::Map => leaf::parse_map(lexer),
        Token::Cbor => leaf::parse_cbor(lexer),

        // For simple leaf patterns, try dcbor-pattern first
        Token::RepeatZeroOrMore => Ok(Pattern::any()), /* dcbor-pattern's *
                                                         * syntax */
        Token::BoolKeyword => Ok(Pattern::any_bool()),
        Token::BoolTrue => Ok(Pattern::bool(true)),
        Token::BoolFalse => Ok(Pattern::bool(false)),
        Token::NumberKeyword => Ok(Pattern::any_number()),
        Token::TextKeyword => Ok(Pattern::any_text()),
        Token::StringLiteral(Ok(s)) => Ok(Pattern::text(s)),
        Token::StringLiteral(Err(e)) => Err(e),
        Token::UnsignedInteger(Ok(n)) => {
            // Check if this is part of a range (e.g., "1...10")
            leaf::parse_number_range_or_comparison(lexer, n as f64)
        }
        Token::UnsignedInteger(Err(e)) => Err(e),
        Token::Integer(Ok(i)) => {
            // Check if this is part of a range (e.g., "-5...10")
            leaf::parse_number_range_or_comparison(lexer, i as f64)
        }
        Token::Integer(Err(e)) => Err(e),
        Token::Float(Ok(f)) => {
            // Check if this is part of a range (e.g., "1.5...10.0")
            leaf::parse_number_range_or_comparison(lexer, f)
        }
        Token::Float(Err(e)) => Err(e),
        Token::GreaterThanOrEqual => leaf::parse_comparison_number(lexer, ">="),
        Token::LessThanOrEqual => leaf::parse_comparison_number(lexer, "<="),
        Token::GreaterThan => leaf::parse_comparison_number(lexer, ">"),
        Token::LessThan => leaf::parse_comparison_number(lexer, "<"),
        Token::NaN => Ok(Pattern::number_nan()),
        Token::Infinity => Ok(Pattern::number(f64::INFINITY)),
        Token::NegativeInfinity => Ok(Pattern::number(f64::NEG_INFINITY)),
        Token::Regex(Ok(regex_str)) => {
            let regex = regex::Regex::new(&regex_str)
                .map_err(|_| Error::InvalidRegex(lexer.span()))?;
            Ok(Pattern::text_regex(regex))
        }
        Token::Regex(Err(e)) => Err(e),
        Token::BracketOpen => leaf::parse_array(lexer),
        Token::ByteString => Ok(Pattern::any_byte_string()),
        Token::HexPattern(Ok(bytes)) => Ok(Pattern::byte_string(bytes)),
        Token::HexPattern(Err(e)) => Err(e),
        Token::HexBinaryRegex(Ok(regex_str)) => {
            let regex = regex::bytes::Regex::new(&regex_str)
                .map_err(|_| Error::InvalidRegex(lexer.span()))?;
            Ok(Pattern::byte_string_binary_regex(regex))
        }
        Token::HexBinaryRegex(Err(e)) => Err(e),
        Token::DateKeyword => Ok(Pattern::any_date()),
        Token::DatePattern(Ok(content)) => leaf::parse_date_content(content),
        Token::DatePattern(Err(e)) => Err(e),
        Token::Tagged => leaf::parse_tag(lexer),
        Token::Known => Ok(Pattern::any_known_value()),
        Token::SingleQuotedPattern(Ok(content)) => {
            if let Ok(value) = content.parse::<u64>() {
                Ok(Pattern::known_value(KnownValue::new(value)))
            } else {
                Ok(Pattern::known_value_named(content))
            }
        }
        Token::SingleQuotedPattern(Err(e)) => Err(e),
        Token::SingleQuotedRegex(Ok(regex_str)) => {
            let regex = regex::Regex::new(&regex_str)
                .map_err(|_| Error::InvalidRegex(lexer.span()))?;
            Ok(Pattern::known_value_regex(regex))
        }
        Token::SingleQuotedRegex(Err(e)) => Err(e),
        Token::Null => leaf::parse_null(lexer),

        t => Err(Error::UnexpectedToken(Box::new(t), lexer.span())),
    }
}
