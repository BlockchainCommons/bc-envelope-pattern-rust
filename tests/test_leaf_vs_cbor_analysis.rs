#[cfg(test)]
mod leaf_vs_cbor_analysis {
    use bc_envelope::prelude::*;
    use bc_envelope_pattern::{Matcher, Pattern};

    #[test]
    fn analyze_leaf_vs_cbor_differences() {
        // Test various envelope types
        let test_cases = vec![
            ("Text", Envelope::new("hello")),
            ("Number", Envelope::new(42)),
            ("Boolean", Envelope::new(true)),
            ("Null", Envelope::null()),
            ("Array", Envelope::new(vec![1, 2, 3])),
            ("Map", {
                let mut map = dcbor::Map::new();
                map.insert("key", "value");
                Envelope::new(map)
            }),
            (
                "KnownValue",
                Envelope::new(known_values::KnownValue::new(42)),
            ),
            ("Assertion", Envelope::new_assertion("predicate", "object")),
            ("Node with assertions", {
                let mut env = Envelope::new("subject");
                env = env.add_assertion("key1", "value1");
                env = env.add_assertion("key2", "value2");
                env
            }),
        ];

        let leaf_pattern = Pattern::any_leaf();
        let cbor_pattern = Pattern::any_cbor();

        println!("Comparing LEAF vs CBOR patterns:");
        println!("=================================");

        for (name, envelope) in test_cases {
            let leaf_matches = leaf_pattern.matches(&envelope);
            let cbor_matches = cbor_pattern.matches(&envelope);

            println!(
                "{:20} | LEAF: {:5} | CBOR: {:5} | Same: {}",
                name,
                leaf_matches,
                cbor_matches,
                leaf_matches == cbor_matches
            );

            // Get paths to see what they actually match
            let leaf_paths = leaf_pattern.paths(&envelope);
            let cbor_paths = cbor_pattern.paths(&envelope);

            if !leaf_paths.is_empty() || !cbor_paths.is_empty() {
                println!(
                    "                     | LEAF paths: {} | CBOR paths: {}",
                    leaf_paths.len(),
                    cbor_paths.len()
                );
            }

            // Check envelope properties
            println!(
                "                     | is_leaf: {} | is_known_value: {} | subject.as_leaf: {}",
                envelope.is_leaf(),
                envelope.is_known_value(),
                envelope.subject().as_leaf().is_some()
            );
        }
    }
}
