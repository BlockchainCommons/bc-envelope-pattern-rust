use bc_envelope::Envelope;
use bc_envelope_pattern::{Matcher, Pattern};

fn main() {
    println!("Testing new dcbor-pattern BSTR syntax");

    // Test 1: Any byte string pattern
    let pattern = Pattern::parse("bstr").unwrap();
    println!("Pattern 'bstr' parsed: {}", pattern);

    // Create test envelope with byte string
    let bytes = vec![0x48, 0x65, 0x6c, 0x6c, 0x6f]; // "Hello"
    let envelope = Envelope::new(dcbor::CBOR::to_byte_string(bytes.clone()));

    let matches = pattern.matches(&envelope);
    println!("Pattern 'bstr' matches byte string envelope: {}", matches);

    // Test 2: Specific hex pattern
    let pattern = Pattern::parse("h'48656c6c6f'").unwrap();
    println!("Pattern 'h'48656c6c6f'' parsed: {}", pattern);

    let matches = pattern.matches(&envelope);
    println!(
        "Pattern 'h'48656c6c6f'' matches 'Hello' envelope: {}",
        matches
    );

    // Test 3: Binary regex pattern
    let pattern = Pattern::parse("h'/^He.*o$/'").unwrap();
    println!("Pattern 'h'/^He.*o$/' parsed: {}", pattern);

    let matches = pattern.matches(&envelope);
    println!(
        "Pattern 'h'/^He.*o$/' matches 'Hello' envelope: {}",
        matches
    );

    // Test 4: Test non-matching patterns
    let text_envelope = Envelope::new("Hello");
    let bstr_pattern = Pattern::parse("bstr").unwrap();
    let matches = bstr_pattern.matches(&text_envelope);
    println!("Pattern 'bstr' matches text envelope: {}", matches);
}
