use bc_envelope_pattern::{parse_pattern, Pattern};
use known_values::KnownValue;

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
    assert_eq!(
        p.to_string(),
        "ANY>BOOL(true)&BOOL(false)>NONE|ANY>BOOL(true)&BOOL(false)>ANY"
    );
}

#[test]
fn parse_text_patterns() {
    let p = parse_pattern("TEXT").unwrap();
    assert_eq!(p, Pattern::any_text());
    assert_eq!(p.to_string(), "TEXT");

    let p = parse_pattern(r#"TEXT("hello")"#).unwrap();
    assert_eq!(p, Pattern::text("hello"));
    assert_eq!(p.to_string(), r#"TEXT("hello")"#);

    let spaced = r#"TEXT ( "hello" )"#;
    let p_spaced = parse_pattern(spaced).unwrap();
    assert_eq!(p_spaced, Pattern::text("hello"));
    assert_eq!(p_spaced.to_string(), r#"TEXT("hello")"#);

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

#[test]
fn parse_leaf_pattern() {
    let p = parse_pattern("LEAF").unwrap();
    assert_eq!(p, Pattern::any_leaf());
    assert_eq!(p.to_string(), "LEAF");
}

#[test]
fn parse_array_patterns() {
    let p = parse_pattern("ARRAY").unwrap();
    assert_eq!(p, Pattern::any_array());
    assert_eq!(p.to_string(), "ARRAY");

    let p = parse_pattern("ARRAY({3})").unwrap();
    assert_eq!(p, Pattern::array_with_count(3));
    assert_eq!(p.to_string(), "ARRAY({3})");

    let p = parse_pattern("ARRAY({2,4})").unwrap();
    assert_eq!(p, Pattern::array_with_range(2..=4));
    assert_eq!(p.to_string(), "ARRAY({2,4})");

    let p = parse_pattern("ARRAY({2,})").unwrap();
    assert_eq!(p, Pattern::array_with_range(2..));
    assert_eq!(p.to_string(), "ARRAY({2,})");
}

#[test]
fn parse_bstr_patterns() {
    let p = parse_pattern("BSTR").unwrap();
    assert_eq!(p, Pattern::any_byte_string());
    assert_eq!(p.to_string(), "BSTR");

    let p = parse_pattern("BSTR(h'0102')").unwrap();
    assert_eq!(p, Pattern::byte_string(vec![1u8, 2]));
    assert_eq!(p.to_string(), "BSTR(h'0102')");

    let p = parse_pattern("BSTR(/abc/)").unwrap();
    let regex = regex::bytes::Regex::new("abc").unwrap();
    assert_eq!(p, Pattern::byte_string_binary_regex(regex));
    assert_eq!(p.to_string(), "BSTR(/abc/)");
}

#[test]
fn parse_date_patterns() {
    use dcbor::Date;

    let p = parse_pattern("DATE").unwrap();
    assert_eq!(p, Pattern::any_date());
    assert_eq!(p.to_string(), "DATE");

    let p = parse_pattern("DATE(2023-12-25)").unwrap();
    let d = Date::from_string("2023-12-25").unwrap();
    assert_eq!(p, Pattern::date(d.clone()));
    assert_eq!(p.to_string(), "DATE(2023-12-25)");

    let p = parse_pattern("DATE(2023-12-24...2023-12-26)").unwrap();
    let start = Date::from_string("2023-12-24").unwrap();
    let end = Date::from_string("2023-12-26").unwrap();
    assert_eq!(p, Pattern::date_range(start..=end));
    assert_eq!(p.to_string(), "DATE(2023-12-24...2023-12-26)");

    let p = parse_pattern("DATE(2023-12-24...)").unwrap();
    let start = Date::from_string("2023-12-24").unwrap();
    assert_eq!(p, Pattern::date_earliest(start.clone()));
    assert_eq!(p.to_string(), "DATE(2023-12-24...)");

    let p = parse_pattern("DATE(...2023-12-26)").unwrap();
    let end = Date::from_string("2023-12-26").unwrap();
    assert_eq!(p, Pattern::date_latest(end.clone()));
    assert_eq!(p.to_string(), "DATE(...2023-12-26)");

    let p = parse_pattern("DATE(/2023-.*/)").unwrap();
    let regex = regex::Regex::new("2023-.*").unwrap();
    assert_eq!(p, Pattern::date_regex(regex));
    assert_eq!(p.to_string(), "DATE(/2023-.*/)");
}

#[test]
fn parse_map_patterns() {
    let p = parse_pattern("MAP").unwrap();
    assert_eq!(p, Pattern::any_map());
    assert_eq!(p.to_string(), "MAP");

    let p = parse_pattern("MAP(3)").unwrap();
    assert_eq!(p, Pattern::map_with_count(3));
    assert_eq!(p.to_string(), "MAP({3})");

    let p = parse_pattern("MAP({2,4})").unwrap();
    assert_eq!(p, Pattern::map_with_range(2..=4));
    assert_eq!(p.to_string(), "MAP({2,4})");

    let p = parse_pattern("MAP({2,})").unwrap();
    assert_eq!(p, Pattern::map_with_range(2..));
    assert_eq!(p.to_string(), "MAP({2,})");
}

#[test]
fn parse_null_pattern() {
    let p = parse_pattern("NULL").unwrap();
    assert_eq!(p, Pattern::null());
    assert_eq!(p.to_string(), "NULL");
}

#[test]
fn parse_tag_patterns() {
    let p = parse_pattern("TAG").unwrap();
    assert_eq!(p, Pattern::any_tag());
    assert_eq!(p.to_string(), "TAG");

    let p = parse_pattern("TAG(100)").unwrap();
    assert_eq!(p, Pattern::tagged_with_value(100));
    assert_eq!(p.to_string(), "TAG(100)");

    let p = parse_pattern("TAG(date)").unwrap();
    assert_eq!(p, Pattern::tagged_with_name("date"));
    assert_eq!(p.to_string(), "TAG(date)");

    let p = parse_pattern("TAG(/da.*/)").unwrap();
    let regex = regex::Regex::new("da.*").unwrap();
    assert_eq!(p, Pattern::tagged_with_regex(regex));
    assert_eq!(p.to_string(), "TAG(/da.*/)");
}

#[test]
fn parse_known_value_patterns() {
    let p = parse_pattern("KNOWN").unwrap();
    assert_eq!(p, Pattern::any_known_value());
    assert_eq!(p.to_string(), "KNOWN");

    let p = parse_pattern("KNOWN('1')").unwrap();
    assert_eq!(p, Pattern::known_value(KnownValue::new(1)));
    assert_eq!(p.to_string(), "KNOWN('1')");

    let p = parse_pattern("KNOWN('date')").unwrap();
    assert_eq!(p, Pattern::known_value_named("date"));
    assert_eq!(p.to_string(), "KNOWN('date')");

    let p = parse_pattern("KNOWN(/da.*/)").unwrap();
    let regex = regex::Regex::new("da.*").unwrap();
    assert_eq!(p, Pattern::known_value_regex(regex));
    assert_eq!(p.to_string(), "KNOWN(/da.*/)");
}

#[test]
fn parse_cbor_patterns() {
    use bc_envelope::prelude::*;
    use bc_tags::register_tags as register_old_tags;
    use dcbor::{Map, CBOR};
    bc_envelope::register_tags();
    register_old_tags();

    let p = parse_pattern("CBOR").unwrap();
    assert_eq!(p, Pattern::any_cbor());
    assert_eq!(p.to_string(), "CBOR");

    let p = parse_pattern("CBOR(true)").unwrap();
    assert_eq!(p, Pattern::cbor(true));
    assert_eq!(p.to_string(), "CBOR(true)");

    let p = parse_pattern("CBOR([1, 2, 3])").unwrap();
    assert_eq!(p, Pattern::cbor(vec![1, 2, 3]));
    assert_eq!(p.to_string(), "CBOR([1, 2, 3])");

    let p = parse_pattern("CBOR({\"a\": 1})").unwrap();
    let mut map = Map::new();
    map.insert("a", 1);
    assert_eq!(p, Pattern::cbor(map.clone()));
    assert_eq!(p.to_string(), "CBOR({\"a\": 1})");

    let p = parse_pattern("CBOR(1(\"hi\"))").unwrap();
    assert_eq!(p, Pattern::cbor(CBOR::to_tagged_value(1, "hi")));
    assert_eq!(p.to_string(), "CBOR(1(\"hi\"))");

    let date = dcbor::Date::from_ymd(2025, 5, 15);
    let ur = date.ur_string();
    let expr = format!("CBOR({})", ur);
    let p = parse_pattern(&expr).unwrap();
    assert_eq!(p, Pattern::cbor(date.clone()));
    assert_eq!(
        p.to_string(),
        format!("CBOR({})", date.to_cbor().diagnostic_flat())
    );
}

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
