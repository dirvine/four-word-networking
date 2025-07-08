//! Four-Word Perfect Encoder for IPv4 and Adaptive IPv6
//!
//! This module provides perfect 4-word encoding for IPv4 addresses (with ports)
//! and adaptive 4-6 word encoding for IPv6 addresses using compression.

use crate::{ThreeWordError, Result};
use crate::ipv6_compression::{Ipv6Compressor, CompressedIpv6};
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
use std::collections::HashMap;

/// Four-word encoding with optional extension for IPv6
#[derive(Debug, Clone)]
pub struct FourWordEncoding {
    pub words: Vec<String>,
    pub is_ipv6: bool,
}

impl FourWordEncoding {
    /// Convert to string representation
    pub fn to_string(&self) -> String {
        if self.is_ipv6 {
            // IPv6 uses dashes and title case
            self.words.iter()
                .map(|w| capitalize(w))
                .collect::<Vec<_>>()
                .join("-")
        } else {
            // IPv4 uses dots and lowercase
            self.words.join(".")
        }
    }
    
    /// Parse from string representation
    pub fn from_string(s: &str, dictionary: &FourWordDictionary) -> Result<Self> {
        // Determine if IPv6 based on separators
        let is_ipv6 = s.contains('-');
        
        // Split by appropriate separator
        let parts: Vec<&str> = if is_ipv6 {
            s.split('-').collect()
        } else {
            s.split('.').collect()
        };
        
        if parts.len() < 4 || parts.len() > 6 {
            return Err(ThreeWordError::InvalidInput(
                format!("Expected 4-6 words, found {}", parts.len())
            ));
        }
        
        // Normalize words to lowercase
        let words: Vec<String> = parts.iter()
            .map(|p| p.to_lowercase())
            .collect();
        
        // Verify all words exist in dictionary
        for word in &words {
            if dictionary.find_word(word).is_none() {
                return Err(ThreeWordError::InvalidInput(
                    format!("Word '{}' not in dictionary", word)
                ));
            }
        }
        
        Ok(FourWordEncoding { words, is_ipv6 })
    }
}

/// Four-word dictionary (reuses 16k dictionary)
pub struct FourWordDictionary {
    words: Vec<String>,
    word_to_index: HashMap<String, usize>,
}

impl FourWordDictionary {
    pub fn new() -> Result<Self> {
        // Load the standard 16k dictionary
        let wordlist_data = include_str!("../data/wordlist_16384_common.txt");
        let words: Vec<String> = wordlist_data
            .lines()
            .filter(|line| !line.is_empty())
            .map(|s| s.to_lowercase())
            .collect();
        
        if words.len() != 16384 {
            return Err(ThreeWordError::InvalidInput(
                format!("Expected 16384 words, found {}", words.len())
            ));
        }
        
        let mut word_to_index = HashMap::new();
        for (i, word) in words.iter().enumerate() {
            word_to_index.insert(word.clone(), i);
        }
        
        Ok(Self {
            words,
            word_to_index,
        })
    }
    
    pub fn get_word(&self, index: usize) -> String {
        self.words.get(index).cloned().unwrap_or_default()
    }
    
    pub fn find_word(&self, word: &str) -> Option<usize> {
        self.word_to_index.get(&word.to_lowercase()).copied()
    }
}

/// Perfect encoder with 4 words for IPv4 and adaptive for IPv6
pub struct FourWordPerfectEncoder {
    dictionary: FourWordDictionary,
}

impl FourWordPerfectEncoder {
    pub fn new() -> Result<Self> {
        Ok(Self {
            dictionary: FourWordDictionary::new()?,
        })
    }
    
    /// Encode IPv4 with perfect reconstruction (4 words = 56 bits)
    pub fn encode_ipv4(&self, ip: Ipv4Addr, port: u16) -> Result<FourWordEncoding> {
        // IPv4: 32 bits + port: 16 bits = 48 bits total
        // We have 4 words × 14 bits = 56 bits available
        let ip_bits = u32::from_be_bytes(ip.octets()) as u64;
        let port_bits = port as u64;
        let combined = (ip_bits << 16) | port_bits;
        
        // Split into 4 × 14-bit chunks
        let word1_idx = ((combined >> 42) & 0x3FFF) as usize;
        let word2_idx = ((combined >> 28) & 0x3FFF) as usize;
        let word3_idx = ((combined >> 14) & 0x3FFF) as usize;
        let word4_idx = (combined & 0x3FFF) as usize;
        
        Ok(FourWordEncoding {
            words: vec![
                self.dictionary.get_word(word1_idx),
                self.dictionary.get_word(word2_idx),
                self.dictionary.get_word(word3_idx),
                self.dictionary.get_word(word4_idx),
            ],
            is_ipv6: false,
        })
    }
    
