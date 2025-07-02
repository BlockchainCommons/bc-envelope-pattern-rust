use bc_envelope_pattern::Pattern;

#[test]
fn parse_any() {
    let src = "*";
    let p = Pattern::parse(src).unwrap();
    assert_eq!(p, Pattern::any());
    assert_eq!(p.to_string(), src);
}
