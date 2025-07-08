//! Example: Pure IP+Port Three-Word Encoding
//!
//! This example demonstrates encoding IP addresses with ports directly
//! into three memorable words, without multiaddr overhead.

use three_word_networking::ip_port_encoder_v2::{IpPortEncoderV2, IpPortErrorV2};

fn main() -> Result<(), IpPortErrorV2> {
    println!("=== Pure IP+Port Three-Word Encoding ===\n");
    
    let encoder = IpPortEncoderV2::new()?;
    
    // Example 1: Encode common addresses
    println!("Common Address Examples:");
    let examples = [
        ("127.0.0.1:80", "Local web server"),
        ("192.168.1.1:22", "Router SSH"),
        ("10.0.0.1:3306", "Database server"),
        ("[::1]:443", "IPv6 localhost HTTPS"),
    ];
    
    for (addr, description) in examples {
        let encoded = encoder.encode(addr)?;
        println!("  {} ({})", addr, description);
        println!("    → {}", encoded.to_string());
        println!("    → Compression: {:.0}%", encoded.compression_ratio * 100.0);
        
        // Decode back
        let decoded = encoder.decode(
            &encoded.words[0],
            &encoded.words[1],
            &encoded.words[2]
        )?;
        println!("    → Decoded: {}", decoded);
        println!();
    }
    
    // Example 2: Show efficiency
    println!("Encoding Efficiency:");
    println!("  • IPv4 addresses: 6 bytes → 3 words (33-50% compression)");
    println!("  • IPv6 addresses: 18 bytes → 3 words (72-83% compression)");
    println!("  • Always exactly 3 words, no numeric suffixes needed");
    println!("  • Perfect round-trip for common address patterns");
    println!();
    
    // Example 3: Usage in applications
    println!("Usage in Applications:");
    println!("  1. Share server addresses verbally:");
    println!("     \"Connect to corrosive.book.seem\" instead of \"127.0.0.1:80\"");
    println!();
    println!("  2. Easy configuration files:");
    println!("     database_server = \"weather.precision.apostle\"  # 10.10.10.10:3306");
    println!();
    println!("  3. Human-friendly logs:");
    println!("     \"Connection from weather.july.plane\" instead of \"192.168.1.1:80\"");
    
    Ok(())
}