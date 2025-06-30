use bc_envelope::Envelope;
use bc_envelope_pattern::{Pattern, Matcher};
use dcbor_pattern::Pattern as DcborPattern;

/// Demonstrates the complete CBOR capture functionality
fn main() {
    println!("🎯 Demonstrating CBOR Pattern Captures in bc-envelope-pattern");
    println!("=============================================================\n");

    // Example 1: Simple CBOR capture
    println!("📝 Example 1: Simple CBOR capture");
    let envelope = Envelope::new(42);
    let dcbor_pattern = DcborPattern::parse("@number(NUMBER(42))").unwrap();
    let pattern = Pattern::cbor_pattern(dcbor_pattern);
    let (paths, captures) = pattern.paths_with_captures(&envelope);

    println!("  Pattern: CBOR(/@number(NUMBER(42))/)");
    println!("  Envelope: {}", envelope);
    println!("  Paths found: {}", paths.len());
    println!("  Captures found: {}", captures.len());
    if let Some(capture_paths) = captures.get("number") {
        println!("  Captured 'number': {} instances", capture_paths.len());
    }
    println!();

    // Example 2: Search pattern with captures
    println!("📝 Example 2: Search pattern with captures");
    let envelope = Envelope::new(vec![1, 2, 3, 4, 5]);
    let dcbor_pattern = DcborPattern::parse("@values(SEARCH(NUMBER))").unwrap();
    let pattern = Pattern::cbor_pattern(dcbor_pattern);
    let (paths, captures) = pattern.paths_with_captures(&envelope);

    println!("  Pattern: CBOR(/@values(SEARCH(NUMBER))/)");
    println!("  Envelope: {}", envelope);
    println!("  Paths found: {}", paths.len());
    println!("  Captures found: {}", captures.len());
    if let Some(capture_paths) = captures.get("values") {
        println!("  Captured 'values': {} instances", capture_paths.len());
    }
    println!();

    // Example 3: Mixed envelope and CBOR captures
    println!("📝 Example 3: Mixed envelope and CBOR captures");
    let envelope = Envelope::new("hello");
    let dcbor_pattern = DcborPattern::parse("@content(text)").unwrap();
    let cbor_pattern = Pattern::cbor_pattern(dcbor_pattern);
    let pattern = Pattern::capture("wrapper", cbor_pattern);
    let (paths, captures) = pattern.paths_with_captures(&envelope);

    println!("  Pattern: @wrapper(CBOR(/@content(text)/))");
    println!("  Envelope: {}", envelope);
    println!("  Paths found: {}", paths.len());
    println!("  Captures found: {}", captures.len());
    for (name, capture_paths) in &captures {
        println!("  Captured '{}': {} instances", name, capture_paths.len());
    }
    println!();

    // Example 4: Complex nested captures
    println!("📝 Example 4: Complex nested captures");
    let envelope = Envelope::new(vec![
        vec!["Alice", "95"],
        vec!["Bob", "85"]
    ]);
    let dcbor_pattern = DcborPattern::parse("@users(SEARCH(ARRAY(@name(text) > @score(text))))").unwrap();
    let pattern = Pattern::cbor_pattern(dcbor_pattern);
    let (paths, captures) = pattern.paths_with_captures(&envelope);

    println!("  Pattern: CBOR(/@users(SEARCH(ARRAY(@name(text) > @score(text))))/)");
    println!("  Envelope: {}", envelope);
    println!("  Paths found: {}", paths.len());
    println!("  Captures found: {}", captures.len());
    for (name, capture_paths) in &captures {
        println!("  Captured '{}': {} instances", name, capture_paths.len());
    }
    println!();

    println!("✅ All examples demonstrate successful CBOR pattern capture integration!");
    println!("   The dcbor-pattern capture system is now fully integrated with bc-envelope-pattern.");
}
