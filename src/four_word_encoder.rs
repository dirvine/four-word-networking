//! Four-word encoder for IPv4 addresses.
//!
//! This module provides encoding and decoding of IPv4 addresses and ports
//! into exactly four memorable words using a 4,096-word dictionary.

use crate::dictionary4k::DICTIONARY;
use crate::error::{FourWordError, Result};
use std::net::{IpAddr, Ipv4Addr, SocketAddr};

/// Represents an encoded four-word address
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FourWordEncoding {
    /// The four words representing the address
    words: [String; 4],
}

impl FourWordEncoding {
    /// Creates a new four-word encoding from individual words
    pub fn new(word1: String, word2: String, word3: String, word4: String) -> Self {
        FourWordEncoding {
            words: [word1, word2, word3, word4],
        }
    }

    /// Returns the words as a dot-separated string (for backward compatibility)
    pub fn to_dotted_string(&self) -> String {
        self.words.join(".")
    }

    /// Returns a slice of the words
    pub fn words(&self) -> &[String] {
        &self.words
    }
}

impl std::fmt::Display for FourWordEncoding {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.words.join(" "))
    }
}

/// Four-word encoder for IPv4 addresses
pub struct FourWordEncoder;

impl FourWordEncoder {
    /// Creates a new four-word encoder
    pub fn new() -> Self {
        FourWordEncoder
    }

    /// Encodes an IPv4 address and port into four words using simple algorithm
    pub fn encode_ipv4(&self, addr: Ipv4Addr, port: u16) -> Result<FourWordEncoding> {
        // IPv4 address: 32 bits
        // Port: 16 bits
        // Total: 48 bits
        // With 4 words at 12 bits each, we have exactly 48 bits

        let octets = addr.octets();

        // Pack the 48 bits: IPv4 (32 bits) + port (16 bits)
        // Convert to big-endian bytes then to integer
        let mut bytes = [0u8; 6];
        bytes[0..4].copy_from_slice(&octets);
        bytes[4..6].copy_from_slice(&port.to_be_bytes());

        // Convert to 48-bit integer
        let mut n = 0u64;
        for byte in bytes {
            n = (n << 8) | (byte as u64);
        }

        // Simple modulo-based encoding (like the Python version)
        let mut words = Vec::with_capacity(4);
        let mut remaining = n;

        for _ in 0..4 {
            let index = (remaining % 4096) as u16;
            let word = DICTIONARY
                .get_word(index)
                .ok_or(FourWordError::InvalidWordIndex(index))?
                .to_string();
            words.push(word);
            remaining /= 4096;
        }

        Ok(FourWordEncoding::new(
            words[0].clone(),
            words[1].clone(),
            words[2].clone(),
            words[3].clone(),
        ))
    }

    /// Decodes four words back to an IPv4 address and port using simple algorithm
    pub fn decode_ipv4(&self, encoding: &FourWordEncoding) -> Result<(Ipv4Addr, u16)> {
        // Reconstruct the 48-bit number from words
        let mut n = 0u64;

        for (i, word) in encoding.words.iter().enumerate() {
            let index = DICTIONARY
                .get_index(word)
                .ok_or_else(|| FourWordError::InvalidWord(word.clone()))?;

            // Each word contributes its index * 4096^position
            n += (index as u64) * 4096u64.pow(i as u32);
        }

        // Convert back to bytes (big-endian)
        let bytes = n.to_be_bytes();

        // Extract IPv4 address (first 4 bytes of the 48-bit value)
        // Since we have 8 bytes from u64, we need the last 6 bytes
        let octet1 = bytes[2];
        let octet2 = bytes[3];
        let octet3 = bytes[4];
        let octet4 = bytes[5];

        // Extract port (last 2 bytes)
        let port = ((bytes[6] as u16) << 8) | (bytes[7] as u16);

        let addr = Ipv4Addr::new(octet1, octet2, octet3, octet4);
        Ok((addr, port))
    }

    /// Encodes a socket address
    pub fn encode(&self, addr: SocketAddr) -> Result<FourWordEncoding> {
        match addr {
            SocketAddr::V4(v4) => self.encode_ipv4(*v4.ip(), v4.port()),
            SocketAddr::V6(_) => Err(FourWordError::InvalidInput(
                "IPv6 addresses require more than 4 words".to_string(),
            )),
        }
    }

    /// Decodes words to a socket address
    pub fn decode(&self, words: &str) -> Result<SocketAddr> {
        // First try space-separated
        let parts: Vec<&str> = words.split_whitespace().collect();
        let parts = if parts.len() == 4 {
            parts
        } else {
            // Try dot-separated for backward compatibility
            let dot_parts: Vec<&str> = words.split('.').collect();
            if dot_parts.len() != 4 {
                return Err(FourWordError::InvalidWordCount {
                    expected: 4,
                    actual: dot_parts.len(),
                });
            }
            dot_parts
        };

        let encoding = FourWordEncoding::new(
            parts[0].to_string(),
            parts[1].to_string(),
            parts[2].to_string(),
            parts[3].to_string(),
        );

        let (addr, port) = self.decode_ipv4(&encoding)?;
        Ok(SocketAddr::new(IpAddr::V4(addr), port))
    }
}

impl Default for FourWordEncoder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_decode_ipv4() {
        let encoder = FourWordEncoder::new();

        // Test various IPv4 addresses
        let test_cases = vec![
            ("192.168.1.1", 443),
            ("10.0.0.1", 80),
            ("127.0.0.1", 8080),
            ("172.16.0.1", 22),
            ("8.8.8.8", 53),
            ("255.255.255.255", 65535),
            ("0.0.0.0", 0),
        ];

        for (ip_str, port) in test_cases {
            let addr: Ipv4Addr = ip_str.parse().unwrap();
            let encoded = encoder.encode_ipv4(addr, port).unwrap();
            let (decoded_addr, decoded_port) = encoder.decode_ipv4(&encoded).unwrap();

            assert_eq!(addr, decoded_addr);
            assert_eq!(port, decoded_port);
        }
    }

    #[test]
    fn test_socket_addr_encoding() {
        let encoder = FourWordEncoder::new();
        let addr: SocketAddr = "192.168.1.1:443".parse().unwrap();

        let encoded = encoder.encode(addr).unwrap();
        let decoded = encoder.decode(&encoded.to_string()).unwrap();

        assert_eq!(addr, decoded);
    }

    #[test]
    fn test_dot_separated_decode() {
        let encoder = FourWordEncoder::new();
        let addr: SocketAddr = "10.0.0.1:80".parse().unwrap();

        let encoded = encoder.encode(addr).unwrap();
        let dotted = encoded.to_dotted_string();
        let decoded = encoder.decode(&dotted).unwrap();

        assert_eq!(addr, decoded);
    }
}
