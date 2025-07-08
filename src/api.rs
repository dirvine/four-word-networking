//! Clean Public API for Four-Word Networking
//!
//! This module provides a simple, intuitive API for converting between
//! network addresses and four-word combinations.

use std::net::{IpAddr, SocketAddr};
use std::str::FromStr;
use crate::{AdaptiveEncoder, FourWordError, Result};

/// Main interface for Four-Word Networking
///
/// Provides simple methods to convert between IP addresses and words.
///
/// # Examples
///
/// ```rust
/// use four_word_networking::FourWordNetworking;
/// 
/// let twn = FourWordNetworking::new()?;
/// 
/// // Convert IP to words
/// let words = twn.encode("192.168.1.1:80")?;
/// println!("{}", words); // e.g. "ocean.thunder.falcon"
/// 
/// // Convert words back to IP
/// let addr = twn.decode("ocean.thunder.falcon")?;
/// println!("{}", addr); // "192.168.1.1:80"
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub struct FourWordNetworking {
    encoder: AdaptiveEncoder,
}

impl FourWordNetworking {
    /// Create a new Four-Word Networking instance
    pub fn new() -> Result<Self> {
        Ok(Self {
            encoder: AdaptiveEncoder::new()?,
        })
    }

    /// Convert an address to words
    ///
    /// Accepts multiple input formats:
    /// - String: "192.168.1.1:80", "[::1]:443", "example.com:80"
    /// - &str: Same as String
    /// - SocketAddr: Direct socket address
    /// - IpAddr: IP address without port (defaults to port 0)
    ///
    /// Returns 4 words for IPv4, 4-6 words for IPv6.
    pub fn encode<T: Into<AddressInput>>(&self, input: T) -> Result<String> {
        let address_input = input.into();
        let address_str = address_input.to_address_string();
        
        let result = self.encoder.encode(&address_str)?;
        Ok(result.words())
    }

    /// Convert words back to a socket address
    ///
    /// Accepts 3-6 dot-separated words and returns the original address.
    /// 4 words always decode to IPv4, 4-6 words always decode to IPv6.
    pub fn decode(&self, words: &str) -> Result<SocketAddr> {
        let address_str = self.encoder.decode(words)?;
        
        // Parse the decoded string into a SocketAddr
        // The adaptive encoder returns strings like "192.168.1.1:80" or "[::1]:443"
        SocketAddr::from_str(&address_str)
            .or_else(|_| {
                // Try parsing as IP without port, then add default port
                if let Ok(ip) = address_str.parse::<IpAddr>() {
                    Ok(SocketAddr::new(ip, 0))
                } else {
                    Err(FourWordError::InvalidInput(
                        format!("Could not parse decoded address: {}", address_str)
                    ))
                }
            })
    }

    /// Get the number of words needed for an address
    ///
    /// Returns:
    /// - 3 for IPv4 addresses
    /// - 4-6 for IPv6 addresses
    pub fn word_count<T: Into<AddressInput>>(&self, input: T) -> Result<usize> {
        let address_input = input.into();
        let address_str = address_input.to_address_string();
        
        let result = self.encoder.encode(&address_str)?;
        Ok(result.encoding.word_count)
    }

    /// Check if a string is a valid four-word address
    ///
    /// Returns true if the input is 3-6 dot-separated dictionary words.
    pub fn is_valid_words(&self, words: &str) -> bool {
        let parts: Vec<&str> = words.split('.').collect();
        
        // Must be 3-6 words
        if parts.len() < 3 || parts.len() > 6 {
            return false;
        }
        
        // All parts must be non-empty alphabetic strings
        parts.iter().all(|&part| {
            !part.is_empty() && part.chars().all(|c| c.is_alphabetic())
        })
    }

    /// Get information about an address encoding
    pub fn analyze<T: Into<AddressInput>>(&self, input: T) -> Result<EncodingInfo> {
        let address_input = input.into();
        let address_str = address_input.to_address_string();
        
        let analysis = self.encoder.analyze(&address_str)?;
        
        Ok(EncodingInfo {
            word_count: analysis.word_count,
            address_type: match analysis.address_type {
                crate::AddressType::Ipv4 => AddressType::Ipv4,
                crate::AddressType::Ipv6 => AddressType::Ipv6,
            },
            compression_ratio: analysis.compression_ratio,
            method: analysis.method,
        })
    }
}

