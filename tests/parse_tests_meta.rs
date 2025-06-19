use bc_envelope_pattern::{Pattern, Reluctance, parse_pattern};

#[test]
fn parse_bool_or() {
    let src = "BOOL(true)|BOOL(false)";
    let p = parse_pattern(src).unwrap();
    assert_eq!(
        p,
        Pattern::or(vec![Pattern::bool(true), Pattern::bool(false)])
    );
    assert_eq!(p.to_string(), src);

    let spaced = "BOOL(true) | BOOL(false)";
    let p_spaced = parse_pattern(spaced).unwrap();
    assert_eq!(
        p_spaced,
        Pattern::or(vec![Pattern::bool(true), Pattern::bool(false)])
    );
    assert_eq!(p_spaced.to_string(), src);
}

#[test]
fn parse_bool_and() {
    let src = "BOOL(true)&BOOL(false)";
    let p = parse_pattern(src).unwrap();
    assert_eq!(
        p,
        Pattern::and(vec![Pattern::bool(true), Pattern::bool(false)])
    );
    assert_eq!(p.to_string(), src);

    let spaced = "BOOL(true) & BOOL(false)";
    let p_spaced = parse_pattern(spaced).unwrap();
    assert_eq!(
        p_spaced,
        Pattern::and(vec![Pattern::bool(true), Pattern::bool(false)])
    );
    assert_eq!(p_spaced.to_string(), src);
}

#[test]
fn parse_bool_sequence() {
    let src = "BOOL(true)>BOOL(false)";
    let p = parse_pattern(src).unwrap();
    assert_eq!(
        p,
        Pattern::sequence(vec![Pattern::bool(true), Pattern::bool(false)])
    );
    assert_eq!(p.to_string(), src);

    let spaced = "BOOL(true) > BOOL(false)";
    let p_spaced = parse_pattern(spaced).unwrap();
    assert_eq!(
        p_spaced,
        Pattern::sequence(vec![Pattern::bool(true), Pattern::bool(false)])
    );
    assert_eq!(p_spaced.to_string(), src);
}

#[test]
fn parse_operator_precedence() {
    let expr = "ANY > BOOL(true) & BOOL(false) > NONE | ANY > BOOL(true) & BOOL(false) > ANY";
    let p = parse_pattern(expr).unwrap();

    let left_seq = Pattern::sequence(vec![
        Pattern::any(),
        Pattern::and(vec![Pattern::bool(true), Pattern::bool(false)]),
        Pattern::none(),
    ]);
    let right_seq = Pattern::sequence(vec![
        Pattern::any(),
        Pattern::and(vec![Pattern::bool(true), Pattern::bool(false)]),
        Pattern::any(),
    ]);
    let expected = Pattern::or(vec![left_seq, right_seq]);

    assert_eq!(p, expected);
    assert_eq!(
        p.to_string(),
        "ANY>BOOL(true)&BOOL(false)>NONE|ANY>BOOL(true)&BOOL(false)>ANY"
    );
}

#[test]
fn parse_not_patterns() {
    let p = parse_pattern("!TEXT(\"hi\")").unwrap();
    assert_eq!(p, Pattern::not_matching(Pattern::text("hi")));
    assert_eq!(p.to_string(), "!TEXT(\"hi\")");

    let expr = "!ANY & NONE";
    let p = parse_pattern(expr).unwrap();
    let expected = Pattern::not_matching(Pattern::and(vec![
        Pattern::any(),
        Pattern::none(),
    ]));
    assert_eq!(p, expected);
    assert_eq!(p.to_string(), "!ANY&NONE");
}

#[test]
fn parse_search_pattern() {
    let p = parse_pattern("SEARCH(TEXT)").unwrap();
    assert_eq!(p, Pattern::search(Pattern::any_text()));
    assert_eq!(p.to_string(), "SEARCH(TEXT)");
}

#[test]
fn parse_repeat_patterns() {
    let p = parse_pattern("(WRAPPED)*").unwrap();
    assert_eq!(
        p,
        Pattern::repeat(Pattern::wrapped_new(), 0.., Reluctance::Greedy)
    );
    assert_eq!(p.to_string(), "(WRAPPED)*");

    let p = parse_pattern("(TEXT)+?").unwrap();
    assert_eq!(
        p,
        Pattern::repeat(Pattern::any_text(), 1.., Reluctance::Lazy)
    );
    assert_eq!(p.to_string(), "(TEXT)+?");

    let p = parse_pattern("(NUMBER){2,4}+").unwrap();
    assert_eq!(
        p,
        Pattern::repeat(Pattern::any_number(), 2..=4, Reluctance::Possessive)
    );
    assert_eq!(p.to_string(), "(NUMBER){2,4}+");
}

#[test]
fn parse_capture_patterns() {
    let src = "@name(NUMBER(1))";
    let p = parse_pattern(src).unwrap();
    assert_eq!(p, Pattern::capture("name", Pattern::number(1)));
    assert_eq!(p.to_string(), src);

    let spaced = "@name ( NUMBER ( 1 ) )";
    let p_spaced = parse_pattern(spaced).unwrap();
    assert_eq!(p_spaced, Pattern::capture("name", Pattern::number(1)));
    assert_eq!(p_spaced.to_string(), src);
}

#[test]
fn parse_nested_capture_patterns() {
    let src = "@outer(@inner(TEXT(\"hi\")))";
    let p = parse_pattern(src).unwrap();
    assert_eq!(
        p,
        Pattern::capture(
            "outer",
            Pattern::capture("inner", Pattern::text("hi"))
        )
    );
    assert_eq!(p.to_string(), src);
}

#[test]
fn parse_capture_name_variants() {
    let src = "@cap_1(NUMBER(42))";
    let p = parse_pattern(src).unwrap();
    assert_eq!(p, Pattern::capture("cap_1", Pattern::number(42)));
    assert_eq!(p.to_string(), src);
}
