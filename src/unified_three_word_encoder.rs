//! Unified Three-Word Network Encoder
//!
//! This module provides a unified interface for encoding network addresses
//! using groups of three words. It automatically selects the appropriate
//! encoding based on the address type:
//! - IPv4: 3 words (1 group)
//! - IPv6: 6 words (2 groups) or 9 words (3 groups)

use crate::three_word_encoder::{ThreeWordEncoder, ThreeWordEncoding};
use crate::three_word_ipv6_encoder::{Ipv6ThreeWordGroupEncoding, ThreeWordIpv6Encoder};
use crate::{FourWordError, Result};
use std::net::{IpAddr, Ipv6Addr, SocketAddr};
use std::str::FromStr;

/// Unified encoding result that can represent IPv4 or IPv6
#[derive(Debug, Clone, PartialEq)]
pub enum UnifiedThreeWordEncoding {
    /// IPv4 address (3 words)
    Ipv4(ThreeWordEncoding),
    /// IPv6 address (6 or 9 words)
    Ipv6(Ipv6ThreeWordGroupEncoding),
}

impl UnifiedThreeWordEncoding {
    /// Convert to string representation
    pub fn to_string(&self) -> String {
        match self {
            UnifiedThreeWordEncoding::Ipv4(encoding) => encoding.to_string(),
            UnifiedThreeWordEncoding::Ipv6(encoding) => encoding.to_string(),
        }
    }

    /// Get word count
    pub fn word_count(&self) -> usize {
        match self {
            UnifiedThreeWordEncoding::Ipv4(_) => 3,
            UnifiedThreeWordEncoding::Ipv6(encoding) => encoding.word_count(),
        }
    }

    /// Get group count
    pub fn group_count(&self) -> usize {
        self.word_count() / 3
    }

    /// Get the original IP address
    pub fn ip_address(&self) -> IpAddr {
        match self {
            UnifiedThreeWordEncoding::Ipv4(encoding) => IpAddr::V4(encoding.original_ip),
            UnifiedThreeWordEncoding::Ipv6(encoding) => match encoding {
                Ipv6ThreeWordGroupEncoding::SixWords { original_ip, .. }
                | Ipv6ThreeWordGroupEncoding::NineWords { original_ip, .. } => {
                    IpAddr::V6(*original_ip)
                }
            },
        }
    }

    /// Get the original port
    pub fn port(&self) -> u16 {
        match self {
            UnifiedThreeWordEncoding::Ipv4(encoding) => encoding.original_port,
            UnifiedThreeWordEncoding::Ipv6(encoding) => match encoding {
                Ipv6ThreeWordGroupEncoding::SixWords { original_port, .. }
                | Ipv6ThreeWordGroupEncoding::NineWords { original_port, .. } => *original_port,
            },
        }
    }

    /// Get human-readable description
    pub fn description(&self) -> String {
        match self {
            UnifiedThreeWordEncoding::Ipv4(_) => "IPv4 (3 words)".to_string(),
            UnifiedThreeWordEncoding::Ipv6(encoding) => match encoding {
                Ipv6ThreeWordGroupEncoding::SixWords { category, .. } => {
                    format!("IPv6 {category:?} (6 words)")
                }
                Ipv6ThreeWordGroupEncoding::NineWords { .. } => "IPv6 Full (9 words)".to_string(),
            },
        }
    }
}

/// Unified three-word encoder for all IP address types
pub struct UnifiedThreeWordEncoder {
    ipv4_encoder: ThreeWordEncoder,
    ipv6_encoder: ThreeWordIpv6Encoder,
}

impl UnifiedThreeWordEncoder {
    /// Create a new unified encoder
    pub fn new() -> Result<Self> {
        Ok(Self {
            ipv4_encoder: ThreeWordEncoder::new()?,
            ipv6_encoder: ThreeWordIpv6Encoder::new()?,
        })
    }

