use bc_envelope_pattern::{parse_pattern, Pattern};

#[test]
fn parse_any() {
    let src = "ANY";
    let p = parse_pattern(src).unwrap();
    assert_eq!(p, Pattern::any());
    assert_eq!(p.to_string(), src);
}

#[test]
fn parse_none() {
    let src = "NONE";
    let p = parse_pattern(src).unwrap();
    assert_eq!(p, Pattern::none());
    assert_eq!(p.to_string(), src);
}

#[test]
fn parse_bool_any() {
    let src = "BOOL";
    let p = parse_pattern(src).unwrap();
    assert_eq!(p, Pattern::any_bool());
    assert_eq!(p.to_string(), src);
}

#[test]
fn parse_bool_true() {
    let src = "BOOL(true)";
    let p = parse_pattern(src).unwrap();
    assert_eq!(p, Pattern::bool(true));
    assert_eq!(p.to_string(), src);
    let spaced = "BOOL ( true )";
    let p_spaced = parse_pattern(spaced).unwrap();
    assert_eq!(p_spaced, Pattern::bool(true));
    assert_eq!(p_spaced.to_string(), src);
}

#[test]
fn parse_bool_false() {
    let src = "BOOL(false)";
    let p = parse_pattern(src).unwrap();
    assert_eq!(p, Pattern::bool(false));
    assert_eq!(p.to_string(), src);
    let spaced = "BOOL ( false )";
    let p_spaced = parse_pattern(spaced).unwrap();
    assert_eq!(p_spaced, Pattern::bool(false));
    assert_eq!(p_spaced.to_string(), src);
}

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
    assert_eq!(p.to_string(), "ANY>BOOL(true)&BOOL(false)>NONE|ANY>BOOL(true)&BOOL(false)>ANY");
}

#[test]
fn parse_text_patterns() {
    let p = parse_pattern("TEXT").unwrap();
    assert_eq!(p, Pattern::any_text());
    assert_eq!(p.to_string(), "TEXT");

    let p = parse_pattern("TEXT(\"hello\")").unwrap();
    assert_eq!(p, Pattern::text("hello"));
    assert_eq!(p.to_string(), "TEXT(\"hello\")");

    let spaced = "TEXT ( \"hello\" )";
    let p_spaced = parse_pattern(spaced).unwrap();
    assert_eq!(p_spaced, Pattern::text("hello"));
    assert_eq!(p_spaced.to_string(), "TEXT(\"hello\")");

    let p = parse_pattern("TEXT(/h.*o/)").unwrap();
    let regex = regex::Regex::new("h.*o").unwrap();
    assert_eq!(p, Pattern::text_regex(regex));
    assert_eq!(p.to_string(), "TEXT(/h.*o/)");
}

#[test]
fn parse_number_patterns() {
    let p = parse_pattern("NUMBER").unwrap();
    assert_eq!(p, Pattern::any_number());
    assert_eq!(p.to_string(), "NUMBER");

    let p = parse_pattern("NUMBER(42)").unwrap();
    assert_eq!(p, Pattern::number(42));
    assert_eq!(p.to_string(), "NUMBER(42)");

    let spaced = "NUMBER ( 42 )";
    let p_spaced = parse_pattern(spaced).unwrap();
    assert_eq!(p_spaced, Pattern::number(42));
    assert_eq!(p_spaced.to_string(), "NUMBER(42)");

    let p = parse_pattern("NUMBER(1...3)").unwrap();
    assert_eq!(p, Pattern::number_range(1..=3));
    assert_eq!(p.to_string(), "NUMBER(1...3)");

    let p = parse_pattern("NUMBER(>5)").unwrap();
    assert_eq!(p, Pattern::number_greater_than(5));
    assert_eq!(p.to_string(), "NUMBER(>5)");

    let p = parse_pattern("NUMBER(>=5)").unwrap();
    assert_eq!(p, Pattern::number_greater_than_or_equal(5));
    assert_eq!(p.to_string(), "NUMBER(>=5)");

    let p = parse_pattern("NUMBER(<5)").unwrap();
    assert_eq!(p, Pattern::number_less_than(5));
    assert_eq!(p.to_string(), "NUMBER(<5)");

    let p = parse_pattern("NUMBER(<=5)").unwrap();
    assert_eq!(p, Pattern::number_less_than_or_equal(5));
    assert_eq!(p.to_string(), "NUMBER(<=5)");

    let p = parse_pattern("NUMBER(NaN)").unwrap();
    assert_eq!(p, Pattern::number_nan());
    assert_eq!(p.to_string(), "NUMBER(NaN)");
}
