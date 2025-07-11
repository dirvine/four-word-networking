#!/usr/bin/env rust
//! Debug which word indices are being selected

use three_word_networking::ThreeWordAdaptiveEncoder;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let encoder = ThreeWordAdaptiveEncoder::new()?;
    
    // Test some simple addresses
    let test_addrs = vec![
        "192.168.1.1:443",
        "10.0.0.1:80",
        "127.0.0.1:8080",
        "8.8.8.8:53",
    ];
    
    for addr in test_addrs {
        let words = encoder.encode(addr)?;
        println!("\n{} -> {}", addr, words);
        
        // This is a bit hacky but let's decode to see the pattern
        let decoded = encoder.decode(&words)?;
        println!("  (decodes back to: {})", decoded);
    }
    
    Ok(())
}