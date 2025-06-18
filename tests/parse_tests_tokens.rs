use bc_envelope_pattern::{Greediness, RepeatRange, Token};
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
    assert_eq!(Token::lexer("ASSERT").next(), Some(Ok(Token::Assertion)));
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
    assert_eq!(
        lx.next(),
        Some(Ok(Token::Regex(Ok("abc\\/def".to_string()))))
    );
    assert_eq!(lx.next(), Some(Ok(Token::Regex(Ok(" ".to_string())))));
    assert_eq!(lx.next(), Some(Ok(Token::Regex(Ok("".to_string())))));
    assert_eq!(lx.next(), Some(Ok(Token::Regex(Ok("a\\/".to_string())))));
    assert_eq!(lx.next(), None);
}

#[test]
fn test_unsigned_integer() {
    let mut lexer = Token::lexer("42");
    if let Some(Ok(Token::UnsignedInteger(Ok(42)))) = lexer.next() {
        // Successfully parsed integer
    } else {
        panic!("Failed to parse integer literal");
    }

    // Test unsigned integer
    let mut lexer = Token::lexer("0");
    if let Some(Ok(Token::UnsignedInteger(Ok(0)))) = lexer.next() {
        // Successfully parsed zero
    } else {
        panic!("Failed to parse zero literal");
    }
}

#[test]
fn test_range() {
    struct RangeTestCase {
        input: &'static str,
        expected: RepeatRange,
    }
    let test_cases = vec![
        RangeTestCase {
            input: "{1, 5}",
            expected: RepeatRange::new(1..=5, Greediness::default()),
        },
        RangeTestCase {
            input: "{ 3 , }",
            expected: RepeatRange::new(3.., Greediness::default()),
        },
        RangeTestCase {
            input: "{ 5 }",
            expected: RepeatRange::new(5..=5, Greediness::default()),
        },
        RangeTestCase {
            input: "{1, 5 }?",
            expected: RepeatRange::new(1..=5, Greediness::Lazy),
        },
        RangeTestCase {
            input: "{ 3 , }?",
            expected: RepeatRange::new(3.., Greediness::Lazy),
        },
        RangeTestCase {
            input: "{5}?",
            expected: RepeatRange::new(5..=5, Greediness::Lazy),
        },
        RangeTestCase {
            input: "{ 1,5}+",
            expected: RepeatRange::new(1..=5, Greediness::Possessive),
        },
        RangeTestCase {
            input: "{ 3 , }+",
            expected: RepeatRange::new(3.., Greediness::Possessive),
        },
        RangeTestCase {
            input: "{5}+",
            expected: RepeatRange::new(5..=5, Greediness::Possessive),
        },
    ];

    let mut failed_cases = vec![];

    for test_case in test_cases {
        let mut lexer = Token::lexer(test_case.input);
        if let Some(Ok(Token::Range(Ok(range)))) = lexer.next() {
            assert_eq!(range, test_case.expected);
        } else {
            failed_cases.push(test_case.input);
        }
    }

    if !failed_cases.is_empty() {
        panic!("Failed to parse ranges: {:?}", failed_cases);
    }
}
