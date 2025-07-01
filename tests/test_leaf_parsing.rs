#[cfg(test)]
mod test_leaf_parsing {
    use bc_envelope_pattern::Pattern;

    #[test]
    fn test_leaf_pattern_parse() {
        let pattern = Pattern::parse("LEAF").unwrap();
        println!("Parsed LEAF pattern: {:?}", pattern);

        // The pattern should be a structure pattern now
        match pattern {
            Pattern::Structure(_) => println!(
                "✓ LEAF is correctly categorized as a structure pattern"
            ),
            Pattern::Leaf(_) => {
                panic!("✗ LEAF should not be a leaf pattern anymore")
            }
            Pattern::Meta(_) => panic!("✗ LEAF should not be a meta pattern"),
        }

        // Test that it displays as "LEAF"
        assert_eq!(pattern.to_string(), "LEAF");
    }
}
