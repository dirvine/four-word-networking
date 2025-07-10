//! Four-Word Perfect Encoder for IPv4 and Perfect IPv6
//!
//! This module provides perfect 4-word encoding for IPv4 addresses (with ports)
//! and perfect 4-5 word encoding for IPv6 addresses using advanced pattern detection
//! and multi-dimensional encoding techniques.

use crate::ipv6_perfect_encoder::{IPv6PerfectEncoder, IPv6PerfectEncoding};
use crate::{FourWordError, Result};
use std::collections::HashMap;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

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
            self.words
                .iter()
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
            return Err(FourWordError::InvalidInput(format!(
                "Expected 4-6 words, found {}",
                parts.len()
            )));
        }

        // Normalize words to lowercase
        let words: Vec<String> = parts.iter().map(|p| p.to_lowercase()).collect();

        // Verify all words exist in dictionary
        for word in &words {
            if dictionary.find_word(word).is_none() {
                return Err(FourWordError::InvalidInput(format!(
                    "Word '{}' not in dictionary",
                    word
                )));
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
        // Load the standard 16k dictionary (now deduplicated)
        let wordlist_data = include_str!("../data/wordlist_16384_common.txt");
        let words: Vec<String> = wordlist_data
            .lines()
            .filter(|line| !line.is_empty())
            .map(|s| s.to_lowercase())
            .collect();

        if words.len() != 16384 {
            return Err(FourWordError::InvalidInput(format!(
                "Expected 16384 words, found {}",
                words.len()
            )));
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

/// Perfect encoder with 4 words for IPv4 and perfect 4-5 words for IPv6
pub struct FourWordPerfectEncoder {
    dictionary: FourWordDictionary,
    ipv6_encoder: IPv6PerfectEncoder,
}

impl FourWordPerfectEncoder {
    pub fn new() -> Result<Self> {
        Ok(Self {
            dictionary: FourWordDictionary::new()?,
            ipv6_encoder: IPv6PerfectEncoder::new()?,
        })
    }

    /// Encode IPv4 with perfect reconstruction (4 words = 56 bits)
    pub fn encode_ipv4(&self, ip: Ipv4Addr, port: Option<u16>) -> Result<FourWordEncoding> {
        // IPv4: 32 bits + port: 16 bits = 48 bits total
        // We have 4 words × 14 bits = 56 bits available
        // We use bit 48 as a flag: 1 = has port, 0 = no port
        let ip_bits = u32::from_be_bytes(ip.octets()) as u64;
        let (port_bits, has_port_flag) = match port {
            Some(p) => (p as u64, 1u64),
            None => (0u64, 0u64),
        };
        let combined = (has_port_flag << 48) | (ip_bits << 16) | port_bits;


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
    pub fn decode_ipv4(&self, encoding: &FourWordEncoding) -> Result<(Ipv4Addr, Option<u16>)> {
        if encoding.words.len() != 4 {
            return Err(FourWordError::InvalidInput(
                "IPv4 requires exactly 4 words".to_string(),
            ));
        }

        // Get word indices
        let mut indices = Vec::new();
        for word in &encoding.words {
            let idx = self
                .dictionary
                .find_word(word)
                .ok_or_else(|| FourWordError::InvalidInput(format!("Word '{}' not found", word)))?;
            indices.push(idx as u64);
        }


        // Reconstruct bits
        let combined = (indices[0] << 42) | (indices[1] << 28) | (indices[2] << 14) | indices[3];


        // Check port flag (bit 48)
        let has_port = (combined >> 48) & 1 == 1;

        // Extract IP and port
        let ip_bits = ((combined >> 16) & 0xFFFFFFFF) as u32;
        let ip = Ipv4Addr::from(ip_bits);

        let port = if has_port {
            Some((combined & 0xFFFF) as u16)
        } else {
            None
        };


        Ok((ip, port))
    }

    /// Encode IPv6 using perfect compression (4-5 words)
    pub fn encode_ipv6(&self, ip: Ipv6Addr, port: u16) -> Result<FourWordEncoding> {
        // Use the new IPv6 perfect encoder
        let perfect_encoding = self.ipv6_encoder.encode(ip, Some(port))?;

        // Convert IPv6PerfectEncoding to FourWordEncoding
        let encoding_str = perfect_encoding.to_string();

        // Parse the multi-dimensional encoding
        let parts: Vec<&str> = encoding_str.split('-').collect();
        let words: Vec<String> = parts.iter().map(|s| s.to_lowercase()).collect();

        Ok(FourWordEncoding {
            words,
            is_ipv6: true,
        })
    }

    /// Decode IPv6 from 4-6 words using perfect reconstruction
    pub fn decode_ipv6(&self, encoding: &FourWordEncoding) -> Result<(Ipv6Addr, Option<u16>)> {
        if encoding.words.len() < 4 || encoding.words.len() > 6 {
            return Err(FourWordError::InvalidInput(
                "IPv6 requires 4-6 words".to_string(),
            ));
        }

        // Convert FourWordEncoding back to IPv6PerfectEncoding format
        let words_str = encoding
            .words
            .iter()
            .map(|w| capitalize(w))
            .collect::<Vec<_>>()
            .join("-");

        // Parse using the IPv6 multi-dimensional decoder
        use crate::ipv6_multi_dimensional::{IPv6Dictionary, IPv6MultiDimEncoding};
        let ipv6_dict = IPv6Dictionary::new()?;
        let multi_dim_encoding = IPv6MultiDimEncoding::from_string(&words_str, &ipv6_dict)?;

        // Create IPv6PerfectEncoding from the multi-dimensional encoding
        // The pattern is stored in the multi-dimensional encoding
        let perfect_encoding = IPv6PerfectEncoding {
            encoding: multi_dim_encoding.clone(),
            pattern: multi_dim_encoding.pattern,
            word_count: encoding.words.len(),
            compression_ratio: 0.0, // Not needed for decoding
            is_perfect: true,
        };

        // Use the IPv6 perfect encoder to decode
        self.ipv6_encoder.decode(&perfect_encoding)
    }
}

/// Main adaptive encoder that automatically handles IPv4/IPv6
///
/// This is the primary encoder for converting IP addresses to four-word format.
/// It automatically detects IPv4 vs IPv6 and applies the appropriate encoding strategy.
///
/// # Examples
///
/// ```
/// use four_word_networking::FourWordAdaptiveEncoder;
///
/// let encoder = FourWordAdaptiveEncoder::new()?;
///
/// // IPv4 encoding (always 4 words, perfect reconstruction)
/// let ipv4_words = encoder.encode("192.168.1.1:443")?;
/// assert_eq!(ipv4_words.split('.').count(), 4);
///
/// // IPv6 encoding (4-6 words, adaptive compression)
/// let ipv6_words = encoder.encode("[::1]:443")?;
/// assert!(ipv6_words.split('-').count() >= 4);
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub struct FourWordAdaptiveEncoder {
    encoder: FourWordPerfectEncoder,
    dictionary: FourWordDictionary,
}

impl FourWordAdaptiveEncoder {
    /// Create a new four-word adaptive encoder
    ///
    /// # Errors
    ///
    /// Returns an error if the dictionary fails to load
    pub fn new() -> Result<Self> {
        Ok(Self {
            encoder: FourWordPerfectEncoder::new()?,
            dictionary: FourWordDictionary::new()?,
        })
    }

    /// Encode any IP address with optional port into four words
    ///
    /// # Arguments
    ///
    /// * `address` - An IP address string in any of these formats:
    ///   - IPv4: `"192.168.1.1"` or `"192.168.1.1:443"`
    ///   - IPv6: `"::1"`, `"[::1]:443"`, `"2001:db8::1"`, etc.
    ///
    /// # Returns
    ///
    /// A four-word string using dots for IPv4 or dashes for IPv6
    ///
    /// # Examples
    ///
    /// ```
    /// use four_word_networking::FourWordAdaptiveEncoder;
    ///
    /// let encoder = FourWordAdaptiveEncoder::new()?;
    ///
    /// // IPv4 without port
    /// let words = encoder.encode("192.168.1.1")?;
    /// assert_eq!(words.split('.').count(), 4);
    ///
    /// // IPv4 with port
    /// let words = encoder.encode("192.168.1.1:443")?;
    /// assert_eq!(words.split('.').count(), 4);
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn encode(&self, address: &str) -> Result<String> {
        let (ip, port) = self.parse_address(address)?;

        let encoding = match ip {
            IpAddr::V4(ipv4) => self.encoder.encode_ipv4(ipv4, port)?,
            IpAddr::V6(ipv6) => self.encoder.encode_ipv6(ipv6, port.unwrap_or(0))?,
        };

        Ok(encoding.to_string())
    }

    /// Decode four words back to IP address with optional port
    ///
    /// # Arguments
    ///
    /// * `words` - A four-word string in either format:
    ///   - IPv4: `"word.word.word.word"` (dots, lowercase)
    ///   - IPv6: `"Word-Word-Word-Word"` (dashes, title case)
    ///
    /// # Returns
    ///
    /// The original IP address string, with port if one was encoded
    ///
    /// # Examples
    ///
    /// ```
    /// use four_word_networking::FourWordAdaptiveEncoder;
    ///
    /// let encoder = FourWordAdaptiveEncoder::new()?;
    ///
    /// // Decode IPv4
    /// let ipv4_words = "paper.broaden.smith.book";
    /// let decoded = encoder.decode(ipv4_words)?;
    /// assert_eq!(decoded, "192.168.1.1");
    ///
    /// // Decode IPv6
    /// let ipv6_words = "Bully-Book-Book-Book";
    /// let decoded = encoder.decode(ipv6_words)?;
    /// assert_eq!(decoded, "[::1]:443");
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
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
            if let Some(p) = port {
                Ok(format!("{}:{}", ip, p))
            } else {
                Ok(ip.to_string())
            }
        }
    }

    /// Parse address string into IP and port
    fn parse_address(&self, input: &str) -> Result<(IpAddr, Option<u16>)> {
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
                    Some(port_str.parse::<u16>().map_err(|_| {
                        FourWordError::InvalidInput(format!("Invalid port: {}", port_str))
                    })?)
                } else {
                    None
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

                    return Ok((IpAddr::V4(ip), Some(port)));
                }
            }
        }

        // Try parsing as standalone IP
        if let Ok(ip) = input.parse::<IpAddr>() {
            Ok((ip, None))
        } else {
            Err(FourWordError::InvalidInput(format!(
                "Invalid IP address: {}",
                input
            )))
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
            println!(
                "{} -> {} ({} words)",
                address,
                encoded,
                encoded.split('.').count()
            );

            // Verify it's exactly 4 words for IPv4
            assert_eq!(
                encoded.split('.').count(),
                4,
                "IPv4 should use exactly 4 words"
            );

            let decoded = encoder.decode(&encoded).unwrap();
            assert_eq!(
                address, decoded,
                "Failed to perfectly reconstruct {}",
                address
            );
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
            assert!(
                word_count >= 4 && word_count <= 6,
                "IPv6 should use 4-6 words, got {}",
                word_count
            );

            // Verify format (dashes and title case)
            assert!(encoded.contains('-'), "IPv6 should use dashes");
            assert!(
                encoded.chars().any(|c| c.is_uppercase()),
                "IPv6 should use title case"
            );
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
        assert!(
            ipv4.contains('.') && !ipv4.contains('-'),
            "IPv4 uses dots only"
        );
        assert!(
            !ipv6.contains('.') && ipv6.contains('-'),
            "IPv6 uses dashes only"
        );
        assert!(
            ipv4.chars().all(|c| !c.is_uppercase() || c == '.'),
            "IPv4 is lowercase"
        );
        assert!(ipv6.chars().any(|c| c.is_uppercase()), "IPv6 has uppercase");
    }
}
