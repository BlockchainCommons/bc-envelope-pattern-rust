mod common;

use bc_envelope::prelude::*;
use bc_envelope_pattern::{Matcher, Pattern, Reluctance, format_paths};
use indoc::indoc;

#[test]
fn test_empty_traversal_pattern() {
    let envelope = Envelope::new(42);

    // An empty traversal pattern never matches.
    let pattern = Pattern::traverse(vec![]);
    let paths = pattern.paths(&envelope);
    assert!(paths.is_empty());
}

#[test]
fn test_and_pattern() {
    let envelope = Envelope::new(42).add_assertion("an", "assertion");

    // A pattern that requires the envelope to match both a number and a text,
    // which is impossible.
    let impossible_pattern =
        Pattern::and(vec![Pattern::number(42), Pattern::text("foo")]);
    assert!(!impossible_pattern.matches(&envelope));
    assert_eq!(
        format!("{}", impossible_pattern),
        r#"42&"foo""#
    );

    // A pattern that requires the envelope to match both a number greater
    // than 40 and a number less than 50, which is possible.
    let number_range_pattern = Pattern::and(vec![
        Pattern::number_greater_than(40),
        Pattern::number_less_than(50),
    ]);
    assert!(number_range_pattern.matches(&envelope));
    assert_eq!(
        format!("{}", number_range_pattern),
        r#"NUMBER(>40)&NUMBER(<50)"#
    );

    // The path includes the assertion.
    let paths = number_range_pattern.paths(&envelope);
    #[rustfmt::skip]
    let expected = indoc! {r#"
        6cb2ea4a NODE 42 [ "an": "assertion" ]
    "#}.trim();
    assert_actual_expected!(format_paths(&paths), expected);

    // A traversal pattern that includes the number range pattern and then
    // extracts the subject.
    let number_range_with_subject_pattern =
        Pattern::traverse(vec![number_range_pattern, Pattern::any_subject()]);
    let paths = number_range_with_subject_pattern.paths(&envelope);
    #[rustfmt::skip]
    let expected = indoc! {r#"
        6cb2ea4a NODE 42 [ "an": "assertion" ]
            7f83f7bd LEAF 42
    "#}.trim();
    assert_actual_expected!(format_paths(&paths), expected);
    assert_eq!(
        format!("{}", number_range_with_subject_pattern),
        r#"NUMBER(>40)&NUMBER(<50)->SUBJECT"#,
    );
}

#[test]
fn test_or_pattern() {
    // A pattern that requires the envelope to match either the string "foo" or
    // the string "bar".
    let pattern = Pattern::or(vec![Pattern::text("bar"), Pattern::text("baz")]);
    assert_eq!(format!("{}", pattern), r#""bar"|"baz""#);

    // An envelope that is a number, so it doesn't match the pattern.
    let envelope = Envelope::new(42).add_assertion("an", "assertion");
    assert!(!pattern.matches(&envelope));

    // A pattern that requires the envelope to match either the string "foo" or
    // a number greater than 40.
    let foo_or_greater_than_40_pattern = Pattern::or(vec![
        Pattern::text("foo"),
        Pattern::number_greater_than(40),
    ]);
    // The subject doesn't match the first pattern but matches the second.
    assert!(foo_or_greater_than_40_pattern.matches(&envelope));
    assert_eq!(
        format!("{}", foo_or_greater_than_40_pattern),
        r#""foo"|NUMBER(>40)"#
    );

    // The match path includes the assertion.
    let paths = foo_or_greater_than_40_pattern.paths(&envelope);
    #[rustfmt::skip]
    let expected = indoc! {r#"
        6cb2ea4a NODE 42 [ "an": "assertion" ]
    "#}.trim();
    assert_actual_expected!(format_paths(&paths), expected);

    let foo_or_greater_than_40_with_subject_pattern = Pattern::traverse(vec![
        foo_or_greater_than_40_pattern,
        Pattern::any_subject(),
    ]);
    let paths = foo_or_greater_than_40_with_subject_pattern.paths(&envelope);
    #[rustfmt::skip]
    let expected = indoc! {r#"
        6cb2ea4a NODE 42 [ "an": "assertion" ]
            7f83f7bd LEAF 42
    "#}.trim();
    assert_actual_expected!(format_paths(&paths), expected);
    assert_eq!(
        format!("{}", foo_or_greater_than_40_with_subject_pattern),
        r#""foo"|NUMBER(>40)->SUBJECT"#
    );
}

#[test]
fn test_one_element_traversal_pattern() {
    // A pattern that matches a the number 42.
    let number_pattern = Pattern::number(42);
    assert_eq!(format!("{}", number_pattern), r#"NUMBER(42)"#);

    let envelope = Envelope::new(42);
    #[rustfmt::skip]
    let expected = indoc! {r#"
        7f83f7bd LEAF 42
    "#}.trim();
    let paths = number_pattern.paths(&envelope);
    assert_actual_expected!(format_paths(&paths), expected);

    // A traversal of one pattern gives the same result as the single pattern.
    let pattern = Pattern::traverse(vec![number_pattern]);
    let paths = pattern.paths(&envelope);
    assert_actual_expected!(format_paths(&paths), expected);
    assert_eq!(format!("{}", pattern), r#"NUMBER(42)"#);
}

#[test]
fn test_wrapped_traversal() {
    let env_1 = Envelope::new("data");
    let wrapped_1 = env_1.wrap();
    let wrapped_2 = wrapped_1.wrap();
    let wrapped_3 = wrapped_2.wrap();
    let wrapped_4 = wrapped_3.wrap();

    // println!("{}", wrapped_4.tree_format());
    #[rustfmt::skip]
    let expected = indoc! {r#"
        25cb582c WRAPPED
            c1426a18 cont WRAPPED
                ee8cade0 cont WRAPPED
                    febc1555 cont WRAPPED
                        e909da9a cont "data"
    "#}.trim();
    assert_actual_expected!(wrapped_4.tree_format(), expected);

    // println!("{}", wrapped_4.format_flat());
    #[rustfmt::skip]
    let expected = indoc! {r#"
        { { { { "data" } } } }
    "#}.trim();
    assert_actual_expected!(wrapped_4.format_flat(), expected);

    // A pattern that matches the contents of a single wrapped envelope.
    let wrapped_1_pattern =
        Pattern::traverse(vec![Pattern::wrapped(), Pattern::unwrap()]);
    let paths = wrapped_1_pattern.paths(&wrapped_4);
    // println!("{}", format_paths(&paths));
    #[rustfmt::skip]
    let expected = indoc! {r#"
        25cb582c WRAPPED { { { { "data" } } } }
            c1426a18 WRAPPED { { { "data" } } }
    "#}
    .trim();
    assert_actual_expected!(format_paths(&paths), expected);

    // A pattern that matches two wrapped envelopes in a traversal path.
    let wrapped_2_pattern =
        Pattern::traverse(vec![Pattern::unwrap(), Pattern::unwrap()]);
    let paths = wrapped_2_pattern.paths(&wrapped_4);
    // println!("{}", format_paths(&paths));
    #[rustfmt::skip]
    let expected = indoc! {r#"
        25cb582c WRAPPED { { { { "data" } } } }
            c1426a18 WRAPPED { { { "data" } } }
                ee8cade0 WRAPPED { { "data" } }
    "#}.trim();
    assert_actual_expected!(format_paths(&paths), expected);

    // A pattern that matches three wrapped envelopes in a traversal path.
    let wrapped_3_pattern = Pattern::traverse(vec![
        Pattern::unwrap(),
        Pattern::unwrap(),
        Pattern::unwrap(),
    ]);
    let paths = wrapped_3_pattern.paths(&wrapped_4);
    // println!("{}", format_paths(&paths));
    #[rustfmt::skip]
    let expected = indoc! {r#"
        25cb582c WRAPPED { { { { "data" } } } }
            c1426a18 WRAPPED { { { "data" } } }
                ee8cade0 WRAPPED { { "data" } }
                    febc1555 WRAPPED { "data" }
    "#}.trim();
    assert_actual_expected!(format_paths(&paths), expected);

    // A pattern that matches four wrapped envelopes in a traversal path.
    let wrapped_4_pattern = Pattern::traverse(vec![
        Pattern::unwrap(),
        Pattern::unwrap(),
        Pattern::unwrap(),
        Pattern::unwrap(),
    ]);
    let paths = wrapped_4_pattern.paths(&wrapped_4);
    // println!("{}", format_paths(&paths));
    #[rustfmt::skip]
    let expected = indoc! {r#"
        25cb582c WRAPPED { { { { "data" } } } }
            c1426a18 WRAPPED { { { "data" } } }
                ee8cade0 WRAPPED { { "data" } }
                    febc1555 WRAPPED { "data" }
                        e909da9a LEAF "data"
    "#}.trim();
    assert_actual_expected!(format_paths(&paths), expected);

    assert_eq!(
        format!("{}", wrapped_4_pattern),
        r#"UNWRAP->UNWRAP->UNWRAP->UNWRAP"#
    );
}

#[test]
fn optional_wrapped_pattern() {
    // A pattern that matches an envelope that may or may not be wrapped.
    let optional_wrapped_pattern = Pattern::traverse(vec![
        Pattern::repeat(Pattern::unwrap(), 0..=1, Reluctance::Greedy),
        Pattern::any_number(),
    ]);
    assert_eq!(
        format!("{}", optional_wrapped_pattern),
        r#"(UNWRAP)?->NUMBER"#
    );

    let inner = Envelope::new(42);
    let wrapped = inner.wrap();

    let inner_paths = optional_wrapped_pattern.paths(&inner);
    #[rustfmt::skip]
    let expected  = indoc! {r#"
        7f83f7bd LEAF 42
    "#}.trim();
    assert_actual_expected!(format_paths(&inner_paths), expected);

    let wrapped_paths = optional_wrapped_pattern.paths(&wrapped);
    #[rustfmt::skip]
    let expected = indoc! {r#"
        58b1ac6a WRAPPED { 42 }
            7f83f7bd LEAF 42
    "#}.trim();
    assert_actual_expected!(format_paths(&wrapped_paths), expected);
}

#[test]
fn test_search_pattern() {
    // A pattern that searches for any text in the envelope
    let text_search_pattern = Pattern::search(Pattern::any_text());
    assert_eq!(format!("{}", text_search_pattern), r#"SEARCH(text)"#);

    // Test searching for text in a simple envelope
    let envelope = Envelope::new("Alice")
        .add_assertion("knows", "Bob")
        .add_assertion("age", 30);

    let text_search_paths = text_search_pattern.paths(&envelope);
    #[rustfmt::skip]
    let expected = indoc! {r#"
        a47bb3d4 NODE "Alice" [ "age": 30, "knows": "Bob" ]
        a47bb3d4 NODE "Alice" [ "age": 30, "knows": "Bob" ]
            13941b48 LEAF "Alice"
        a47bb3d4 NODE "Alice" [ "age": 30, "knows": "Bob" ]
            0eb5609b ASSERTION "age": 30
                5943be12 LEAF "age"
        a47bb3d4 NODE "Alice" [ "age": 30, "knows": "Bob" ]
            78d666eb ASSERTION "knows": "Bob"
                db7dd21c LEAF "knows"
        a47bb3d4 NODE "Alice" [ "age": 30, "knows": "Bob" ]
            78d666eb ASSERTION "knows": "Bob"
                13b74194 LEAF "Bob"
    "#}.trim();
    assert_actual_expected!(format_paths(&text_search_paths), expected);

    // A pattern that searches for the text "Bob" in the envelope
    let bob_search_pattern = Pattern::search(Pattern::text("Bob"));
    assert_eq!(format!("{}", bob_search_pattern), r#"SEARCH("Bob")"#);
    let bob_search_paths = bob_search_pattern.paths(&envelope);
    #[rustfmt::skip]
    let expected = indoc! {r#"
        a47bb3d4 NODE "Alice" [ "age": 30, "knows": "Bob" ]
            78d666eb ASSERTION "knows": "Bob"
                13b74194 LEAF "Bob"
    "#}.trim();
    assert_actual_expected!(format_paths(&bob_search_paths), expected);

    // A pattern that searches for any number in the envelope
    let number_search_pattern = Pattern::search(Pattern::any_number());
    assert_eq!(format!("{}", number_search_pattern), r#"SEARCH(NUMBER)"#);
    let number_search_paths = number_search_pattern.paths(&envelope);
    #[rustfmt::skip]
    let expected = indoc! {r#"
        a47bb3d4 NODE "Alice" [ "age": 30, "knows": "Bob" ]
            0eb5609b ASSERTION "age": 30
                cf972730 LEAF 30
    "#}.trim();
    assert_actual_expected!(format_paths(&number_search_paths), expected);

    // A pattern that searches for any assertion with an object
    // that is a number
    let number_object_search_pattern =
        Pattern::search(Pattern::assertion_with_object(Pattern::any_number()));
    assert_eq!(
        format!("{}", number_object_search_pattern),
        r#"SEARCH(ASSERTOBJ(NUMBER))"#
    );
    let number_object_search_paths =
        number_object_search_pattern.paths(&envelope);
    #[rustfmt::skip]
    let expected = indoc! {r#"
        a47bb3d4 NODE "Alice" [ "age": 30, "knows": "Bob" ]
            0eb5609b ASSERTION "age": 30
    "#}.trim();
    assert_actual_expected!(
        format_paths(&number_object_search_paths),
        expected
    );
}

#[test]
fn test_search_pattern_nested() {
    // A pattern that searches for any text in the envelope
    let text_search_pattern = Pattern::search(Pattern::any_text());
    assert_eq!(format!("{}", text_search_pattern), r#"SEARCH(text)"#);

    // Test searching in a more complex nested envelope
    let inner_envelope =
        Envelope::new("Carol").add_assertion("title", "Engineer");

    let envelope = Envelope::new("Alice")
        .add_assertion("knows", inner_envelope)
        .add_assertion("department", "Engineering");

    // Search for all text should find text at all levels
    let text_search_paths = text_search_pattern.paths(&envelope);

    assert_eq!(text_search_paths.len(), 9);
    #[rustfmt::skip]
    let expected = indoc! {r#"
        a69103e9 NODE "Alice" [ "department": "Engineering", "knows": "Carol" [ "title": "Engineer" ] ]
        a69103e9 NODE "Alice" [ "department": "Engineering", "knows": "Carol" [ "title": "Engineer" ] ]
            13941b48 LEAF "Alice"
        a69103e9 NODE "Alice" [ "department": "Engineering", "knows": "Carol" [ "title": "Engineer" ] ]
            2a26d42a ASSERTION "department": "Engineering"
                8aaec3ab LEAF "department"
        a69103e9 NODE "Alice" [ "department": "Engineering", "knows": "Carol" [ "title": "Engineer" ] ]
            2a26d42a ASSERTION "department": "Engineering"
                71d7c10e LEAF "Engineering"
        a69103e9 NODE "Alice" [ "department": "Engineering", "knows": "Carol" [ "title": "Engineer" ] ]
            c0c35c79 ASSERTION "knows": "Carol" [ "title": "Engineer" ]
                db7dd21c LEAF "knows"
        a69103e9 NODE "Alice" [ "department": "Engineering", "knows": "Carol" [ "title": "Engineer" ] ]
            c0c35c79 ASSERTION "knows": "Carol" [ "title": "Engineer" ]
                59e8c540 NODE "Carol" [ "title": "Engineer" ]
        a69103e9 NODE "Alice" [ "department": "Engineering", "knows": "Carol" [ "title": "Engineer" ] ]
            c0c35c79 ASSERTION "knows": "Carol" [ "title": "Engineer" ]
                59e8c540 NODE "Carol" [ "title": "Engineer" ]
                    afb8122e LEAF "Carol"
        a69103e9 NODE "Alice" [ "department": "Engineering", "knows": "Carol" [ "title": "Engineer" ] ]
            c0c35c79 ASSERTION "knows": "Carol" [ "title": "Engineer" ]
                59e8c540 NODE "Carol" [ "title": "Engineer" ]
                    a4d32c8f ASSERTION "title": "Engineer"
                        d380cf3f LEAF "title"
        a69103e9 NODE "Alice" [ "department": "Engineering", "knows": "Carol" [ "title": "Engineer" ] ]
            c0c35c79 ASSERTION "knows": "Carol" [ "title": "Engineer" ]
                59e8c540 NODE "Carol" [ "title": "Engineer" ]
                    a4d32c8f ASSERTION "title": "Engineer"
                        df9ac43f LEAF "Engineer"
    "#}.trim();
    assert_actual_expected!(format_paths(&text_search_paths), expected);

    // Verify we can find "Carol" nested inside
    let carol_paths: Vec<_> = text_search_paths
        .iter()
        .filter(|path| path.last().unwrap().format_flat().contains("Carol"))
        .collect();
    assert_eq!(carol_paths.len(), 3); // Root envelope, Carol envelope, and Carol subject

    // The path to "Carol" subject should be: envelope -> knows assertion ->
    // Carol envelope -> "Carol"
    let carol_subject_path = carol_paths
        .iter()
        .find(|path| path.last().unwrap().format_flat() == r#""Carol""#)
        .unwrap();
    assert_eq!(carol_subject_path.len(), 4);
}

#[test]
fn test_search_pattern_with_wrapped() {
    // A pattern that searches for the text "secret" in the envelope
    let secret_text_search_pattern = Pattern::search(Pattern::text("secret"));
    assert_eq!(
        format!("{}", secret_text_search_pattern),
        r#"SEARCH("secret")"#
    );

    let inner =
        Envelope::new("secret").add_assertion("classification", "top-secret");
    let envelope = Envelope::new("Alice").add_assertion("data", inner.wrap());

    let secret_text_search_paths = secret_text_search_pattern.paths(&envelope);
    // println!("{}", format_paths(&paths));
    #[rustfmt::skip]
    let expected = indoc! {r#"
        1435493d NODE "Alice" [ "data": { "secret" [ "classification": "top-secret" ] } ]
            a5d4710e ASSERTION "data": { "secret" [ "classification": "top-secret" ] }
                41dca0cd WRAPPED { "secret" [ "classification": "top-secret" ] }
                    f66baec9 NODE "secret" [ "classification": "top-secret" ]
        1435493d NODE "Alice" [ "data": { "secret" [ "classification": "top-secret" ] } ]
            a5d4710e ASSERTION "data": { "secret" [ "classification": "top-secret" ] }
                41dca0cd WRAPPED { "secret" [ "classification": "top-secret" ] }
                    f66baec9 NODE "secret" [ "classification": "top-secret" ]
                        fa445f41 LEAF "secret"
    "#}.trim();
    assert_actual_expected!(format_paths(&secret_text_search_paths), expected);

    // A pattern that searches for any text containing the word "secret"
    let secret_regex_search_pattern = Pattern::search(Pattern::text_regex(
        regex::Regex::new("secret").unwrap(),
    ));
    assert_eq!(
        format!("{}", secret_regex_search_pattern),
        r#"SEARCH(/secret/)"#
    );
    let secret_regex_search_paths =
        secret_regex_search_pattern.paths(&envelope);
    #[rustfmt::skip]
    let expected = indoc! {r#"
        1435493d NODE "Alice" [ "data": { "secret" [ "classification": "top-secret" ] } ]
            a5d4710e ASSERTION "data": { "secret" [ "classification": "top-secret" ] }
                41dca0cd WRAPPED { "secret" [ "classification": "top-secret" ] }
                    f66baec9 NODE "secret" [ "classification": "top-secret" ]
        1435493d NODE "Alice" [ "data": { "secret" [ "classification": "top-secret" ] } ]
            a5d4710e ASSERTION "data": { "secret" [ "classification": "top-secret" ] }
                41dca0cd WRAPPED { "secret" [ "classification": "top-secret" ] }
                    f66baec9 NODE "secret" [ "classification": "top-secret" ]
                        fa445f41 LEAF "secret"
        1435493d NODE "Alice" [ "data": { "secret" [ "classification": "top-secret" ] } ]
            a5d4710e ASSERTION "data": { "secret" [ "classification": "top-secret" ] }
                41dca0cd WRAPPED { "secret" [ "classification": "top-secret" ] }
                    f66baec9 NODE "secret" [ "classification": "top-secret" ]
                        7e14bb9e ASSERTION "classification": "top-secret"
                            c2d8f15f LEAF "top-secret"
    "#}.trim();
    assert_actual_expected!(format_paths(&secret_regex_search_paths), expected);
}

#[test]
fn test_search_pattern_credential() {
    use bc_envelope_pattern::Path;

    use crate::common::test_data::credential;

    // A pattern that searches for any text in the envelepe
    let text_search_pattern = Pattern::search(Pattern::any_text());

    let cred = credential();
    // println!("{}", cred.tree_format());
    #[rustfmt::skip]
    let expected = indoc! {r#"
        0b721f78 NODE
            397a2d4c subj WRAPPED
                8122ffa9 cont NODE
                    10d3de01 subj ARID(4676635a)
                    1f9ff098 ASSERTION
                        9e3bff3a pred "certificateNumber"
                        21c21808 obj "123-456-789"
                    36c254d0 ASSERTION
                        6e5d379f pred "expirationDate"
                        639ae9bf obj 2028-01-01
                    3c114201 ASSERTION
                        5f82a16a pred "lastName"
                        fe4d5230 obj "Maxwell"
                    4a9b2e4d ASSERTION
                        222afe69 pred "issueDate"
                        cb67f31d obj 2020-01-01
                    4d67bba0 ASSERTION
                        2be2d79b pred 'isA'
                        051beee6 obj "Certificate of Completion"
                    5171cbaf ASSERTION
                        3976ef74 pred "photo"
                        231b8527 obj "This is James Maxwell's photo."
                    54b3e1e7 ASSERTION
                        f13aa855 pred "professionalDevelopmentHours"
                        dc0e9c36 obj 15
                    5dc6d4e3 ASSERTION
                        4395643b pred "firstName"
                        d6d0b768 obj "James"
                    68895d8e ASSERTION
                        e6bf4dd3 pred "topics"
                        543fcc09 obj ["Subject 1", "Subject 2"]
                    8ec5e912 ASSERTION
                        2b191589 pred "continuingEducationUnits"
                        4bf5122f obj 1
                    9b3d4785 ASSERTION
                        af10ee92 pred 'controller'
                        f8489ac1 obj "Example Electrical Engineering Board"
                    caf5ced3 ASSERTION
                        8e4e62eb pred "subject"
                        202c10ef obj "RF and Microwave Engineering"
                    d3e0cc15 ASSERTION
                        6dd16ba3 pred 'issuer'
                        f8489ac1 obj "Example Electrical Engineering Board"
            46a02aaf ASSERTION
                d0e39e78 pred 'signed'
                34c14941 obj Signature
            e6d7fca0 ASSERTION
                0fcd6a39 pred 'note'
                f106bad1 obj "Signed by Example Electrical Engineering…"
    "#}.trim();
    assert_actual_expected!(cred.tree_format(), expected);

    // Search for all text in the credential
    let text_paths = text_search_pattern.paths(&cred);
    // Get the last element of each path as a single-element path for output
    let found_elements: Vec<Path> = text_paths
        .iter()
        .map(|path| vec![(*path.last().unwrap()).clone()])
        .collect();
    #[rustfmt::skip]
    let expected = indoc! {r#"
        9e3bff3a LEAF "certificateNumber"
        21c21808 LEAF "123-456-789"
        6e5d379f LEAF "expirationDate"
        5f82a16a LEAF "lastName"
        fe4d5230 LEAF "Maxwell"
        222afe69 LEAF "issueDate"
        051beee6 LEAF "Certificate of Completion"
        3976ef74 LEAF "photo"
        231b8527 LEAF "This is James Maxwell's photo."
        f13aa855 LEAF "professionalDevelopmentHours"
        4395643b LEAF "firstName"
        d6d0b768 LEAF "James"
        e6bf4dd3 LEAF "topics"
        2b191589 LEAF "continuingEducationUnits"
        f8489ac1 LEAF "Example Electrical Engineering Board"
        8e4e62eb LEAF "subject"
        202c10ef LEAF "RF and Microwave Engineering"
        f8489ac1 LEAF "Example Electrical Engineering Board"
        f106bad1 LEAF "Signed by Example Electrical Engineering Board"
    "#}.trim();
    assert_actual_expected!(format_paths(&found_elements), expected);

    // A pattern that searches for the text "James" in the credential
    let james_search_pattern = Pattern::search(Pattern::text("James"));
    // Search for specific strings that should be in the credential
    let james_paths = james_search_pattern.paths(&cred);
    assert_eq!(james_paths.len(), 1);

    // A pattern that searches for the text "Maxwell" in the credential
    let maxwell_search_pattern = Pattern::search(Pattern::text("Maxwell"));
    let maxwell_paths = maxwell_search_pattern.paths(&cred);
    assert_eq!(maxwell_paths.len(), 1);

    // A pattern that searches for numbers in the credential
    let number_search_pattern =
        Pattern::search(Pattern::assertion_with_object(Pattern::any_number()));
    // Should find education units and hours
    let number_paths = number_search_pattern.paths(&cred);
    // Get the last element of each path as a single-element path for output
    let number_paths: Vec<Path> = number_paths
        .iter()
        .map(|path| vec![(*path.last().unwrap()).clone()])
        .collect();
    #[rustfmt::skip]
    let expected = indoc! {r#"
        54b3e1e7 ASSERTION "professionalDevelopmentHours": 15
        8ec5e912 ASSERTION "continuingEducationUnits": 1
    "#}.trim();
    assert_actual_expected!(format_paths(&number_paths), expected);
}

#[test]
fn test_not_pattern() {
    // Create a test envelope
    let envelope = Envelope::new("test_subject")
        .add_assertion("key1", "value1")
        .add_assertion("key2", "value2")
        .add_assertion("number", 42);

    // Test not pattern with text pattern that doesn't match
    let not_matching_text_pattern =
        Pattern::not_matching(Pattern::text("non_matching_text"));
    assert_eq!(
        format!("{}", not_matching_text_pattern),
        r#"!"non_matching_text""#
    );
    let not_matches = not_matching_text_pattern.matches(&envelope);
    assert!(
        not_matches,
        "Should match when the inner pattern doesn't match"
    );

    // Test not pattern with text pattern that does match
    let not_matches =
        Pattern::not_matching(Pattern::text("test_subject")).matches(&envelope);
    assert!(
        !not_matches,
        "Should not match when the inner pattern matches"
    );

    // Test not pattern with object pattern - this should find no matches
    // because we have an assertion with object 42
    let search_pattern = Pattern::search(Pattern::not_matching(
        Pattern::object(Pattern::number(42)),
    ));
    assert_eq!(
        format!("{}", search_pattern),
        r#"SEARCH(!OBJECT(NUMBER(42)))"#
    );
    let not_patterns = search_pattern.paths(&envelope);

    // Should not match the assertion with object 42, but will match other
    // elements
    let found_objects = not_patterns
        .iter()
        .filter(|path| path.last().unwrap().is_assertion())
        .filter_map(|path| {
            let assertion = path.last().unwrap();
            assertion.extract_object::<i32>().ok()
        })
        .collect::<Vec<_>>();

    assert!(
        !found_objects.contains(&42),
        "Should not match assertions with object 42"
    );

    // Test combination of not pattern with other patterns
    let complex_pattern = Pattern::and(vec![
        Pattern::not_matching(Pattern::text("wrong_subject")),
        Pattern::assertion_with_predicate(Pattern::text("key1")),
    ]);
    assert_eq!(
        format!("{}", complex_pattern),
        r#"!"wrong_subject"&ASSERTPRED("key1")"#
    );

    let matches = complex_pattern.matches(&envelope);
    assert!(matches, "Complex pattern with not should match");

    // The path includes the assertion for a successful not pattern
    let pattern = Pattern::not_matching(Pattern::text("wrong"));
    let paths = pattern.paths(&envelope);

    // Instead of checking exact digest (which can change), check the content
    assert_eq!(paths.len(), 1, "Should have one path");
    let returned_envelope = paths[0][0].clone();
    assert_eq!(
        returned_envelope.extract_subject::<String>().unwrap(),
        "test_subject"
    );
}

#[test]
fn test_not_pattern_with_search() {
    // Create a nested envelope structure
    let inner_envelope =
        Envelope::new("inner").add_assertion("inner_key", "inner_value");

    let outer_envelope =
        Envelope::new("outer").add_assertion("contains", inner_envelope);

    // Search for elements that are NOT obscured (everything in this case)
    let pattern = Pattern::search(Pattern::not_matching(Pattern::obscured()));
    assert_eq!(format!("{}", pattern), r#"SEARCH(!OBSCURED)"#);
    let not_obscured_paths = pattern.paths(&outer_envelope);

    // We should find multiple matches (everything, since nothing is obscured)
    assert!(
        !not_obscured_paths.is_empty(),
        "Should find elements that are not obscured"
    );

    // Create envelope with elided content
    let envelope_with_elided = Envelope::new("test")
        .add_assertion("visible", "data")
        .add_assertion("hidden", Envelope::new("secret").elide());

    // Search for elements that are NOT elided
    let pattern = Pattern::search(Pattern::not_matching(Pattern::elided()));
    assert_eq!(format!("{}", pattern), r#"SEARCH(!ELIDED)"#);
    let not_elided_paths = pattern.paths(&envelope_with_elided);

    // Should find multiple elements that are not elided
    assert!(
        !not_elided_paths.is_empty(),
        "Should find elements that are not elided"
    );

    // Verify we didn't match the elided element
    for path in &not_elided_paths {
        if let Some(element) = path.last() {
            assert!(!element.is_elided(), "Should not match elided elements");
        }
    }
}

#[test]
fn test_capture_pattern() {
    let envelope = Envelope::new(42);

    let inner = Pattern::number(42);
    let capture = Pattern::capture("num", inner.clone());

    assert_eq!(format!("{}", capture), "@num(NUMBER(42))");
    assert!(capture.matches(&envelope));

    let inner_paths = inner.paths(&envelope);
    let (capture_paths, captures) = capture.paths_with_captures(&envelope);
    assert!(capture_paths.iter().any(|p| *p == inner_paths[0]));
    assert!(captures.contains_key("num"));
    assert!(captures.get("num").unwrap().contains(&inner_paths[0]));
}

#[test]
fn test_capture_multiple_matches() {
    let envelope = Envelope::new(42);

    let pattern = Pattern::or(vec![
        Pattern::capture("num", Pattern::number(42)),
        Pattern::capture("num", Pattern::number_greater_than(40)),
    ]);

    let (_paths, captures) = pattern.paths_with_captures(&envelope);
    let nums = captures.get("num").unwrap();
    assert_eq!(nums.len(), 2);
}

#[test]
fn test_capture_in_and_failure() {
    let envelope = Envelope::new(42);

    let pattern = Pattern::and(vec![
        Pattern::capture("num", Pattern::number(42)),
        Pattern::bool(true),
    ]);

    let (paths, captures) = pattern.paths_with_captures(&envelope);
    assert!(paths.is_empty());
    assert!(!captures.contains_key("num"));
}

#[test]
fn test_capture_in_traversal_failure() {
    let envelope = Envelope::new(42).add_assertion("an", "assertion");

    let pattern = Pattern::traverse(vec![
        Pattern::capture("num", Pattern::subject(Pattern::number(42))),
        Pattern::bool(true),
    ]);

    let (paths, captures) = pattern.paths_with_captures(&envelope);
    assert!(paths.is_empty());
    assert!(!captures.contains_key("num"));
}
