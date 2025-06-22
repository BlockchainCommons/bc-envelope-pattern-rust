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
        Token::GroupName(name) => parse_capture(lexer, name),
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
        Token::None => Ok(Pattern::none()),
        Token::Null => leaf::parse_null(lexer),
        Token::Number => leaf::parse_number(lexer),
        Token::Text => leaf::parse_text(lexer),
        t => Err(Error::UnexpectedToken(Box::new(t), lexer.span())),
    }
}
