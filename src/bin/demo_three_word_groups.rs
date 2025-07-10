#!/usr/bin/env rust
//! Demo of the Three-Word Group Encoding System
//!
//! Shows how network addresses are encoded using consistent groups of three words:
//! - IPv4: 3 words (1 group)
//! - IPv6: 6 words (2 groups) or 9 words (3 groups)

use std::net::{Ipv4Addr, Ipv6Addr, SocketAddr};
use std::str::FromStr;
use three_word_networking::{ThreeWordEncoder, ThreeWordIpv6Encoder, UnifiedThreeWordEncoder};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🌐 Three-Word Group Network Encoding Demo");
    println!("=========================================");
    println!();
    println!("A consistent UX pattern: Always groups of 3 words");
    println!();

    // Demo IPv4 (3 words = 1 group)
    demo_ipv4_encoding()?;

    // Demo IPv6 (6 or 9 words = 2 or 3 groups)
    demo_ipv6_encoding()?;

    // Demo unified encoder
    demo_unified_encoding()?;

    // Show the mathematical beauty
    show_mathematical_properties()?;

    Ok(())
}

fn demo_ipv4_encoding() -> Result<(), Box<dyn std::error::Error>> {
    println!("📍 IPv4 Encoding (3 words = 1 group)");
    println!("------------------------------------");

    let encoder = ThreeWordEncoder::new()?;

    let examples = [
        (Ipv4Addr::new(192, 168, 1, 1), 443, "Home router HTTPS"),
        (Ipv4Addr::new(8, 8, 8, 8), 53, "Google DNS"),
        (Ipv4Addr::new(127, 0, 0, 1), 8080, "Localhost dev server"),
        (Ipv4Addr::new(10, 0, 0, 1), 22, "Private network SSH"),
    ];

    for (ip, port, description) in &examples {
        let encoded = encoder.encode(*ip, *port)?;
        println!("  {} ({})", description, format!("{}:{}", ip, port));
        println!("  → {}", encoded.to_string());

        // Verify roundtrip
        let words: Vec<&str> = encoded.words().iter().map(|s| s.as_str()).collect();
        let (decoded_ip, decoded_port) = encoder.decode(&words)?;
        assert_eq!(decoded_ip, *ip);
        assert_eq!(decoded_port, *port);
        println!("  ✓ Perfect reconstruction verified");
        println!();
    }

    Ok(())
}

fn demo_ipv6_encoding() -> Result<(), Box<dyn std::error::Error>> {
    println!("📍 IPv6 Encoding (6 or 9 words = 2 or 3 groups)");
    println!("-----------------------------------------------");

    let encoder = ThreeWordIpv6Encoder::new()?;

    // Show both 6-word and 9-word examples
    let examples = [
        (Ipv6Addr::LOCALHOST, 80, "Loopback"),
        (Ipv6Addr::UNSPECIFIED, 443, "Unspecified"),
        (Ipv6Addr::from_str("fe80::1")?, 22, "Link-local"),
        (Ipv6Addr::from_str("2001:db8::1")?, 443, "Documentation"),
        (
            Ipv6Addr::from_str("2001:4860:4001:801::1")?,
            443,
            "Google (complex)",
        ),
    ];

    for (ip, port, description) in &examples {
        let encoded = encoder.encode(*ip, *port)?;
        println!("  {} ({})", description, format!("[{}]:{}", ip, port));
        println!("  → {}", encoded.to_string());
        println!(
            "  Groups: {} ({} words total)",
            encoded.word_count() / 3,
            encoded.word_count()
        );

        // Note: IPv6 reconstruction is simplified in this demo
        // Full implementation would have complete decompression logic
        println!();
    }

    Ok(())
}

fn demo_unified_encoding() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔄 Unified Encoder (Automatic Detection)");
    println!("----------------------------------------");

    let encoder = UnifiedThreeWordEncoder::new()?;

    let addresses = [
        "192.168.1.1:443",
        "[::1]:80",
        "10.0.0.1:22",
        "[2001:db8::1]:443",
    ];

    for addr_str in &addresses {
        let addr = SocketAddr::from_str(addr_str)?;
        let encoded = encoder.encode(addr)?;

        println!("  Input:  {addr_str}");
        println!("  Output: {}", encoded.to_string());
        println!("  Type:   {}", encoded.description());
        println!("  Groups: {}", encoded.group_count());
        println!();
    }

    Ok(())
}

fn show_mathematical_properties() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔢 Mathematical Properties");
    println!("-------------------------");
    println!();
    println!("Dictionary: 65,536 words (2^16)");
    println!("Bits per word: 16");
    println!();
    println!("IPv4 + port: 48 bits");
    println!("  → 3 words × 16 bits = 48 bits ✓ Perfect match!");
    println!();
    println!("IPv6 + port: 144 bits");
    println!("  → 9 words × 16 bits = 144 bits ✓ Perfect match!");
    println!("  → 6 words × 16 bits = 96 bits (for compressed patterns)");
    println!();
    println!("Group Pattern:");
    println!("  • IPv4: 1 group  (3 words)");
    println!("  • IPv6: 2 groups (6 words) for common addresses");
    println!("  • IPv6: 3 groups (9 words) for complex addresses");
    println!();
    println!("This creates a consistent UX where every address is");
    println!("represented as 1, 2, or 3 groups of exactly 3 words.");

    Ok(())
}
