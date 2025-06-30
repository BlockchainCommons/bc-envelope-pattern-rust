use crate::{Error, Pattern, Result, parse::Token};

pub(crate) fn parse_number_range_or_comparison(
    lexer: &mut logos::Lexer<Token>,
    first_value: f64,
) -> Result<Pattern> {
    // Look ahead to see if this is a range or comparison
    let mut lookahead = lexer.clone();
    match lookahead.next() {
        Some(Ok(Token::Ellipsis)) => {
            // This is a range: value...value
            lexer.next(); // consume the ellipsis
            match lexer.next() {
                Some(Ok(Token::UnsignedInteger(Ok(n)))) => {
                    Ok(Pattern::number_range(first_value..=(n as f64)))
                }
                Some(Ok(Token::Integer(Ok(i)))) => {
                    Ok(Pattern::number_range(first_value..=(i as f64)))
                }
                Some(Ok(Token::Float(Ok(f)))) => {
                    Ok(Pattern::number_range(first_value..=f))
                }
                Some(Ok(t)) => {
                    Err(Error::UnexpectedToken(Box::new(t), lexer.span()))
                }
                Some(Err(e)) => Err(e),
                None => Err(Error::UnexpectedEndOfInput),
            }
        }
        _ => {
            // Just a single number
            Ok(Pattern::number(first_value))
        }
    }
}

pub(crate) fn parse_comparison_number(
    lexer: &mut logos::Lexer<Token>,
    comparison: &str,
) -> Result<Pattern> {
    match lexer.next() {
        Some(Ok(Token::UnsignedInteger(Ok(n)))) => {
            let value = n as f64;
            match comparison {
                ">=" => Ok(Pattern::number_greater_than_or_equal(value)),
                "<=" => Ok(Pattern::number_less_than_or_equal(value)),
                ">" => Ok(Pattern::number_greater_than(value)),
                "<" => Ok(Pattern::number_less_than(value)),
                _ => unreachable!(),
            }
        }
        Some(Ok(Token::Integer(Ok(i)))) => {
            let value = i as f64;
            match comparison {
                ">=" => Ok(Pattern::number_greater_than_or_equal(value)),
                "<=" => Ok(Pattern::number_less_than_or_equal(value)),
                ">" => Ok(Pattern::number_greater_than(value)),
                "<" => Ok(Pattern::number_less_than(value)),
                _ => unreachable!(),
            }
        }
        Some(Ok(Token::Float(Ok(f)))) => {
            let value = f;
            match comparison {
                ">=" => Ok(Pattern::number_greater_than_or_equal(value)),
                "<=" => Ok(Pattern::number_less_than_or_equal(value)),
                ">" => Ok(Pattern::number_greater_than(value)),
                "<" => Ok(Pattern::number_less_than(value)),
                _ => unreachable!(),
            }
        }
        Some(Ok(t)) => Err(Error::UnexpectedToken(Box::new(t), lexer.span())),
        Some(Err(e)) => Err(e),
        None => Err(Error::UnexpectedEndOfInput),
    }
}
