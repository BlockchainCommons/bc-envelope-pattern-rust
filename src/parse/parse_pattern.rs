use logos::Logos;

use super::{Token, meta};
use crate::{Error, Pattern, Result, dcbor_integration::convert_dcbor_pattern_to_envelope_pattern};

impl Pattern {
    /// Parse a pattern expression.
    pub fn parse(input: impl AsRef<str>) -> Result<Pattern> {
        let input_str = input.as_ref();
        let mut lexer = Token::lexer(input_str);

        // Try envelope-pattern parsing first
        match meta::parse_or(&mut lexer) {
            Ok(pattern) => {
                match lexer.next() {
                    None => Ok(pattern),
                    Some(Ok(_)) => Err(Error::ExtraData(lexer.span())),
                    Some(Err(e)) => {
                        match e {
                            Error::Unknown => {
                                Err(Error::UnrecognizedToken(lexer.span()))
                            }
                            _ => Err(e),
                        }
                    }
                }
            }
            Err(_envelope_error) => {
                // If envelope-pattern parsing failed, try dcbor-pattern as fallback
                match dcbor_pattern::Pattern::parse(input_str) {
                    Ok(dcbor_pattern) => {
                        convert_dcbor_pattern_to_envelope_pattern(dcbor_pattern)
                    }
                    Err(_dcbor_error) => {
                        // Both parsers failed, return the original envelope error
                        Err(_envelope_error)
                    }
                }
            }
        }
    }
}

impl TryFrom<&str> for Pattern {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self> {
        Self::parse(value)
    }
}
