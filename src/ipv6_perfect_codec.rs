//! IPv6 Perfect Codec - Advanced compression for IPv6 addresses
//!
//! This module implements aggressive compression for IPv6 addresses to fit within
//! the enhanced multi-dimensional encoding space.

use std::net::Ipv6Addr;
use crate::{
    perfect_encoder::{PerfectEncoder, MultiDimEncoding, Separator, CasePattern},
    ThreeWordError, Result,
};

/// IPv6 compression categories
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IPv6Category {
    Loopback,       // ::1
    Unspecified,    // ::
    LinkLocal,      // fe80::/10
    UniqueLocal,    // fc00::/7
    Multicast,      // ff00::/8
    IPv4Mapped,     // ::ffff:0:0/96
    Documentation,  // 2001:db8::/32
    GlobalUnicast,  // Everything else
}

/// Compressed IPv6 representation
pub struct CompressedIPv6 {
    category: IPv6Category,
    compressed_data: Vec<u8>,
    port: u16,
}

/// IPv6 codec with advanced compression
pub struct IPv6PerfectCodec {
    encoder: PerfectEncoder,
}

impl IPv6PerfectCodec {
    pub fn new() -> Result<Self> {
        Ok(Self {
            encoder: PerfectEncoder::new()?,
        })
    }
    
    /// Compress IPv6 address for encoding
    fn compress_ipv6(&self, ip: Ipv6Addr, port: u16) -> CompressedIPv6 {
        let segments = ip.segments();
        
        // Categorize the address
        let category = if ip == Ipv6Addr::UNSPECIFIED {
            IPv6Category::Unspecified
        } else if ip == Ipv6Addr::LOCALHOST {
            IPv6Category::Loopback
        } else if segments[0] >= 0xfe80 && segments[0] <= 0xfebf {
            IPv6Category::LinkLocal
        } else if segments[0] >= 0xfc00 && segments[0] <= 0xfdff {
            IPv6Category::UniqueLocal
        } else if segments[0] >= 0xff00 {
            IPv6Category::Multicast
        } else if segments[0] == 0 && segments[1] == 0 && segments[2] == 0 && 
                  segments[3] == 0 && segments[4] == 0 && segments[5] == 0xffff {
            IPv6Category::IPv4Mapped
        } else if segments[0] == 0x2001 && segments[1] == 0x0db8 {
            IPv6Category::Documentation
        } else {
            IPv6Category::GlobalUnicast
        };
        
        // Compress based on category
        let compressed_data = match category {
            IPv6Category::Unspecified => vec![],
            IPv6Category::Loopback => vec![],
            IPv6Category::LinkLocal => {
                // Store only the interface ID (last 64 bits)
                let mut data = Vec::new();
                for i in 4..8 {
                    data.extend_from_slice(&segments[i].to_be_bytes());
                }
                data
            }
            IPv6Category::UniqueLocal => {
                // Store subnet (40 bits) + interface ID (64 bits)
                let mut data = Vec::new();
                // Skip fc00, store next 40 bits
                data.push((segments[1] >> 8) as u8);
                data.push(segments[1] as u8);
                data.extend_from_slice(&segments[2].to_be_bytes());
                data.extend_from_slice(&segments[3].to_be_bytes());
                // Interface ID
                for i in 4..8 {
                    data.extend_from_slice(&segments[i].to_be_bytes());
                }
                data
            }
            IPv6Category::IPv4Mapped => {
                // Store only the IPv4 address (last 32 bits)
                let mut data = Vec::new();
                data.extend_from_slice(&segments[6].to_be_bytes());
                data.extend_from_slice(&segments[7].to_be_bytes());
                data
            }
            _ => {
                // For other categories, use hash-based compression
                // Store first 48 bits + last 48 bits
                let mut data = Vec::new();
                for i in 0..3 {
                    data.extend_from_slice(&segments[i].to_be_bytes());
                }
                for i in 5..8 {
                    data.extend_from_slice(&segments[i].to_be_bytes());
                }
                data
            }
        };
        
        CompressedIPv6 {
            category,
            compressed_data,
            port,
        }
    }
    
