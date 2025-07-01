use bc_components::Digest;
use bc_envelope::prelude::*;
use bc_envelope_pattern::Pattern;

#[test]
fn parse_node_patterns() {
    let p = Pattern::parse("NODE").unwrap();
    assert_eq!(p, Pattern::any_node());
    assert_eq!(p.to_string(), "NODE");

    let p = Pattern::parse("NODE({1,3})").unwrap();
    assert_eq!(p, Pattern::node_with_assertions_range(1..=3));
    assert_eq!(p.to_string(), "NODE({1,3})");
}

#[test]
fn parse_wrapped_pattern() {
    let p = Pattern::parse("WRAPPED").unwrap();
    assert_eq!(p, Pattern::wrapped());
    assert_eq!(p.to_string(), "WRAPPED");
}

#[test]
fn parse_unwrap_pattern() {
    let p = Pattern::parse("UNWRAP").unwrap();
    assert_eq!(p, Pattern::unwrap());
    assert_eq!(p.to_string(), "UNWRAP");

    let p = Pattern::parse("UNWRAP(NODE)").unwrap();
    assert_eq!(p, Pattern::unwrap_matching(Pattern::any_node()));
    assert_eq!(p.to_string(), "UNWRAP(NODE)");
}

#[test]
fn parse_subject_patterns() {
    let p = Pattern::parse("SUBJECT").unwrap();
    assert_eq!(p, Pattern::any_subject());
    assert_eq!(p.to_string(), "SUBJECT");

    let p = Pattern::parse(r#"SUBJECT("hi")"#).unwrap();
    assert_eq!(p, Pattern::subject(Pattern::text("hi")));
    assert_eq!(p.to_string(), r#"SUBJECT("hi")"#);
}

#[test]
fn parse_assert_patterns() {
    let p = Pattern::parse("assert").unwrap();
    assert_eq!(p, Pattern::any_assertion());
    assert_eq!(p.to_string(), "assert");

    let p = Pattern::parse(r#"assertpred("hi")"#).unwrap();
    assert_eq!(p, Pattern::assertion_with_predicate(Pattern::text("hi")));
    assert_eq!(p.to_string(), r#"assertpred("hi")"#);

    let spaced = r#"assertpred ( "hi" )"#;
    let p_spaced = Pattern::parse(spaced).unwrap();
    assert_eq!(
        p_spaced,
        Pattern::assertion_with_predicate(Pattern::text("hi"))
    );
    assert_eq!(p_spaced.to_string(), r#"assertpred("hi")"#);

    let p = Pattern::parse("assertobj(1)").unwrap();
    assert_eq!(p, Pattern::assertion_with_object(Pattern::number(1)));
    assert_eq!(p.to_string(), "assertobj(1)");
}

#[test]
fn parse_object_patterns() {
    let p = Pattern::parse("OBJ").unwrap();
    assert_eq!(p, Pattern::any_object());
    assert_eq!(p.to_string(), "OBJECT");

    let p = Pattern::parse(r#"OBJ("hi")"#).unwrap();
    assert_eq!(p, Pattern::object(Pattern::text("hi")));
    assert_eq!(p.to_string(), r#"OBJECT("hi")"#);

    let spaced = r#"OBJ ( "hi" )"#;
    let p_spaced = Pattern::parse(spaced).unwrap();
    assert_eq!(p_spaced, Pattern::object(Pattern::text("hi")));
    assert_eq!(p_spaced.to_string(), r#"OBJECT("hi")"#);
}

#[test]
fn parse_predicate_patterns() {
    let p = Pattern::parse("PRED").unwrap();
    assert_eq!(p, Pattern::any_predicate());
    assert_eq!(p.to_string(), "PRED");

    let p = Pattern::parse("PRED(1)").unwrap();
    assert_eq!(p, Pattern::predicate(Pattern::number(1)));
    assert_eq!(p.to_string(), "PRED(1)");

    let spaced = "PRED ( 1 )";
    let p_spaced = Pattern::parse(spaced).unwrap();
    assert_eq!(p_spaced, Pattern::predicate(Pattern::number(1)));
    assert_eq!(p_spaced.to_string(), "PRED(1)");
}

#[test]
fn parse_obscured_patterns() {
    let p = Pattern::parse("OBSCURED").unwrap();
    assert_eq!(p, Pattern::obscured());
    assert_eq!(p.to_string(), "OBSCURED");

    let p = Pattern::parse("ELIDED").unwrap();
    assert_eq!(p, Pattern::elided());
    assert_eq!(p.to_string(), "ELIDED");

    let p = Pattern::parse("ENCRYPTED").unwrap();
    assert_eq!(p, Pattern::encrypted());
    assert_eq!(p.to_string(), "ENCRYPTED");

    let p = Pattern::parse("COMPRESSED").unwrap();
    assert_eq!(p, Pattern::compressed());
    assert_eq!(p.to_string(), "COMPRESSED");
}

#[test]
fn parse_digest_patterns() {
    let p = Pattern::parse("DIGEST(a1b2c3)").unwrap();
    assert_eq!(p, Pattern::digest_prefix(hex::decode("a1b2c3").unwrap()));
    assert_eq!(p.to_string(), "DIGEST(a1b2c3)");

    let spaced = "DIGEST ( a1b2c3 )";
    let p_spaced = Pattern::parse(spaced).unwrap();
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
    let p = Pattern::parse(&expr).unwrap();
    assert_eq!(p, Pattern::digest(digest.clone()));
    assert_eq!(p.to_string(), format!("DIGEST({})", digest));
}
