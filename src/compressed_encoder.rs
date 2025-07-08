//! Compressed encoder that integrates IP compression with four-word encoding
//! 
//! This module provides the integration layer between the advanced compression
//! techniques and the 16K dictionary to enable four-word addresses for IP+port.

use crate::{
    compression::{IpCompressor, CompressedAddress},
    dictionary16k::Dictionary16K,
    error::FourWordError,
};
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

/// Main encoder that combines compression with four-word encoding
pub struct CompressedEncoder {
    compressor: IpCompressor,
    dictionary: Dictionary16K,
}

impl CompressedEncoder {
    /// Create a new compressed encoder
    pub fn new() -> Result<Self, FourWordError> {
        Ok(Self {
            compressor: IpCompressor::new(),
            dictionary: Dictionary16K::new()
                .map_err(|e| FourWordError::InvalidInput(e.to_string()))?,
        })
    }

    /// Encode an IP address string with optional port into four words
    pub fn encode(&self, address: &str) -> Result<String, FourWordError> {
        // Parse the address
        let (ip, port) = self.parse_address(address)?;
        
        // Compress the address
        let compressed = self.compressor.compress(&ip, port)?;
        
        // Pack into bit stream first to get actual size
        let packed = compressed.pack();
        
        // Check actual packed size
        if packed.len() > 5 {
            return Err(FourWordError::InvalidInput(
                format!("Compressed address requires {} bytes, exceeds 5-byte limit for four words", packed.len())
            ));
        }
        
        // Encode as four words
        let words = self.dictionary.encode_bytes(&packed)
            .map_err(|e| FourWordError::InvalidInput(e.to_string()))?;
        
        Ok(words.join("."))
    }

    /// Decode four words back to IP address string with optional port
    pub fn decode(&self, words: &str) -> Result<String, FourWordError> {
        // Split and validate words
        let word_vec: Vec<&str> = words.split('.').collect();
        if word_vec.len() != 3 {
            return Err(FourWordError::InvalidInput(
                format!("Expected 4 words, got {}", word_vec.len())
            ));
        }
        
        // Decode words to bytes
        let packed = self.dictionary.decode_words(&word_vec)
            .map_err(|e| FourWordError::InvalidInput(e.to_string()))?;
        
        // Unpack from bit stream
        let (ip, port) = CompressedAddress::unpack(&packed, &self.compressor)?;
        
        // Format result
        match (ip, port) {
            (IpAddr::V4(addr), Some(p)) => Ok(format!("{}:{}", addr, p)),
            (IpAddr::V4(addr), None) => Ok(addr.to_string()),
            (IpAddr::V6(addr), Some(p)) => Ok(format!("[{}]:{}", addr, p)),
            (IpAddr::V6(addr), None) => Ok(addr.to_string()),
        }
    }

    /// Parse an IP address string with optional port
    fn parse_address(&self, input: &str) -> Result<(IpAddr, Option<u16>), FourWordError> {
        // Handle IPv6 with port: [addr]:port
        if input.starts_with('[') {
            if let Some(close_idx) = input.find(']') {
                let addr_part = &input[1..close_idx];
                let port_part = if close_idx + 1 < input.len() && &input[close_idx + 1..close_idx + 2] == ":" {
                    Some(&input[close_idx + 2..])
                } else {
                    None
                };
                
                let ip = addr_part.parse::<Ipv6Addr>()
                    .map_err(|_| FourWordError::InvalidInput(format!("Invalid IPv6 address: {}", addr_part)))?;
                
                let port = if let Some(port_str) = port_part {
                    Some(port_str.parse::<u16>()
                        .map_err(|_| FourWordError::InvalidInput(format!("Invalid port: {}", port_str)))?)
                } else {
                    None
                };
                
                return Ok((IpAddr::V6(ip), port));
            }
        }
        
        // Handle IPv4 with port or IPv6 without brackets
        if let Some(last_colon) = input.rfind(':') {
            // Check if this is part of IPv6 (multiple colons) or IPv4:port
            let colon_count = input.matches(':').count();
            
            if colon_count == 1 {
                // IPv4:port
                let addr_part = &input[..last_colon];
                let port_part = &input[last_colon + 1..];
                
                let ip = addr_part.parse::<Ipv4Addr>()
                    .map_err(|_| FourWordError::InvalidInput(format!("Invalid IPv4 address: {}", addr_part)))?;
                
                let port = port_part.parse::<u16>()
                    .map_err(|_| FourWordError::InvalidInput(format!("Invalid port: {}", port_part)))?;
                
                return Ok((IpAddr::V4(ip), Some(port)));
            }
        }
        
        // Try parsing as standalone IP
        let ip = input.parse::<IpAddr>()
            .map_err(|_| FourWordError::InvalidInput(format!("Invalid IP address: {}", input)))?;
        
        Ok((ip, None))
    }