    /// Decompress IPv6 address
    fn decompress_ipv6(&self, compressed: &CompressedIPv6) -> Result<Ipv6Addr> {
        let segments = match compressed.category {
            IPv6Category::Unspecified => [0u16; 8],
            IPv6Category::Loopback => [0, 0, 0, 0, 0, 0, 0, 1],
            IPv6Category::LinkLocal => {
                if compressed.compressed_data.len() < 8 {
                    return Err(ThreeWordError::InvalidInput("Invalid link-local data".to_string()));
                }
                let mut segments = [0u16; 8];
                segments[0] = 0xfe80;
                for i in 0..4 {
                    segments[4 + i] = u16::from_be_bytes([
                        compressed.compressed_data[i * 2],
                        compressed.compressed_data[i * 2 + 1],
                    ]);
                }
                segments
            }
            IPv6Category::UniqueLocal => {
                if compressed.compressed_data.len() < 13 {
                    return Err(ThreeWordError::InvalidInput("Invalid unique-local data".to_string()));
                }
                let mut segments = [0u16; 8];
                segments[0] = 0xfc00;
                segments[1] = u16::from_be_bytes([compressed.compressed_data[0], compressed.compressed_data[1]]);
                segments[2] = u16::from_be_bytes([compressed.compressed_data[2], compressed.compressed_data[3]]);
                segments[3] = u16::from_be_bytes([compressed.compressed_data[4], compressed.compressed_data[5]]);
                for i in 0..4 {
                    segments[4 + i] = u16::from_be_bytes([
                        compressed.compressed_data[6 + i * 2],
                        compressed.compressed_data[6 + i * 2 + 1],
                    ]);
                }
                segments
            }
            IPv6Category::IPv4Mapped => {
                if compressed.compressed_data.len() < 4 {
                    return Err(ThreeWordError::InvalidInput("Invalid IPv4-mapped data".to_string()));
                }
                let mut segments = [0u16; 8];
                segments[5] = 0xffff;
                segments[6] = u16::from_be_bytes([compressed.compressed_data[0], compressed.compressed_data[1]]);
                segments[7] = u16::from_be_bytes([compressed.compressed_data[2], compressed.compressed_data[3]]);
                segments
            }
            _ => {
                // For hash-compressed addresses, we can only reconstruct approximately
                // This is a limitation we document
                return Err(ThreeWordError::InvalidInput(
                    "Cannot perfectly reconstruct hash-compressed IPv6 addresses".to_string()
                ));
            }
        };
        
        Ok(Ipv6Addr::from(segments))
    }
    
    /// Encode IPv6 address + port using enhanced multi-dimensional encoding
    pub fn encode(&self, ip: Ipv6Addr, port: u16) -> Result<MultiDimEncoding> {
        let compressed = self.compress_ipv6(ip, port);
        
        // Pack compressed data into bits we can encode
        let mut packed_bits = 0u64;
        
        // Category (3 bits) - shifted down by 1 to make room for IPv6 flag at bit 47
        packed_bits |= (compressed.category as u64) << 44;
        
        // Port (16 bits)
        packed_bits |= (compressed.port as u64) << 28;
        
        // Compressed data (up to 29 bits for simple cases)
        let data_bits = match compressed.category {
            IPv6Category::Unspecified | IPv6Category::Loopback => 0u64,
            IPv6Category::IPv4Mapped => {
                // 32 bits of IPv4 address, but we only have 29 bits
                // Use compression: store /24 network + 5 bits of host
                let ipv4_bits = u32::from_be_bytes([
                    compressed.compressed_data[0],
                    compressed.compressed_data[1],
                    compressed.compressed_data[2],
                    compressed.compressed_data[3],
                ]);
                (((ipv4_bits >> 8) << 5) | ((ipv4_bits & 0xFF) >> 3)) as u64
            }
            _ => {
                // Hash the compressed data to fit in available bits
                let mut hash = 0u64;
                for (i, &byte) in compressed.compressed_data.iter().enumerate() {
                    hash ^= (byte as u64) << ((i % 4) * 8);
                    hash = hash.rotate_left(13);
                }
                hash & 0x1FFFFFFF // 29 bits
            }
        };
        
        packed_bits |= data_bits;
        
        // Use enhanced encoding with all dimensions
        let mut encoding = self.encoder.encode_48_bits(packed_bits)?;
        
        // Force IPv6 to always use dashes and title case
        // This ensures visual distinction from IPv4
        encoding.separators = [Separator::Dash, Separator::Dash];
        encoding.case_patterns = [CasePattern::Title, CasePattern::Title, CasePattern::Title];
        
        Ok(encoding)
    }
    
