use bc_envelope_pattern::Token;
use logos::Logos;

#[test]
fn test_basic_tokens() {
    // Test meta pattern operators
    assert_eq!(Token::lexer("&").next(), Some(Ok(Token::And)));
    assert_eq!(Token::lexer("|").next(), Some(Ok(Token::Or)));
    assert_eq!(Token::lexer("!").next(), Some(Ok(Token::Not)));
    assert_eq!(Token::lexer(">").next(), Some(Ok(Token::Sequence)));
    assert_eq!(Token::lexer("*").next(), Some(Ok(Token::RepeatZeroOrMore)));
    assert_eq!(Token::lexer("+").next(), Some(Ok(Token::RepeatOneOrMore)));
    assert_eq!(Token::lexer("?").next(), Some(Ok(Token::RepeatZeroOrOne)));

    // Test structure pattern keywords
    assert_eq!(Token::lexer("ASSERTION").next(), Some(Ok(Token::Assertion)));
    assert_eq!(Token::lexer("NODE").next(), Some(Ok(Token::Node)));
    assert_eq!(Token::lexer("SUBJECT").next(), Some(Ok(Token::Subject)));

    // Test leaf pattern keywords
    assert_eq!(Token::lexer("ARRAY").next(), Some(Ok(Token::Array)));
    assert_eq!(Token::lexer("BOOL").next(), Some(Ok(Token::Bool)));
    assert_eq!(Token::lexer("TEXT").next(), Some(Ok(Token::Text)));
    assert_eq!(Token::lexer("NUMBER").next(), Some(Ok(Token::Number)));

    // Test literals
    assert_eq!(Token::lexer("true").next(), Some(Ok(Token::BoolTrue)));
    assert_eq!(Token::lexer("false").next(), Some(Ok(Token::BoolFalse)));
    assert_eq!(Token::lexer("NaN").next(), Some(Ok(Token::NaN)));
}

#[test]
fn test_complex_tokens() {
    // Group name
    let mut lexer = Token::lexer("@name");
    if let Some(Ok(Token::GroupName(name))) = lexer.next() {
        assert_eq!(name, "name");
    } else {
        panic!("Failed to parse group name");
    }

    // Test regex
    let mut lexer = Token::lexer("/[a-z]+/");
    if let Some(Ok(Token::Regex(Ok(regex)))) = lexer.next() {
        assert_eq!(regex, "[a-z]+");
    } else {
        panic!("Failed to parse regex");
    }

    let mut lx = Token::lexer(r"/abc\/def/  / /  //  /a\//");
    assert_eq!(lx.next(), Some(Ok(Token::Regex(Ok("abc\\/def".to_string())))));
    assert_eq!(lx.next(), Some(Ok(Token::Regex(Ok(" ".to_string())))));
    assert_eq!(lx.next(), Some(Ok(Token::Regex(Ok("".to_string())))));
    assert_eq!(lx.next(), Some(Ok(Token::Regex(Ok("a\\/".to_string())))));
    assert_eq!(lx.next(), None);
}

// #[test]
// fn test_complex_tokens_2() {
//     // Test range
//     let mut lexer = Token::lexer("{1, 5}");
//     if let Some(Ok(Token::Range(Ok(range)))) = lexer.next() {
//         assert_eq!(range, (1, 5));
//     } else {
//         panic!("Failed to parse range");
//     }

//     // Test string literal
//     let mut lexer = Token::lexer(r#""hello world""#);
//     if let Some(Ok(Token::StringLiteral(s))) = lexer.next() {
//         assert_eq!(s, "hello world");
//     } else {
//         panic!("Failed to parse string literal");
//     }

//     // Test hex string
//     let mut lexer = Token::lexer("h'0123456789abcdef'");
//     if let Some(Ok(Token::HexString(Ok(bytes)))) = lexer.next() {
//         assert_eq!(bytes, vec![0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef]);
//     } else {
//         panic!("Failed to parse hex string");
//     }

//     // Test number
//     let mut lexer = Token::lexer("123.45");
//     if let Some(Ok(Token::NumberLiteral(Ok(num)))) = lexer.next() {
//         assert_eq!(num, 123.45);
//     } else {
//         panic!("Failed to parse number");
//     }

//     // Test ISO-8601 date
//     let mut lexer = Token::lexer("2023-01-23T14:35:42Z");
//     if let Some(Ok(Token::DateISO8601(date))) = lexer.next() {
//         assert_eq!(date, "2023-01-23T14:35:42Z");
//     } else {
//         panic!("Failed to parse ISO-8601 date");
//     }
// }

// #[test]
// fn test_pattern_tokenization() {
//     // Test a complete pattern string
//     let pattern = "SUBJECT > ASSERTION-PRED(TEXT(/name/)) > ASSERTION-OBJ(NUMBER(>=18))";
//     let tokens: Vec<_> = Token::lexer(pattern)
//         .collect::<Result<Vec<_>, _>>()
//         .expect("Failed to tokenize pattern");

//     assert!(tokens.len() > 10, "Should have tokenized the entire pattern");

//     // Test a pattern with repetition
//     let pattern = "NODE > (ASSERTION-PRED(TEXT(/^is/)) > ASSERTION-OBJ(ANY))*";
//     let tokens: Vec<_> = Token::lexer(pattern)
//         .collect::<Result<Vec<_>, _>>()
//         .expect("Failed to tokenize pattern");

//     assert!(tokens.len() > 8, "Should have tokenized the entire pattern");
// }
