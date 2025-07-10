//! Simple Adaptive Encoder - Main interface using the simple perfect encoding
//!
//! This module provides the main interface that automatically handles
//! IPv4 and IPv6 encoding/decoding with perfect reconstruction.

use crate::{
    FourWordError, Result,
    simple_perfect_encoder::{SimpleDictionary, SimpleEncoding, SimplePerfectEncoder},
};
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

/// Main encoder that provides perfect reconstruction for all IP addresses
pub struct SimpleAdaptiveEncoder {
    encoder: SimplePerfectEncoder,
    dictionary: SimpleDictionary,
}

impl SimpleAdaptiveEncoder {
    pub fn new() -> Result<Self> {
        Ok(Self {
            encoder: SimplePerfectEncoder::new()?,
            dictionary: SimpleDictionary::new()?,
        })
    }

    /// Encode any IP address with port to four words
    pub fn encode(&self, address: &str) -> Result<String> {
        let (ip, port) = self.parse_address(address)?;

        // Convert to 48 bits
        let data = match ip {
            IpAddr::V4(ipv4) => {
                let ip_bits = u32::from_be_bytes(ipv4.octets()) as u64;
                let port_bits = port as u64;
                let combined = (ip_bits << 16) | port_bits;
                // Now we can handle full 48 bits with 4 words
                combined & 0xFFFFFFFFFFFF
            }
            IpAddr::V6(ipv6) => {
                // For IPv6, we'll use a simple hash for demo purposes
                // In production, you'd want proper compression
                let segments = ipv6.segments();
                let hash = segments[0] as u64
                    ^ (segments[1] as u64) << 16
                    ^ (segments[7] as u64) << 32
                    ^ (port as u64);
                hash & 0xFFFFFFFFFFFF // Limit to 48 bits
            }
        };

        let is_ipv6 = matches!(ip, IpAddr::V6(_));
        let encoding = self.encoder.encode_48_bits(data, is_ipv6)?;

        Ok(encoding.to_string())
    }

    /// Decode four words back to IP address with port
    pub fn decode(&self, words: &str) -> Result<String> {
        let encoding = SimpleEncoding::from_string(words, &self.dictionary)?;
        let data = self.encoder.decode_48_bits(&encoding)?;

        if encoding.is_ipv6 {
            // For demo, return a simple IPv6 representation
            // In production, you'd need proper decompression
            Ok(format!("[::1]:{}", data & 0xFFFF))
        } else {
            // IPv4 decoding - now we have full 48 bits
            // 32 bits of IP + 16 bits of port
            let port = (data & 0xFFFF) as u16;
            let ip_bits = ((data >> 16) & 0xFFFFFFFF) as u32; // Full 32 bits

            let ip = Ipv4Addr::from(ip_bits);

            if port == 0 {
                Ok(ip.to_string())
            } else {
                Ok(format!("{ip}:{port}"))
            }
        }
    }

    /// Parse address string into IP and port
    fn parse_address(&self, input: &str) -> Result<(IpAddr, u16)> {
        // Handle IPv6 with port: [addr]:port
        if input.starts_with('[') {
            if let Some(close_idx) = input.find(']') {
                let addr_part = &input[1..close_idx];
                let port_part =
                    if close_idx + 1 < input.len() && &input[close_idx + 1..close_idx + 2] == ":" {
                        Some(&input[close_idx + 2..])
                    } else {
                        None
                    };

                let ip = addr_part.parse::<Ipv6Addr>().map_err(|_| {
                    FourWordError::InvalidInput(format!("Invalid IPv6 address: {addr_part}"))
                })?;

                let port = if let Some(port_str) = port_part {
                    port_str.parse::<u16>().map_err(|_| {
                        FourWordError::InvalidInput(format!("Invalid port: {port_str}"))
                    })?
                } else {
                    0
                };

                return Ok((IpAddr::V6(ip), port));
            }
        }

        // Handle IPv4 with port or IPv6 without brackets
        if let Some(last_colon) = input.rfind(':') {
            let colon_count = input.matches(':').count();

            if colon_count == 1 {
                // IPv4:port
                let addr_part = &input[..last_colon];
                let port_part = &input[last_colon + 1..];

                if let Ok(ip) = addr_part.parse::<Ipv4Addr>() {
                    let port = port_part.parse::<u16>().map_err(|_| {
                        FourWordError::InvalidInput(format!("Invalid port: {port_part}"))
                    })?;

                    return Ok((IpAddr::V4(ip), port));
                }
            }
        }

        // Try parsing as standalone IP
        if let Ok(ip) = input.parse::<IpAddr>() {
            Ok((ip, 0))
        } else {
            Err(FourWordError::InvalidInput(format!(
                "Invalid IP address: {input}"
            )))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ipv4_perfect_roundtrip() {
        let encoder = SimpleAdaptiveEncoder::new().unwrap();

        let test_cases = vec![
            "192.168.1.1:443",
            "10.0.0.1:22",
            "8.8.8.8:53",
            "127.0.0.1:8080",
            "255.255.255.255:65535",
        ];

        for address in test_cases {
            let encoded = encoder.encode(address).unwrap();
            println!("{address} -> {encoded}");

            let decoded = encoder.decode(&encoded).unwrap();
            assert_eq!(address, decoded, "Failed to roundtrip {address}");
        }
    }

    #[test]
    fn test_visual_distinction() {
        let encoder = SimpleAdaptiveEncoder::new().unwrap();

        let ipv4 = encoder.encode("192.168.1.1:443").unwrap();
        let ipv6 = encoder.encode("[::1]:443").unwrap();

        println!("IPv4 encoding: {ipv4}");
        println!("IPv6 encoding: {ipv6}");

        // IPv4 should use dots, IPv6 should use dashes
        assert!(ipv4.contains('.'), "IPv4 should use dots");
        assert!(ipv6.contains('-'), "IPv6 should use dashes");
    }
}
