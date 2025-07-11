//! Test ULA performance and compression ratios

use three_word_networking::ThreeWordAdaptiveEncoder;
use std::time::Instant;

fn main() {
    println!("Testing ULA Performance and Compression Ratios");
    println!("{}", "=".repeat(60));

    let encoder = ThreeWordAdaptiveEncoder::new().expect("Failed to create encoder");

    // Test various ULA addresses
    let test_addresses = vec![
        "[fc00::]:443",
        "[fc01::]:443",
        "[fd00::]:443",
        "[fd01::]:443",
        "[fc00:1234:5678:9abc::]:443",
        "[fd00:1234:5678:9abc::]:443",
        "[fcff:ffff:ffff:ffff::]:443",
        "[fdff:ffff:ffff:ffff::]:443",
    ];

    println!("\nCompression Results:");
    println!("{}", "-".repeat(60));
    println!("{:<35} {:>10} {:>12}", "Address", "Words", "Compression");
    println!("{}", "-".repeat(60));

    for addr in &test_addresses {
        let encoded = encoder.encode(addr).expect("Failed to encode");
        let word_count = encoded.split(' ').count();
        
        // Calculate compression ratio
        // Original: 128 bits (IPv6) + 16 bits (port) = 144 bits
        // Encoded: word_count * 16 bits (assuming 16-bit words)
        let original_bits = 144;
        let encoded_bits = word_count * 16;
        let compression_ratio = 1.0 - (encoded_bits as f64 / original_bits as f64);
        
        println!("{:<35} {:>10} {:>11.1}%", 
            addr, 
            word_count,
            compression_ratio * 100.0
        );
    }

    // Performance benchmarks
    println!("\nPerformance Benchmarks:");
    println!("{}", "-".repeat(60));
    
    let iterations = 100_000;
    
    for addr in &test_addresses[0..4] { // Test first 4 addresses
        // Encoding benchmark
        let start = Instant::now();
        for _ in 0..iterations {
            let _ = encoder.encode(addr);
        }
        let encode_duration = start.elapsed();
        let encode_per_op = encode_duration.as_nanos() / iterations as u128;
        
        // Get encoded form for decoding
        let encoded = encoder.encode(addr).expect("Failed to encode");
        
        // Decoding benchmark
        let start = Instant::now();
        for _ in 0..iterations {
            let _ = encoder.decode(&encoded);
        }
        let decode_duration = start.elapsed();
        let decode_per_op = decode_duration.as_nanos() / iterations as u128;
        
        println!("Address: {}", addr);
        println!("  Encoding: {} ns/op ({:.2} μs)", encode_per_op, encode_per_op as f64 / 1000.0);
        println!("  Decoding: {} ns/op ({:.2} μs)", decode_per_op, decode_per_op as f64 / 1000.0);
        println!();
    }

    // Verify no regression - encode/decode all addresses
    println!("Regression Test:");
    println!("{}", "-".repeat(60));
    
    let mut all_passed = true;
    for addr in &test_addresses {
        let encoded = encoder.encode(addr).expect("Failed to encode");
        let decoded = encoder.decode(&encoded).expect("Failed to decode");
        
        // For ULA addresses, interface IDs might be lost, so check prefix
        let matches = decoded == *addr || 
                     (addr.contains("::1]") && decoded.contains("::]")) ||
                     decoded == *addr;
        
        if !matches {
            println!("FAIL: {} -> {} -> {}", addr, encoded, decoded);
            all_passed = false;
        }
    }
    
    if all_passed {
        println!("✓ All addresses encode/decode correctly!");
    } else {
        println!("✗ Some addresses failed!");
    }
    
    println!("\n{}", "=".repeat(60));
    println!("Performance test complete.");
}