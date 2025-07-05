use bc_envelope::prelude::*;
use bc_envelope_pattern::{Matcher, Pattern};

#[test]
fn test_cbor_pattern_dcbor_pattern_integration() {
    // Test basic dcbor-pattern expressions - these should work reliably
    let number_pattern = Pattern::parse(r#"cbor(/number/)"#)
        .expect("number pattern should parse successfully");
    let envelope = Envelope::new(42);
    assert!(
        number_pattern.matches(&envelope),
        "dcbor-pattern /number/ should match integer"
    );

    // Test dcbor-pattern with array
    let array_pattern = Pattern::parse(r#"cbor(/array/)"#)
        .expect("array pattern should parse successfully");
    let array_envelope = Envelope::new(vec![1, 2, 3]);
    assert!(
        array_pattern.matches(&array_envelope),
        "dcbor-pattern /array/ should match arrays"
    );

    // Test dcbor-pattern with text
    let text_pattern = Pattern::parse(r#"cbor(/text/)"#)
        .expect("text pattern should parse successfully");
    let text_envelope = Envelope::new("hello");
    assert!(
        text_pattern.matches(&text_envelope),
        "dcbor-pattern /text/ should match text"
    );
}

#[test]
fn test_cbor_pattern_any() {
    let envelope = Envelope::new(123);

    // Test `cbor` pattern
    let any_pattern = Pattern::parse("cbor").unwrap();
    assert!(
        any_pattern.matches(&envelope),
        "`cbor` should match any CBOR value"
    );
}

#[test]
fn test_cbor_pattern_exact_values() {
    // Test exact numeric match
    let envelope = Envelope::new(42);
    let exact_pattern = Pattern::parse("cbor(42)").unwrap();
    assert!(
        exact_pattern.matches(&envelope),
        "Should match exact CBOR value"
    );

    // Test exact string match
    let envelope = Envelope::new("hello");
    let text_pattern = Pattern::parse(r#"cbor("hello")"#).unwrap();
    assert!(
        text_pattern.matches(&envelope),
        "Should match exact CBOR text"
    );

    // Test exact array match
    let envelope = Envelope::new(vec![1, 2, 3]);
    let array_pattern = Pattern::parse("cbor([1, 2, 3])").unwrap();
    assert!(
        array_pattern.matches(&envelope),
        "Should match exact CBOR array"
    );
}

#[test]
fn test_cbor_pattern_complex_structures() {
    // Test complex map structure - let's try simpler syntax first
    let mut map = dcbor::Map::new();
    map.insert("name", "Alice");
    map.insert("age", 42);
    let envelope = Envelope::new(map);

    // Match with exact diagnostic notation
    let pattern2 =
        Pattern::parse(r#"cbor({"name": "Alice", "age": 42})"#).unwrap();
    assert!(
        pattern2.matches(&envelope),
        "Diagnostic notation should match map"
    );
}

#[test]
fn test_cbor_pattern_debug_parser() {
    // Let's test what the parser actually supports
    println!("Testing basic `cbor` parsing...");

    // Test basic `cbor` patterns that should work
    assert!(Pattern::parse("cbor").is_ok());
    assert!(Pattern::parse("cbor(42)").is_ok());
    assert!(Pattern::parse(r#"cbor("hello")"#).is_ok());
    assert!(Pattern::parse("cbor([1, 2, 3])").is_ok());

    // Test dcbor-pattern syntax
    let dcbor_result = Pattern::parse(r#"cbor(/number/)"#);
    println!("cbor(/number/) parse result: {:?}", dcbor_result);

    let dcbor_array_result = Pattern::parse(r#"cbor(/[*]/)"#);
    println!("cbor(/[*]/) parse result: {:?}", dcbor_array_result);
}

#[test]
fn test_cbor_pattern_parsing_errors() {
    // Test invalid dcbor-pattern syntax - "uint" is not a valid dcbor-pattern
    // keyword
    let invalid_pattern = Pattern::parse(r#"cbor(/uint/)"#);
    assert!(
        invalid_pattern.is_err(),
        "Invalid dcbor-pattern should fail to parse"
    );

    // Test another invalid keyword
    let invalid_pattern2 = Pattern::parse(r#"cbor(/int/)"#);
    assert!(
        invalid_pattern2.is_err(),
        "Invalid dcbor-pattern should fail to parse"
    );

    // Test invalid diagnostic notation
    let invalid_diag = Pattern::parse(r#"cbor({invalid: syntax)"#);
    assert!(
        invalid_diag.is_err(),
        "Invalid diagnostic notation should fail to parse"
    );

    // Test valid dcbor-pattern syntax should work
    let valid_pattern = Pattern::parse(r#"cbor(/number/)"#);
    assert!(
        valid_pattern.is_ok(),
        "Valid dcbor-pattern should parse successfully"
    );

    let valid_pattern2 = Pattern::parse(r#"cbor(/text/)"#);
    assert!(
        valid_pattern2.is_ok(),
        "Valid dcbor-pattern should parse successfully"
    );
}

#[test]
fn test_dcbor_patterns_work_directly() {
    // These should work without any fallbacks - dcbor-pattern integration is
    // complete
    let number_pattern = Pattern::parse(r#"cbor(/number/)"#)
        .expect("number pattern should parse");
    let envelope = Envelope::new(42);
    assert!(
        number_pattern.matches(&envelope),
        "number pattern should match integer"
    );

    let array_pattern =
        Pattern::parse(r#"cbor(/array/)"#).expect("array of any pattern should parse");
    let array_envelope = Envelope::new(vec![1, 2, 3]);
    assert!(
        array_pattern.matches(&array_envelope),
        "array pattern should match array"
    );

    let text_pattern =
        Pattern::parse(r#"cbor(/text/)"#).expect("text pattern should parse");
    let text_envelope = Envelope::new("hello");
    assert!(
        text_pattern.matches(&text_envelope),
        "text pattern should match string"
    );
}
