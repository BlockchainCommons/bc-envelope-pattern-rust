use logos::Logos;

use super::{Result, Token};
use crate::Pattern;

pub fn parse_pattern(input: impl AsRef<str>) -> Result<Pattern> {
    let mut lexer = Token::lexer(input.as_ref());
    todo!();
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_parse_pattern() {
//         let input = "ANY";
//         let pattern = parse_pattern(input).unwrap();
//         assert_eq!(pattern.to_string(), "ANY");
//     }
// }