    /// Encode any IP address with port
    pub fn encode(&self, addr: SocketAddr) -> Result<UnifiedThreeWordEncoding> {
        match addr {
            SocketAddr::V4(v4) => {
                let encoded = self.ipv4_encoder.encode(*v4.ip(), v4.port())?;
                Ok(UnifiedThreeWordEncoding::Ipv4(encoded))
            }
            SocketAddr::V6(v6) => {
                let encoded = self.ipv6_encoder.encode(*v6.ip(), v6.port())?;
                Ok(UnifiedThreeWordEncoding::Ipv6(encoded))
            }
        }
    }

    /// Encode from string (auto-detects IPv4 vs IPv6)
    pub fn encode_string(&self, input: &str) -> Result<UnifiedThreeWordEncoding> {
        // Parse as socket address
        let addr = SocketAddr::from_str(input)
            .map_err(|e| FourWordError::InvalidInput(format!("Invalid address: {e}")))?;

        self.encode(addr)
    }

    /// Decode from unified encoding
    pub fn decode(&self, encoding: &UnifiedThreeWordEncoding) -> Result<SocketAddr> {
        match encoding {
            UnifiedThreeWordEncoding::Ipv4(enc) => {
                let words: Vec<&str> = enc.words().iter().map(|s| s.as_str()).collect();
                let (ip, port) = self.ipv4_encoder.decode(&words)?;
                Ok(SocketAddr::V4(std::net::SocketAddrV4::new(ip, port)))
            }
            UnifiedThreeWordEncoding::Ipv6(enc) => {
                let (ip, port) = self.ipv6_encoder.decode(enc)?;
                Ok(SocketAddr::V6(std::net::SocketAddrV6::new(ip, port, 0, 0)))
            }
        }
    }

    /// Decode from string (auto-detects format)
    pub fn decode_string(&self, input: &str) -> Result<SocketAddr> {
        // Count words to determine type
        let word_count = input
            .split_whitespace()
            .filter(|s| !s.is_empty() && *s != "|")
            .count();

        match word_count {
            3 => {
                // IPv4 format
                let (ip, port) = self.ipv4_encoder.decode_string(input)?;
                Ok(SocketAddr::V4(std::net::SocketAddrV4::new(ip, port)))
            }
            6 | 9 => {
                // IPv6 format - need to parse groups
                let groups = self.parse_ipv6_groups(input)?;
                let encoding = if groups.len() == 2 {
                    // 6 words
                    self.parse_six_word_encoding(&groups)?
                } else if groups.len() == 3 {
                    // 9 words
                    self.parse_nine_word_encoding(&groups)?
                } else {
                    return Err(FourWordError::InvalidInput(format!(
                        "Invalid group count: {}",
                        groups.len()
                    )));
                };

                let (ip, port) = self.ipv6_encoder.decode(&encoding)?;
                Ok(SocketAddr::V6(std::net::SocketAddrV6::new(ip, port, 0, 0)))
            }
            _ => Err(FourWordError::InvalidInput(format!(
                "Invalid word count: {word_count}. Expected 3, 6, or 9 words"
            ))),
        }
    }

    /// Parse IPv6 groups from string
    fn parse_ipv6_groups(&self, input: &str) -> Result<Vec<Vec<String>>> {
        let groups: Vec<Vec<String>> = input
            .split('|')
            .map(|group| {
                group
                    .split_whitespace()
                    .map(|s| s.trim_end_matches('.').to_string())
                    .collect()
            })
            .filter(|g: &Vec<String>| !g.is_empty())
            .collect();

        Ok(groups)
    }

    /// Parse six-word encoding from groups
    fn parse_six_word_encoding(
        &self,
        groups: &[Vec<String>],
    ) -> Result<Ipv6ThreeWordGroupEncoding> {
        use crate::ipv6_compression::Ipv6Category;
        use crate::three_word_ipv6_encoder::ThreeWordGroup;

        if groups.len() != 2 {
            return Err(FourWordError::InvalidInput(
                "Expected 2 groups for 6-word encoding".to_string(),
            ));
        }

        let group1 = ThreeWordGroup {
            words: [
                groups[0][0].clone(),
                groups[0][1].clone(),
                groups[0][2].clone(),
            ],
        };

        let group2 = ThreeWordGroup {
            words: [
                groups[1][0].clone(),
                groups[1][1].clone(),
                groups[1][2].clone(),
            ],
        };

        // We need to decode to get the actual values, but for now return a placeholder
        // In production, this would properly reconstruct the encoding
        Ok(Ipv6ThreeWordGroupEncoding::SixWords {
            groups: [group1, group2],
            original_ip: Ipv6Addr::UNSPECIFIED, // Will be set during decode
            original_port: 0,                   // Will be set during decode
            category: Ipv6Category::Unspecified, // Will be determined during decode
        })
    }