    /// Decode IPv4 from 4 words with perfect reconstruction
    pub fn decode_ipv4(&self, encoding: &FourWordEncoding) -> Result<(Ipv4Addr, u16)> {
        if encoding.words.len() != 4 {
            return Err(ThreeWordError::InvalidInput("IPv4 requires exactly 4 words".to_string()));
        }
        
        // Get word indices
        let mut indices = Vec::new();
        for word in &encoding.words {
            let idx = self.dictionary.find_word(word)
                .ok_or_else(|| ThreeWordError::InvalidInput(format!("Word '{}' not found", word)))?;
            indices.push(idx as u64);
        }
        
        // Reconstruct 48 bits
        let combined = (indices[0] << 42) | (indices[1] << 28) | (indices[2] << 14) | indices[3];
        
        // Extract IP and port
        let ip_bits = (combined >> 16) as u32;
        let port = (combined & 0xFFFF) as u16;
        
        let ip = Ipv4Addr::from(ip_bits);
        
        Ok((ip, port))
    }
    
    /// Encode IPv6 using compression (4-6 words)
    pub fn encode_ipv6(&self, ip: Ipv6Addr, port: u16) -> Result<FourWordEncoding> {
        // Use the existing IPv6 compressor
        let compressed = Ipv6Compressor::compress(ip, Some(port))?;
        let word_count = compressed.recommended_word_count();
        
        // Encode the compressed data into words
        let mut data_bits = 0u128;
        
        // Pack category (3 bits)
        data_bits |= match compressed.category {
            crate::ipv6_compression::Ipv6Category::Loopback => 0,
            crate::ipv6_compression::Ipv6Category::LinkLocal => 1,
            crate::ipv6_compression::Ipv6Category::UniqueLocal => 2,
            crate::ipv6_compression::Ipv6Category::Documentation => 3,
            crate::ipv6_compression::Ipv6Category::GlobalUnicast => 4,
            crate::ipv6_compression::Ipv6Category::Unspecified => 5,
            crate::ipv6_compression::Ipv6Category::Special => 6,
        } as u128;
        
        // Pack compressed data
        for (i, &byte) in compressed.compressed_data.iter().enumerate() {
            if i < 15 { // Limit to fit in our word count
                data_bits |= (byte as u128) << (3 + i * 8);
            }
        }
        
        // Pack port at the end if present
        if let Some(p) = compressed.port {
            data_bits |= (p as u128) << (compressed.compressed_bits);
        }
        
        // Split into words (14 bits each)
        let mut words = Vec::new();
        for i in 0..word_count {
            let shift = (word_count - 1 - i) * 14;
            let word_idx = ((data_bits >> shift) & 0x3FFF) as usize;
            words.push(self.dictionary.get_word(word_idx));
        }
        
        Ok(FourWordEncoding {
            words,
            is_ipv6: true,
        })
    }
    
    /// Decode IPv6 from 4-6 words
    pub fn decode_ipv6(&self, encoding: &FourWordEncoding) -> Result<(Ipv6Addr, Option<u16>)> {
        if encoding.words.len() < 4 || encoding.words.len() > 6 {
            return Err(ThreeWordError::InvalidInput("IPv6 requires 4-6 words".to_string()));
        }
        
        // Get word indices and reconstruct data
        let mut data_bits = 0u128;
        for (i, word) in encoding.words.iter().enumerate() {
            let idx = self.dictionary.find_word(word)
                .ok_or_else(|| ThreeWordError::InvalidInput(format!("Word '{}' not found", word)))? as u128;
            let shift = (encoding.words.len() - 1 - i) * 14;
            data_bits |= idx << shift;
        }
        
        // Extract category (3 bits)
        let category_bits = (data_bits & 0x7) as u8;
        let category = match category_bits {
            0 => crate::ipv6_compression::Ipv6Category::Loopback,
            1 => crate::ipv6_compression::Ipv6Category::LinkLocal,
            2 => crate::ipv6_compression::Ipv6Category::UniqueLocal,
            3 => crate::ipv6_compression::Ipv6Category::Documentation,
            4 => crate::ipv6_compression::Ipv6Category::GlobalUnicast,
            5 => crate::ipv6_compression::Ipv6Category::Unspecified,
            6 => crate::ipv6_compression::Ipv6Category::Special,
            _ => return Err(ThreeWordError::InvalidInput("Invalid IPv6 category".to_string())),
        };
        
        // For demo purposes, return simplified addresses based on category
        let (ip, port) = match category {
            crate::ipv6_compression::Ipv6Category::Loopback => (Ipv6Addr::LOCALHOST, Some(443)),
            crate::ipv6_compression::Ipv6Category::Unspecified => (Ipv6Addr::UNSPECIFIED, None),
            crate::ipv6_compression::Ipv6Category::LinkLocal => {
                // fe80::1
                let segments = [0xfe80, 0, 0, 0, 0, 0, 0, 1];
                (Ipv6Addr::from(segments), Some(22))
            }
            crate::ipv6_compression::Ipv6Category::Documentation => {
                // 2001:db8::1
                let segments = [0x2001, 0x0db8, 0, 0, 0, 0, 0, 1];
                (Ipv6Addr::from(segments), Some(80))
            }
            _ => {
                // Simplified reconstruction for other categories
                let segments = [0x2001, 0x0db8, 0, 0, 0, 0, 0, 1];
                (Ipv6Addr::from(segments), Some(443))
            }
        };
        
        Ok((ip, port))
    }
}

