use bc_envelope::prelude::*;
use bc_envelope_pattern::{Matcher, Pattern, format_paths};

fn main() {
    // Test CBOR pattern with dcbor captures
    let envelope = Envelope::new(42);

    // Create a CBOR pattern with an inner dcbor capture
    let pattern = Pattern::parse("CBOR(/@inner_num(NUMBER(42))/)").unwrap();

    println!("Pattern: {}", pattern);
    println!("Envelope: {}", envelope.format());

    // Test if it matches
    println!("Matches: {}", pattern.matches(&envelope));

    // Test paths_with_captures
    let (paths, captures) = pattern.paths_with_captures(&envelope);

    println!("Paths found: {}", paths.len());
    if !paths.is_empty() {
        println!("Paths: {}", format_paths(&paths));
    }

    println!("Captures found: {}", captures.len());
    for (name, capture_paths) in &captures {
        println!("Capture '{}': {} paths", name, capture_paths.len());
        if !capture_paths.is_empty() {
            println!("  Paths: {}", format_paths(capture_paths));
        }
    }

    // Test nested: envelope capture wrapping CBOR capture
    println!("\n--- Testing nested captures ---");
    let nested_pattern = Pattern::capture(
        "envelope_level",
        Pattern::parse("CBOR(/@dcbor_level(NUMBER(42))/)").unwrap(),
    );

    println!("Nested Pattern: {}", nested_pattern);
    let (nested_paths, nested_captures) =
        nested_pattern.paths_with_captures(&envelope);

    println!("Nested Paths found: {}", nested_paths.len());
    println!("Nested Captures found: {}", nested_captures.len());
    for (name, capture_paths) in &nested_captures {
        println!("Nested Capture '{}': {} paths", name, capture_paths.len());
    }
}
