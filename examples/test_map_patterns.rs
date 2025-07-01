use bc_envelope::prelude::*;
use bc_envelope_pattern::{Matcher, Pattern};

fn main() {
    println!("Testing map patterns:");

    // Create test maps
    let empty_map_cbor: CBOR = Map::new().into();
    let empty_map_envelope = Envelope::new(empty_map_cbor);

    let mut single_map = Map::new();
    single_map.insert("key", "value");
    let single_map_cbor: CBOR = single_map.into();
    let single_map_envelope = Envelope::new(single_map_cbor);

    let mut triple_map = Map::new();
    triple_map.insert("a", 1);
    triple_map.insert("b", 2);
    triple_map.insert("c", 3);
    let triple_map_cbor: CBOR = triple_map.into();
    let triple_map_envelope = Envelope::new(triple_map_cbor);

    // Test patterns
    let patterns = [
        ("{*}", "any map"),
        ("{{0}}", "empty map"),
        ("{{1}}", "single entry map"),
        ("{{3}}", "three entry map"),
        ("{{1,3}}", "one to three entries"),
        ("{{2,}}", "at least two entries"),
    ];

    for (pattern_str, description) in patterns {
        println!("\nPattern: {} ({})", pattern_str, description);
        match Pattern::parse(pattern_str) {
            Ok(pattern) => {
                println!("  Parsed as: {}", pattern);

                println!(
                    "  Empty map matches: {}",
                    !pattern
                        .paths_with_captures(&empty_map_envelope)
                        .0
                        .is_empty()
                );
                println!(
                    "  Single map matches: {}",
                    !pattern
                        .paths_with_captures(&single_map_envelope)
                        .0
                        .is_empty()
                );
                println!(
                    "  Triple map matches: {}",
                    !pattern
                        .paths_with_captures(&triple_map_envelope)
                        .0
                        .is_empty()
                );
            }
            Err(e) => {
                println!("  Failed to parse: {}", e);
            }
        }
    }
}
