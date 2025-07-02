//! Comprehensive integration tests for dcbor-pattern conversion
//!
//! These tests demonstrate how dcbor-pattern patterns can be converted to
//! envelope patterns and used in envelope matching scenarios.

use bc_envelope::Envelope;
use bc_envelope_pattern::{
    DCBORPattern, Matcher,
    dcbor_integration::convert_dcbor_pattern_to_envelope_pattern,
};
use dcbor_parse::parse_dcbor_item;

/// Helper function to create an envelope from CBOR diagnostic notation
fn envelope_from_cbor(diagnostic: &str) -> Envelope {
    let cbor = parse_dcbor_item(diagnostic).unwrap();
    Envelope::new(cbor)
}

#[test]
fn test_integration_bool_patterns() {
    // Create dcbor patterns for boolean values
    let dcbor_any_bool = DCBORPattern::any_bool();
    let dcbor_true = DCBORPattern::bool(true);
    let dcbor_false = DCBORPattern::bool(false);

    // Convert to envelope patterns
    let env_any_bool =
        convert_dcbor_pattern_to_envelope_pattern(dcbor_any_bool).unwrap();
    let env_true =
        convert_dcbor_pattern_to_envelope_pattern(dcbor_true).unwrap();
    let env_false =
        convert_dcbor_pattern_to_envelope_pattern(dcbor_false).unwrap();

    // Test with envelope data
    let true_env = envelope_from_cbor("true");
    let false_env = envelope_from_cbor("false");
    let number_env = envelope_from_cbor("42");

    // Test any bool pattern
    assert!(env_any_bool.matches(&true_env));
    assert!(env_any_bool.matches(&false_env));
    assert!(!env_any_bool.matches(&number_env));

    // Test specific bool patterns
    assert!(env_true.matches(&true_env));
    assert!(!env_true.matches(&false_env));
    assert!(!env_true.matches(&number_env));

    assert!(env_false.matches(&false_env));
    assert!(!env_false.matches(&true_env));
    assert!(!env_false.matches(&number_env));
}

#[test]
fn test_integration_number_patterns() {
    // Create dcbor number patterns
    let dcbor_any_number = DCBORPattern::any_number();
    let dcbor_specific = DCBORPattern::number(42);
    let dcbor_range = DCBORPattern::number_range(1.0..=100.0);
    let dcbor_greater = DCBORPattern::number_greater_than(50);

    // Convert to envelope patterns
    let env_any_number =
        convert_dcbor_pattern_to_envelope_pattern(dcbor_any_number).unwrap();
    let env_specific =
        convert_dcbor_pattern_to_envelope_pattern(dcbor_specific).unwrap();
    let env_range =
        convert_dcbor_pattern_to_envelope_pattern(dcbor_range).unwrap();
    let env_greater =
        convert_dcbor_pattern_to_envelope_pattern(dcbor_greater).unwrap();

    // Test with envelope data
    let num_42_env = envelope_from_cbor("42");
    let num_75_env = envelope_from_cbor("75");
    let num_150_env = envelope_from_cbor("150");
    let text_env = envelope_from_cbor("\"hello\"");

    // Test any number pattern
    assert!(env_any_number.matches(&num_42_env));
    assert!(env_any_number.matches(&num_75_env));
    assert!(env_any_number.matches(&num_150_env));
    assert!(!env_any_number.matches(&text_env));

    // Test specific number pattern
    assert!(env_specific.matches(&num_42_env));
    assert!(!env_specific.matches(&num_75_env));

    // Test range pattern
    assert!(env_range.matches(&num_42_env));
    assert!(env_range.matches(&num_75_env));
    assert!(!env_range.matches(&num_150_env));

    // Test greater than pattern
    assert!(!env_greater.matches(&num_42_env));
    assert!(env_greater.matches(&num_75_env));
    assert!(env_greater.matches(&num_150_env));
}

