//! Three-Word Adaptive Encoder for IPv4 and IPv6
//!
//! This module provides perfect 3-word encoding for IPv4 addresses (with ports)
//! and adaptive 6 or 9 word encoding for IPv6 addresses using advanced pattern detection
//! and multi-dimensional encoding techniques.

use crate::three_word_encoder::ThreeWordEncoder;
use crate::three_word_ipv6_encoder::{Ipv6ThreeWordGroupEncoding, ThreeWordIpv6Encoder};
use crate::{FourWordError, Result};
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

/// Three-word encoding for IPv4 or groups of three for IPv6
#[derive(Debug, Clone)]
pub struct ThreeWordEncoding {
    pub words: Vec<String>,
    pub is_ipv6: bool,
}

impl ThreeWordEncoding {
    /// Convert to string representation
    pub fn to_string(&self) -> String {
        // All addresses use space-separated lowercase words
        self.words.join(" ")
    }

    /// Parse from string representation
    pub fn from_string(s: &str) -> Result<Self> {
        // Split by whitespace, dots, or dashes (backward compatibility)
        let parts: Vec<&str> = if s.contains('.') {
            s.split('.').collect()
        } else if s.contains('-') {
            s.split('-').collect()
        } else {
            s.split_whitespace().collect()
        };

        // Determine IPv4 vs IPv6 based on word count
        let is_ipv6 = parts.len() == 6 || parts.len() == 9;
        
        // IPv4 needs exactly 3 words, IPv6 needs 6 or 9 (groups of 3)
        let valid_count = match is_ipv6 {
            false => parts.len() == 3,
            true => parts.len() == 6 || parts.len() == 9,
        };

        if !valid_count {
            return Err(FourWordError::InvalidInput(format!(
                "Expected {} words, found {}",
                if is_ipv6 { "6 or 9" } else { "3" },
                parts.len()
            )));
        }

        // Normalize words to lowercase
        let words: Vec<String> = parts.iter().map(|p| p.to_lowercase()).collect();

        // Note: Word validation will happen during decoding
        Ok(ThreeWordEncoding { words, is_ipv6 })
    }
}

/// Perfect encoder with 3 words for IPv4 and 6/9 words for IPv6
pub struct ThreeWordPerfectEncoder {
    ipv4_encoder: ThreeWordEncoder,
    ipv6_encoder: ThreeWordIpv6Encoder,
}

impl ThreeWordPerfectEncoder {
    pub fn new() -> Result<Self> {
        Ok(Self {
            ipv4_encoder: ThreeWordEncoder::new()?,
            ipv6_encoder: ThreeWordIpv6Encoder::new()?,
        })
    }

    /// Encode IPv4 with perfect reconstruction (3 words)
    pub fn encode_ipv4(&self, ip: Ipv4Addr, port: Option<u16>) -> Result<ThreeWordEncoding> {
        // Use the ThreeWordEncoder for IPv4
        let encoding = self.ipv4_encoder.encode(ip, port.unwrap_or(80))?;

        Ok(ThreeWordEncoding {
            words: encoding.words().to_vec(),
            is_ipv6: false,
        })
    }

    /// Decode IPv4 from 3 words with perfect reconstruction
    pub fn decode_ipv4(&self, encoding: &ThreeWordEncoding) -> Result<(Ipv4Addr, Option<u16>)> {
        if encoding.words.len() != 3 {
            return Err(FourWordError::InvalidInput(
                "IPv4 requires exactly 3 words".to_string(),
            ));
        }

        // Use the ThreeWordEncoder for decoding
        let words: Vec<&str> = encoding.words.iter().map(|s| s.as_str()).collect();
        let (ip, port) = self.ipv4_encoder.decode(&words)?;

        Ok((ip, Some(port)))
    }