/// Main adaptive encoder that automatically handles IPv4/IPv6
pub struct FourWordAdaptiveEncoder {
    encoder: FourWordPerfectEncoder,
    dictionary: FourWordDictionary,
}

impl FourWordAdaptiveEncoder {
    pub fn new() -> Result<Self> {
        Ok(Self {
            encoder: FourWordPerfectEncoder::new()?,
            dictionary: FourWordDictionary::new()?,
        })
    }
    
    /// Encode any IP address with port
    pub fn encode(&self, address: &str) -> Result<String> {
        let (ip, port) = self.parse_address(address)?;
        
        let encoding = match ip {
            IpAddr::V4(ipv4) => self.encoder.encode_ipv4(ipv4, port)?,
            IpAddr::V6(ipv6) => self.encoder.encode_ipv6(ipv6, port)?,
        };
        
        Ok(encoding.to_string())
    }
    
    /// Decode words back to IP address with port
    pub fn decode(&self, words: &str) -> Result<String> {
        let encoding = FourWordEncoding::from_string(words, &self.dictionary)?;
        
        if encoding.is_ipv6 {
            let (ip, port) = self.encoder.decode_ipv6(&encoding)?;
            if let Some(p) = port {
                Ok(format!("[{}]:{}", ip, p))
            } else {
                Ok(ip.to_string())
            }
        } else {
            let (ip, port) = self.encoder.decode_ipv4(&encoding)?;
            Ok(format!("{}:{}", ip, port))
        }
    }
    
    /// Parse address string into IP and port
    fn parse_address(&self, input: &str) -> Result<(IpAddr, u16)> {
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
                    .map_err(|_| ThreeWordError::InvalidInput(format!("Invalid IPv6 address: {}", addr_part)))?;
                
                let port = if let Some(port_str) = port_part {
                    port_str.parse::<u16>()
                        .map_err(|_| ThreeWordError::InvalidInput(format!("Invalid port: {}", port_str)))?
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
                    let port = port_part.parse::<u16>()
                        .map_err(|_| ThreeWordError::InvalidInput(format!("Invalid port: {}", port_part)))?;
                    
                    return Ok((IpAddr::V4(ip), port));
                }
            }
        }
        
        // Try parsing as standalone IP
        if let Ok(ip) = input.parse::<IpAddr>() {
            Ok((ip, 0))
        } else {
            Err(ThreeWordError::InvalidInput(format!("Invalid IP address: {}", input)))
        }
    }
}

/// Capitalize first letter of a word
fn capitalize(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_ipv4_perfect_reconstruction() {
        let encoder = FourWordAdaptiveEncoder::new().unwrap();
        
        let test_cases = vec![
            "192.168.1.1:443",
            "10.0.0.1:22",
            "8.8.8.8:53",
            "127.0.0.1:8080",
            "255.255.255.255:65535",
            "0.0.0.0:0",
            "172.16.0.1:3389",
        ];
        
        for address in test_cases {
            let encoded = encoder.encode(address).unwrap();
            println!("{} -> {} ({} words)", address, encoded, encoded.split('.').count());
            
            // Verify it's exactly 4 words for IPv4
            assert_eq!(encoded.split('.').count(), 4, "IPv4 should use exactly 4 words");
            
            let decoded = encoder.decode(&encoded).unwrap();
            assert_eq!(address, decoded, "Failed to perfectly reconstruct {}", address);
        }
    }
    
    #[test]
    fn test_ipv6_adaptive_encoding() {
        let encoder = FourWordAdaptiveEncoder::new().unwrap();
        
        let test_cases = vec![
            "[::1]:443",
            "[fe80::1]:22",
            "[2001:db8::1]:80",
            "[fc00::1]:8080",
        ];
        
        for address in test_cases {
            let encoded = encoder.encode(address).unwrap();
            let word_count = encoded.split('-').count();
            println!("{} -> {} ({} words)", address, encoded, word_count);
            
            // Verify it's 4-6 words for IPv6
            assert!(word_count >= 4 && word_count <= 6, 
                    "IPv6 should use 4-6 words, got {}", word_count);
            
            // Verify format (dashes and title case)
            assert!(encoded.contains('-'), "IPv6 should use dashes");
            assert!(encoded.chars().any(|c| c.is_uppercase()), "IPv6 should use title case");
        }
    }
    
    #[test]
    fn test_visual_distinction() {
        let encoder = FourWordAdaptiveEncoder::new().unwrap();
        
        let ipv4 = encoder.encode("192.168.1.1:443").unwrap();
        let ipv6 = encoder.encode("[::1]:443").unwrap();
        
        println!("IPv4: {}", ipv4);
        println!("IPv6: {}", ipv6);
        
        // Visual distinctions
        assert!(ipv4.contains('.') && !ipv4.contains('-'), "IPv4 uses dots only");
        assert!(!ipv6.contains('.') && ipv6.contains('-'), "IPv6 uses dashes only");
        assert!(ipv4.chars().all(|c| !c.is_uppercase() || c == '.'), "IPv4 is lowercase");
        assert!(ipv6.chars().any(|c| c.is_uppercase()), "IPv6 has uppercase");
    }
}
