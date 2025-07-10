#!/usr/bin/env rust
//! Test the three-word IPv4 encoder system
//!
//! This binary tests the new three-word encoding system for IPv4 addresses
//! with perfect reconstruction using a 65,536-word dictionary.

use std::net::Ipv4Addr;
use std::time::Instant;
use three_word_networking::{FourWordError, ThreeWordEncoder};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Three-Word IPv4 Encoder Test");
    println!("============================");

    let encoder = ThreeWordEncoder::new()?;
    println!("{}", encoder.dictionary_stats());
    println!();

    // Test basic encoding/decoding
    test_basic_functionality(&encoder)?;

    // Test edge cases
    test_edge_cases(&encoder)?;

    // Test Feistel diffusion
    test_feistel_diffusion(&encoder)?;

    // Performance test
    test_performance(&encoder)?;

    // Random address test
    test_random_addresses(&encoder)?;

    println!("\n✅ All tests passed! Three-word encoding is working correctly.");
    Ok(())
}

fn test_basic_functionality(encoder: &ThreeWordEncoder) -> Result<(), FourWordError> {
    println!("Testing basic functionality...");

    let test_cases = [
        (Ipv4Addr::new(192, 168, 1, 1), 443),
        (Ipv4Addr::new(127, 0, 0, 1), 80),
        (Ipv4Addr::new(8, 8, 8, 8), 53),
        (Ipv4Addr::new(10, 0, 0, 1), 22),
    ];

    for (ip, port) in test_cases {
        let encoded = encoder.encode(ip, port)?;
        println!("  {}:{} -> {}", ip, port, encoded.to_string());

        let words: Vec<&str> = encoded.words().iter().map(|s| s.as_str()).collect();
        let (decoded_ip, decoded_port) = encoder.decode(&words)?;

        assert_eq!(decoded_ip, ip, "IP mismatch for {ip}:{port}");
        assert_eq!(decoded_port, port, "Port mismatch for {ip}:{port}");
    }

    println!("✓ Basic functionality test passed\n");
    Ok(())
}

fn test_edge_cases(encoder: &ThreeWordEncoder) -> Result<(), FourWordError> {
    println!("Testing edge cases...");

    let edge_cases = [
        (Ipv4Addr::new(0, 0, 0, 0), 0),             // Minimum values
        (Ipv4Addr::new(255, 255, 255, 255), 65535), // Maximum values
        (Ipv4Addr::new(127, 0, 0, 1), 8080),        // Common localhost
        (Ipv4Addr::new(172, 16, 0, 1), 443),        // Private range
    ];

    for (ip, port) in edge_cases {
        let encoded = encoder.encode(ip, port)?;
        println!("  {}:{} -> {}", ip, port, encoded.to_string());

        let words: Vec<&str> = encoded.words().iter().map(|s| s.as_str()).collect();
        let (decoded_ip, decoded_port) = encoder.decode(&words)?;

        assert_eq!(decoded_ip, ip, "IP mismatch for edge case {ip}:{port}");
        assert_eq!(
            decoded_port, port,
            "Port mismatch for edge case {ip}:{port}"
        );
    }

    println!("✓ Edge cases test passed\n");
    Ok(())
}