    /// Encode IPv6 using groups of 3 words (6 or 9 total)
    pub fn encode_ipv6(&self, ip: Ipv6Addr, port: u16) -> Result<ThreeWordEncoding> {
        // Use the ThreeWordIpv6Encoder
        let encoding = self.ipv6_encoder.encode(ip, port)?;

        // Extract all words from groups
        let mut all_words = Vec::new();
        match &encoding {
            Ipv6ThreeWordGroupEncoding::SixWords { groups, .. } => {
                for group in groups {
                    all_words.extend(group.words.iter().cloned());
                }
            }
            Ipv6ThreeWordGroupEncoding::NineWords { groups, .. } => {
                for group in groups {
                    all_words.extend(group.words.iter().cloned());
                }
            }
        }

        Ok(ThreeWordEncoding {
            words: all_words,
            is_ipv6: true,
        })
    }

    /// Decode IPv6 from 6 or 9 words (groups of 3)
    pub fn decode_ipv6(&self, encoding: &ThreeWordEncoding) -> Result<(Ipv6Addr, Option<u16>)> {
        if encoding.words.len() != 6 && encoding.words.len() != 9 {
            return Err(FourWordError::InvalidInput(
                "IPv6 requires exactly 6 or 9 words".to_string(),
            ));
        }

        // Use the ThreeWordIpv6Encoder for decoding
        let words: Vec<&str> = encoding.words.iter().map(|s| s.as_str()).collect();
        let (ip, port) = self.ipv6_encoder.decode_from_words(&words)?;

        Ok((ip, Some(port)))
    }
}

/// Main adaptive encoder that automatically handles IPv4/IPv6
///
/// This is the primary encoder for converting IP addresses to three-word format.
/// It automatically detects IPv4 vs IPv6 and applies the appropriate encoding strategy.
///
/// # Examples
///
/// ```
/// use three_word_networking::ThreeWordAdaptiveEncoder;
///
/// let encoder = ThreeWordAdaptiveEncoder::new()?;
///
/// // IPv4 encoding (always 3 words, perfect reconstruction)
/// let ipv4_words = encoder.encode("192.168.1.1:443")?;
/// assert_eq!(ipv4_words.split('.').count(), 3);
///
/// // IPv6 encoding (6 or 9 words, groups of 3)
/// let ipv6_words = encoder.encode("[::1]:443")?;
/// assert!(ipv6_words.split('-').count() == 6 || ipv6_words.split('-').count() == 9);
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub struct ThreeWordAdaptiveEncoder {
    encoder: ThreeWordPerfectEncoder,
}

impl ThreeWordAdaptiveEncoder {
    /// Create a new three-word adaptive encoder
    ///
    /// # Errors
    ///
    /// Returns an error if the dictionary fails to load
    pub fn new() -> Result<Self> {
        Ok(Self {
            encoder: ThreeWordPerfectEncoder::new()?,
        })
    }

    /// Encode any IP address with optional port into three words (IPv4) or groups of three (IPv6)
    ///
    /// # Arguments
    ///
    /// * `address` - An IP address string in any of these formats:
    ///   - IPv4: `"192.168.1.1"` or `"192.168.1.1:443"`
    ///   - IPv6: `"::1"`, `"[::1]:443"`, `"2001:db8::1"`, etc.
    ///
    /// # Returns
    ///
    /// A three-word string using dots for IPv4 or multi-word string with dashes for IPv6
    ///
    /// # Examples
    ///
    /// ```
    /// use three_word_networking::ThreeWordAdaptiveEncoder;
    ///
    /// let encoder = ThreeWordAdaptiveEncoder::new()?;
    ///
    /// // IPv4 without port
    /// let words = encoder.encode("192.168.1.1")?;
    /// assert_eq!(words.split('.').count(), 3);
    ///
    /// // IPv4 with port
    /// let words = encoder.encode("192.168.1.1:443")?;
    /// assert_eq!(words.split('.').count(), 3);
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

