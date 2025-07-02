mod common;

use bc_envelope::prelude::*;
use bc_envelope_pattern::{Matcher, Pattern, format_paths};
use dcbor_parse::parse_dcbor_item;
use indoc::indoc;

#[test]
fn test_dcbor_pattern_extended_paths() {
    // Create a complex nested `cbor` structure
    let nested_cbor =
        parse_dcbor_item(r#"{"numbers": [1, 2, 3], "nested": {"value": 42}}"#)
            .unwrap();
    let envelope = Envelope::new(nested_cbor);

    // Test `search(number)` - should find all numbers in the structure
    let pattern = Pattern::parse("cbor(/search(number)/)").unwrap();
    let paths = pattern.paths(&envelope);

    // Should find 4 numbers: 1, 2, 3, and 42
    assert_eq!(
        paths.len(),
        4,
        "Should find 4 numbers in the nested structure"
    );

    // Format the paths for comparison
    let actual = format_paths(&paths);
    #[rustfmt::skip]
    let expected = indoc! {r#"
        4bd40828 LEAF {"nested": {"value": 42}, "numbers": [1, 2, 3]}
            563fb650 LEAF {"value": 42}
                7f83f7bd LEAF 42
        4bd40828 LEAF {"nested": {"value": 42}, "numbers": [1, 2, 3]}
            4abc3113 LEAF [1, 2, 3]
                4bf5122f LEAF 1
        4bd40828 LEAF {"nested": {"value": 42}, "numbers": [1, 2, 3]}
            4abc3113 LEAF [1, 2, 3]
                dbc1b4c9 LEAF 2
        4bd40828 LEAF {"nested": {"value": 42}, "numbers": [1, 2, 3]}
            4abc3113 LEAF [1, 2, 3]
                084fed08 LEAF 3
    "#}
    .trim();

    assert_actual_expected!(
        actual,
        expected,
        "`cbor` pattern should return extended paths for all numbers in nested structure"
    );
}

#[test]
fn test_dcbor_pattern_simple_leaf_paths() {
    // Test with a simple number - should have minimal path extension
    let envelope = Envelope::new(42);
    let pattern = Pattern::parse("cbor(/number/)").unwrap();
    let paths = pattern.paths(&envelope);

    assert_eq!(paths.len(), 1, "Should find 1 number");

    // Format the paths for comparison - single value should not extend
    let actual = format_paths(&paths);
    let expected = "7f83f7bd LEAF 42";

    assert_actual_expected!(
        actual,
        expected,
        "Single value `cbor` pattern should return just the root envelope"
    );
}

#[test]
fn test_dcbor_pattern_array_paths() {
    // Test array access with specific element patterns
    let array_cbor = parse_dcbor_item("[1, \"hello\", true]").unwrap();
    let envelope = Envelope::new(array_cbor);

    // Find all text values in the array
    let pattern = Pattern::parse("cbor(/search(text)/)").unwrap();
    let paths = pattern.paths(&envelope);

    assert_eq!(paths.len(), 1, "Should find 1 text element"); // Should find "hello"

    // Format the paths for comparison
    let actual = format_paths(&paths);
    #[rustfmt::skip]
    let expected = indoc! {r#"
        4cd61f73 LEAF [1, "hello", true]
            cb835593 LEAF "hello"
    "#}
    .trim();

    assert_actual_expected!(
        actual,
        expected,
        "Text search should find hello with proper path extension"
    );
}

#[test]
fn test_dcbor_pattern_array_elements() {
    // Test what dcbor-pattern returns for array structure
    let array_cbor = parse_dcbor_item("[1, 2, 3]").unwrap();
    let envelope = Envelope::new(array_cbor);

    let pattern = Pattern::parse("cbor(/search(number)/)").unwrap();
    let paths = pattern.paths(&envelope);

    // Should find 3 numbers: 1, 2, 3
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
    "#}
    .trim();

    assert_actual_expected!(
        actual,
        expected,
        "Array numbers should be found with proper path extension"
    );
}

#[test]
fn test_cbor_pattern_multiple_paths() {
    // Create a `cbor` structure with nested numbers
    let cbor_data = dcbor_parse::parse_dcbor_item(
        r#"
        {
            "numbers": [1, 2, 3],
            "value": 42
        }
    "#,
    )
    .unwrap();

    let envelope = Envelope::new(cbor_data);

    // Test `search(number)` pattern to find all numbers in the structure
    let pattern = Pattern::parse("cbor(/search(number)/)").unwrap();
    let paths = pattern.paths(&envelope);

    // We should find 4 numbers: 1, 2, 3, 42
    // Each should have its own path showing the route through the `cbor`
    // structure
    assert_eq!(
        paths.len(),
        4,
        "Should find 4 numbers in the `cbor` structure"
    );

    // Format the paths for comparison
    let actual = format_paths(&paths);
    #[rustfmt::skip]
    let expected = indoc! {r#"
        832e44f1 LEAF {"value": 42, "numbers": [1, 2, 3]}
            7f83f7bd LEAF 42
        832e44f1 LEAF {"value": 42, "numbers": [1, 2, 3]}
            4abc3113 LEAF [1, 2, 3]
                4bf5122f LEAF 1
        832e44f1 LEAF {"value": 42, "numbers": [1, 2, 3]}
            4abc3113 LEAF [1, 2, 3]
                dbc1b4c9 LEAF 2
        832e44f1 LEAF {"value": 42, "numbers": [1, 2, 3]}
            4abc3113 LEAF [1, 2, 3]
                084fed08 LEAF 3
    "#}
    .trim();

    assert_actual_expected!(
        actual,
        expected,
        "Complex structure should show proper path extension to all numbers"
    );
}

#[test]
fn test_array_element_access() {
    // Test with a simple array to understand path extension behavior
    let array_data = vec![1, 2, 3];
    let envelope = Envelope::new(array_data.clone());

    // Test our pattern
    let pattern = Pattern::parse("cbor(/search(number)/)").unwrap();
    let paths = pattern.paths(&envelope);

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
    "#}
    .trim();

    assert_actual_expected!(
        actual,
        expected,
        "Array search should return extended paths to each element"
    );
}
