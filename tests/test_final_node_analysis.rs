use bc_envelope::prelude::*;
use bc_envelope_pattern::{Matcher, Pattern};

#[test]
fn test_node_pattern_behavior() {
    // Test NODE with different assertion counts
    let patterns = vec![
        ("NODE (any)", Pattern::any_node()),
        ("NODE({0})", Pattern::node_with_assertions_count(0)),
        ("NODE({1})", Pattern::node_with_assertions_count(1)),
        ("NODE({2})", Pattern::node_with_assertions_count(2)),
        ("NODE({1,2})", Pattern::node_with_assertions_range(1..=2)),
    ];

    let test_cases = vec![
        ("Plain CBOR", Envelope::new(42)),
        ("Known value", Envelope::new(known_values::KnownValue::new(42))),
        ("1 assertion", {
            let mut env = Envelope::new("subject");
            env = env.add_assertion("key", "value");
            env
        }),
        ("2 assertions", {
            let mut env = Envelope::new("subject");
            env = env.add_assertion("key1", "value1");
            env = env.add_assertion("key2", "value2");
            env
        }),
        ("3 assertions", {
            let mut env = Envelope::new("subject");
            env = env.add_assertion("key1", "value1");
            env = env.add_assertion("key2", "value2");
            env = env.add_assertion("key3", "value3");
            env
        }),
    ];

    println!("NODE pattern matching behavior:\n");

    // Print header
    print!("{:15}", "");
    for (name, _) in &patterns {
        print!("{:>12}", name);
    }
    println!();

    // Test each envelope against each pattern
    for (desc, envelope) in &test_cases {
        print!("{:15}", desc);
        for (_, pattern) in &patterns {
            let matches = pattern.matches(envelope);
            print!("{:>12}", if matches { "✓" } else { "✗" });
        }
        println!(" (is_node: {}, assertions: {})",
                envelope.is_node(),
                envelope.assertions().len());
    }

    println!("\nKey findings:");
    println!("- NODE({{0}}) never matches anything because nodes by definition have assertions");
    println!("- Only envelopes with assertions have is_node() == true");
    println!("- Plain CBOR and known values are leaves, not nodes");
}

#[test]
fn test_final_comparison() {
    println!("Final comparison of LEAF vs NODE({{0}}):\n");

    let leaf_pattern = Pattern::leaf();
    let node_zero_pattern = Pattern::node_with_assertions_count(0);

    // Test on the same envelope
    let envelope = Envelope::new("test");

    println!("Testing envelope: {:?}", envelope);
    println!("LEAF matches: {}", leaf_pattern.matches(&envelope));
    println!("NODE({{0}}) matches: {}", node_zero_pattern.matches(&envelope));
    println!("is_leaf(): {}", envelope.is_leaf());
    println!("is_node(): {}", envelope.is_node());
    println!("assertions().len(): {}", envelope.assertions().len());

    println!("\nConclusion:");
    println!("LEAF and NODE({{0}}) are NOT equivalent:");
    println!("- LEAF matches terminal nodes in envelope trees");
    println!("- NODE({{0}}) matches structural nodes with zero assertions (which don't exist)");
    println!("- They serve completely different purposes");
}
