use bc_envelope_pattern::{Pattern, parse_pattern};

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
