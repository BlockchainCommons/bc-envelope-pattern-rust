mod common;

use indoc::indoc;
use bc_envelope::prelude::*;
use bc_envelope_pattern::{Matcher, Pattern, format_paths};
use dcbor_parse::parse_dcbor_item;
use dcbor_pattern::{Matcher as _, Pattern as DcborPattern};

/// Test CBOR pattern path extension functionality using proper formatting and assertions.
/// These tests verify that CBOR patterns extend paths to include the internal CBOR structure
/// elements as individual Envelope path components.

#[test]
fn test_cbor_pattern_simple_array_paths() {
    // Create a simple array envelope
    let array_data = vec![1, 2, 3];
    let envelope = Envelope::new(array_data);

    // Use SEARCH(NUMBER) to find all numbers in the array
    let pattern = Pattern::parse("CBOR(/SEARCH(NUMBER)/)").unwrap();
    let paths = pattern.paths(&envelope);

    // Should find 3 numbers with extended paths
    assert_eq!(paths.len(), 3, "Should find 3 numbers in the array");

    // Format the paths for comparison
    let actual = format_paths(&paths);
    let expected = indoc! {r#"
        4abc3113 LEAF [1, 2, 3]
            4bf5122f LEAF 1
        4abc3113 LEAF [1, 2, 3]
            dbc1b4c9 LEAF 2
        4abc3113 LEAF [1, 2, 3]
            084fed08 LEAF 3
    "#}.trim();

    assert_actual_expected!(actual, expected, "CBOR pattern should return extended paths for array elements");
}

#[test]
fn test_cbor_pattern_nested_structure_paths() {
    // Create a nested CBOR structure
    let nested_cbor = parse_dcbor_item(r#"{"scores": [95, 87, 92], "value": 42}"#).unwrap();
    let envelope = Envelope::new(nested_cbor);

    // Search for all numbers in the structure
    let pattern = Pattern::parse("CBOR(/SEARCH(NUMBER)/)").unwrap();
    let paths = pattern.paths(&envelope);

    // Should find 4 numbers: 42, 95, 87, 92
    assert_eq!(paths.len(), 4, "Should find 4 numbers in the nested structure");

    // Format the paths and verify the nested structure
    let actual = format_paths(&paths);
    let expected = indoc! {r#"
        df80ebe9 LEAF {"value": 42, "scores": [95, 87, 92]}
            7f83f7bd LEAF 42
        df80ebe9 LEAF {"value": 42, "scores": [95, 87, 92]}
            3a129d53 LEAF [95, 87, 92]
                672fa214 LEAF 92
        df80ebe9 LEAF {"value": 42, "scores": [95, 87, 92]}
            3a129d53 LEAF [95, 87, 92]
                8fa86205 LEAF 87
        df80ebe9 LEAF {"value": 42, "scores": [95, 87, 92]}
            3a129d53 LEAF [95, 87, 92]
                61544f78 LEAF 95
    "#}.trim();

    assert_actual_expected!(actual, expected, "Nested structure should show proper path extension with intermediate elements");
}

#[test]
fn test_cbor_pattern_single_value_paths() {
    // Test with a single number - should match with no extension needed
    let envelope = Envelope::new(42);

    let pattern = Pattern::parse("CBOR(/NUMBER/)").unwrap();
    let paths = pattern.paths(&envelope);

    // Should find exactly one path with just the root envelope
    assert_eq!(paths.len(), 1, "Should find exactly one number");

    let actual = format_paths(&paths);
    let expected = "7f83f7bd LEAF 42";

    assert_actual_expected!(actual, expected, "Single value CBOR pattern should return just the root envelope");
}

#[test]
fn test_cbor_pattern_text_search_paths() {
    // Test searching for text values in a complex structure
    let cbor_data = parse_dcbor_item(r#"{"name": "Alice", "items": ["apple", "banana"], "count": 2}"#).unwrap();
    let envelope = Envelope::new(cbor_data);

    let pattern = Pattern::parse("CBOR(/SEARCH(TEXT)/)").unwrap();
    let paths = pattern.paths(&envelope);

    // Should find 6 text values including map keys
    assert_eq!(paths.len(), 6, "Should find 6 text values including map keys");

    let actual = format_paths(&paths);
    let expected = indoc! {r#"
        6254f700 LEAF {"name": "Alice", "count": 2, "items": ["apple", "banana"]}
            800a0588 LEAF "name"
        6254f700 LEAF {"name": "Alice", "count": 2, "items": ["apple", "banana"]}
            a3ad5766 LEAF ["apple", "banana"]
                b863e7f4 LEAF "banana"
        6254f700 LEAF {"name": "Alice", "count": 2, "items": ["apple", "banana"]}
            a3ad5766 LEAF ["apple", "banana"]
                cc1e16a1 LEAF "apple"
        6254f700 LEAF {"name": "Alice", "count": 2, "items": ["apple", "banana"]}
            9e381786 LEAF "items"
        6254f700 LEAF {"name": "Alice", "count": 2, "items": ["apple", "banana"]}
            8a72e186 LEAF "count"
        6254f700 LEAF {"name": "Alice", "count": 2, "items": ["apple", "banana"]}
            13941b48 LEAF "Alice"
    "#}.trim();

    assert_actual_expected!(actual, expected, "Text search should find all text elements with proper path extension");
}

#[test]
fn test_cbor_pattern_no_matches_paths() {
    // Test pattern that doesn't match anything
    let envelope = Envelope::new("just text");

    let pattern = Pattern::parse("CBOR(/NUMBER/)").unwrap();
    let paths = pattern.paths(&envelope);

    // Should find no paths
    assert_eq!(paths.len(), 0, "Should find no matches");

    let actual = format_paths(&paths);
    let expected = "";

    assert_actual_expected!(actual, expected, "Pattern with no matches should return empty paths");
}

#[test]
fn test_cbor_pattern_paths_preserve_order() {
    // Test that paths maintain consistent ordering as found by dcbor-pattern
    let array_cbor = parse_dcbor_item("[10, 20, 30]").unwrap();
    let envelope = Envelope::new(array_cbor);

    let pattern = Pattern::parse("CBOR(/SEARCH(NUMBER)/)").unwrap();
    let paths = pattern.paths(&envelope);

    assert_eq!(paths.len(), 3, "Should find 3 numbers");

    let actual = format_paths(&paths);
    let expected = indoc! {r#"
        5e81a0f3 LEAF [10, 20, 30]
            01ba4719 LEAF 10
        5e81a0f3 LEAF [10, 20, 30]
            cf972730 LEAF 30
        5e81a0f3 LEAF [10, 20, 30]
            83891d7f LEAF 20
    "#}.trim();

    assert_actual_expected!(actual, expected, "Numbers should be found in dcbor-pattern order");
}

#[test]
fn test_cbor_pattern_complex_nested_paths() {
    // Test deeply nested structure to verify path extension works at all levels
    let complex_cbor = parse_dcbor_item(r#"
        {
            "users": [
                {"name": "Alice", "age": 30},
                {"name": "Bob", "age": 25}
            ],
            "total": 2
        }
    "#).unwrap();
    let envelope = Envelope::new(complex_cbor);

    let pattern = Pattern::parse("CBOR(/SEARCH(NUMBER)/)").unwrap();
    let paths = pattern.paths(&envelope);

    // Should find 3 numbers: 30, 25, 2
    assert_eq!(paths.len(), 3, "Should find 3 numbers in complex structure");

    let actual = format_paths(&paths);
    let expected = indoc! {r#"
        e341ba42 LEAF {"total": 2, "users": [{"age": 30, "name": "Alice"}, {"age": 25, "name": "Bob"}]}
            dbc1b4c9 LEAF 2
        e341ba42 LEAF {"total": 2, "users": [{"age": 30, "name": "Alice"}, {"age": 25, "name": "Bob"}]}
            c83073cd LEAF [{"age": 30, "name": "Alice"}, {"age": 25, "name": "Bob"}]
                728f5697 LEAF {"age": 25, "name": "Bob"}
                    eb55bbe1 LEAF 25
        e341ba42 LEAF {"total": 2, "users": [{"age": 30, "name": "Alice"}, {"age": 25, "name": "Bob"}]}
            c83073cd LEAF [{"age": 30, "name": "Alice"}, {"age": 25, "name": "Bob"}]
                a9c2e8b9 LEAF {"age": 30, "name": "Alice"}
                    cf972730 LEAF 30
    "#}.trim();

    assert_actual_expected!(actual, expected, "Complex nested structure should show full path extension through all levels");
}

#[test]
fn test_cbor_pattern_map_key_value_paths() {
    // Test that we can extend paths through map structures properly
    let map_cbor = parse_dcbor_item(r#"{"a": 1, "b": {"c": 2}}"#).unwrap();
    let envelope = Envelope::new(map_cbor);

    let pattern = Pattern::parse("CBOR(/SEARCH(NUMBER)/)").unwrap();
    let paths = pattern.paths(&envelope);

    // Should find 2 numbers: 1 and 2
    assert_eq!(paths.len(), 2, "Should find 2 numbers in map structure");

    // Verify that each path starts with the root and extends appropriately
    for (i, path) in paths.iter().enumerate() {
        assert_eq!(path[0], envelope, "Path {} should start with root envelope", i);
        assert!(path.len() > 1, "Path {} should be extended beyond root", i);

        // The last element should be a number envelope
        let last_element = path.last().unwrap();
        if let Some(cbor) = last_element.as_leaf() {
            assert!(matches!(cbor.as_case(), dcbor::CBORCase::Unsigned(_)),
                   "Last element of path {} should be a number", i);
        }
    }
}


#[test]
fn test_search_array_order() {
    let cbor = dcbor_parse::parse_dcbor_item(r#"[[1, 2, 3], [4, 5, 6]]"#).unwrap();
    let dcbor_pattern = DcborPattern::parse("SEARCH(ARRAY)").unwrap();

    let paths = dcbor_pattern.paths(&cbor);
    #[rustfmt::skip]
    let expected = indoc! {r#"
        [[1, 2, 3], [4, 5, 6]]
        [[1, 2, 3], [4, 5, 6]]
            [1, 2, 3]
        [[1, 2, 3], [4, 5, 6]]
            [4, 5, 6]
    "#}.trim();
    assert_actual_expected!(dcbor_pattern::format_paths(&paths), expected);

    let pattern = DcborPattern::parse("SEARCH(NUMBER)").unwrap();
    let paths = pattern.paths(&cbor);
    #[rustfmt::skip]
    let expected = indoc! {r#"
        [[1, 2, 3], [4, 5, 6]]
            [1, 2, 3]
                1
        [[1, 2, 3], [4, 5, 6]]
            [1, 2, 3]
                2
        [[1, 2, 3], [4, 5, 6]]
            [1, 2, 3]
                3
        [[1, 2, 3], [4, 5, 6]]
            [4, 5, 6]
                4
        [[1, 2, 3], [4, 5, 6]]
            [4, 5, 6]
                5
        [[1, 2, 3], [4, 5, 6]]
            [4, 5, 6]
                6
    "#}.trim();
    assert_actual_expected!(dcbor_pattern::format_paths(&paths), expected);

    let envelope = Envelope::new(cbor);
    let pattern = Pattern::parse("CBOR(/SEARCH(NUMBER)/)").unwrap();
    let paths = pattern.paths(&envelope);
    // The traversal order below should be the same as the one above.
    #[rustfmt::skip]
    let expected = indoc! {r#"
        88c5c85e LEAF [[1, 2, 3], [4, 5, 6]]
            4abc3113 LEAF [1, 2, 3]
                4bf5122f LEAF 1
        88c5c85e LEAF [[1, 2, 3], [4, 5, 6]]
            4abc3113 LEAF [1, 2, 3]
                dbc1b4c9 LEAF 2
        88c5c85e LEAF [[1, 2, 3], [4, 5, 6]]
            4abc3113 LEAF [1, 2, 3]
                084fed08 LEAF 3
        88c5c85e LEAF [[1, 2, 3], [4, 5, 6]]
            f215fbf4 LEAF [4, 5, 6]
                e52d9c50 LEAF 4
        88c5c85e LEAF [[1, 2, 3], [4, 5, 6]]
            f215fbf4 LEAF [4, 5, 6]
                e77b9a9a LEAF 5
        88c5c85e LEAF [[1, 2, 3], [4, 5, 6]]
            f215fbf4 LEAF [4, 5, 6]
                67586e98 LEAF 6
    "#}.trim();
    assert_actual_expected!(format_paths(&paths), expected);
}
