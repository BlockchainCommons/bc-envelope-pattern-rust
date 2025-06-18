use bc_components::Digest;
use bc_envelope::prelude::*;
use bc_envelope_pattern::{Pattern, parse_pattern};

#[test]
fn parse_node_patterns() {
    let p = parse_pattern("NODE").unwrap();
    assert_eq!(p, Pattern::any_node());
    assert_eq!(p.to_string(), "NODE");

    let p = parse_pattern("NODE({1,3})").unwrap();
    assert_eq!(p, Pattern::node_with_assertions_range(1..=3));
    assert_eq!(p.to_string(), "NODE({1,3})");
}

#[test]
fn parse_wrapped_pattern() {
    let p = parse_pattern("WRAPPED").unwrap();
    assert_eq!(p, Pattern::wrapped());
    assert_eq!(p.to_string(), "WRAPPED");
}

#[test]
fn parse_subject_patterns() {
    let p = parse_pattern("SUBJECT").unwrap();
    assert_eq!(p, Pattern::any_subject());
    assert_eq!(p.to_string(), "SUBJECT");

    let p = parse_pattern("SUBJECT(TEXT(\"hi\"))").unwrap();
    assert_eq!(p, Pattern::subject(Pattern::text("hi")));
    assert_eq!(p.to_string(), "SUBJECT(TEXT(\"hi\"))");
}

#[test]
fn parse_assert_patterns() {
    let p = parse_pattern("ASSERT").unwrap();
    assert_eq!(p, Pattern::any_assertion());
    assert_eq!(p.to_string(), "ASSERT");

    let p = parse_pattern("ASSERTPRED(TEXT(\"hi\"))").unwrap();
    assert_eq!(p, Pattern::assertion_with_predicate(Pattern::text("hi")));
    assert_eq!(p.to_string(), "ASSERTPRED(TEXT(\"hi\"))");

    let spaced = "ASSERTPRED ( TEXT(\"hi\") )";
    let p_spaced = parse_pattern(spaced).unwrap();
    assert_eq!(
        p_spaced,
        Pattern::assertion_with_predicate(Pattern::text("hi"))
    );
    assert_eq!(p_spaced.to_string(), "ASSERTPRED(TEXT(\"hi\"))");

    let p = parse_pattern("ASSERTOBJ(NUMBER(1))").unwrap();
    assert_eq!(p, Pattern::assertion_with_object(Pattern::number(1)));
    assert_eq!(p.to_string(), "ASSERTOBJ(NUMBER(1))");
}

#[test]
fn parse_object_patterns() {
    let p = parse_pattern("OBJ").unwrap();
    assert_eq!(p, Pattern::any_object());
    assert_eq!(p.to_string(), "OBJECT");

    let p = parse_pattern("OBJ(TEXT(\"hi\"))").unwrap();
    assert_eq!(p, Pattern::object(Pattern::text("hi")));
    assert_eq!(p.to_string(), "OBJECT(TEXT(\"hi\"))");

    let spaced = "OBJ ( TEXT(\"hi\") )";
    let p_spaced = parse_pattern(spaced).unwrap();
    assert_eq!(p_spaced, Pattern::object(Pattern::text("hi")));
    assert_eq!(p_spaced.to_string(), "OBJECT(TEXT(\"hi\"))");
}

#[test]
fn parse_predicate_patterns() {
    let p = parse_pattern("PRED").unwrap();
    assert_eq!(p, Pattern::any_predicate());
    assert_eq!(p.to_string(), "PRED");

    let p = parse_pattern("PRED(NUMBER(1))").unwrap();
    assert_eq!(p, Pattern::predicate(Pattern::number(1)));
    assert_eq!(p.to_string(), "PRED(NUMBER(1))");

    let spaced = "PRED ( NUMBER(1) )";
    let p_spaced = parse_pattern(spaced).unwrap();
    assert_eq!(p_spaced, Pattern::predicate(Pattern::number(1)));
    assert_eq!(p_spaced.to_string(), "PRED(NUMBER(1))");
}

#[test]
fn parse_obscured_patterns() {
    let p = parse_pattern("OBSCURED").unwrap();
    assert_eq!(p, Pattern::obscured());
    assert_eq!(p.to_string(), "OBSCURED");

    let p = parse_pattern("ELIDED").unwrap();
    assert_eq!(p, Pattern::elided());
    assert_eq!(p.to_string(), "ELIDED");

    let p = parse_pattern("ENCRYPTED").unwrap();
    assert_eq!(p, Pattern::encrypted());
    assert_eq!(p.to_string(), "ENCRYPTED");

    let p = parse_pattern("COMPRESSED").unwrap();
    assert_eq!(p, Pattern::compressed());
    assert_eq!(p.to_string(), "COMPRESSED");
}

#[test]
fn parse_digest_patterns() {
    let p = parse_pattern("DIGEST(a1b2c3)").unwrap();
    assert_eq!(p, Pattern::digest_prefix(hex::decode("a1b2c3").unwrap()));
    assert_eq!(p.to_string(), "DIGEST(a1b2c3)");

    let spaced = "DIGEST ( a1b2c3 )";
    let p_spaced = parse_pattern(spaced).unwrap();
    assert_eq!(
        p_spaced,
        Pattern::digest_prefix(hex::decode("a1b2c3").unwrap())
    );
    assert_eq!(p_spaced.to_string(), "DIGEST(a1b2c3)");
}

#[test]
fn parse_digest_ur_pattern() {
    bc_envelope::register_tags();
    let digest = Digest::from_image(b"hello world");
    let ur = digest.ur_string();
    let expr = format!("DIGEST({})", ur);
    let p = parse_pattern(&expr).unwrap();
    assert_eq!(p, Pattern::digest(digest.clone()));
    assert_eq!(p.to_string(), format!("DIGEST({})", digest));
}
