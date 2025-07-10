#![no_main]

use libfuzzer_sys::fuzz_target;
use three_word_networking::*;
use arbitrary::Arbitrary;
use std::net::{IpAddr, SocketAddr};

/// Comprehensive fuzzing input structure
#[derive(Arbitrary, Debug)]
struct FuzzInput {
    operation: Operation,
    data: Vec<u8>,
}

#[derive(Arbitrary, Debug)]
enum Operation {
    EncodeIpv4,
    EncodeIpv6,
    EncodeSocket,
    DecodeWords,
    ValidateWords,
    CompressAddr,
    RoundTrip,
}

fuzz_target!(|input: FuzzInput| {
    // Initialize encoder once
    let encoder = match FourWordAdaptiveEncoder::new() {
        Ok(encoder) => encoder,
        Err(_) => return,
    };

    match input.operation {
        Operation::EncodeIpv4 => {
            if let Ok(ip_str) = std::str::from_utf8(&input.data) {
                if let Ok(ip) = ip_str.parse::<std::net::Ipv4Addr>() {
                    let _ = encoder.encode(&ip.to_string());
                }
            }
        }
        
        Operation::EncodeIpv6 => {
            if let Ok(ip_str) = std::str::from_utf8(&input.data) {
                if let Ok(ip) = ip_str.parse::<std::net::Ipv6Addr>() {
                    let _ = encoder.encode(&ip.to_string());
                }
            }
        }
        
        Operation::EncodeSocket => {
            if let Ok(socket_str) = std::str::from_utf8(&input.data) {
                if let Ok(socket) = socket_str.parse::<SocketAddr>() {
                    let _ = encoder.encode(&socket.to_string());
                }
            }
        }
        
        Operation::DecodeWords => {
            if let Ok(words) = std::str::from_utf8(&input.data) {
                let _ = encoder.decode(words);
            }
        }
        
        Operation::ValidateWords => {
            if let Ok(words) = std::str::from_utf8(&input.data) {
                let _ = validate_word_format(words);
            }
        }
        
        Operation::CompressAddr => {
            if input.data.len() >= 4 {
                // Try to compress raw bytes as an address
                let _ = test_compression(&input.data);
            }
        }
        
        Operation::RoundTrip => {
            if let Ok(input_str) = std::str::from_utf8(&input.data) {
                // Try full roundtrip
                if let Ok(encoded) = encoder.encode(input_str) {
                    let _ = encoder.decode(&encoded);
                }
            }
        }
    }
});

/// Test compression functionality
fn test_compression(data: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
    // Test various compression scenarios
    if data.len() >= 4 {
        // Try as IPv4
        let ipv4_bytes = &data[..4];
        let ip = std::net::Ipv4Addr::new(ipv4_bytes[0], ipv4_bytes[1], ipv4_bytes[2], ipv4_bytes[3]);
        let encoder = FourWordAdaptiveEncoder::new()?;
        let _ = encoder.encode(&ip.to_string());
    }
    
    if data.len() >= 16 {
        // Try as IPv6
        let ipv6_bytes = &data[..16];
        let mut segments = [0u16; 8];
        for (i, chunk) in ipv6_bytes.chunks(2).enumerate() {
            if i < 8 && chunk.len() == 2 {
                segments[i] = u16::from_be_bytes([chunk[0], chunk[1]]);
            }
        }
        let ip = std::net::Ipv6Addr::new(
            segments[0], segments[1], segments[2], segments[3],
            segments[4], segments[5], segments[6], segments[7]
        );
        let encoder = FourWordAdaptiveEncoder::new()?;
        let _ = encoder.encode(&ip.to_string());
    }
    
    Ok(())
}

/// Validate word format
fn validate_word_format(words: &str) -> Result<(), Box<dyn std::error::Error>> {
    let parts: Vec<&str> = words.split('.').collect();
    if parts.len() != 4 {
        return Err("Must have exactly 4 words".into());
    }
    
    for part in parts {
        if part.is_empty() {
            return Err("Words cannot be empty".into());
        }
        if part.len() > 20 {
            return Err("Word too long".into());
        }
        if !part.chars().all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_') {
            return Err("Invalid characters in word".into());
        }
    }
    
    Ok(())
}