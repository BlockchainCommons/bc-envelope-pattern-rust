use bc_envelope::prelude::*;
use bc_envelope_pattern::{Matcher, Pattern};

fn create_test_envelopes() -> Vec<Envelope> {
    vec![
        // Pure leaf - just a CBOR value
        Envelope::new("hello"),
        // Known value leaf
        Envelope::new(known_values::KnownValue::new(42)),
        // Node with zero assertions - question: does this exist?
        // Let's try creating a subject without any assertions
        Envelope::new("subject"),
        // Node with one assertion
        {
            let mut env = Envelope::new("subject");
            env = env.add_assertion("key", "value");
            env
        },
        // Node with multiple assertions
        {
            let mut env = Envelope::new("subject");
            env = env.add_assertion("key1", "value1");
            env = env.add_assertion("key2", "value2");
            env
        },
        // Wrapped envelope (node containing a leaf)
        Envelope::new(Envelope::new("wrapped")),
    ]
}

#[test]
fn test_leaf_vs_node_zero_comparison() {
    let envelopes = create_test_envelopes();
    let descriptions = ["Pure leaf (hello)",
        "Known value leaf",
        "Node with 0 assertions?",
        "Node with 1 assertion",
        "Node with 2 assertions",
        "Wrapped envelope"];

    let leaf_pattern = Pattern::leaf();
    let node_zero_pattern = Pattern::node_with_assertions_count(0);

    println!("Comparison of LEAF vs NODE({{0}}) patterns:\n");

    for (envelope, desc) in envelopes.iter().zip(descriptions.iter()) {
        let leaf_matches = leaf_pattern.matches(envelope);
        let node_zero_matches = node_zero_pattern.matches(envelope);

        println!(
            "{}: LEAF={}, NODE({{0}})={}",
            desc, leaf_matches, node_zero_matches
        );

        // Additional debugging info
        println!("  is_leaf(): {}", envelope.is_leaf());
        println!("  is_known_value(): {}", envelope.is_known_value());
        println!("  is_node(): {}", envelope.is_node());
        println!("  assertions().len(): {}", envelope.assertions().len());
        println!();
    }
}

#[test]
fn test_structural_differences() {
    // Test specific cases where LEAF and NODE({0}) might differ

    // Case 1: Pure CBOR leaf
    let cbor_leaf = Envelope::new(42);

    // Case 2: Node with subject but no assertions - can we create this?
    let bare_node = Envelope::new(42);

    let leaf_pattern = Pattern::leaf();
    let node_zero_pattern = Pattern::node_with_assertions_count(0);

    println!("Structural differences test:");
    println!("CBOR leaf (42):");
    println!("  LEAF matches: {}", leaf_pattern.matches(&cbor_leaf));
    println!(
        "  NODE({{0}}) matches: {}",
        node_zero_pattern.matches(&cbor_leaf)
    );
    println!("  is_leaf(): {}", cbor_leaf.is_leaf());
    println!("  is_node(): {}", cbor_leaf.is_node());
    println!();

    println!("Bare node (subject=42, no assertions):");
    println!("  LEAF matches: {}", leaf_pattern.matches(&bare_node));
    println!(
        "  NODE({{0}}) matches: {}",
        node_zero_pattern.matches(&bare_node)
    );
    println!("  is_leaf(): {}", bare_node.is_leaf());
    println!("  is_node(): {}", bare_node.is_node());
    println!();
}

#[test]
fn test_envelope_structure_details() {
    // Let's understand the envelope structure better
    let cbor_leaf = Envelope::new(42);
    let bare_node = Envelope::new(42); // Same as cbor_leaf - no distinction in bc-envelope
    let node_with_assertion = {
        let mut env = Envelope::new(42);
        env = env.add_assertion("key", "value");
        env
    };

    println!("Envelope structure analysis:");

    println!("CBOR leaf:");
    println!("  Debug: {:?}", cbor_leaf);
    println!("  is_leaf(): {}", cbor_leaf.is_leaf());
    println!("  is_node(): {}", cbor_leaf.is_node());
    println!("  subject(): {:?}", cbor_leaf.subject());
    println!("  assertions().len(): {}", cbor_leaf.assertions().len());
    println!();

    println!("Bare node:");
    println!("  Debug: {:?}", bare_node);
    println!("  is_leaf(): {}", bare_node.is_leaf());
    println!("  is_node(): {}", bare_node.is_node());
    println!("  subject(): {:?}", bare_node.subject());
    println!("  assertions().len(): {}", bare_node.assertions().len());
    println!();

    println!("Node with assertion:");
    println!("  Debug: {:?}", node_with_assertion);
    println!("  is_leaf(): {}", node_with_assertion.is_leaf());
    println!("  is_node(): {}", node_with_assertion.is_node());
    println!("  subject(): {:?}", node_with_assertion.subject());
    println!(
        "  assertions().len(): {}",
        node_with_assertion.assertions().len()
    );
    println!();
}
