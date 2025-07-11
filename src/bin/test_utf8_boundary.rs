//! Test UTF-8 boundary handling in address parsing

use three_word_networking::ThreeWordAdaptiveEncoder;

fn main() {
    println!("Testing UTF-8 Boundary Handling");
    println!("{}", "=".repeat(60));
    
    let encoder = ThreeWordAdaptiveEncoder::new().expect("Failed to create encoder");
    
    // Test cases that might trigger UTF-8 boundary issues
    let test_cases = vec![
        // Normal cases
        "[::1]:443",
        "[2001:db8::1]:80",
        
        // UTF-8 characters near bracket boundaries
        "[::1]🚀:443",  // Emoji after bracket
        "[::1]é:443",   // Accented character after bracket
        "[::1]中:443",  // Chinese character after bracket
        "[::1]]:443",   // Double bracket
        
        // Invalid formats with UTF-8
        "[::1]🚀",      // No port after emoji
        "[::1]:",       // Missing port number
        "[::1]:🚀",     // Emoji as port
        
        // Edge cases
        "[",            // Incomplete bracket
        "[::1",          // Missing closing bracket
        "[::1]",        // No port specifier
    ];
    
    println!("\nTesting address parsing with UTF-8 edge cases:");
    println!("{}", "-".repeat(60));
    
    for test in test_cases {
        print!("Testing '{}': ", test);
        
        match encoder.encode(test) {
            Ok(encoded) => {
                println!("✓ Encoded to: {}", encoded);
                
                // Try to decode it back
                match encoder.decode(&encoded) {
                    Ok(decoded) => println!("  Decoded back to: {}", decoded),
                    Err(e) => println!("  ✗ Decode error: {}", e),
                }
            }
            Err(e) => {
                println!("✗ Error: {}", e);
            }
        }
        println!();
    }
    
    // Test specific UTF-8 boundary issue
    println!("\nTesting specific UTF-8 boundary scenarios:");
    println!("{}", "-".repeat(60));
    
    // Create a string where the closing bracket is followed by a multibyte character
    let tricky_cases = vec![
        "[::1]中国:443",    // Multibyte chars between ] and :
        "[::1]🎯:443",     // 4-byte emoji between ] and :
        "[::1]ñ:443",      // 2-byte character
        "[2001:db8::café]:443", // Accent in the address itself
    ];
    
    for test in tricky_cases {
        print!("Testing '{}': ", test);
        match encoder.encode(test) {
            Ok(encoded) => println!("✓ Success: {}", encoded),
            Err(e) => println!("✗ Error: {}", e),
        }
    }
    
    println!("\n{}", "=".repeat(60));
    println!("UTF-8 boundary test complete.");
}