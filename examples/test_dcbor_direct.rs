use dcbor::prelude::*;
use dcbor_pattern::{Matcher as DcborMatcher, Pattern as DcborPattern};

fn main() {
    // Test if dcbor-pattern captures work directly
    let cbor = 42.to_cbor();

    // Create a dcbor pattern with a capture
    let pattern = DcborPattern::parse("@inner_num(NUMBER(42))").unwrap();

    println!("DCBOR Pattern: {}", pattern);
    println!("CBOR: {:?}", cbor);

    // Test if it matches
    println!("Matches: {}", pattern.matches(&cbor));

    // Test paths_with_captures directly
    let (paths, captures) = pattern.paths_with_captures(&cbor);

    println!("Paths found: {}", paths.len());
    println!("Captures found: {}", captures.len());
    for (name, capture_paths) in &captures {
        println!("Capture '{}': {} paths", name, capture_paths.len());
    }
}
