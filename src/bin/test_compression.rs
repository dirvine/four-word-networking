//! Test compression capabilities and demonstrate efficiency

use three_word_networking::{CompressedEncoder, CompressionStats};

fn test_address(encoder: &CompressedEncoder, address: &str) {
    println!("\nTesting: {}", address);
    println!("─────────────────────────────────");
    
    match encoder.compression_stats(address) {
        Ok(stats) => {
            println!("{}", stats.summary());
            
            if stats.fits_in_three_words {
                match encoder.encode(address) {
                    Ok(words) => {
                        println!("✓ Three words: {}", words);
                        
                        // Test round-trip
                        match encoder.decode(&words) {
                            Ok(decoded) => {
                                if decoded == address {
                                    println!("✓ Round-trip successful");
                                } else {
                                    println!("✗ Round-trip failed: {} != {}", decoded, address);
                                }
                            }
                            Err(e) => println!("✗ Decode error: {}", e),
                        }
                    }
                    Err(e) => println!("✗ Encode error: {}", e),
                }
            } else {
                println!("✗ Cannot encode in three words (needs {} bits)", stats.compressed_bits);
            }
        }
        Err(e) => println!("✗ Compression analysis error: {}", e),
    }
}

fn main() {
    println!("Three-Word Networking: Advanced Compression Test");
    println!("================================================");
    
    let encoder = match CompressedEncoder::new() {
        Ok(e) => e,
        Err(e) => {
            eprintln!("Failed to create encoder: {}", e);
            return;
        }
    };
    
    println!("\n## Private Networks (Should all compress well)");
    
    // Localhost addresses
    test_address(&encoder, "127.0.0.1:80");
    test_address(&encoder, "127.0.0.1:443");
    test_address(&encoder, "127.0.0.1:8080");
    test_address(&encoder, "127.0.0.100:22");
    
    // 192.168.x.x addresses
    test_address(&encoder, "192.168.1.1:80");
    test_address(&encoder, "192.168.1.100:443");
    test_address(&encoder, "192.168.255.255:8080");
    test_address(&encoder, "192.168.0.1");  // No port
    
    // 10.x.x.x addresses
    test_address(&encoder, "10.0.0.1:22");
    test_address(&encoder, "10.1.2.3:80");
    test_address(&encoder, "10.255.255.255:443");
    
    // 172.16-31.x.x addresses
    test_address(&encoder, "172.16.0.1:80");
    test_address(&encoder, "172.20.1.1:443");
    test_address(&encoder, "172.31.255.255:22");
    
    println!("\n## IPv6 Addresses (Limited support)");
    
    // IPv6 localhost
    test_address(&encoder, "[::1]:80");
    test_address(&encoder, "[::1]:443");
    test_address(&encoder, "::1");  // No port
    
    // IPv6 link-local (partial support)
    test_address(&encoder, "[fe80::1]:80");
    
    println!("\n## Public IP Addresses (Should fail - too many bits)");
    
    // Public IPv4 addresses
    test_address(&encoder, "8.8.8.8:53");
    test_address(&encoder, "1.1.1.1:443");
    test_address(&encoder, "93.184.216.34:80");  // example.com
    
    println!("\n## Edge Cases");
    
    // Uncommon ports
    test_address(&encoder, "192.168.1.1:12345");
    test_address(&encoder, "10.0.0.1:65535");
    
    // Addresses at boundaries
    test_address(&encoder, "172.15.255.255:80");  // Just outside 172.16-31
    test_address(&encoder, "172.32.0.0:80");      // Just outside 172.16-31
    
    println!("\n## Summary");
    println!("═══════════════════════════════════════════════════");
    println!("✓ Private networks compress to fit in 3 words");
    println!("✓ Common ports (80, 443, 22, etc.) use only 4 bits");
    println!("✓ IPv6 localhost is supported");
    println!("✗ Public IPv4 addresses need 35+ bits (too large)");
    println!("✗ Most IPv6 addresses need 36+ bits (too large)");
    println!("\nRecommendation: Use for private networks and local addresses");
}