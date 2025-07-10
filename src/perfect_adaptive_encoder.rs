//! Perfect Adaptive Encoder - Main interface for 100% perfect encoding/decoding
//!
//! This module provides the main interface that automatically selects between
//! IPv4 and IPv6 perfect codecs based on the input.

use crate::{
    ipv4_perfect_codec::IPv4PerfectCodec,
    ipv6_perfect_codec::IPv6PerfectCodec,
    perfect_encoder::{MultiDimEncoding, PerfectDictionary},
    FourWordError, Result,
};
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

/// Main encoder that provides perfect reconstruction for all IP addresses
pub struct PerfectAdaptiveEncoder {
    ipv4_codec: IPv4PerfectCodec,
    ipv6_codec: IPv6PerfectCodec,
    dictionary: PerfectDictionary,
}

impl PerfectAdaptiveEncoder {
    pub fn new() -> Result<Self> {
        Ok(Self {
            ipv4_codec: IPv4PerfectCodec::new()?,
            ipv6_codec: IPv6PerfectCodec::new()?,
            dictionary: PerfectDictionary::new()?,
        })
    }

    /// Encode any IP address with port to four words
    pub fn encode(&self, address: &str) -> Result<String> {
        let (ip, port) = self.parse_address(address)?;

        let encoding = match ip {
            IpAddr::V4(ipv4) => self.ipv4_codec.encode(ipv4, port)?,
            IpAddr::V6(ipv6) => self.ipv6_codec.encode(ipv6, port)?,
        };

        Ok(encoding.to_string())
    }

    /// Decode four words back to IP address with port
    pub fn decode(&self, words: &str) -> Result<String> {
        let encoding = MultiDimEncoding::from_string(words, &self.dictionary)?;

        // Determine IP version based on separator format
        let is_ipv6 = words.contains('-');
        let is_ipv4 = words.contains('.');

        if is_ipv6 && !is_ipv4 {
            // IPv6 format (dashes)
            let (ip, port) = self.ipv6_codec.decode(&encoding)?;
            if port == 0 {
                Ok(format!("[{}]", ip))
            } else {
                Ok(format!("[{}]:{}", ip, port))
            }
        } else if is_ipv4 && !is_ipv6 {
            // IPv4 format (dots)
            let (ip, port) = self.ipv4_codec.decode(&encoding)?;
            if port == 0 {
                Ok(ip.to_string())
            } else {
                Ok(format!("{}:{}", ip, port))
            }
        } else {
            // Fallback: try IPv4 first, then IPv6
            match self.ipv4_codec.decode(&encoding) {
                Ok((ip, port)) => {
                    if port == 0 {
                        Ok(ip.to_string())
                    } else {
                        Ok(format!("{}:{}", ip, port))
                    }
                }
                Err(_) => {
                    let (ip, port) = self.ipv6_codec.decode(&encoding)?;
                    if port == 0 {
                        Ok(format!("[{}]", ip))
                    } else {
                        Ok(format!("[{}]:{}", ip, port))
                    }
                }
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
                    FourWordError::InvalidInput(format!("Invalid IPv6 address: {}", addr_part))
                })?;

                let port = if let Some(port_str) = port_part {
                    port_str.parse::<u16>().map_err(|_| {
                        FourWordError::InvalidInput(format!("Invalid port: {}", port_str))
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
                        FourWordError::InvalidInput(format!("Invalid port: {}", port_part))
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
                "Invalid IP address: {}",
                input
            )))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_perfect_ipv4_roundtrip() {
        let encoder = PerfectAdaptiveEncoder::new().unwrap();

        let test_cases = vec![
            "192.168.1.1:443",
            "10.0.0.1:22",
            "8.8.8.8:53",
            "127.0.0.1:8080",
            "255.255.255.255:65535",
        ];

        for address in test_cases {
            let encoded = encoder.encode(address).unwrap();
            println!("{} -> {}", address, encoded);

            let decoded = encoder.decode(&encoded).unwrap();
            assert_eq!(address, decoded, "Failed to roundtrip {}", address);
        }
    }

    #[test]
    fn test_perfect_ipv6_special_cases() {
        let encoder = PerfectAdaptiveEncoder::new().unwrap();

        let test_cases = vec!["[::]:0", "[::1]:80", "[::1]:443"];

        for address in test_cases {
            let encoded = encoder.encode(address).unwrap();
            println!("{} -> {}", address, encoded);

            let decoded = encoder.decode(&encoded).unwrap();

            // Normalize the expected format
            let expected = if address.ends_with(":0") {
                address[..address.len() - 2].to_string()
            } else {
                address.to_string()
            };

            assert_eq!(expected, decoded, "Failed to roundtrip {}", address);
        }
    }

    #[test]
    fn test_visual_distinction() {
        let encoder = PerfectAdaptiveEncoder::new().unwrap();

        let ipv4 = encoder.encode("192.168.1.1:443").unwrap();
        let ipv6 = encoder.encode("[::1]:443").unwrap();

        println!("IPv4 encoding: {}", ipv4);
        println!("IPv6 encoding: {}", ipv6);

        // IPv4 should use dots, IPv6 should use dashes
        assert!(ipv4.contains('.'), "IPv4 should use dots");
        assert!(ipv6.contains('-'), "IPv6 should use dashes");

        // IPv6 should have title case
        let ipv6_parts: Vec<&str> = ipv6.split('-').collect();
        for part in ipv6_parts {
            if let Some(first) = part.chars().next() {
                assert!(first.is_uppercase(), "IPv6 should use title case");
            }
        }
    }
}
