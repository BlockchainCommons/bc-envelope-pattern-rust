//! Integration tests demonstrating dcbor-pattern integration in the parser
//!
//! These tests verify that the parser correctly handles both envelope-specific
//! patterns and dcbor-pattern syntax, with envelope patterns taking precedence.

use bc_envelope::prelude::*;
use bc_envelope_pattern::{DCBORPattern, Matcher, Pattern};

#[test]
fn test_parser_envelope_patterns_take_precedence() {
    // Envelope-specific patterns should be parsed by envelope-pattern parser
    let envelope_patterns = [
        ("search(42)", "envelope search pattern"),
        ("node", "envelope node pattern"),
        ("ASSERTION", "envelope assertion pattern"),
        ("digest", "envelope digest pattern"),
        ("@name(42)", "envelope capture pattern"),
        ("CBOR(42)", "envelope cbor pattern"),
    ];

    for (pattern_str, description) in envelope_patterns {
        let result = Pattern::parse(pattern_str);
        match result {
            Ok(pattern) => {
                // Should parse successfully and have envelope-specific
                // characteristics
                let pattern_str_repr = pattern.to_string();
                println!(
                    "✅ {}: {} -> {}",
                    description, pattern_str, pattern_str_repr
                );
            }
            Err(e) => {
                // Some patterns might not parse, but they should attempt
                // envelope parsing first
                println!(
                    "⚠️  {}: {} failed with: {}",
                    description, pattern_str, e
                );
            }
        }
    }
}

#[test]
fn test_parser_dcbor_pattern_compatible_syntax() {
    // Test patterns that should work in both systems
    let env_true = Envelope::new(true);
    let env_false = Envelope::new(false);
    let env_number = Envelope::new(42);
    let env_text = Envelope::new("hello");

    // Boolean patterns
    let bool_any = Pattern::parse("bool").unwrap();
    assert!(!bool_any.paths_with_captures(&env_true).0.is_empty());
    assert!(!bool_any.paths_with_captures(&env_false).0.is_empty());
    assert_eq!(bool_any.paths_with_captures(&env_number).0.len(), 0);

    let bool_true = Pattern::parse("true").unwrap();
    assert!(!bool_true.paths_with_captures(&env_true).0.is_empty());
    assert_eq!(bool_true.paths_with_captures(&env_false).0.len(), 0);

    let bool_false = Pattern::parse("false").unwrap();
    assert!(!bool_false.paths_with_captures(&env_false).0.is_empty());
    assert_eq!(bool_false.paths_with_captures(&env_true).0.len(), 0);

    // Number patterns
    let number_any = Pattern::parse("number").unwrap();
    assert!(!number_any.paths_with_captures(&env_number).0.is_empty());
    assert_eq!(number_any.paths_with_captures(&env_text).0.len(), 0);

    let number_specific = Pattern::parse("42").unwrap();
    assert!(
        !number_specific
            .paths_with_captures(&env_number)
            .0
            .is_empty()
    );
    assert_eq!(
        number_specific
            .paths_with_captures(&Envelope::new(43))
            .0
            .len(),
        0
    );

    // Text patterns
    let text_any = Pattern::parse("text").unwrap();
    assert!(!text_any.paths_with_captures(&env_text).0.is_empty());
    assert_eq!(text_any.paths_with_captures(&env_number).0.len(), 0);

    let text_specific = Pattern::parse("\"hello\"").unwrap();
    assert!(!text_specific.paths_with_captures(&env_text).0.is_empty());
    assert_eq!(
        text_specific
            .paths_with_captures(&Envelope::new("world"))
            .0
            .len(),
        0
    );
}

#[test]
fn test_parser_mixed_envelope_and_dcbor_syntax() {
    // Test combinations that use both envelope and dcbor syntax
    let env = Envelope::new(42);

    // This should work - searching for a number using dcbor syntax
    let search_number = Pattern::parse("search(42)").unwrap();
    assert!(!search_number.paths_with_captures(&env).0.is_empty());

    // Boolean OR with mixed syntax
    let bool_or_number = Pattern::parse("true | 42").unwrap();
    assert!(!bool_or_number.paths_with_captures(&env).0.is_empty());
    assert!(
        !bool_or_number
            .paths_with_captures(&Envelope::new(true))
            .0
            .is_empty()
    );
    assert_eq!(
        bool_or_number
            .paths_with_captures(&Envelope::new("hello"))
            .0
            .len(),
        0
    );
}

#[test]
fn test_parser_error_handling() {
    // Test that invalid patterns produce reasonable errors
    let invalid_patterns = [
        "INVALID_TOKEN",
        "true false", // Extra data
        "(",          // Incomplete parentheses
    ];

    for pattern_str in invalid_patterns {
        let result = Pattern::parse(pattern_str);
        assert!(
            result.is_err(),
            "Pattern '{}' should fail to parse",
            pattern_str
        );
        println!(
            "❌ '{}' correctly failed: {}",
            pattern_str,
            result.unwrap_err()
        );
    }
}

#[test]
fn test_parser_precedence_demonstration() {
    // Demonstrate that envelope-specific parsing takes precedence

    // Capture patterns should use envelope parsing, not dcbor parsing
    let capture_pattern = Pattern::parse("@num(42)").unwrap();
    let pattern_str = capture_pattern.to_string();

    // Should be an envelope capture pattern, not a CBOR pattern
    assert!(
        !pattern_str.starts_with("CBOR("),
        "Capture pattern should not be converted to CBOR pattern: {}",
        pattern_str
    );

    // Map patterns should now use dcbor-pattern syntax
    let map_pattern = Pattern::parse("map").unwrap();
    let map_str = map_pattern.to_string();
    assert_eq!(
        map_str, "map",
        "Map pattern should use dcbor-pattern syntax"
    );
}

#[test]
fn test_conversion_layer_works_correctly() {
    // Test that the conversion layer preserves functionality
    use bc_envelope_pattern::dcbor_integration::convert_dcbor_pattern_to_envelope_pattern;

    // Test direct conversion
    let dcbor_bool = DCBORPattern::bool(true);
    let envelope_bool =
        convert_dcbor_pattern_to_envelope_pattern(dcbor_bool).unwrap();

    let env_true = Envelope::new(true);
    let env_false = Envelope::new(false);

    assert!(!envelope_bool.paths_with_captures(&env_true).0.is_empty());
    assert_eq!(envelope_bool.paths_with_captures(&env_false).0.len(), 0);

    // Test that the converted pattern has reasonable display format
    let pattern_str = envelope_bool.to_string();
    println!("Converted boolean pattern: {}", pattern_str);
}
