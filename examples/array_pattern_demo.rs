use bc_envelope::prelude::*;
use bc_envelope_pattern::{Matcher, Pattern};

fn main() {
    println!("Array Pattern Demo - New dcbor-pattern Syntax");
    println!("==============================================");

    // Create test arrays
    let array_empty = Vec::<i32>::new().to_cbor();
    let array_three = vec![1, 2, 3].to_cbor();
    let array_five = vec![1, 2, 3, 4, 5].to_cbor();
    let array_strings = vec!["a", "b", "c"].to_cbor();

    // Test patterns
    let patterns = vec![
        ("[*]", "matches any array"),
        ("[{3}]", "matches arrays with exactly 3 elements"),
        ("[{2,4}]", "matches arrays with 2-4 elements"),
        ("[{3,}]", "matches arrays with at least 3 elements"),
    ];

    let test_data = vec![
        (Envelope::new(array_empty), "empty array []"),
        (Envelope::new(array_three), "three elements [1, 2, 3]"),
        (Envelope::new(array_five), "five elements [1, 2, 3, 4, 5]"),
        (
            Envelope::new(array_strings),
            "strings [\"a\", \"b\", \"c\"]",
        ),
    ];

    for (pattern_str, description) in patterns {
        println!("\nPattern: {} ({})", pattern_str, description);
        println!("-----");

        match Pattern::parse(pattern_str) {
            Ok(pattern) => {
                for (envelope, data_desc) in &test_data {
                    let matches = !pattern.paths(envelope).is_empty();
                    println!(
                        "  {} against {}: {}",
                        pattern_str,
                        data_desc,
                        if matches {
                            "✓ matches"
                        } else {
                            "✗ no match"
                        }
                    );
                }
            }
            Err(e) => {
                println!("  Error parsing pattern: {}", e);
            }
        }
    }
}
