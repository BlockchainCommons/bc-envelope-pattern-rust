mod common;

use bc_envelope::prelude::*;
use bc_envelope_pattern::{
    FormatPathsOpts, Matcher, Pattern, format_paths_with_captures,
};
use dcbor_pattern::Pattern as DcborPattern;
use indoc::indoc;

/// Test simple dcbor captures within CBOR patterns
#[test]
fn test_simple_dcbor_capture() {
    // Create envelope with number 42
    let envelope = Envelope::new(42);

    // Create CBOR pattern with dcbor capture: /@num(NUMBER(42))/
    let dcbor_pattern = DcborPattern::parse("@num(NUMBER(42))").unwrap();
    let pattern = Pattern::cbor_pattern(dcbor_pattern);

    // Execute pattern
    let (paths, captures) = pattern.paths_with_captures(&envelope);

    // Verify basic expectations
    assert_eq!(paths.len(), 1, "Should find exactly one path");
    assert_eq!(captures.len(), 1, "Should have exactly one capture group");
    assert!(captures.contains_key("num"), "Should have 'num' capture");
    assert_eq!(captures["num"].len(), 1, "Should capture one instance");

    // Verify formatted output follows rubric
    let actual = format_paths_with_captures(
        &paths,
        &captures,
        FormatPathsOpts::default(),
    );

    // Follow the rubric: run test to see actual output, then set expected
    #[rustfmt::skip]
    let expected = indoc! {r#"
        @num
            7f83f7bd LEAF 42
        7f83f7bd LEAF 42
    "#}.trim();

    assert_actual_expected!(
        actual,
        expected,
        "CBOR pattern should capture and format dcbor captures correctly"
    );
}

/// Test dcbor captures with search patterns
#[test]
fn test_dcbor_capture_with_search() {
    // Create envelope with array [1, 2, 3]
    let envelope = Envelope::new(vec![1, 2, 3]);

    // Create CBOR pattern with search: /@values(SEARCH(NUMBER))/
    let dcbor_pattern = DcborPattern::parse("@values(SEARCH(NUMBER))").unwrap();
    let pattern = Pattern::cbor_pattern(dcbor_pattern);

    // Execute pattern
    let (paths, captures) = pattern.paths_with_captures(&envelope);

    // Verify basic expectations
    assert_eq!(paths.len(), 3, "Should find exactly three paths");
    assert_eq!(captures.len(), 1, "Should have exactly one capture group");
    assert!(
        captures.contains_key("values"),
        "Should have 'values' capture"
    );
    assert_eq!(
        captures["values"].len(),
        9,
        "Should capture nine instances (3 numbers found 3 times each)"
    );

    // Verify formatted output follows rubric
    let actual = format_paths_with_captures(
        &paths,
        &captures,
        FormatPathsOpts::default(),
    );

    #[rustfmt::skip]
    let expected = indoc! {r#"
        @values
            4abc3113 LEAF [1, 2, 3]
                4bf5122f LEAF 1
            4abc3113 LEAF [1, 2, 3]
                dbc1b4c9 LEAF 2
            4abc3113 LEAF [1, 2, 3]
                084fed08 LEAF 3
            4abc3113 LEAF [1, 2, 3]
                4bf5122f LEAF 1
            4abc3113 LEAF [1, 2, 3]
                dbc1b4c9 LEAF 2
            4abc3113 LEAF [1, 2, 3]
                084fed08 LEAF 3
            4abc3113 LEAF [1, 2, 3]
                4bf5122f LEAF 1
            4abc3113 LEAF [1, 2, 3]
                dbc1b4c9 LEAF 2
            4abc3113 LEAF [1, 2, 3]
                084fed08 LEAF 3
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
        "CBOR search pattern should capture all number matches"
    );
}

/// Test multiple dcbor captures in single pattern
#[test]
fn test_multiple_dcbor_captures() {
    // Create envelope with simple mixed-type array for now
    // This simulates a map-like structure using alternating key-value pairs
    let envelope = Envelope::new(vec!["name", "Alice", "age", "30"]);

    // Create CBOR pattern with text search: /@names(SEARCH(TEXT))/
    let dcbor_pattern = DcborPattern::parse("@names(SEARCH(TEXT))").unwrap();
    let pattern = Pattern::cbor_pattern(dcbor_pattern);

    // Execute pattern
    let (paths, captures) = pattern.paths_with_captures(&envelope);

    // Verify basic expectations - SEARCH returns multiple paths (one per text
    // string)
    assert_eq!(
        paths.len(),
        4,
        "Should find exactly four paths (one per text string)"
    );
    assert_eq!(captures.len(), 1, "Should have exactly one capture group");
    assert!(
        captures.contains_key("names"),
        "Should have 'names' capture"
    );
    // Due to SEARCH behavior, we get more captures than unique texts (each text
    // found multiple times)
    assert_eq!(
        captures["names"].len(),
        16,
        "Should capture sixteen instances (each text found 4 times)"
    );

    // Verify formatted output follows rubric
    let actual = format_paths_with_captures(
        &paths,
        &captures,
        FormatPathsOpts::default(),
    );

    #[rustfmt::skip]
    let expected = indoc! {r#"
        @names
            ce1042d4 LEAF ["name", "Alice", "age", "30"]
                800a0588 LEAF "name"
            ce1042d4 LEAF ["name", "Alice", "age", "30"]
                13941b48 LEAF "Alice"
            ce1042d4 LEAF ["name", "Alice", "age", "30"]
                5943be12 LEAF "age"
            ce1042d4 LEAF ["name", "Alice", "age", "30"]
                08e52634 LEAF "30"
            ce1042d4 LEAF ["name", "Alice", "age", "30"]
                800a0588 LEAF "name"
            ce1042d4 LEAF ["name", "Alice", "age", "30"]
                13941b48 LEAF "Alice"
            ce1042d4 LEAF ["name", "Alice", "age", "30"]
                5943be12 LEAF "age"
            ce1042d4 LEAF ["name", "Alice", "age", "30"]
                08e52634 LEAF "30"
            ce1042d4 LEAF ["name", "Alice", "age", "30"]
                800a0588 LEAF "name"
            ce1042d4 LEAF ["name", "Alice", "age", "30"]
                13941b48 LEAF "Alice"
            ce1042d4 LEAF ["name", "Alice", "age", "30"]
                5943be12 LEAF "age"
            ce1042d4 LEAF ["name", "Alice", "age", "30"]
                08e52634 LEAF "30"
            ce1042d4 LEAF ["name", "Alice", "age", "30"]
                800a0588 LEAF "name"
            ce1042d4 LEAF ["name", "Alice", "age", "30"]
                13941b48 LEAF "Alice"
            ce1042d4 LEAF ["name", "Alice", "age", "30"]
                5943be12 LEAF "age"
            ce1042d4 LEAF ["name", "Alice", "age", "30"]
                08e52634 LEAF "30"
        ce1042d4 LEAF ["name", "Alice", "age", "30"]
            800a0588 LEAF "name"
        ce1042d4 LEAF ["name", "Alice", "age", "30"]
            13941b48 LEAF "Alice"
        ce1042d4 LEAF ["name", "Alice", "age", "30"]
            5943be12 LEAF "age"
        ce1042d4 LEAF ["name", "Alice", "age", "30"]
            08e52634 LEAF "30"
    "#}
    .trim();

    assert_actual_expected!(
        actual,
        expected,
        "CBOR pattern should capture all text values found"
    );
}

/// Test nested dcbor captures in complex structures
#[test]
fn test_nested_dcbor_captures() {
    // Create envelope with nested array structure: [["Alice", "95"], ["Bob",
    // "85"]]
    let envelope = Envelope::new(vec![vec!["Alice", "95"], vec!["Bob", "85"]]);

    // Create CBOR pattern with nested captures:
    // /@users(SEARCH(ARRAY(@name(TEXT) > @score(TEXT))))/
    let dcbor_pattern = DcborPattern::parse(
        "@users(SEARCH(ARRAY(@name(TEXT) > @score(TEXT))))",
    )
    .unwrap();
    let pattern = Pattern::cbor_pattern(dcbor_pattern);

    // Execute pattern
    let (paths, captures) = pattern.paths_with_captures(&envelope);

    // Verify basic expectations - SEARCH returns multiple paths
    assert_eq!(
        paths.len(),
        2,
        "Should find exactly two paths (one per nested array)"
    );
    assert_eq!(
        captures.len(),
        3,
        "Should have exactly three capture groups"
    );
    assert!(
        captures.contains_key("users"),
        "Should have 'users' capture"
    );
    assert!(captures.contains_key("name"), "Should have 'name' capture");
    assert!(
        captures.contains_key("score"),
        "Should have 'score' capture"
    );
    assert_eq!(
        captures["users"].len(),
        4,
        "Should capture four user instances (due to SEARCH behavior)"
    );
    assert_eq!(
        captures["name"].len(),
        4,
        "Should capture four name instances (due to SEARCH behavior)"
    );
    assert_eq!(
        captures["score"].len(),
        4,
        "Should capture four score instances (due to SEARCH behavior)"
    );
    // Verify formatted output follows rubric
    let actual = format_paths_with_captures(
        &paths,
        &captures,
        FormatPathsOpts::default(),
    );

    #[rustfmt::skip]
    let expected = indoc! {r#"
        @name
            7dfc2858 LEAF [["Alice", "95"], ["Bob", "85"]]
                6daf5539 LEAF ["Alice", "95"]
            7dfc2858 LEAF [["Alice", "95"], ["Bob", "85"]]
                43a6ef66 LEAF ["Bob", "85"]
            7dfc2858 LEAF [["Alice", "95"], ["Bob", "85"]]
                6daf5539 LEAF ["Alice", "95"]
            7dfc2858 LEAF [["Alice", "95"], ["Bob", "85"]]
                43a6ef66 LEAF ["Bob", "85"]
        @score
            7dfc2858 LEAF [["Alice", "95"], ["Bob", "85"]]
                6daf5539 LEAF ["Alice", "95"]
            7dfc2858 LEAF [["Alice", "95"], ["Bob", "85"]]
                43a6ef66 LEAF ["Bob", "85"]
            7dfc2858 LEAF [["Alice", "95"], ["Bob", "85"]]
                6daf5539 LEAF ["Alice", "95"]
            7dfc2858 LEAF [["Alice", "95"], ["Bob", "85"]]
                43a6ef66 LEAF ["Bob", "85"]
        @users
            7dfc2858 LEAF [["Alice", "95"], ["Bob", "85"]]
                6daf5539 LEAF ["Alice", "95"]
            7dfc2858 LEAF [["Alice", "95"], ["Bob", "85"]]
                43a6ef66 LEAF ["Bob", "85"]
            7dfc2858 LEAF [["Alice", "95"], ["Bob", "85"]]
                6daf5539 LEAF ["Alice", "95"]
            7dfc2858 LEAF [["Alice", "95"], ["Bob", "85"]]
                43a6ef66 LEAF ["Bob", "85"]
        7dfc2858 LEAF [["Alice", "95"], ["Bob", "85"]]
            6daf5539 LEAF ["Alice", "95"]
        7dfc2858 LEAF [["Alice", "95"], ["Bob", "85"]]
            43a6ef66 LEAF ["Bob", "85"]
    "#}
    .trim();

    assert_actual_expected!(
        actual,
        expected,
        "CBOR pattern should handle nested captures correctly"
    );
}

/// Test integration of envelope-level and dcbor-level captures
#[test]
fn test_mixed_envelope_and_dcbor_captures() {
    // Create envelope with number 42
    let envelope = Envelope::new(42);

    // Create pattern with envelope capture wrapping CBOR pattern with dcbor
    // capture @envelope_level(CBOR(/@dcbor_level(NUMBER(42))/))
    let dcbor_pattern =
        DcborPattern::parse("@dcbor_level(NUMBER(42))").unwrap();
    let cbor_pattern = Pattern::cbor_pattern(dcbor_pattern);
    let pattern = Pattern::capture("envelope_level", cbor_pattern);

    // Execute pattern
    let (paths, captures) = pattern.paths_with_captures(&envelope);

    // Verify basic expectations
    assert_eq!(paths.len(), 1, "Should find exactly one path");
    assert_eq!(captures.len(), 2, "Should have exactly two capture groups");
    assert!(
        captures.contains_key("envelope_level"),
        "Should have 'envelope_level' capture"
    );
    assert!(
        captures.contains_key("dcbor_level"),
        "Should have 'dcbor_level' capture"
    );
    assert_eq!(
        captures["envelope_level"].len(),
        1,
        "Should capture one envelope instance"
    );
    assert_eq!(
        captures["dcbor_level"].len(),
        1,
        "Should capture one dcbor instance"
    );

    // Verify formatted output follows rubric
    let actual = format_paths_with_captures(
        &paths,
        &captures,
        FormatPathsOpts::default(),
    );

    #[rustfmt::skip]
    let expected = indoc! {r#"
        @dcbor_level
            7f83f7bd LEAF 42
        @envelope_level
            7f83f7bd LEAF 42
        7f83f7bd LEAF 42
    "#}
    .trim();

    assert_actual_expected!(
        actual,
        expected,
        "Pattern should capture at both envelope and dcbor levels"
    );
}

/// Test capture name uniqueness and conflict detection
#[test]
fn test_capture_name_conflicts() {
    // Create envelope with number 42
    let envelope = Envelope::new(42);

    // Create pattern with same capture name at envelope and dcbor levels
    // @same_name(CBOR(/@same_name(NUMBER(42))/))
    let dcbor_pattern = DcborPattern::parse("@same_name(NUMBER(42))").unwrap();
    let cbor_pattern = Pattern::cbor_pattern(dcbor_pattern);
    let pattern = Pattern::capture("same_name", cbor_pattern);

    // Execute pattern
    let (paths, captures) = pattern.paths_with_captures(&envelope);

    // Verify basic expectations
    assert_eq!(paths.len(), 1, "Should find exactly one path");
    assert_eq!(captures.len(), 1, "Should have exactly one capture group");
    assert!(
        captures.contains_key("same_name"),
        "Should have 'same_name' capture"
    );
    assert_eq!(
        captures["same_name"].len(),
        2,
        "Should capture from both levels"
    );

    // Verify formatted output follows rubric
    let actual = format_paths_with_captures(
        &paths,
        &captures,
        FormatPathsOpts::default(),
    );

    #[rustfmt::skip]
    let expected = indoc! {r#"
        @same_name
            7f83f7bd LEAF 42
            7f83f7bd LEAF 42
        7f83f7bd LEAF 42
    "#}
    .trim();

    assert_actual_expected!(
        actual,
        expected,
        "Pattern should merge captures with same name from different levels"
    );
}

/// Test CBOR captures with complex array traversal
#[test]
fn test_array_traversal_captures() {
    // Create envelope with mixed array ["hello", "42", "world", "123"]
    let envelope = Envelope::new(vec!["hello", "42", "world", "123"]); // All strings for simplicity

    // Create CBOR pattern with search captures: /@text(SEARCH(TEXT))/
    let dcbor_pattern = DcborPattern::parse("@text(SEARCH(TEXT))").unwrap();
    let pattern = Pattern::cbor_pattern(dcbor_pattern);

    // Execute pattern
    let (paths, captures) = pattern.paths_with_captures(&envelope);

    // Verify basic expectations - SEARCH returns multiple paths (one per text
    // element)
    assert_eq!(
        paths.len(),
        4,
        "Should find exactly four paths (one per text element)"
    );
    assert_eq!(captures.len(), 1, "Should have exactly one capture group");
    assert!(captures.contains_key("text"), "Should have 'text' capture");
    assert_eq!(
        captures["text"].len(),
        16,
        "Should capture sixteen instances (each text found 4 times)"
    );

    // Verify formatted output follows rubric
    let actual = format_paths_with_captures(
        &paths,
        &captures,
        FormatPathsOpts::default(),
    );

    #[rustfmt::skip]
    let expected = indoc! {r#"
        @text
            162867a4 LEAF ["hello", "42", "world", "123"]
                cb835593 LEAF "hello"
            162867a4 LEAF ["hello", "42", "world", "123"]
                9fa6eb00 LEAF "42"
            162867a4 LEAF ["hello", "42", "world", "123"]
                29651e19 LEAF "world"
            162867a4 LEAF ["hello", "42", "world", "123"]
                9bf5bb3e LEAF "123"
            162867a4 LEAF ["hello", "42", "world", "123"]
                cb835593 LEAF "hello"
            162867a4 LEAF ["hello", "42", "world", "123"]
                9fa6eb00 LEAF "42"
            162867a4 LEAF ["hello", "42", "world", "123"]
                29651e19 LEAF "world"
            162867a4 LEAF ["hello", "42", "world", "123"]
                9bf5bb3e LEAF "123"
            162867a4 LEAF ["hello", "42", "world", "123"]
                cb835593 LEAF "hello"
            162867a4 LEAF ["hello", "42", "world", "123"]
                9fa6eb00 LEAF "42"
            162867a4 LEAF ["hello", "42", "world", "123"]
                29651e19 LEAF "world"
            162867a4 LEAF ["hello", "42", "world", "123"]
                9bf5bb3e LEAF "123"
            162867a4 LEAF ["hello", "42", "world", "123"]
                cb835593 LEAF "hello"
            162867a4 LEAF ["hello", "42", "world", "123"]
                9fa6eb00 LEAF "42"
            162867a4 LEAF ["hello", "42", "world", "123"]
                29651e19 LEAF "world"
            162867a4 LEAF ["hello", "42", "world", "123"]
                9bf5bb3e LEAF "123"
        162867a4 LEAF ["hello", "42", "world", "123"]
            cb835593 LEAF "hello"
        162867a4 LEAF ["hello", "42", "world", "123"]
            9fa6eb00 LEAF "42"
        162867a4 LEAF ["hello", "42", "world", "123"]
            29651e19 LEAF "world"
        162867a4 LEAF ["hello", "42", "world", "123"]
            9bf5bb3e LEAF "123"
    "#}
    .trim();

    assert_actual_expected!(
        actual,
        expected,
        "CBOR pattern should capture all text elements via SEARCH"
    );
}

/// Test CBOR captures with no matches
#[test]
fn test_cbor_captures_no_match() {
    // Create envelope with text "hello"
    let envelope = Envelope::new("hello");

    // Create CBOR pattern that won't match: /@num(NUMBER)/
    let dcbor_pattern = DcborPattern::parse("@num(NUMBER)").unwrap();
    let pattern = Pattern::cbor_pattern(dcbor_pattern);

    // Execute pattern
    let (paths, captures) = pattern.paths_with_captures(&envelope);

    // Verify basic expectations
    assert_eq!(paths.len(), 0, "Should find no paths");
    assert_eq!(captures.len(), 0, "Should have no captures");

    // Verify formatted output follows rubric
    let actual = format_paths_with_captures(
        &paths,
        &captures,
        FormatPathsOpts::default(),
    );

    #[rustfmt::skip]
    let expected = "";

    assert_actual_expected!(
        actual,
        expected,
        "CBOR pattern with no matches should return empty formatted output"
    );
}

/// Test CBOR pattern performance with many captures
#[test]
fn test_cbor_captures_performance() {
    // Create envelope with smaller array to avoid excessive repetition
    let numbers: Vec<i32> = (1..=3).collect(); // Use just 3 numbers for clarity
    let envelope = Envelope::new(numbers);

    // Create CBOR pattern that captures all numbers: /@nums(SEARCH(NUMBER))/
    let dcbor_pattern = DcborPattern::parse("@nums(SEARCH(NUMBER))").unwrap();
    let pattern = Pattern::cbor_pattern(dcbor_pattern);

    // Execute pattern with timing
    let start = std::time::Instant::now();
    let (paths, captures) = pattern.paths_with_captures(&envelope);
    let duration = start.elapsed();

    // Verify basic expectations - SEARCH pattern returns multiple paths (one
    // per number)
    assert_eq!(
        paths.len(),
        3,
        "Should find exactly three paths (one per number)"
    );
    assert_eq!(captures.len(), 1, "Should have exactly one capture group");
    assert!(captures.contains_key("nums"), "Should have 'nums' capture");
    // Due to SEARCH behavior, we get more captures than unique numbers
    assert_eq!(
        captures["nums"].len(),
        9,
        "Should capture nine instances (each number found 3 times)"
    );

    // Verify formatted output follows rubric
    let actual = format_paths_with_captures(
        &paths,
        &captures,
        FormatPathsOpts::default(),
    );

    #[rustfmt::skip]
    let expected = indoc! {r#"
        @nums
            4abc3113 LEAF [1, 2, 3]
                4bf5122f LEAF 1
            4abc3113 LEAF [1, 2, 3]
                dbc1b4c9 LEAF 2
            4abc3113 LEAF [1, 2, 3]
                084fed08 LEAF 3
            4abc3113 LEAF [1, 2, 3]
                4bf5122f LEAF 1
            4abc3113 LEAF [1, 2, 3]
                dbc1b4c9 LEAF 2
            4abc3113 LEAF [1, 2, 3]
                084fed08 LEAF 3
            4abc3113 LEAF [1, 2, 3]
                4bf5122f LEAF 1
            4abc3113 LEAF [1, 2, 3]
                dbc1b4c9 LEAF 2
            4abc3113 LEAF [1, 2, 3]
                084fed08 LEAF 3
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
        "CBOR pattern should capture all numbers with SEARCH behavior"
    );

    println!(
        "✅ Performance test completed in {}ms",
        duration.as_millis()
    );
}

/// Test comprehensive integration with all CBOR capture features
#[test]
fn test_comprehensive_cbor_captures() {
    // Create simple but comprehensive envelope structure: ["Alice", "Bob",
    // "Charlie"]
    let envelope = Envelope::new(vec!["Alice", "Bob", "Charlie"]);

    // Create comprehensive CBOR pattern with search captures
    let dcbor_pattern = DcborPattern::parse("@people(SEARCH(TEXT))").unwrap();
    let cbor_pattern = Pattern::cbor_pattern(dcbor_pattern);
    let pattern = Pattern::capture("data", cbor_pattern);

    // Execute pattern
    let (paths, captures) = pattern.paths_with_captures(&envelope);

    // Verify basic expectations - SEARCH returns multiple paths (one per
    // person)
    assert_eq!(
        paths.len(),
        3,
        "Should find exactly three paths (one per person)"
    );
    assert_eq!(captures.len(), 2, "Should have exactly two capture groups");
    assert!(captures.contains_key("data"), "Should have 'data' capture");
    assert!(
        captures.contains_key("people"),
        "Should have 'people' capture"
    );
    assert_eq!(
        captures["data"].len(),
        3,
        "Should capture three data instances (one per path)"
    );
    assert_eq!(
        captures["people"].len(),
        9,
        "Should capture nine people instances (each person found 3 times)"
    );

    // Verify formatted output follows rubric
    let actual = format_paths_with_captures(
        &paths,
        &captures,
        FormatPathsOpts::default(),
    );

    #[rustfmt::skip]
    let expected = indoc! {r#"
        @data
            aea55aad LEAF ["Alice", "Bob", "Charlie"]
                13941b48 LEAF "Alice"
            aea55aad LEAF ["Alice", "Bob", "Charlie"]
                13b74194 LEAF "Bob"
            aea55aad LEAF ["Alice", "Bob", "Charlie"]
                ee8e3b02 LEAF "Charlie"
        @people
            aea55aad LEAF ["Alice", "Bob", "Charlie"]
                13941b48 LEAF "Alice"
            aea55aad LEAF ["Alice", "Bob", "Charlie"]
                13b74194 LEAF "Bob"
            aea55aad LEAF ["Alice", "Bob", "Charlie"]
                ee8e3b02 LEAF "Charlie"
            aea55aad LEAF ["Alice", "Bob", "Charlie"]
                13941b48 LEAF "Alice"
            aea55aad LEAF ["Alice", "Bob", "Charlie"]
                13b74194 LEAF "Bob"
            aea55aad LEAF ["Alice", "Bob", "Charlie"]
                ee8e3b02 LEAF "Charlie"
            aea55aad LEAF ["Alice", "Bob", "Charlie"]
                13941b48 LEAF "Alice"
            aea55aad LEAF ["Alice", "Bob", "Charlie"]
                13b74194 LEAF "Bob"
            aea55aad LEAF ["Alice", "Bob", "Charlie"]
                ee8e3b02 LEAF "Charlie"
        aea55aad LEAF ["Alice", "Bob", "Charlie"]
            13941b48 LEAF "Alice"
        aea55aad LEAF ["Alice", "Bob", "Charlie"]
            13b74194 LEAF "Bob"
        aea55aad LEAF ["Alice", "Bob", "Charlie"]
            ee8e3b02 LEAF "Charlie"
    "#}
    .trim();

    assert_actual_expected!(
        actual,
        expected,
        "Comprehensive CBOR pattern should capture via SEARCH at multiple levels"
    );
}