    /// Decode three words (IPv4) or groups of three (IPv6) back to IP address with optional port
    ///
    /// # Arguments
    ///
    /// * `words` - A word string in either format:
    ///   - IPv4: `"word.word.word"` (3 words, dots, lowercase)
    ///   - IPv6: `"Word-Word-Word-Word-Word-Word"` (6 or 9 words, dashes, title case)
    ///
    /// # Returns
    ///
    /// The original IP address string, with port if one was encoded
    ///
    /// # Examples
    ///
    /// ```
    /// use three_word_networking::ThreeWordAdaptiveEncoder;
    ///
    /// let encoder = ThreeWordAdaptiveEncoder::new()?;
    ///
    /// // Decode IPv4
    /// let ipv4_words = "lehr.delfs.enrages";
    /// let decoded = encoder.decode(ipv4_words)?;
    /// assert_eq!(decoded, "192.168.1.1:443");
    ///
    /// // Decode IPv6
    /// let ipv6_words = "Kaufhof-Dingley-Inno-Roupe-Stimuli-Bugger";
    /// let decoded = encoder.decode(ipv6_words)?;
    /// assert_eq!(decoded, "[::1]:443");
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn decode(&self, words: &str) -> Result<String> {
        let encoding = ThreeWordEncoding::from_string(words)?;

        if encoding.is_ipv6 {
            let (ip, port) = self.encoder.decode_ipv6(&encoding)?;
            if let Some(p) = port {
                Ok(format!("[{ip}]:{p}"))
            } else {
                Ok(ip.to_string())
            }
        } else {
            let (ip, port) = self.encoder.decode_ipv4(&encoding)?;
            if let Some(p) = port {
                Ok(format!("{ip}:{p}"))
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
                    FourWordError::InvalidInput(format!("Invalid IPv6 address: {addr_part}"))
                })?;

                let port = if let Some(port_str) = port_part {
                    Some(port_str.parse::<u16>().map_err(|_| {
                        FourWordError::InvalidInput(format!("Invalid port: {port_str}"))
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
                        FourWordError::InvalidInput(format!("Invalid port: {port_part}"))
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
                "Invalid IP address: {input}"
            )))
        }
    }
}

/// Capitalize first letter of a word

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ipv4_perfect_reconstruction() {
        let encoder = ThreeWordAdaptiveEncoder::new().unwrap();

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

            // Verify it's exactly 3 words for IPv4
            assert_eq!(
                encoded.split(' ').count(),
                3,
                "IPv4 should use exactly 3 words"
            );

            let decoded = encoder.decode(&encoded).unwrap();
            assert_eq!(
                address, decoded,
                "Failed to perfectly reconstruct {address}"
            );
        }
    }

    #[test]
    fn test_ipv6_adaptive_encoding() {
        let encoder = ThreeWordAdaptiveEncoder::new().unwrap();

        let test_cases = vec![
            "[::1]:443",
            "[fe80::1]:22",
            "[2001:db8::1]:80",
            "[fc00::1]:8080",
        ];

        for address in test_cases {
            let encoded = encoder.encode(address).unwrap();
            let word_count = encoded.split(' ').count();
            println!("{address} -> {encoded} ({word_count} words)");

            // Verify it's 6 or 9 words for IPv6
            assert!(
                word_count == 6 || word_count == 9,
                "IPv6 should use 6 or 9 words, got {word_count}"
            );
        }
    }

    #[test]
    fn test_word_count_distinction() {
        let encoder = ThreeWordAdaptiveEncoder::new().unwrap();

        let ipv4 = encoder.encode("192.168.1.1:443").unwrap();
        let ipv6 = encoder.encode("[::1]:443").unwrap();

        println!("IPv4: {ipv4}");
        println!("IPv6: {ipv6}");

        // Count distinctions
        assert_eq!(
            ipv4.split(' ').count(),
            3,
            "IPv4 uses exactly 3 words"
        );
        let ipv6_count = ipv6.split(' ').count();
        assert!(
            ipv6_count == 6 || ipv6_count == 9,
            "IPv6 uses 6 or 9 words"
        );
    }
}
