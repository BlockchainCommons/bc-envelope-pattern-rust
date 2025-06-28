use bc_envelope::Envelope;
use bc_envelope_pattern::{Pattern, Matcher};
use dcbor_pattern::{Pattern as DcborPattern, Matcher as DcborMatcher};

fn main() {
    println!("Testing DCBOR capture detection...");

    // Create a dcbor pattern with capture
    let dcbor_pattern: DcborPattern = "@num(NUMBER(42))".try_into().unwrap();
    println!("DCBOR pattern: {}", dcbor_pattern);

    // Test it directly against a CBOR value
    let test_cbor = dcbor::CBOR::from(42u64);
    let (paths, captures) = dcbor_pattern.paths_with_captures(&test_cbor);
    println!("Direct DCBOR test - Paths: {}, Captures: {:?}", paths.len(), captures);

    // Test with null value (what the helper uses)
    let null_cbor = dcbor::CBOR::null();
    let (null_paths, null_captures) = dcbor_pattern.paths_with_captures(&null_cbor);
    println!("DCBOR with null - Paths: {}, Captures: {:?}", null_paths.len(), null_captures);

    // Now test the full envelope pattern
    let envelope = Envelope::new(42);
    let pattern = Pattern::cbor_pattern(dcbor_pattern);

    let (env_paths, env_captures) = pattern.paths_with_captures(&envelope);
    println!("Envelope pattern - Paths: {}, Captures: {:?}", env_paths.len(), env_captures);
}
