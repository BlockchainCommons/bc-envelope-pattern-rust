use dcbor::prelude::*;
use bc_envelope_pattern::{DCBORPattern, DCBORMatcher};

fn main() {
    // Test if dcbor-pattern captures work directly
    let cbor = 42.to_cbor();

    // Create a dcbor pattern with a capture
    let pattern = DCBORPattern::parse("@inner_num(42)").unwrap();

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
