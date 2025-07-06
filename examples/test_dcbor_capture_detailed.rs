use bc_envelope::prelude::*;
use bc_envelope_pattern::{DCBORMatcher, DCBORPattern, Matcher, Pattern};

fn main() {
    println!("Testing dcbor capture integration in detail...\n");

    // Test the dcbor pattern directly first
    let dcbor_pattern_str = "@inner_num(42)";
    let dcbor_pattern = DCBORPattern::parse(dcbor_pattern_str).unwrap();

    println!("Created dcbor pattern: {}", dcbor_pattern);

    // Test the dcbor pattern directly on CBOR
    let test_cbor = dcbor::CBOR::from(42);
    let (dcbor_paths, dcbor_captures) =
        dcbor_pattern.paths_with_captures(&test_cbor);

    println!("Direct dcbor test:");
    println!("  CBOR: {}", test_cbor.diagnostic_flat());
    println!("  Paths found: {}", dcbor_paths.len());
    println!("  Captures found: {}", dcbor_captures.len());
    for (name, paths) in &dcbor_captures {
        println!("    '{}': {} paths", name, paths.len());
        for (i, path) in paths.iter().enumerate() {
            println!(
                "      Path {}: {:?}",
                i,
                path.iter().map(|c| c.diagnostic_flat()).collect::<Vec<_>>()
            );
        }
    }
    println!();

    // Now test through bc-envelope-pattern
    let pattern_str = "CBOR(/@inner_num(42)/)";
    let envelope_pattern = Pattern::parse(pattern_str).unwrap();
    let envelope = Envelope::new(42);

    println!("Testing through bc-envelope-pattern:");
    println!("  Pattern string: {}", pattern_str);
    println!("  Parsed pattern: {}", envelope_pattern);
    println!("  Envelope: {}", envelope.format());

    let (envelope_paths, envelope_captures) =
        envelope_pattern.paths_with_captures(&envelope);

    println!("  Paths found: {}", envelope_paths.len());
    println!("  Captures found: {}", envelope_captures.len());
    for (name, paths) in &envelope_captures {
        println!("    '{}': {} paths", name, paths.len());
        for (i, path) in paths.iter().enumerate() {
            println!(
                "      Path {}: {}",
                i,
                path.iter()
                    .map(|e| e.digest().to_string())
                    .collect::<Vec<_>>()
                    .join(" -> ")
            );
        }
    }

    // Test a non-matching case
    println!("\n--- Testing non-matching case ---");
    let non_match_envelope = Envelope::new("hello");
    let (non_match_paths, non_match_captures) =
        envelope_pattern.paths_with_captures(&non_match_envelope);
    println!("Non-matching envelope: {}", non_match_envelope.format());
    println!("Paths found: {}", non_match_paths.len());
    println!("Captures found: {}", non_match_captures.len());
}
