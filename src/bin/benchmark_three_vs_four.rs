#!/usr/bin/env rust
//! Benchmark comparison between three-word and four-word encoding systems

use std::net::Ipv4Addr;
use std::time::Instant;
use three_word_networking::{ThreeWordAdaptiveEncoder, ThreeWordEncoder};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Three-Word vs Four-Word Encoding Benchmark");
    println!("==========================================");

    let three_encoder = ThreeWordEncoder::new()?;
    let adaptive_encoder = ThreeWordAdaptiveEncoder::new()?;

    // Test addresses
    let test_addresses = vec![
        (Ipv4Addr::new(192, 168, 1, 1), 443),
        (Ipv4Addr::new(127, 0, 0, 1), 80),
        (Ipv4Addr::new(8, 8, 8, 8), 53),
        (Ipv4Addr::new(10, 0, 0, 1), 22),
        (Ipv4Addr::new(172, 16, 0, 1), 8080),
    ];

    println!("\nComparison of encoding formats:");
    println!("Address                  | Direct Three-Word        | Adaptive Three-Word");
    println!("{}", "-".repeat(80));

    for (ip, port) in &test_addresses {
        let three_encoded = three_encoder.encode(*ip, *port)?;
        let address_str = format!("{ip}:{port}");
        let adaptive_encoded = adaptive_encoder.encode(&address_str)?;

        println!(
            "{:24} | {:24} | {}",
            address_str,
            three_encoded.to_string(),
            adaptive_encoded
        );
    }

    println!("\nPerformance Comparison:");
    println!("======================");

    // Benchmark parameters
    let iterations = 50_000;
    let test_ip = Ipv4Addr::new(203, 0, 113, 42);
    let test_port = 8080;
    let test_address = format!("{test_ip}:{test_port}");

    // Three-word encoding benchmark
    println!("\nThree-Word System:");
    let start = Instant::now();
    for _ in 0..iterations {
        let encoded = three_encoder.encode(test_ip, test_port)?;
        let words: Vec<&str> = encoded.words().iter().map(|s| s.as_str()).collect();
        let _decoded = three_encoder.decode(&words)?;
    }
    let three_time = start.elapsed();
    let three_ops_per_sec = iterations as f64 / three_time.as_secs_f64();
    let three_us_per_op = three_time.as_micros() as f64 / iterations as f64;

    println!("  Roundtrip: {three_ops_per_sec:.0} ops/sec ({three_us_per_op:.2}μs per operation)");

    // Adaptive three-word encoding benchmark
    println!("\nAdaptive Three-Word System:");
    let start = Instant::now();
    for _ in 0..iterations {
        let encoded = adaptive_encoder.encode(&test_address)?;
        let _decoded = adaptive_encoder.decode(&encoded)?;
    }
    let adaptive_time = start.elapsed();
    let adaptive_ops_per_sec = iterations as f64 / adaptive_time.as_secs_f64();
    let adaptive_us_per_op = adaptive_time.as_micros() as f64 / iterations as f64;

    println!(
        "  Roundtrip: {adaptive_ops_per_sec:.0} ops/sec ({adaptive_us_per_op:.2}μs per operation)"
    );

    // Comparison
    println!("\nPerformance Summary:");
    let speedup = three_ops_per_sec / adaptive_ops_per_sec;
    println!(
        "  Direct three-word is {:.2}x {} than adaptive",
        if speedup > 1.0 {
            speedup
        } else {
            1.0 / speedup
        },
        if speedup > 1.0 { "faster" } else { "slower" }
    );

    // Word count comparison
    println!("\nWord Count Analysis:");
    let three_word_count = 3;
    let four_word_count = 4;
    let word_reduction =
        (four_word_count - three_word_count) as f64 / four_word_count as f64 * 100.0;
    println!("  Three-word: {three_word_count} words");
    println!("  Four-word:  {four_word_count} words");
    println!("  Reduction:  {word_reduction:.1}% fewer words");

    // Dictionary size comparison
    println!("\nDictionary Size Analysis:");
    println!("  Three-word: 65,536 words (16 bits per word)");
    println!("  Four-word:  16,384 words (14 bits per word)");
    println!("  Coverage:   Three-word has 4x larger vocabulary");

    // Mathematical precision
    println!("\nMathematical Properties:");
    println!("  Three-word: 3 × 16 bits = 48 bits (perfect for IPv4+port)");
    println!("  Four-word:  4 × 14 bits = 56 bits (8 bits overhead for IPv4+port)");
    println!("  Efficiency: Three-word is exactly sized for IPv4+port");

    // Entropy analysis
    let three_entropy = 3.0 * 16.0; // 3 words × 16 bits
    let four_entropy = 4.0 * 14.0; // 4 words × 14 bits
    println!("\nEntropy Analysis:");
    println!("  Three-word: {three_entropy:.0} bits of entropy");
    println!("  Four-word:  {four_entropy:.0} bits of entropy");
    println!("  IPv4+port:  48 bits required");
    println!(
        "  Three-word perfect match, four-word has {:.0} bits overhead",
        four_entropy - 48.0
    );

    Ok(())
}
