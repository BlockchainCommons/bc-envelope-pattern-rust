use bc_envelope_pattern::Pattern;

#[test]
fn parse_any() {
    let src = "*";
    let p = Pattern::parse(src).unwrap();
    assert_eq!(p, Pattern::any());
    assert_eq!(p.to_string(), src);
}

#[test]
fn parse_none() {
    let src = "NONE";
    let p = Pattern::parse(src).unwrap();
    assert_eq!(p, Pattern::none());
    assert_eq!(p.to_string(), src);
}