    /// Get compression statistics for an address
    pub fn compression_stats(&self, address: &str) -> Result<CompressionStats, FourWordError> {
        let (ip, port) = self.parse_address(address)?;
        let compressed = self.compressor.compress(&ip, port)?;
        
        // Get actual packed size to determine real bit count
        let packed = compressed.pack();
        let actual_bits = packed.len() * 8;
        
        let original_bits = match ip {
            IpAddr::V4(_) => 32 + port.map_or(0, |_| 16),
            IpAddr::V6(_) => 128 + port.map_or(0, |_| 16),
        };
        
        Ok(CompressionStats {
            original_bits,
            compressed_bits: actual_bits,
            compression_ratio: 1.0 - (actual_bits as f64 / original_bits as f64),
            fits_in_three_words: packed.len() <= 5, // 5 bytes = 40 bits < 42 bits
            address_type: format!("{:?}", compressed.addr_type),
        })
    }
}

/// Statistics about compression efficiency
#[derive(Debug, Clone)]
pub struct CompressionStats {
    pub original_bits: usize,
    pub compressed_bits: usize,
    pub compression_ratio: f64,
    pub fits_in_three_words: bool,
    pub address_type: String,
}

impl CompressionStats {
    /// Get a human-readable summary
    pub fn summary(&self) -> String {
        format!(
            "{} → {} bits ({:.1}% compression) - {} - Four words: {}",
            self.original_bits,
            self.compressed_bits,
            self.compression_ratio * 100.0,
            self.address_type,
            if self.fits_in_three_words { "✓" } else { "✗" }
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_localhost_encoding() {
        let encoder = CompressedEncoder::new().unwrap();
        
        // Test localhost with common port
        let encoded = encoder.encode("127.0.0.1:80").unwrap();
        assert_eq!(encoded.split('.').count(), 3);
        
        let decoded = encoder.decode(&encoded).unwrap();
        assert_eq!(decoded, "127.0.0.1:80");
    }

    #[test]
    fn test_private_network_encoding() {
        let encoder = CompressedEncoder::new().unwrap();
        
        let test_cases = vec![
            "192.168.1.100:443",
            "10.0.0.1:22",
            "172.16.0.1:8080",
        ];
        
        for addr in test_cases {
            let encoded = encoder.encode(addr).unwrap();
            assert_eq!(encoded.split('.').count(), 3);
            
            let decoded = encoder.decode(&encoded).unwrap();
            assert_eq!(decoded, addr);
        }
    }

    #[test]
    fn test_ipv6_localhost() {
        let encoder = CompressedEncoder::new().unwrap();
        
        let encoded = encoder.encode("[::1]:443").unwrap();
        assert_eq!(encoded.split('.').count(), 3);
        
        let decoded = encoder.decode(&encoded).unwrap();
        assert_eq!(decoded, "[::1]:443");
    }

    #[test]
    fn test_compression_stats() {
        let encoder = CompressedEncoder::new().unwrap();
        
        // Localhost should compress very well
        let stats = encoder.compression_stats("127.0.0.1:80").unwrap();
        assert!(stats.compression_ratio > 0.4); // 50% compression is good
        assert!(stats.fits_in_three_words);
        
        // Public IP with port should not fit
        let stats = encoder.compression_stats("8.8.8.8:53").unwrap();
        assert!(!stats.fits_in_three_words);
    }

    #[test]
    fn test_edge_cases() {
        let encoder = CompressedEncoder::new().unwrap();
        
        // Test without port
        let encoded = encoder.encode("192.168.1.1").unwrap();
        let decoded = encoder.decode(&encoded).unwrap();
        assert_eq!(decoded, "192.168.1.1");
        
        // Test IPv6 without port
        let encoded = encoder.encode("::1").unwrap();
        let decoded = encoder.decode(&encoded).unwrap();
        assert_eq!(decoded, "::1");
    }

    #[test]
    fn test_error_cases() {
        let encoder = CompressedEncoder::new().unwrap();
        
        // Invalid addresses
        assert!(encoder.encode("invalid").is_err());
        assert!(encoder.encode("256.256.256.256").is_err());
        
        // Public IPs that don't compress enough
        assert!(encoder.encode("8.8.8.8:1234").is_err());
        assert!(encoder.encode("1.2.3.4:65535").is_err());
        
        // Invalid four-word format
        assert!(encoder.decode("one.two").is_err());
        assert!(encoder.decode("one.two.three.four").is_err());
    }
}