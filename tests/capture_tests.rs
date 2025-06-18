mod common;

use bc_envelope::prelude::*;
use bc_envelope_pattern::{parse_pattern, Matcher};

#[test]
fn capture_simple_number() {
    let env = Envelope::new(42);
    let pat = parse_pattern("@num(NUMBER(42))").unwrap();
    let (paths, caps) = pat.paths_with_captures(&env);
    assert_eq!(paths.len(), 1);
    assert_eq!(caps.get("num").unwrap().len(), 1);
    assert_eq!(pat.to_string(), "@num(NUMBER(42))");
}

#[test]
fn capture_multiple_or() {
    let env = Envelope::new(42);
    let pat = parse_pattern("@num(NUMBER(42))|@num(NUMBER(>40))").unwrap();
    let (_paths, caps) = pat.paths_with_captures(&env);
    let nums = caps.get("num").unwrap();
    assert_eq!(nums.len(), 2);
}

#[test]
fn capture_nested_number() {
    let env = Envelope::new(42);
    let pat = parse_pattern("@outer(@inner(NUMBER(42)))").unwrap();
    let (paths, caps) = pat.paths_with_captures(&env);
    assert_eq!(paths.len(), 1);
    assert!(caps.contains_key("outer"));
    assert!(caps.contains_key("inner"));
}

#[test]
fn capture_no_match() {
    let env = Envelope::new(1);
    let pat = parse_pattern("@n(NUMBER(2))").unwrap();
    let (paths, caps) = pat.paths_with_captures(&env);
    assert!(paths.is_empty());
    assert!(caps.get("n").is_none());
}