#[test]
fn test_integration_text_patterns() {
    // Create dcbor text patterns
    let dcbor_any_text = DCBORPattern::any_text();
    let dcbor_specific = DCBORPattern::text("hello");
    let dcbor_regex =
        DCBORPattern::text_regex(regex::Regex::new(r"^h.*o$").unwrap());

    // Convert to envelope patterns
    let env_any_text =
        convert_dcbor_pattern_to_envelope_pattern(dcbor_any_text).unwrap();
    let env_specific =
        convert_dcbor_pattern_to_envelope_pattern(dcbor_specific).unwrap();
    let env_regex =
        convert_dcbor_pattern_to_envelope_pattern(dcbor_regex).unwrap();

    // Test with envelope data
    let hello_env = envelope_from_cbor("\"hello\"");
    let world_env = envelope_from_cbor("\"world\"");
    let hero_env = envelope_from_cbor("\"hero\"");
    let number_env = envelope_from_cbor("42");

    // Test any text pattern
    assert!(env_any_text.matches(&hello_env));
    assert!(env_any_text.matches(&world_env));
    assert!(env_any_text.matches(&hero_env));
    assert!(!env_any_text.matches(&number_env));

    // Test specific text pattern
    assert!(env_specific.matches(&hello_env));
    assert!(!env_specific.matches(&world_env));
    assert!(!env_specific.matches(&hero_env));

    // Test regex pattern (starts with 'h', ends with 'o')
    assert!(env_regex.matches(&hello_env));
    assert!(!env_regex.matches(&world_env));
    assert!(env_regex.matches(&hero_env));
}

#[test]
fn test_integration_meta_patterns() {
    // Create dcbor meta patterns
    let dcbor_and = DCBORPattern::and(vec![
        DCBORPattern::number_greater_than(10),
        DCBORPattern::number_less_than(50),
    ]);

    let dcbor_or = DCBORPattern::or(vec![
        DCBORPattern::text("hello"),
        DCBORPattern::number(42),
    ]);

    // Convert to envelope patterns
    let env_and = convert_dcbor_pattern_to_envelope_pattern(dcbor_and).unwrap();
    let env_or = convert_dcbor_pattern_to_envelope_pattern(dcbor_or).unwrap();

    // Test with envelope data
    let num_25_env = envelope_from_cbor("25");
    let num_75_env = envelope_from_cbor("75");
    let num_42_env = envelope_from_cbor("42");
    let hello_env = envelope_from_cbor("\"hello\"");

    // Test AND pattern (10 < x < 50)
    assert!(env_and.matches(&num_25_env));
    assert!(env_and.matches(&num_42_env));
    assert!(!env_and.matches(&num_75_env));

    // Test OR pattern (text "hello" OR number 42)
    assert!(env_or.matches(&hello_env));
    assert!(env_or.matches(&num_42_env));
    assert!(!env_or.matches(&num_25_env));
}

#[test]
fn test_integration_error_handling() {
    // Test patterns that should successfully convert
    let valid_patterns = vec![
        DCBORPattern::any_bool(),
        DCBORPattern::number(42),
        DCBORPattern::text("hello"),
        DCBORPattern::or(vec![
            DCBORPattern::bool(true),
            DCBORPattern::number(123),
        ]),
    ];

    for pattern in valid_patterns {
        let result = convert_dcbor_pattern_to_envelope_pattern(pattern);
        assert!(result.is_ok(), "Pattern conversion should succeed");
    }
}

#[test]
fn test_integration_display_formatting() {
    // Test that converted patterns have reasonable string representations
    let dcbor_bool = DCBORPattern::bool(true);
    let dcbor_number = DCBORPattern::number(42);
    let dcbor_text = DCBORPattern::text("hello");

    let env_bool =
        convert_dcbor_pattern_to_envelope_pattern(dcbor_bool).unwrap();
    let env_number =
        convert_dcbor_pattern_to_envelope_pattern(dcbor_number).unwrap();
    let env_text =
        convert_dcbor_pattern_to_envelope_pattern(dcbor_text).unwrap();

    // Check that string representations are meaningful
    let bool_str = format!("{}", env_bool);
    let number_str = format!("{}", env_number);
    let text_str = format!("{}", env_text);

    assert!(!bool_str.is_empty());
    assert!(!number_str.is_empty());
    assert!(!text_str.is_empty());

    // They should contain some indication of their type/value
    assert!(bool_str.contains("true") || bool_str.contains("bool"));
    assert!(number_str.contains("42") || number_str.contains("number"));
    assert!(text_str.contains("hello") || text_str.contains("text"));
}
