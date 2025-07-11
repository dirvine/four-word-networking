//! Test for fc00:: duplication bug at the higher level

use three_word_networking::ThreeWordAdaptiveEncoder;

fn main() {
    println!("Testing fc00:: duplication bug fix at higher level...");
    println!("{}", "=".repeat(60));

    let encoder = ThreeWordAdaptiveEncoder::new().expect("Failed to create encoder");

    // Test cases that were failing
    let test_addresses = vec![
        "fc00::",
        "fc00::1", 
        "fc01::",
        "fd00::",
        "fd01::",
        "fc00:1234:5678:9abc::",
        "fd00:1234:5678:9abc::",
    ];

    for addr_str in test_addresses {
        println!("\nTesting: {}", addr_str);
        println!("{}", "-".repeat(40));

        // Parse the address
        let addr_with_port = format!("[{}]:443", addr_str);

        // Encode
        match encoder.encode(&addr_with_port) {
            Ok(encoded) => {
                println!("Encoded: {}", encoded);
                let word_count = encoded.split(' ').count();
                println!("Word count: {}", word_count);

                // Decode
                match encoder.decode(&encoded) {
                    Ok(decoded) => {
                        println!("Decoded: {}", decoded);
                        
                        // Check if it matches (accounting for interface ID loss)
                        let expected_prefix = if addr_str.contains("::") && !addr_str.ends_with("::") {
                            // For addresses like fc00::1, we expect to lose the interface ID
                            let prefix = addr_str.split("::").next().unwrap();
                            format!("[{}::]:443", prefix)
                        } else {
                            addr_with_port.clone()
                        };
                        
                        if decoded == addr_with_port {
                            println!("✓ Perfect match!");
                        } else if decoded == expected_prefix {
                            println!("✓ Expected prefix match (interface ID lost by design)");
                        } else {
                            println!("✗ MISMATCH!");
                            println!("  Expected: {} or {}", addr_with_port, expected_prefix);
                            println!("  Got:      {}", decoded);
                            
                            // Check for the duplication bug
                            if decoded.contains("fc00:fc00::") || decoded.contains("fd00:fd00::") {
                                println!("  ERROR: Duplication bug detected!");
                            }
                        }
                    }
                    Err(e) => {
                        println!("✗ Decode error: {}", e);
                    }
                }
            }
            Err(e) => {
                println!("✗ Encode error: {}", e);
            }
        }
    }

    println!("\n{}", "=".repeat(60));
    println!("Test complete.");
}