fn test_feistel_diffusion(encoder: &ThreeWordEncoder) -> Result<(), FourWordError> {
    println!("Testing Feistel diffusion (adjacent IPs should have very different words)...");

    let base_ip = Ipv4Addr::new(192, 168, 1, 100);
    let port = 80;

    let encoded1 = encoder.encode(base_ip, port)?;

    // Test adjacent IP
    let adjacent_ip = Ipv4Addr::new(192, 168, 1, 101);
    let encoded2 = encoder.encode(adjacent_ip, port)?;

    println!("  {}:{} -> {}", base_ip, port, encoded1.to_string());
    println!("  {}:{} -> {}", adjacent_ip, port, encoded2.to_string());

    // Words should be completely different due to Feistel network
    assert_ne!(
        encoded1.words(),
        encoded2.words(),
        "Adjacent IPs should have different word encodings"
    );

    // But both should decode correctly
    let words1: Vec<&str> = encoded1.words().iter().map(|s| s.as_str()).collect();
    let words2: Vec<&str> = encoded2.words().iter().map(|s| s.as_str()).collect();

    let (decoded_ip1, decoded_port1) = encoder.decode(&words1)?;
    let (decoded_ip2, decoded_port2) = encoder.decode(&words2)?;

    assert_eq!(decoded_ip1, base_ip);
    assert_eq!(decoded_port1, port);
    assert_eq!(decoded_ip2, adjacent_ip);
    assert_eq!(decoded_port2, port);

    println!("✓ Feistel diffusion test passed (words are properly diffused)\n");
    Ok(())
}

fn test_performance(encoder: &ThreeWordEncoder) -> Result<(), FourWordError> {
    println!("Testing performance...");

    let test_ip = Ipv4Addr::new(203, 0, 113, 42);
    let test_port = 8080;
    let iterations = 10_000;

    // Test encoding performance
    let start = Instant::now();
    for _ in 0..iterations {
        let _encoded = encoder.encode(test_ip, test_port)?;
    }
    let encode_time = start.elapsed();
    let encode_per_sec = iterations as f64 / encode_time.as_secs_f64();

    // Test decoding performance
    let encoded = encoder.encode(test_ip, test_port)?;
    let words: Vec<&str> = encoded.words().iter().map(|s| s.as_str()).collect();

    let start = Instant::now();
    for _ in 0..iterations {
        let (_ip, _port) = encoder.decode(&words)?;
    }
    let decode_time = start.elapsed();
    let decode_per_sec = iterations as f64 / decode_time.as_secs_f64();

    println!(
        "  Encoding: {:.2} addresses/sec ({:.2}μs per address)",
        encode_per_sec,
        encode_time.as_micros() as f64 / iterations as f64
    );
    println!(
        "  Decoding: {:.2} addresses/sec ({:.2}μs per address)",
        decode_per_sec,
        decode_time.as_micros() as f64 / iterations as f64
    );

    // Performance should be reasonable (target: >1000 ops/sec)
    assert!(
        encode_per_sec > 1000.0,
        "Encoding performance too slow: {encode_per_sec:.2} ops/sec"
    );
    assert!(
        decode_per_sec > 1000.0,
        "Decoding performance too slow: {decode_per_sec:.2} ops/sec"
    );

    println!("✓ Performance test passed\n");
    Ok(())
}

fn test_random_addresses(encoder: &ThreeWordEncoder) -> Result<(), FourWordError> {
    println!("Testing random addresses...");

    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    // Use deterministic "random" addresses for reproducible tests
    let mut hasher = DefaultHasher::new();
    "test_seed".hash(&mut hasher);
    let mut seed = hasher.finish();

    let test_count = 100;
    for i in 0..test_count {
        // Generate pseudo-random IP and port
        seed = seed.wrapping_mul(1103515245).wrapping_add(12345);
        let ip_val = (seed >> 16) as u32;
        let port_val = (seed & 0xFFFF) as u16;

        let ip = Ipv4Addr::from(ip_val);
        let port = if port_val == 0 { 1 } else { port_val }; // Avoid port 0

        let encoded = encoder.encode(ip, port)?;
        let words: Vec<&str> = encoded.words().iter().map(|s| s.as_str()).collect();
        let (decoded_ip, decoded_port) = encoder.decode(&words)?;

        assert_eq!(decoded_ip, ip, "Random test {i}: IP mismatch");
        assert_eq!(decoded_port, port, "Random test {i}: Port mismatch");

        if i < 5 {
            println!(
                "  Random {}: {}:{} -> {}",
                i + 1,
                ip,
                port,
                encoded.to_string()
            );
        }
    }

    println!("✓ Random addresses test passed ({test_count} addresses tested)\n");
    Ok(())
}
