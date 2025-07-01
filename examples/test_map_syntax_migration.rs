use bc_envelope_pattern::Pattern;

fn main() {
    println!("Testing old MAP syntax vs new map syntax:");

    // Test old MAP syntax
    match Pattern::parse("MAP") {
        Ok(pattern) => {
            println!("Old 'MAP' syntax: {} ❌ (should fail)", pattern)
        }
        Err(e) => println!("Old 'MAP' syntax: Failed ✅ - {}", e),
    }

    // Test new map syntax
    match Pattern::parse("{*}") {
        Ok(pattern) => println!("New '{{*}}' syntax: {} ✅", pattern),
        Err(e) => println!("New '{{*}}' syntax: Failed ❌ - {}", e),
    }

    match Pattern::parse("{{3}}") {
        Ok(pattern) => println!("New '{{{{3}}}}' syntax: {} ✅", pattern),
        Err(e) => println!("New '{{{{3}}}}' syntax: Failed ❌ - {}", e),
    }
}
