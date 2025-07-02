mod common;

use indoc::indoc;
use bc_envelope::Envelope;
use bc_envelope_pattern::{Pattern, Matcher, format_paths};
use dcbor_parse::parse_dcbor_item;

#[test]
fn test_cbor_pattern_extended_paths() {
    // Test that `cbor` patterns now return extended paths that include the internal `cbor` structure

    // Test with a simple array - should return paths to each number
    let array_data = vec![1, 2, 3];
    let envelope = Envelope::new(array_data.clone());

    let pattern = Pattern::parse("cbor(/search(number)/)").unwrap();
    let paths = pattern.paths(&envelope);

    // Should return 3 paths, one for each number in the array
    assert_eq!(paths.len(), 3, "Should find 3 numbers in the array");

    // Format the paths for comparison
    let actual = format_paths(&paths);
    #[rustfmt::skip]
    let expected = indoc! {r#"
        4abc3113 LEAF [1, 2, 3]
            4bf5122f LEAF 1
        4abc3113 LEAF [1, 2, 3]
            dbc1b4c9 LEAF 2
        4abc3113 LEAF [1, 2, 3]
            084fed08 LEAF 3
    "#}.trim();

    assert_actual_expected!(actual, expected, "`cbor` pattern should return extended paths for array elements");
}

#[test]
fn test_cbor_pattern_extended_paths_nested_structure() {
    // Test with a more complex nested structure
    let nested_cbor = parse_dcbor_item(r#"{"name": "Alice", "scores": [95, 87, 92]}"#).unwrap();
    let envelope = Envelope::new(nested_cbor.clone());

    let pattern = Pattern::parse("cbor(/search(number)/)").unwrap();
    let paths = pattern.paths(&envelope);

    // Should return 3 paths, one for each score in the array
    assert_eq!(paths.len(), 3, "Should find 3 numbers in the nested structure");

    // Format the paths for comparison
    let actual = format_paths(&paths);
    #[rustfmt::skip]
    let expected = indoc! {r#"
        73d02807 LEAF {"name": "Alice", "scores": [95, 87, 92]}
            3a129d53 LEAF [95, 87, 92]
                61544f78 LEAF 95
        73d02807 LEAF {"name": "Alice", "scores": [95, 87, 92]}
            3a129d53 LEAF [95, 87, 92]
                8fa86205 LEAF 87
        73d02807 LEAF {"name": "Alice", "scores": [95, 87, 92]}
            3a129d53 LEAF [95, 87, 92]
                672fa214 LEAF 92
    "#}.trim();

    assert_actual_expected!(actual, expected, "Nested structure should show proper path extension with intermediate elements");
}

#[test]
fn test_cbor_pattern_extended_paths_single_match() {
    // Test with a pattern that matches the root element itself
    let envelope = Envelope::new(42);

    let pattern = Pattern::parse("cbor(/number/)").unwrap();
    let paths = pattern.paths(&envelope);

    // Should return 1 path
    assert_eq!(paths.len(), 1, "Should find 1 number");

    // Format the paths for comparison - single value should not extend
    let actual = format_paths(&paths);
    let expected = "7f83f7bd LEAF 42";

    assert_actual_expected!(actual, expected, "Single value `cbor` pattern should return just the root envelope");
}

#[test]
fn test_cbor_pattern_no_extended_paths_for_non_pattern() {
    // Test that non-pattern `cbor` matchers still work as before
    let envelope = Envelope::new(42);

    // Test cbor() - should match any `cbor`
    let any_pattern = Pattern::parse("cbor").unwrap();
    let paths = any_pattern.paths(&envelope);
    assert_eq!(paths.len(), 1);

    let actual = format_paths(&paths);
    let expected = "7f83f7bd LEAF 42";
    assert_actual_expected!(actual, expected, "`cbor` without pattern should return just the root envelope");

    // Test cbor(42) - should match the exact value
    let exact_pattern = Pattern::parse("cbor(42)").unwrap();
    let paths = exact_pattern.paths(&envelope);
    assert_eq!(paths.len(), 1);

    let actual = format_paths(&paths);
    let expected = "7f83f7bd LEAF 42";
    assert_actual_expected!(actual, expected, "`cbor` with exact value should return just the root envelope");
}
