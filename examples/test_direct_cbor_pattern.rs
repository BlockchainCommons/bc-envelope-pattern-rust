use bc_envelope::Envelope;
use bc_envelope_pattern::{Pattern, Matcher};
use dcbor_pattern::Pattern as DcborPattern;

fn main() {
    println!("Testing direct CBORPattern capture behavior...");

    // Create a dcbor pattern with capture
    let dcbor_pattern = DcborPattern::parse("@num(NUMBER(42))").unwrap();

    // Create a CBORPattern directly (bypass the top-level Pattern and VM)
    let cbor_pattern = bc_envelope_pattern::pattern::leaf::CBORPattern::pattern(dcbor_pattern);

    // Test it directly against an envelope
    let envelope = Envelope::new(42);
    let (paths, captures) = cbor_pattern.paths_with_captures(&envelope);

    println!("Direct CBORPattern test:");
    println!("  Paths: {}", paths.len());
    println!("  Captures: {:?}", captures);
    println!("  Paths details: {:?}", paths);
}
