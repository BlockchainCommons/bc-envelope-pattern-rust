mod common;

use bc_envelope::prelude::*;
use bc_envelope_pattern::{Matcher, Pattern, format_paths};
use dcbor_parse::parse_dcbor_item;
use indoc::indoc;

/// Test CBOR pattern path extension with proper formatting and assertions.
/// This test verifies that when a CBOR pattern matches, the returned paths
/// include the internal structure of the CBOR as Envelope path elements.
#[test]
fn test_cbor_pattern_simple_array_paths() {
    // Create a simple array envelope
    let array_data = vec![1, 2, 3];
    let envelope = Envelope::new(array_data);

    // Use SEARCH(number) to find all numbers in the array
    let pattern = Pattern::parse("CBOR(/SEARCH(number)/)").unwrap();
    let paths = pattern.paths(&envelope);

    // Format the paths for comparison
    let actual = format_paths(&paths);

    // Expected: 3 paths, each with root envelope + specific number envelope
    // Note: The order matches what dcbor-pattern returns (corrected order)
    #[rustfmt::skip]
    let expected = indoc! {r#"
        4abc3113 LEAF [1, 2, 3]
            4bf5122f LEAF 1
        4abc3113 LEAF [1, 2, 3]
            dbc1b4c9 LEAF 2
        4abc3113 LEAF [1, 2, 3]
            084fed08 LEAF 3
    "#}
    .trim();

    assert_actual_expected!(
        actual,
        expected,
        "CBOR pattern should return extended paths for array elements"
    );
}

#[test]
fn test_cbor_pattern_nested_structure_paths() {
    // Create a nested CBOR structure
    let nested_cbor =
        parse_dcbor_item(r#"{"scores": [95, 87, 92], "value": 42}"#).unwrap();
    let envelope = Envelope::new(nested_cbor);

    // Search for all numbers in the structure
    let pattern = Pattern::parse("CBOR(/SEARCH(number)/)").unwrap();
    let paths = pattern.paths(&envelope);

    // Should find 4 numbers: 42, 95, 87, 92
    assert_eq!(
        paths.len(),
        4,
        "Should find 4 numbers in the nested structure"
    );

    // Verify that each path starts with the root envelope and extends
    for (i, path) in paths.iter().enumerate() {
        assert_eq!(
            path[0], envelope,
            "Path {} should start with root envelope",
            i
        );
        assert!(path.len() > 1, "Path {} should be extended beyond root", i);
    }
}

#[test]
fn test_cbor_pattern_single_value_paths() {
    // Test with a single number - should match with minimal extension
    let envelope = Envelope::new(42);

    let pattern = Pattern::parse("CBOR(/number/)").unwrap();
    let paths = pattern.paths(&envelope);

    let actual = format_paths(&paths);
    let expected = "7f83f7bd LEAF 42";

    assert_actual_expected!(
        actual,
        expected,
        "Single value CBOR pattern should return just the root envelope"
    );
}

#[test]
fn test_cbor_pattern_text_search_paths() {
    // Test searching for text values in a complex structure
    let cbor_data = parse_dcbor_item(
        r#"{"name": "Alice", "items": ["apple", "banana"], "count": 2}"#,
    )
    .unwrap();
    let envelope = Envelope::new(cbor_data);

    let pattern = Pattern::parse("CBOR(/SEARCH(text)/)").unwrap();
    let paths = pattern.paths(&envelope);

    // Should find 6 text values: "Alice", "apple", "banana", "name", "items",
    // "count"
    assert_eq!(
        paths.len(),
        6,
        "Should find 6 text values including map keys"
    );

    // Verify path structure
    for path in &paths {
        assert_eq!(
            path[0], envelope,
            "Each path should start with root envelope"
        );
        assert!(path.len() > 1, "Each path should be extended beyond root");

        // The last element should be a text envelope
        let last_element = path.last().unwrap();
        if let Some(cbor) = last_element.as_leaf() {
            assert!(
                matches!(cbor.as_case(), dcbor::CBORCase::Text(_)),
                "Last element should be text CBOR"
            );
        }
    }
}

#[test]
fn test_cbor_pattern_no_matches_paths() {
    // Test pattern that doesn't match anything
    let envelope = Envelope::new("just text");

    let pattern = Pattern::parse("CBOR(/number/)").unwrap();
    let paths = pattern.paths(&envelope);

    let actual = format_paths(&paths);
    let expected = "";

    assert_actual_expected!(
        actual,
        expected,
        "Pattern with no matches should return empty paths"
    );
}

#[test]
fn test_cbor_pattern_paths_preserve_order() {
    // Test that paths maintain consistent ordering
    let array_cbor = parse_dcbor_item("[10, 20, 30]").unwrap();
    let envelope = Envelope::new(array_cbor);

    let pattern = Pattern::parse("CBOR(/SEARCH(number)/)").unwrap();
    let paths = pattern.paths(&envelope);

    assert_eq!(paths.len(), 3, "Should find 3 numbers");

    // Extract the numbers from the paths to verify order
    let mut found_numbers = Vec::new();
    for path in &paths {
        if let Some(last_element) = path.last() {
            if let Some(cbor) = last_element.as_leaf() {
                if let dcbor::CBORCase::Unsigned(n) = cbor.as_case() {
                    found_numbers.push(*n);
                }
            }
        }
    }

    // Numbers should be found in dcbor-pattern order: 10, 20, 30
    assert_eq!(
        found_numbers,
        vec![10, 20, 30],
        "Numbers should be found in dcbor-pattern order"
    );
}

#[test]
fn test_cbor_pattern_complex_nested_paths() {
    // Test deeply nested structure to verify path extension works at all levels
    let complex_cbor = parse_dcbor_item(
        r#"
        {
            "users": [
                {"name": "Alice", "age": 30},
                {"name": "Bob", "age": 25}
            ],
            "total": 2
        }
    "#,
    )
    .unwrap();
    let envelope = Envelope::new(complex_cbor);

    let pattern = Pattern::parse("CBOR(/SEARCH(number)/)").unwrap();
    let paths = pattern.paths(&envelope);

    // Should find 3 numbers: 30, 25, 2
    assert_eq!(paths.len(), 3, "Should find 3 numbers in complex structure");

    // Verify that paths have appropriate lengths based on nesting
    let path_lengths: Vec<usize> = paths.iter().map(|p| p.len()).collect();

    // All paths should be extended beyond just the root
    for &length in &path_lengths {
        assert!(
            length > 1,
            "All paths should be extended beyond root envelope"
        );
    }
}