impl Default for FourWordNetworking {
    fn default() -> Self {
        Self::new().expect("Failed to initialize Four-Word Networking")
    }
}

/// Input types that can be converted to addresses
pub enum AddressInput {
    /// A string representation of an address
    String(String),
    /// A socket address
    SocketAddr(SocketAddr),
    /// An IP address (will use port 0)
    IpAddr(IpAddr),
}

impl AddressInput {
    /// Convert to a string representation suitable for encoding
    fn to_address_string(&self) -> String {
        match self {
            AddressInput::String(s) => s.clone(),
            AddressInput::SocketAddr(addr) => addr.to_string(),
            AddressInput::IpAddr(ip) => match ip {
                IpAddr::V4(ipv4) => format!("{}:0", ipv4),
                IpAddr::V6(ipv6) => format!("[{}]:0", ipv6),
            },
        }
    }
}

// Implement conversions for convenience
impl From<String> for AddressInput {
    fn from(s: String) -> Self {
        AddressInput::String(s)
    }
}

impl From<&str> for AddressInput {
    fn from(s: &str) -> Self {
        AddressInput::String(s.to_string())
    }
}

impl From<SocketAddr> for AddressInput {
    fn from(addr: SocketAddr) -> Self {
        AddressInput::SocketAddr(addr)
    }
}

impl From<IpAddr> for AddressInput {
    fn from(ip: IpAddr) -> Self {
        AddressInput::IpAddr(ip)
    }
}

/// Address type enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AddressType {
    /// IPv4 address (always 4 words)
    Ipv4,
    /// IPv6 address (always 4-6 words)
    Ipv6,
}

impl AddressType {
    /// Get a description of the address type
    pub fn description(&self) -> &'static str {
        match self {
            AddressType::Ipv4 => "IPv4",
            AddressType::Ipv6 => "IPv6",
        }
    }
}

/// Information about an address encoding
#[derive(Debug, Clone)]
pub struct EncodingInfo {
    /// Number of words in the encoding
    pub word_count: usize,
    /// Type of address
    pub address_type: AddressType,
    /// Compression ratio achieved
    pub compression_ratio: f64,
    /// Compression method used
    pub method: String,
}

impl EncodingInfo {
    /// Get a human-readable summary
    pub fn summary(&self) -> String {
        format!(
            "{} address: {} words, {:.1}% compression via {}",
            self.address_type.description(),
            self.word_count,
            self.compression_ratio * 100.0,
            self.method
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_encoding() {
        let twn = FourWordNetworking::new().unwrap();
        
        // Test IPv4
        let words = twn.encode("192.168.1.1:80").unwrap();
        assert_eq!(words.split('.').count(), 3);
        
        // Test IPv6
        let words = twn.encode("[::1]:443").unwrap();
        assert_eq!(words.split('.').count(), 4); // Minimum 4 for IPv6
    }

    #[test]
    fn test_socket_addr_encoding() {
        let twn = FourWordNetworking::new().unwrap();
        
        let addr: SocketAddr = "192.168.1.1:80".parse().unwrap();
        let words = twn.encode(addr).unwrap();
        assert_eq!(words.split('.').count(), 3);
    }

    #[test]
    fn test_ip_addr_encoding() {
        let twn = FourWordNetworking::new().unwrap();
        
        let ip: IpAddr = "192.168.1.1".parse().unwrap();
        let words = twn.encode(ip).unwrap();
        assert_eq!(words.split('.').count(), 3);
    }

    #[test]
    fn test_word_validation() {
        let twn = FourWordNetworking::new().unwrap();
        
        assert!(twn.is_valid_words("ocean.thunder.falcon"));
        assert!(twn.is_valid_words("book.book.smell.book"));
        assert!(!twn.is_valid_words("ocean.thunder")); // Too few
        assert!(!twn.is_valid_words("192.168.1.1")); // Not words
    }

    #[test]
    fn test_word_count() {
        let twn = FourWordNetworking::new().unwrap();
        
        assert_eq!(twn.word_count("192.168.1.1:80").unwrap(), 3);
        assert!(twn.word_count("[::1]:443").unwrap() >= 4);
    }
}