    /// Parse nine-word encoding from groups
    fn parse_nine_word_encoding(
        &self,
        groups: &[Vec<String>],
    ) -> Result<Ipv6ThreeWordGroupEncoding> {
        use crate::three_word_ipv6_encoder::ThreeWordGroup;

        if groups.len() != 3 {
            return Err(FourWordError::InvalidInput(
                "Expected 3 groups for 9-word encoding".to_string(),
            ));
        }

        let group1 = ThreeWordGroup {
            words: [
                groups[0][0].clone(),
                groups[0][1].clone(),
                groups[0][2].clone(),
            ],
        };

        let group2 = ThreeWordGroup {
            words: [
                groups[1][0].clone(),
                groups[1][1].clone(),
                groups[1][2].clone(),
            ],
        };

        let group3 = ThreeWordGroup {
            words: [
                groups[2][0].clone(),
                groups[2][1].clone(),
                groups[2][2].clone(),
            ],
        };

        Ok(Ipv6ThreeWordGroupEncoding::NineWords {
            groups: [group1, group2, group3],
            original_ip: Ipv6Addr::UNSPECIFIED, // Will be set during decode
            original_port: 0,                   // Will be set during decode
        })
    }
}

impl Default for UnifiedThreeWordEncoder {
    fn default() -> Self {
        Self::new().expect("Failed to create UnifiedThreeWordEncoder")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ipv4_encoding() {
        let encoder = UnifiedThreeWordEncoder::new().unwrap();
        let addr = SocketAddr::from_str("192.168.1.1:443").unwrap();

        let encoded = encoder.encode(addr).unwrap();
        assert_eq!(encoded.word_count(), 3);
        assert_eq!(encoded.group_count(), 1);

        let decoded = encoder.decode(&encoded).unwrap();
        assert_eq!(decoded, addr);
    }

    #[test]
    fn test_ipv6_loopback_encoding() {
        let encoder = UnifiedThreeWordEncoder::new().unwrap();
        let addr = SocketAddr::from_str("[::1]:80").unwrap();

        let encoded = encoder.encode(addr).unwrap();
        assert!(encoded.word_count() == 6 || encoded.word_count() == 9);
        assert!(encoded.group_count() == 2 || encoded.group_count() == 3);

        let decoded = encoder.decode(&encoded).unwrap();
        assert_eq!(decoded, addr);
    }

    #[test]
    fn test_string_encoding() {
        let encoder = UnifiedThreeWordEncoder::new().unwrap();

        // Test IPv4
        let encoded_v4 = encoder.encode_string("10.0.0.1:22").unwrap();
        assert_eq!(encoded_v4.word_count(), 3);

        // Test IPv6
        let encoded_v6 = encoder.encode_string("[2001:db8::1]:443").unwrap();
        assert!(encoded_v6.word_count() % 3 == 0); // Multiple of 3
    }

    #[test]
    fn test_unified_formatting() {
        let encoder = UnifiedThreeWordEncoder::new().unwrap();

        // IPv4
        let addr_v4 = SocketAddr::from_str("127.0.0.1:80").unwrap();
        let encoded_v4 = encoder.encode(addr_v4).unwrap();
        let formatted_v4 = encoded_v4.to_string();
        assert!(!formatted_v4.contains(" | ")); // No group separator for 3 words

        // IPv6
        let addr_v6 = SocketAddr::from_str("[::1]:443").unwrap();
        let encoded_v6 = encoder.encode(addr_v6).unwrap();
        let formatted_v6 = encoded_v6.to_string();
        assert!(formatted_v6.contains(" | ")); // Has group separator
    }
}
