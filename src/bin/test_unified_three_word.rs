#!/usr/bin/env rust
//! Test the unified three-word encoding system for both IPv4 and IPv6

use std::net::SocketAddr;
use std::str::FromStr;
use three_word_networking::UnifiedThreeWordEncoder;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Unified Three-Word Encoding System Test");
    println!("=======================================");
    println!("IPv4: 3 words (1 group)");
    println!("IPv6: 6 words (2 groups) or 9 words (3 groups)");
    println!();

    let encoder = UnifiedThreeWordEncoder::new()?;

    // Test IPv4 addresses
    test_ipv4_addresses(&encoder)?;

    // Test IPv6 addresses
    test_ipv6_addresses(&encoder)?;

    // Test roundtrip accuracy
    test_roundtrip_accuracy(&encoder)?;

    // Test string parsing
    test_string_parsing(&encoder)?;

    // Display formatting examples
    display_formatting_examples(&encoder)?;

    println!("\n✅ All tests passed! Unified three-word encoding is working correctly.");
    Ok(())
}

fn test_ipv4_addresses(
    encoder: &UnifiedThreeWordEncoder,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("Testing IPv4 Addresses (3 words):");
    println!("---------------------------------");

    let ipv4_addresses = [
        "192.168.1.1:443",
        "127.0.0.1:80",
        "8.8.8.8:53",
        "10.0.0.1:22",
        "172.16.0.1:8080",
    ];

    for addr_str in &ipv4_addresses {
        let addr = SocketAddr::from_str(addr_str)?;
        let encoded = encoder.encode(addr)?;

        println!("  {} → {}", addr_str, encoded.to_string());
        println!("    Type: {}", encoded.description());

        // Verify it's 3 words
        assert_eq!(encoded.word_count(), 3);
        assert_eq!(encoded.group_count(), 1);

        // Test roundtrip
        let decoded = encoder.decode(&encoded)?;
        assert_eq!(decoded, addr);
    }

    println!("✓ IPv4 encoding test passed\n");
    Ok(())
}

fn test_ipv6_addresses(
    encoder: &UnifiedThreeWordEncoder,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("Testing IPv6 Addresses (6 or 9 words):");
    println!("--------------------------------------");

    let ipv6_addresses = [
        ("[::1]:80", "Loopback"),
        ("[::]:443", "Unspecified"),
        ("[fe80::1]:22", "Link-local"),
        ("[2001:db8::1]:443", "Documentation"),
        ("[2001:db8:85a3::8a2e:370:7334]:8080", "Complex"),
        ("[2001:4860:4001:801::1]:443", "Google"),
    ];

    for (addr_str, description) in &ipv6_addresses {
        let addr = SocketAddr::from_str(addr_str)?;
        let encoded = encoder.encode(addr)?;

        println!("  {addr_str} ({description})");
        println!("    → {}", encoded.to_string());
        println!("    Type: {}", encoded.description());
        println!(
            "    Groups: {} ({} words)",
            encoded.group_count(),
            encoded.word_count()
        );

        // Verify it's a multiple of 3
        assert_eq!(encoded.word_count() % 3, 0);

        // Test roundtrip
        let decoded = encoder.decode(&encoded)?;
        assert_eq!(decoded, addr);
    }

    println!("✓ IPv6 encoding test passed\n");
    Ok(())
}

fn test_roundtrip_accuracy(
    encoder: &UnifiedThreeWordEncoder,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("Testing Roundtrip Accuracy:");
    println!("---------------------------");

    let test_addresses = [
        "0.0.0.0:0",
        "255.255.255.255:65535",
        "[::]:0",
        "[ffff:ffff:ffff:ffff:ffff:ffff:ffff:ffff]:65535",
        "[2001:db8:1234:5678:90ab:cdef:1234:5678]:12345",
    ];

    let mut passed = 0;
    let mut failed = 0;

    for addr_str in &test_addresses {
        let addr = SocketAddr::from_str(addr_str)?;
        let encoded = encoder.encode(addr)?;
        let decoded = encoder.decode(&encoded)?;

        if decoded == addr {
            passed += 1;
            println!("  ✓ {} ({})", addr_str, encoded.word_count());
        } else {
            failed += 1;
            println!("  ✗ {addr_str} failed roundtrip!");
        }
    }

    println!("  Passed: {}/{}", passed, test_addresses.len());
    assert_eq!(failed, 0);

    println!("✓ Roundtrip accuracy test passed\n");
    Ok(())
}

fn test_string_parsing(
    encoder: &UnifiedThreeWordEncoder,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("Testing String Parsing:");
    println!("-----------------------");

    // Encode some addresses first
    let ipv4_addr = SocketAddr::from_str("192.168.1.1:443")?;
    let ipv6_simple = SocketAddr::from_str("[::1]:80")?;
    let ipv6_complex = SocketAddr::from_str("[2001:db8::1]:443")?;

    let encoded_v4 = encoder.encode(ipv4_addr)?;
    let encoded_v6_simple = encoder.encode(ipv6_simple)?;
    let encoded_v6_complex = encoder.encode(ipv6_complex)?;

    // Test decoding from strings
    println!("  IPv4 string: {}", encoded_v4.to_string());
    let decoded_v4 = encoder.decode_string(&encoded_v4.to_string())?;
    assert_eq!(decoded_v4, ipv4_addr);
    println!("    ✓ Decoded correctly");

    println!("  IPv6 simple: {}", encoded_v6_simple.to_string());
    let decoded_v6_simple = encoder.decode_string(&encoded_v6_simple.to_string())?;
    assert_eq!(decoded_v6_simple, ipv6_simple);
    println!("    ✓ Decoded correctly");

    println!("  IPv6 complex: {}", encoded_v6_complex.to_string());
    let decoded_v6_complex = encoder.decode_string(&encoded_v6_complex.to_string())?;
    assert_eq!(decoded_v6_complex, ipv6_complex);
    println!("    ✓ Decoded correctly");

    println!("✓ String parsing test passed\n");
    Ok(())
}

fn display_formatting_examples(
    encoder: &UnifiedThreeWordEncoder,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("Formatting Examples:");
    println!("--------------------");

    // IPv4 example
    let ipv4 = SocketAddr::from_str("8.8.8.8:53")?;
    let enc_v4 = encoder.encode(ipv4)?;
    println!("IPv4 (1 group of 3):");
    println!("  Input:  {ipv4}");
    println!("  Output: {}", enc_v4.to_string());
    println!();

    // IPv6 6-word example
    let ipv6_short = SocketAddr::from_str("[::1]:443")?;
    let enc_v6_short = encoder.encode(ipv6_short)?;
    println!("IPv6 Simple (2 groups of 3):");
    println!("  Input:  {ipv6_short}");
    println!("  Output: {}", enc_v6_short.to_string());
    println!();

    // IPv6 9-word example
    let ipv6_long = SocketAddr::from_str("[2001:db8:85a3:1234:5678:90ab:cdef:1234]:8080")?;
    let enc_v6_long = encoder.encode(ipv6_long)?;
    println!("IPv6 Complex (3 groups of 3):");
    println!("  Input:  {ipv6_long}");
    println!("  Output: {}", enc_v6_long.to_string());

    Ok(())
}
