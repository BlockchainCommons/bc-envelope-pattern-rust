use bc_envelope::prelude::*;
use bc_envelope_pattern::Pattern;
use dcbor::Date;
use known_values::KnownValue;

#[test]
fn parse_bool_any() {
    let src = "bool";
    let p = Pattern::parse(src).unwrap();
    assert_eq!(p, Pattern::any_bool());
    assert_eq!(p.to_string(), "bool");
}

#[test]
fn parse_bool_true() {
    let src = "true";
    let p = Pattern::parse(src).unwrap();
    assert_eq!(p, Pattern::bool(true));
    assert_eq!(p.to_string(), "true");
}

#[test]
fn parse_bool_false() {
    let src = "false";
    let p = Pattern::parse(src).unwrap();
    assert_eq!(p, Pattern::bool(false));
    assert_eq!(p.to_string(), "false");
}

#[test]
fn parse_text_patterns() {
    let p = Pattern::parse("TEXT").unwrap();
    assert_eq!(p, Pattern::any_text());
    assert_eq!(p.to_string(), "TEXT");

    let p = Pattern::parse(r#"TEXT("hello")"#).unwrap();
    assert_eq!(p, Pattern::text("hello"));
    assert_eq!(p.to_string(), r#"TEXT("hello")"#);

    let spaced = r#"TEXT ( "hello" )"#;
    let p_spaced = Pattern::parse(spaced).unwrap();
    assert_eq!(p_spaced, Pattern::text("hello"));
    assert_eq!(p_spaced.to_string(), r#"TEXT("hello")"#);

    let p = Pattern::parse("TEXT(/h.*o/)").unwrap();
    let regex = regex::Regex::new("h.*o").unwrap();
    assert_eq!(p, Pattern::text_regex(regex));
    assert_eq!(p.to_string(), "TEXT(/h.*o/)");
}

#[test]
fn parse_number_patterns() {
    let p = Pattern::parse("NUMBER").unwrap();
    assert_eq!(p, Pattern::any_number());
    assert_eq!(p.to_string(), "NUMBER");

    let p = Pattern::parse("NUMBER(42)").unwrap();
    assert_eq!(p, Pattern::number(42));
    assert_eq!(p.to_string(), "NUMBER(42)");

    let spaced = "NUMBER ( 42 )";
    let p_spaced = Pattern::parse(spaced).unwrap();
    assert_eq!(p_spaced, Pattern::number(42));
    assert_eq!(p_spaced.to_string(), "NUMBER(42)");

    let p = Pattern::parse("NUMBER(1...3)").unwrap();
    assert_eq!(p, Pattern::number_range(1..=3));
    assert_eq!(p.to_string(), "NUMBER(1...3)");

    let p = Pattern::parse("NUMBER(>5)").unwrap();
    assert_eq!(p, Pattern::number_greater_than(5));
    assert_eq!(p.to_string(), "NUMBER(>5)");

    let p = Pattern::parse("NUMBER(>=5)").unwrap();
    assert_eq!(p, Pattern::number_greater_than_or_equal(5));
    assert_eq!(p.to_string(), "NUMBER(>=5)");

    let p = Pattern::parse("NUMBER(<5)").unwrap();
    assert_eq!(p, Pattern::number_less_than(5));
    assert_eq!(p.to_string(), "NUMBER(<5)");

    let p = Pattern::parse("NUMBER(<=5)").unwrap();
    assert_eq!(p, Pattern::number_less_than_or_equal(5));
    assert_eq!(p.to_string(), "NUMBER(<=5)");

    let p = Pattern::parse("NUMBER(NaN)").unwrap();
    assert_eq!(p, Pattern::number_nan());
    assert_eq!(p.to_string(), "NUMBER(NaN)");
}

#[test]
fn parse_leaf_pattern() {
    let p = Pattern::parse("LEAF").unwrap();
    assert_eq!(p, Pattern::any_leaf());
    assert_eq!(p.to_string(), "LEAF");
}

#[test]
fn parse_array_patterns() {
    let p = Pattern::parse("ARRAY").unwrap();
    assert_eq!(p, Pattern::any_array());
    assert_eq!(p.to_string(), "ARRAY");

    let p = Pattern::parse("ARRAY({3})").unwrap();
    assert_eq!(p, Pattern::array_with_count(3));
    assert_eq!(p.to_string(), "ARRAY({3})");

    let p = Pattern::parse("ARRAY({2,4})").unwrap();
    assert_eq!(p, Pattern::array_with_range(2..=4));
    assert_eq!(p.to_string(), "ARRAY({2,4})");

    let p = Pattern::parse("ARRAY({2,})").unwrap();
    assert_eq!(p, Pattern::array_with_range(2..));
    assert_eq!(p.to_string(), "ARRAY({2,})");
}

#[test]
fn parse_bstr_patterns() {
    let p = Pattern::parse("bstr").unwrap();
    assert_eq!(p, Pattern::any_byte_string());
    assert_eq!(p.to_string(), "bstr");

    let p = Pattern::parse("h'0102'").unwrap();
    assert_eq!(p, Pattern::byte_string(vec![1u8, 2]));
    assert_eq!(p.to_string(), "h'0102'");

    let p = Pattern::parse("h'/abc/'").unwrap();
    let regex = regex::bytes::Regex::new("abc").unwrap();
    assert_eq!(p, Pattern::byte_string_binary_regex(regex));
    assert_eq!(p.to_string(), "h'/abc/'");
}

#[test]
fn parse_date_patterns() {
    let p = Pattern::parse("DATE").unwrap();
    assert_eq!(p, Pattern::any_date());
    assert_eq!(p.to_string(), "DATE");

    let p = Pattern::parse("DATE(2023-12-25)").unwrap();
    let d = Date::from_string("2023-12-25").unwrap();
    assert_eq!(p, Pattern::date(d.clone()));
    assert_eq!(p.to_string(), "DATE(2023-12-25)");

    let p = Pattern::parse("DATE(2023-12-24...2023-12-26)").unwrap();
    let start = Date::from_string("2023-12-24").unwrap();
    let end = Date::from_string("2023-12-26").unwrap();
    assert_eq!(p, Pattern::date_range(start..=end));
    assert_eq!(p.to_string(), "DATE(2023-12-24...2023-12-26)");

    let p = Pattern::parse("DATE(2023-12-24...)").unwrap();
    let start = Date::from_string("2023-12-24").unwrap();
    assert_eq!(p, Pattern::date_earliest(start.clone()));
    assert_eq!(p.to_string(), "DATE(2023-12-24...)");

    let p = Pattern::parse("DATE(...2023-12-26)").unwrap();
    let end = Date::from_string("2023-12-26").unwrap();
    assert_eq!(p, Pattern::date_latest(end.clone()));
    assert_eq!(p.to_string(), "DATE(...2023-12-26)");

    let p = Pattern::parse("DATE(/2023-.*/)").unwrap();
    let regex = regex::Regex::new("2023-.*").unwrap();
    assert_eq!(p, Pattern::date_regex(regex));
    assert_eq!(p.to_string(), "DATE(/2023-.*/)");
}

#[test]
fn parse_map_patterns() {
    let p = Pattern::parse("MAP").unwrap();
    assert_eq!(p, Pattern::any_map());
    assert_eq!(p.to_string(), "MAP");

    let p = Pattern::parse("MAP(3)").unwrap();
    assert_eq!(p, Pattern::map_with_count(3));
    assert_eq!(p.to_string(), "MAP({3})");

    let p = Pattern::parse("MAP({2,4})").unwrap();
    assert_eq!(p, Pattern::map_with_range(2..=4));
    assert_eq!(p.to_string(), "MAP({2,4})");

    let p = Pattern::parse("MAP({2,})").unwrap();
    assert_eq!(p, Pattern::map_with_range(2..));
    assert_eq!(p.to_string(), "MAP({2,})");
}

#[test]
fn parse_null_pattern() {
    let p = Pattern::parse("null").unwrap();
    assert_eq!(p, Pattern::null());
    assert_eq!(p.to_string(), "null");
}

#[test]
fn parse_tag_patterns() {
    let p = Pattern::parse("TAG").unwrap();
    assert_eq!(p, Pattern::any_tag());
    assert_eq!(p.to_string(), "TAG");

    let p = Pattern::parse("TAG(100)").unwrap();
    assert_eq!(p, Pattern::tagged_with_value(100));
    assert_eq!(p.to_string(), "TAG(100)");

    let p = Pattern::parse("TAG(date)").unwrap();
    assert_eq!(p, Pattern::tagged_with_name("date"));
    assert_eq!(p.to_string(), "TAG(date)");

    let p = Pattern::parse("TAG(/da.*/)").unwrap();
    let regex = regex::Regex::new("da.*").unwrap();
    assert_eq!(p, Pattern::tagged_with_regex(regex));
    assert_eq!(p.to_string(), "TAG(/da.*/)");
}

#[test]
fn parse_known_value_patterns() {
    let p = Pattern::parse("KNOWN").unwrap();
    assert_eq!(p, Pattern::any_known_value());
    assert_eq!(p.to_string(), "KNOWN");

    let p = Pattern::parse("KNOWN('1')").unwrap();
    assert_eq!(p, Pattern::known_value(KnownValue::new(1)));
    assert_eq!(p.to_string(), "KNOWN('1')");

    let p = Pattern::parse("KNOWN('date')").unwrap();
    assert_eq!(p, Pattern::known_value_named("date"));
    assert_eq!(p.to_string(), "KNOWN('date')");

    let p = Pattern::parse("KNOWN(/da.*/)").unwrap();
    let regex = regex::Regex::new("da.*").unwrap();
    assert_eq!(p, Pattern::known_value_regex(regex));
    assert_eq!(p.to_string(), "KNOWN(/da.*/)");
}

#[test]
fn parse_cbor_patterns() {
    let cases: Vec<(&str, CBOR)> = vec![
        ("CBOR(true)", true.into()),
        ("CBOR(42)", 42.into()),
        (r#"CBOR("hello")"#, "hello".into()),
        ("CBOR([1, 2])", vec![1, 2].into()),
        ("CBOR({1: 2})", {
            let mut m = Map::new();
            m.insert(1, 2);
            m.into()
        }),
        (r#"CBOR(1("t"))"#, CBOR::to_tagged_value(1, "t")),
    ];

    for (src, cbor) in cases {
        let p = Pattern::parse(src).unwrap();
        assert_eq!(p, Pattern::cbor(cbor.clone()));
        assert_eq!(p.to_string(), src);
    }
}

#[test]
fn parse_cbor_patterns_2() {
    bc_envelope::register_tags();

    let p = Pattern::parse("CBOR").unwrap();
    assert_eq!(p, Pattern::any_cbor());
    assert_eq!(p.to_string(), "CBOR");

    let p = Pattern::parse("CBOR(true)").unwrap();
    assert_eq!(p, Pattern::cbor(true));
    assert_eq!(p.to_string(), "CBOR(true)");

    let p = Pattern::parse("CBOR([1, 2, 3])").unwrap();
    assert_eq!(p, Pattern::cbor(vec![1, 2, 3]));
    assert_eq!(p.to_string(), "CBOR([1, 2, 3])");

    let p = Pattern::parse(r#"CBOR({"a": 1})"#).unwrap();
    let mut map = Map::new();
    map.insert("a", 1);
    assert_eq!(p, Pattern::cbor(map.clone()));
    assert_eq!(p.to_string(), r#"CBOR({"a": 1})"#);

    let p = Pattern::parse(r#"CBOR(1("hi"))"#).unwrap();
    assert_eq!(p, Pattern::cbor(CBOR::to_tagged_value(1, "hi")));
    assert_eq!(p.to_string(), r#"CBOR(1("hi"))"#);

    let date = dcbor::Date::from_ymd(2025, 5, 15);
    let ur = date.ur_string();
    let expr = format!(r#"CBOR({})"#, ur);
    let p = Pattern::parse(&expr).unwrap();
    assert_eq!(p, Pattern::cbor(date.clone()));
    assert_eq!(
        p.to_string(),
        format!("CBOR({})", date.to_cbor().diagnostic_flat())
    );
}