    /// Decode from multi-dimensional encoding
    pub fn decode(&self, encoding: &MultiDimEncoding) -> Result<(Ipv6Addr, u16)> {
        // Decode bits
        let packed_bits = self.encoder.decode_48_bits(encoding)?;
        
        // Extract fields
        let category_bits = ((packed_bits >> 45) & 0x7) as u8;
        let port = ((packed_bits >> 29) & 0xFFFF) as u16;
        let data_bits = packed_bits & 0x1FFFFFFF;
        
        let category = match category_bits {
            0 => IPv6Category::Unspecified,
            1 => IPv6Category::Loopback,
            2 => IPv6Category::LinkLocal,
            3 => IPv6Category::UniqueLocal,
            4 => IPv6Category::Multicast,
            5 => IPv6Category::IPv4Mapped,
            6 => IPv6Category::Documentation,
            7 => IPv6Category::GlobalUnicast,
            _ => return Err(ThreeWordError::InvalidInput("Invalid category".to_string())),
        };
        
        // Reconstruct compressed data based on category
        let compressed_data = match category {
            IPv6Category::Unspecified | IPv6Category::Loopback => vec![],
            IPv6Category::IPv4Mapped => {
                // Reconstruct IPv4 address from compressed form
                let network = (data_bits >> 5) & 0xFFFFFF;
                let host_bits = (data_bits & 0x1F) << 3;
                vec![
                    (network >> 16) as u8,
                    (network >> 8) as u8,
                    network as u8,
                    host_bits as u8,
                ]
            }
            _ => {
                // For hash-compressed data, we can't perfectly reconstruct
                // Return empty data which will trigger an error in decompress
                vec![]
            }
        };
        
        let compressed = CompressedIPv6 {
            category,
            compressed_data,
            port,
        };
        
        let ip = self.decompress_ipv6(&compressed)?;
        Ok((ip, port))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_ipv6_special_addresses() {
        let codec = IPv6PerfectCodec::new().unwrap();
        
        // Test special addresses that can be perfectly reconstructed
        let test_cases = vec![
            (Ipv6Addr::UNSPECIFIED, 0),
            (Ipv6Addr::LOCALHOST, 80),
            (Ipv6Addr::LOCALHOST, 443),
        ];
        
        for (ip, port) in test_cases {
            let encoded = codec.encode(ip, port).unwrap();
            let (decoded_ip, decoded_port) = codec.decode(&encoded).unwrap();
            
            assert_eq!(ip, decoded_ip, "IP mismatch for {}", ip);
            assert_eq!(port, decoded_port, "Port mismatch for {}:{}", ip, port);
        }
    }
    
    #[test]
    fn test_ipv6_encoding_format() {
        let codec = IPv6PerfectCodec::new().unwrap();
        
        let encoded = codec.encode(Ipv6Addr::LOCALHOST, 443).unwrap();
        let encoded_str = encoded.to_string();
        
        // Should use dashes for IPv6
        assert!(encoded_str.contains('-'), "IPv6 should use dash separators");
        
        // Should have title case
        let parts: Vec<&str> = encoded_str.split('-').collect();
        for part in parts {
            let first_char = part.chars().next().unwrap();
            assert!(first_char.is_uppercase(), "IPv6 should use title case");
        }
    }
}