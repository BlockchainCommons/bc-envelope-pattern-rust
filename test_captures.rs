use bc_envelope::prelude::*;
use bc_envelope_pattern::{Pattern, format_paths};

fn main() {
    // Test basic capture functionality
    let envelope = Envelope::new(42);

    // Create a capture pattern that wraps a number pattern
    let pattern = Pattern::capture("my_number", Pattern::number(42));

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
}
