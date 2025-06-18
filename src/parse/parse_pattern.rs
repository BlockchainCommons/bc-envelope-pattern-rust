use logos::Logos;

use super::Token;
use crate::{Pattern, Result};

pub fn parse_pattern(_input: impl AsRef<str>) -> Result<Pattern> {
    let mut _lexer = Token::lexer(_input.as_ref());
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
