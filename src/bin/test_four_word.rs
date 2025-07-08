//! Test the four-word encoding system
//!
//! This demonstrates perfect reconstruction for IPv4 (4 words)
//! and adaptive encoding for IPv6 (4-6 words).

use four_word_networking::FourWordAdaptiveEncoder;
use std::str::FromStr;
use std::net::{Ipv4Addr, Ipv6Addr};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🌟 Four-Word Perfect Encoding Demo");
    println!("=====================================\n");
    
    let encoder = FourWordAdaptiveEncoder::new()?;
    
    println!("📍 IPv4 Perfect Reconstruction (4 words)");
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
        
        println!("  {} → {} ({} words)", address, encoded, word_count);
        
        if address == decoded {
            println!("  ✅ Perfect reconstruction verified!");
        } else {
            println!("  ❌ Reconstruction failed: {} != {}", address, decoded);
        }
        println!();
    }
    
    println!("\n🌐 IPv6 Adaptive Encoding (4-6 words)");
    println!("--------------------------------------");
    
    // Test IPv6 addresses
    let ipv6_tests = vec![
        "[::1]:443",                    // Loopback
        "[fe80::1]:22",                // Link-local
        "[2001:db8::1]:80",            // Documentation
        "[fc00::1]:8080",              // Unique local
        "[::]:0",                      // Unspecified
        "[2001:4860:4860::8888]:53",   // Google DNS
    ];
    
    for address in ipv6_tests {
        let encoded = encoder.encode(address)?;
        let word_count = encoded.split('-').count();
        
        println!("  {} → {} ({} words)", address, encoded, word_count);
        println!("  📊 Compression: IPv6 category-based encoding");
        
        // Try to decode
        match encoder.decode(&encoded) {
            Ok(decoded) => {
                println!("  🔄 Decoded to: {}", decoded);
            }
            Err(e) => {
                println!("  ⚠️  Decode not implemented: {}", e);
            }
        }
        println!();
    }
    
    println!("\n📋 Summary");
    println!("----------");
    println!("• IPv4: Perfect reconstruction with exactly 4 words");
    println!("• IPv6: Adaptive encoding with 4-6 words based on pattern");
    println!("• Visual distinction: IPv4 uses dots, IPv6 uses dashes");
    println!("• Dictionary: 16,384 words (14 bits per word)");
    println!("• IPv4 capacity: 4 × 14 = 56 bits (perfect for 48-bit IPv4+port)");
    println!("• IPv6 compression: Category-based for common patterns");
    
    Ok(())
}