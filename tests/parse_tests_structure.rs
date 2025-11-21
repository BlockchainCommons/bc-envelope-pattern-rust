use bc_components::Digest;
use bc_envelope::prelude::*;
use bc_envelope_pattern::Pattern;

#[test]
fn parse_node_patterns() {
    let p = Pattern::parse("node").unwrap();
    assert_eq!(p, Pattern::any_node());
    assert_eq!(p.to_string(), "node");

    let p = Pattern::parse("node({1,3})").unwrap();
    assert_eq!(p, Pattern::node_with_assertions_range(1..=3));
    assert_eq!(p.to_string(), "node({1,3})");
}

#[test]
fn parse_wrapped_pattern() {
    let p = Pattern::parse("wrapped").unwrap();
    assert_eq!(p, Pattern::wrapped());
    assert_eq!(p.to_string(), "wrapped");
}

#[test]
fn parse_unwrap_pattern() {
    let p = Pattern::parse("unwrap").unwrap();
    assert_eq!(p, Pattern::unwrap());
    assert_eq!(p.to_string(), "unwrap");

    let p = Pattern::parse("unwrap(node)").unwrap();
    assert_eq!(p, Pattern::unwrap_matching(Pattern::any_node()));
    assert_eq!(p.to_string(), "unwrap(node)");
}

#[test]
fn parse_subject_patterns() {
    let p = Pattern::parse("subj").unwrap();
    assert_eq!(p, Pattern::any_subject());
    assert_eq!(p.to_string(), "subj");

    let p = Pattern::parse(r#"subj("hi")"#).unwrap();
    assert_eq!(p, Pattern::subject(Pattern::text("hi")));
    assert_eq!(p.to_string(), r#"subj("hi")"#);
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
    let p = Pattern::parse("obj").unwrap();
    assert_eq!(p, Pattern::any_object());
    assert_eq!(p.to_string(), "obj");

    let p = Pattern::parse(r#"obj("hi")"#).unwrap();
    assert_eq!(p, Pattern::object(Pattern::text("hi")));
    assert_eq!(p.to_string(), r#"obj("hi")"#);

    let spaced = r#"obj ( "hi" )"#;
    let p_spaced = Pattern::parse(spaced).unwrap();
    assert_eq!(p_spaced, Pattern::object(Pattern::text("hi")));
    assert_eq!(p_spaced.to_string(), r#"obj("hi")"#);
}

#[test]
fn parse_predicate_patterns() {
    let p = Pattern::parse("pred").unwrap();
    assert_eq!(p, Pattern::any_predicate());
    assert_eq!(p.to_string(), "pred");

    let p = Pattern::parse("pred(1)").unwrap();
    assert_eq!(p, Pattern::predicate(Pattern::number(1)));
    assert_eq!(p.to_string(), "pred(1)");

    let spaced = "pred ( 1 )";
    let p_spaced = Pattern::parse(spaced).unwrap();
    assert_eq!(p_spaced, Pattern::predicate(Pattern::number(1)));
    assert_eq!(p_spaced.to_string(), "pred(1)");
}

#[test]
fn parse_obscured_patterns() {
    let p = Pattern::parse("obscured").unwrap();
    assert_eq!(p, Pattern::obscured());
    assert_eq!(p.to_string(), "obscured");

    let p = Pattern::parse("elided").unwrap();
    assert_eq!(p, Pattern::elided());
    assert_eq!(p.to_string(), "elided");

    let p = Pattern::parse("encrypted").unwrap();
    assert_eq!(p, Pattern::encrypted());
    assert_eq!(p.to_string(), "encrypted");

    let p = Pattern::parse("compressed").unwrap();
    assert_eq!(p, Pattern::compressed());
    assert_eq!(p.to_string(), "compressed");
}

#[test]
fn parse_digest_patterns() {
    let p = Pattern::parse("digest(a1b2c3)").unwrap();
    assert_eq!(p, Pattern::digest_prefix(hex::decode("a1b2c3").unwrap()));
    assert_eq!(p.to_string(), "digest(a1b2c3)");

    let spaced = "digest ( a1b2c3 )";
    let p_spaced = Pattern::parse(spaced).unwrap();
    assert_eq!(
        p_spaced,
        Pattern::digest_prefix(hex::decode("a1b2c3").unwrap())
    );
    assert_eq!(p_spaced.to_string(), "digest(a1b2c3)");
}

#[test]
fn parse_digest_ur_pattern() {
    bc_envelope::register_tags();
    let digest = Digest::from_image(b"hello world");
    let ur = digest.ur_string();
    let expr = format!("digest({})", ur);
    let p = Pattern::parse(&expr).unwrap();
    assert_eq!(p, Pattern::digest(digest));
    assert_eq!(p.to_string(), format!("digest({})", digest));
}
