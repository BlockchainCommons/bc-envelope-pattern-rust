use bc_envelope::prelude::*;
use bc_envelope_pattern::{DCBORPattern, Pattern};
use known_values::KnownValue;
mod common;

#[test]
fn parse_bool_any() {
    let src = "bool";
    let p = Pattern::parse(src).unwrap();
    assert_eq!(p, Pattern::any_bool());
    assert_actual_expected!(p.to_string(), "bool");
}

#[test]
fn parse_bool_true() {
    let src = "true";
    let p = Pattern::parse(src).unwrap();
    assert_eq!(p, Pattern::bool(true));
    assert_actual_expected!(p.to_string(), "true");
}

#[test]
fn parse_bool_false() {
    let src = "false";
    let p = Pattern::parse(src).unwrap();
    assert_eq!(p, Pattern::bool(false));
    assert_actual_expected!(p.to_string(), "false");
}

#[test]
fn parse_text_dcbor_pattern_syntax() {
    // Test the new dcbor-pattern syntax
    let p = Pattern::parse("text").unwrap();
    assert_eq!(p, Pattern::any_text());
    assert_actual_expected!(p.to_string(), "text");

    let p = Pattern::parse(r#""hello""#).unwrap();
    assert_eq!(p, Pattern::text("hello"));
    assert_actual_expected!(p.to_string(), r#""hello""#);

    let p = Pattern::parse("/h.*o/").unwrap();
    let regex = regex::Regex::new("h.*o").unwrap();
    assert_eq!(p, Pattern::text_regex(regex));
    assert_actual_expected!(p.to_string(), "/h.*o/");

    // Test with various string content
    let p = Pattern::parse(r#""hello world""#).unwrap();
    assert_eq!(p, Pattern::text("hello world"));
    assert_actual_expected!(p.to_string(), r#""hello world""#);

    // Test with escaped quotes
    let p = Pattern::parse(r#""say \"hello\"""#).unwrap();
    assert_eq!(p, Pattern::text(r#"say "hello""#));
    assert_actual_expected!(p.to_string(), r#""say \"hello\"""#);
}

#[test]
fn parse_number_patterns() {
    // Test dcbor-pattern syntax
    let p = Pattern::parse("number").unwrap();
    assert_eq!(p, Pattern::any_number());
    assert_actual_expected!(p.to_string(), "number");

    let p = Pattern::parse("42").unwrap();
    assert_eq!(p, Pattern::number(42));
    assert_actual_expected!(p.to_string(), "42");

    let p = Pattern::parse("3.75").unwrap();
    assert_eq!(p, Pattern::number(3.75));
    assert_actual_expected!(p.to_string(), "3.75");

    let p = Pattern::parse("1...3").unwrap();
    assert_eq!(p, Pattern::number_range(1.0..=3.0));
    assert_actual_expected!(p.to_string(), "1...3");

    let p = Pattern::parse(">5").unwrap();
    assert_eq!(p, Pattern::number_greater_than(5));
    assert_actual_expected!(p.to_string(), ">5");

    let p = Pattern::parse(">=5").unwrap();
    assert_eq!(p, Pattern::number_greater_than_or_equal(5));
    assert_actual_expected!(p.to_string(), ">=5");

    let p = Pattern::parse("<5").unwrap();
    assert_eq!(p, Pattern::number_less_than(5));
    assert_actual_expected!(p.to_string(), "<5");

    let p = Pattern::parse("<=5").unwrap();
    assert_eq!(p, Pattern::number_less_than_or_equal(5));
    assert_actual_expected!(p.to_string(), "<=5");

    let p = Pattern::parse("NaN").unwrap();
    assert_eq!(p, Pattern::number_nan());
    assert_actual_expected!(p.to_string(), "NaN");

    let p = Pattern::parse("Infinity").unwrap();
    assert_eq!(p, Pattern::number(f64::INFINITY));
    // Note: dcbor-pattern displays infinity as "inf"
    assert_actual_expected!(p.to_string(), "inf");

    let p = Pattern::parse("-Infinity").unwrap();
    assert_eq!(p, Pattern::number(f64::NEG_INFINITY));
    // Note: dcbor-pattern displays negative infinity as "-inf"
    assert_actual_expected!(p.to_string(), "-inf");
}

#[test]
fn parse_leaf_pattern() {
    let p = Pattern::parse("leaf").unwrap();
    assert_eq!(p, Pattern::leaf());
    assert_actual_expected!(p.to_string(), "leaf");
}

#[test]
fn parse_array_patterns() {
    let p = Pattern::parse("array").unwrap();
    assert_eq!(p, Pattern::any_array());
    assert_actual_expected!(p.to_string(), "array");

    let p = Pattern::parse("[{3}]").unwrap();
    assert_eq!(p, Pattern::array_with_count(3));
    assert_actual_expected!(p.to_string(), "[{3}]");

    let p = Pattern::parse("[{2,4}]").unwrap();
    assert_eq!(p, Pattern::array_with_range(2..=4));
    assert_actual_expected!(p.to_string(), "[{2,4}]");

    let p = Pattern::parse("[{2,}]").unwrap();
    assert_eq!(p, Pattern::array_with_range(2..));
    assert_actual_expected!(p.to_string(), "[{2,}]");
}

#[test]
fn parse_bstr_patterns() {
    let p = Pattern::parse("bstr").unwrap();
    assert_eq!(p, Pattern::any_byte_string());
    assert_actual_expected!(p.to_string(), "bstr");

    let p = Pattern::parse("h'0102'").unwrap();
    assert_eq!(p, Pattern::byte_string(vec![1u8, 2]));
    assert_actual_expected!(p.to_string(), "h'0102'");

    let p = Pattern::parse("h'/abc/'").unwrap();
    let regex = regex::bytes::Regex::new("abc").unwrap();
    assert_eq!(p, Pattern::byte_string_binary_regex(regex));
    assert_actual_expected!(p.to_string(), "h'/abc/'");
}

#[test]
fn parse_date_patterns() {
    let p = Pattern::parse("date").unwrap();
    assert_eq!(p, Pattern::any_date());
    assert_actual_expected!(p.to_string(), "date");

    let p = Pattern::parse("date'2023-12-25'").unwrap();
    let d = Date::from_string("2023-12-25").unwrap();
    assert_eq!(p, Pattern::date(d.clone()));
    assert_actual_expected!(p.to_string(), "date'2023-12-25'");

    let p = Pattern::parse("date'2023-12-24...2023-12-26'").unwrap();
    let start = Date::from_string("2023-12-24").unwrap();
    let end = Date::from_string("2023-12-26").unwrap();
    assert_eq!(p, Pattern::date_range(start..=end));
    assert_actual_expected!(p.to_string(), "date'2023-12-24...2023-12-26'");

    let p = Pattern::parse("date'2023-12-24...'").unwrap();
    let start = Date::from_string("2023-12-24").unwrap();
    assert_eq!(p, Pattern::date_earliest(start.clone()));
    assert_actual_expected!(p.to_string(), "date'2023-12-24...'");

    let p = Pattern::parse("date'...2023-12-26'").unwrap();
    let end = Date::from_string("2023-12-26").unwrap();
    assert_eq!(p, Pattern::date_latest(end.clone()));
    assert_actual_expected!(p.to_string(), "date'...2023-12-26'");

    let p = Pattern::parse("date'/2023-.*/'").unwrap();
    let regex = regex::Regex::new("2023-.*").unwrap();
    assert_eq!(p, Pattern::date_regex(regex));
    assert_actual_expected!(p.to_string(), "date'/2023-.*/'");
}

#[test]
fn parse_map_patterns() {
    // dcbor-pattern map syntax - any map
    let p = Pattern::parse("map").unwrap();
    assert_actual_expected!(p.to_string(), "map");

    // dcbor-pattern map syntax - specific count
    let p = Pattern::parse("{{3}}").unwrap();
    assert_actual_expected!(p.to_string(), "{{3}}");

    // dcbor-pattern map syntax - range
    let p = Pattern::parse("{{2,4}}").unwrap();
    assert_actual_expected!(p.to_string(), "{{2,4}}");

    // dcbor-pattern map syntax - at least N
    let p = Pattern::parse("{{2,}}").unwrap();
    assert_actual_expected!(p.to_string(), "{{2,}}");
}

#[test]
fn parse_null_pattern() {
    let p = Pattern::parse("null").unwrap();
    assert_eq!(p, Pattern::null());
    assert_actual_expected!(p.to_string(), "null");
}

#[test]
fn parse_tag_patterns() {
    bc_envelope::register_tags();

    let p = Pattern::parse("tagged").unwrap();
    assert_actual_expected!(p.to_string(), "tagged");

    let p = Pattern::parse("tagged(100, [number, (number)*])").unwrap();
    assert_actual_expected!(p.to_string(), "tagged(100, [number, (number)*])");

    let p = Pattern::parse(r#"tagged(100, { "key": * })"#).unwrap();
    assert_actual_expected!(p.to_string(), r#"tagged(100, {"key": *})"#);

    let p = Pattern::parse("tagged(100, *)").unwrap();
    assert_actual_expected!(p.to_string(), "tagged(100, *)");

    let p = Pattern::parse("tagged(date, *)").unwrap();
    assert_actual_expected!(p.to_string(), "tagged(date, *)");

    let p = Pattern::parse("tagged(/da.*/, *)").unwrap();
    assert_actual_expected!(p.to_string(), "tagged(/da.*/, *)");

    // Test the new API methods
    let p = Pattern::any_tag();
    assert_actual_expected!(p.to_string(), "tagged");

    let p = Pattern::tagged(100, DCBORPattern::any());
    assert_actual_expected!(p.to_string(), "tagged(100, *)");

    let p = Pattern::tagged_name("date", DCBORPattern::any());
    assert_actual_expected!(p.to_string(), "tagged(date, *)");

    let regex = regex::Regex::new("da.*").unwrap();
    let p = Pattern::tagged_regex(regex, DCBORPattern::any());
    assert_actual_expected!(p.to_string(), "tagged(/da.*/, *)");
}

#[test]
fn parse_known_value_patterns() {
    let p = Pattern::parse("known").unwrap();
    assert_eq!(p, Pattern::any_known_value());
    assert_actual_expected!(p.to_string(), "known");

    let p = Pattern::parse("'1'").unwrap();
    assert_eq!(p, Pattern::known_value(KnownValue::new(1)));
    assert_actual_expected!(p.to_string(), "'1'");

    let p = Pattern::parse("'date'").unwrap();
    assert_eq!(p, Pattern::known_value_named("date"));
    assert_actual_expected!(p.to_string(), "'date'");

    let p = Pattern::parse("'/da.*/'").unwrap();
    let regex = regex::Regex::new("da.*").unwrap();
    assert_eq!(p, Pattern::known_value_regex(regex));
    assert_actual_expected!(p.to_string(), "'/da.*/'");
}

#[test]
fn parse_cbor_patterns() {
    let cases: Vec<(&str, CBOR)> = vec![
        ("cbor(true)", true.into()),
        ("cbor(42)", 42.into()),
        (r#"cbor("hello")"#, "hello".into()),
        ("cbor([1, 2])", vec![1, 2].into()),
        ("cbor({1: 2})", {
            let mut m = Map::new();
            m.insert(1, 2);
            m.into()
        }),
        (r#"cbor(1("t"))"#, CBOR::to_tagged_value(1, "t")),
    ];

    for (src, cbor) in cases {
        let p = Pattern::parse(src).unwrap();
        assert_eq!(p, Pattern::cbor(cbor.clone()));
        assert_actual_expected!(p.to_string(), src);
    }
}

#[test]
fn parse_cbor_patterns_2() {
    bc_envelope::register_tags();

    let p = Pattern::parse("cbor").unwrap();
    assert_eq!(p, Pattern::any_cbor());
    assert_actual_expected!(p.to_string(), "cbor");

    let p = Pattern::parse("cbor(true)").unwrap();
    assert_eq!(p, Pattern::cbor(true));
    assert_actual_expected!(p.to_string(), "cbor(true)");

    let p = Pattern::parse("cbor([1, 2, 3])").unwrap();
    assert_eq!(p, Pattern::cbor(vec![1, 2, 3]));
    assert_actual_expected!(p.to_string(), "cbor([1, 2, 3])");

    let p = Pattern::parse(r#"cbor({"a": 1})"#).unwrap();
    let mut map = Map::new();
    map.insert("a", 1);
    assert_eq!(p, Pattern::cbor(map.clone()));
    assert_actual_expected!(p.to_string(), r#"cbor({"a": 1})"#);

    let p = Pattern::parse(r#"cbor(1("hi"))"#).unwrap();
    assert_eq!(p, Pattern::cbor(CBOR::to_tagged_value(1, "hi")));
    assert_actual_expected!(p.to_string(), r#"cbor(1("hi"))"#);

    let date = Date::from_ymd(2025, 5, 15);
    let ur = date.ur_string();
    let expr = format!(r#"cbor({})"#, ur);
    let p = Pattern::parse(&expr).unwrap();
    assert_eq!(p, Pattern::cbor(date.clone()));
    assert_actual_expected!(
        p.to_string(),
        format!("cbor({})", date.to_cbor().diagnostic_flat())
    );
}
