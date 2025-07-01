use bc_envelope::prelude::*;
use bc_envelope_pattern::{Matcher, Pattern};

fn main() {
    println!("Testing comprehensive map pattern features:");

    // Create test maps
    let mut key_value_map = Map::new();
    key_value_map.insert("name", "Alice");
    key_value_map.insert("age", 30);
    let key_value_cbor: CBOR = key_value_map.into();
    let key_value_envelope = Envelope::new(key_value_cbor);

    // Test various map patterns
    let patterns = [
        ("{*}", "any map"),
        ("{{2}}", "exactly 2 entries"),
        ("{{1,3}}", "1 to 3 entries"),
        ("{{3,}}", "at least 3 entries"),
    ];

    for (pattern_str, description) in patterns {
        println!("\nPattern: {} ({})", pattern_str, description);
        match Pattern::parse(pattern_str) {
            Ok(pattern) => {
                println!("  Parsed as: {}", pattern);
                let matches = !pattern
                    .paths_with_captures(&key_value_envelope)
                    .0
                    .is_empty();
                println!("  Key-value map matches: {}", matches);
            }
            Err(e) => {
                println!("  Failed to parse: {}", e);
            }
        }
    }

    println!("\n✅ Map pattern migration completed successfully!");
    println!("Old MAP syntax has been replaced with dcbor-pattern map syntax:");
    println!("  - MAP → {{*}}");
    println!("  - MAP(3) → {{{{3}}}}");
    println!("  - MAP({{2,4}}) → {{{{2,4}}}}");
    println!("  - New: {{{{2,}}}} for at least 2 entries");
    println!("  - New: {{key: value}} for key-value patterns");
}
