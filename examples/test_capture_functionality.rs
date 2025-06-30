use bc_envelope::prelude::*;
use bc_envelope_pattern::{Pattern, Matcher, format_paths};

fn main() {
    println!("Testing CBOR pattern captures...");

    // Create test data - an envelope containing CBOR with a number
    let cbor = CBOR::from(42);
    let envelope = Envelope::new(cbor);
    println!("Test envelope: {}", envelope.format());

    // Create a CBOR pattern with a named capture
    let cbor_pattern_str = "/@name(number)";
    let pattern = Pattern::cbor(cbor_pattern_str);

    println!("CBOR Pattern: {}", cbor_pattern_str);
    println!("Pattern: {}", pattern);

    // Test if it matches
    let matches = pattern.matches(&envelope);
    println!("Matches: {}", matches);

    // Test paths_with_captures - this should now return captures from dcbor
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

    // Test with a more complex structure
    let array_cbor = CBOR::from(vec![1, 2, 3]);
    let array_envelope = Envelope::new(array_cbor);

    println!("\n--- Testing array pattern ---");
    println!("Array envelope: {}", array_envelope.format());

    let array_pattern_str = "/@arr(ARRAY)";
    let array_pattern = Pattern::cbor(array_pattern_str);

    println!("CBOR Pattern: {}", array_pattern_str);

    let array_matches = array_pattern.matches(&array_envelope);
    println!("Matches: {}", array_matches);

    let (array_paths, array_captures) = array_pattern.paths_with_captures(&array_envelope);

    println!("Paths found: {}", array_paths.len());
    println!("Captures found: {}", array_captures.len());
    for (name, capture_paths) in &array_captures {
        println!("Capture '{}': {} paths", name, capture_paths.len());
    }
}
