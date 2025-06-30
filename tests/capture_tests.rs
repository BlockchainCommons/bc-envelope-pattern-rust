mod common;

use bc_envelope::prelude::*;
use bc_envelope_pattern::{Matcher, Pattern, format_paths_with_captures};
use indoc::indoc;

#[test]
fn capture_simple_number() {
    let env = Envelope::new(42);
    let pat = Pattern::parse("@num(42)").unwrap();
    assert_actual_expected!(pat.to_string(), "@num(42)");

    let (paths, caps) = pat.paths_with_captures(&env);
    #[rustfmt::skip]
    let expected = indoc! {r#"
        @num
            7f83f7bd LEAF 42
        7f83f7bd LEAF 42
    "#}.trim();
    assert_actual_expected!(
        format_paths_with_captures(&paths, &caps),
        expected
    );
}

#[test]
fn capture_multiple_or() {
    let env = Envelope::new(42);
    let pat = Pattern::parse("@num(42)|@num(>40)").unwrap();
    let (paths, caps) = pat.paths_with_captures(&env);

    // TODO: The output contains duplicates, which is incorrect.
    #[rustfmt::skip]
    let expected = indoc! {r#"
        @num
            7f83f7bd LEAF 42
            7f83f7bd LEAF 42
        7f83f7bd LEAF 42
        7f83f7bd LEAF 42
    "#}.trim();
    assert_actual_expected!(
        format_paths_with_captures(&paths, &caps),
        expected,
        "Multiple OR capture test"
    );
}

#[test]
fn capture_nested_number() {
    let env = Envelope::new(42);
    let pat = Pattern::parse("@outer(@inner(42))").unwrap();
    let (paths, caps) = pat.paths_with_captures(&env);

    #[rustfmt::skip]
    let expected = indoc! {r#"
        @inner
            7f83f7bd LEAF 42
        @outer
            7f83f7bd LEAF 42
        7f83f7bd LEAF 42
    "#}.trim();
    assert_actual_expected!(
        format_paths_with_captures(&paths, &caps),
        expected,
        "Nested capture test"
    );
}

#[test]
fn capture_no_match() {
    let env = Envelope::new(1);
    let pat = Pattern::parse("@n(2)").unwrap();
    let (paths, caps) = pat.paths_with_captures(&env);

    #[rustfmt::skip]
    let expected = "";
    assert_actual_expected!(
        format_paths_with_captures(&paths, &caps),
        expected,
        "No match capture test"
    );
}
