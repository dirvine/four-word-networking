//! Test the three-word encoding system
//!
//! This demonstrates perfect reconstruction for IPv4 (3 words)
//! and adaptive encoding for IPv6 (6 or 9 words).

#[allow(unused_imports)]
use std::net::{Ipv4Addr, Ipv6Addr};
#[allow(unused_imports)]
use std::str::FromStr;
use three_word_networking::ThreeWordAdaptiveEncoder;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🌟 Three-Word Perfect Encoding Demo");
    println!("=====================================\n");

    let encoder = ThreeWordAdaptiveEncoder::new()?;

    println!("📍 IPv4 Perfect Reconstruction (3 words)");
    println!("----------------------------------------");

    // Test IPv4 addresses
    let ipv4_tests = vec![
        "192.168.1.1:443",
        "10.0.0.1:22",
        "8.8.8.8:53",
        "127.0.0.1:8080",
        "255.255.255.255:65535",
        "0.0.0.0:0",
        "172.16.0.1:3389",
        "1.1.1.1:443",
    ];

    for address in ipv4_tests {
        let encoded = encoder.encode(address)?;
        let decoded = encoder.decode(&encoded)?;
        let word_count = encoded.split('.').count();

        println!("  {address} → {encoded} ({word_count} words)");

        if address == decoded {
            println!("  ✅ Perfect reconstruction verified!");
        } else {
            println!("  ❌ Reconstruction failed: {address} != {decoded}");
        }
        println!();
    }

    println!("\n🌐 IPv6 Adaptive Encoding (6 or 9 words)");
    println!("--------------------------------------");

    // Test IPv6 addresses
    let ipv6_tests = vec![
        "[::1]:443",                 // Loopback
        "[fe80::1]:22",              // Link-local
        "[2001:db8::1]:80",          // Documentation
        "[fc00::1]:8080",            // Unique local
        "[::]:0",                    // Unspecified
        "[2001:4860:4860::8888]:53", // Google DNS
    ];

    for address in ipv6_tests {
        let encoded = encoder.encode(address)?;
        let word_count = encoded.split('-').count();

        println!("  {address} → {encoded} ({word_count} words)");
        println!("  📊 Compression: IPv6 category-based encoding");

        // Try to decode
        match encoder.decode(&encoded) {
            Ok(decoded) => {
                println!("  🔄 Decoded to: {decoded}");
            }
            Err(e) => {
                println!("  ⚠️  Decode not implemented: {e}");
            }
        }
        println!();
    }

    println!("\n📋 Summary");
    println!("----------");
    println!("• IPv4: Perfect reconstruction with exactly 3 words");
    println!("• IPv6: Adaptive encoding with 6 or 9 words (groups of 3)");
    println!("• Visual distinction: IPv4 uses dots, IPv6 uses dashes");
    println!("• Dictionary: 65,536 words for IPv4 (16 bits per word)");
    println!("• IPv4 capacity: 3 × 16 = 48 bits (perfect for IPv4+port)");
    println!("• IPv6 compression: Groups of 3 for consistent UX");

    Ok(())
}
