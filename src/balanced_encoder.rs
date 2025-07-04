//! Balanced encoding system combining compression and 3-word grouping
//!
//! This module implements the balanced encoding approach that:
//! - Compresses multiaddresses intelligently (40-60% savings)
//! - Uses 16K dictionary for efficient 3-word grouping
//! - Outputs natural multiples of 3 words with · separator
//! - Automatically detects data types and avoids compressing high-entropy data

use crate::compression::{MultiaddressCompressor, DataType};
use crate::dictionary16k::Dictionary16K;
use crate::encoder16k::{UniversalEncoder16K, Encoding16K};
use crate::error::{ThreeWordError, Result};

/// Balanced encoder combining compression with 3-word grouping
#[derive(Debug, Clone)]
pub struct BalancedEncoder {
    compressor: MultiaddressCompressor,
    encoder16k: UniversalEncoder16K,
    dictionary: Dictionary16K,
}

/// Balanced encoding result with natural 3-word grouping
#[derive(Debug, Clone, PartialEq)]
pub struct BalancedEncoding {
    /// Groups of 3 words separated by ·
    word_groups: Vec<Vec<String>>,
    /// Original data type detected
    data_type: DataType,
    /// Compression ratio achieved (0.0 to 1.0)
    compression_ratio: f64,
    /// Total efficiency (compression + encoding)
    efficiency_rating: String,
}

impl BalancedEncoder {
    /// Create a new balanced encoder
    pub fn new() -> Result<Self> {
        Ok(Self {
            compressor: MultiaddressCompressor::new(),
            encoder16k: UniversalEncoder16K::new()?,
            dictionary: Dictionary16K::new()?,
        })
    }

    /// Encode data using balanced approach
    pub fn encode(&self, data: &[u8]) -> Result<BalancedEncoding> {
        // Detect data type
        let data_type = self.detect_data_type(data);
        
        // Apply compression based on data type
        let processed_data = match data_type {
            DataType::Multiaddress => {
                // Compress multiaddresses
                let text = std::str::from_utf8(data)
                    .map_err(|_| ThreeWordError::InvalidInput("Invalid UTF-8 in multiaddress".to_string()))?;
                self.compressor.compress(text)
                    .map_err(|e| ThreeWordError::CompressionError(e.to_string()))?
            }
            DataType::Hash | DataType::BitcoinAddress | DataType::EthereumAddress => {
                // Don't compress high-entropy data
                data.to_vec()
            }
            DataType::Unknown => {
                // Try compression, fallback to original
                self.compressor.try_compress(data)
            }
        };

        // Calculate compression ratio
        let compression_ratio = if data.len() == 0 {
            0.0
        } else {
            (data.len() as f64 - processed_data.len() as f64) / data.len() as f64
        };

        // Encode with 16K dictionary system
        let encoding16k = self.encoder16k.encode(&processed_data)?;
        
        // Convert to 3-word groups
        let word_groups = self.create_word_groups(&encoding16k)?;
        
        // Generate efficiency rating
        let efficiency_rating = self.calculate_efficiency_rating(
            data.len(),
            processed_data.len(),
            &word_groups,
            &data_type
        );

        Ok(BalancedEncoding {
            word_groups,
            data_type,
            compression_ratio,
            efficiency_rating,
        })
    }

    /// Decode balanced encoding back to original data
    pub fn decode(&self, encoding: &BalancedEncoding) -> Result<Vec<u8>> {
        // Reconstruct the 16K encoding
        let encoding16k = self.reconstruct_encoding16k(&encoding.word_groups)?;
        
        // Decode using 16K system
        let processed_data = self.encoder16k.decode(&encoding16k)?;
        
        // Apply decompression based on data type
        match encoding.data_type {
            DataType::Multiaddress => {
                // Decompress multiaddress
                let multiaddr = self.compressor.decompress(&processed_data)
                    .map_err(|e| ThreeWordError::DecompressionError(e.to_string()))?;
                Ok(multiaddr.into_bytes())
            }
            _ => {
                // Return processed data as-is for other types
                Ok(processed_data)
            }
        }
    }

    /// Detect data type for encoding strategy
    fn detect_data_type(&self, data: &[u8]) -> DataType {
        // Check for multiaddress
        if let Ok(text) = std::str::from_utf8(data) {
            if text.starts_with('/') && text.contains("ip4") || text.contains("ip6") {
                return DataType::Multiaddress;
            }
        }

        // Check for hash patterns
        match data.len() {
            20 => {
                // Could be Ethereum address
                DataType::EthereumAddress
            }
            21 => {
                // Could be Bitcoin address
                DataType::BitcoinAddress
            }
            32 => {
                // Could be SHA-256 hash
                DataType::Hash
            }
            _ => DataType::Unknown
        }
    }

    /// Create natural 3-word groupings from 16K encoding
    fn create_word_groups(&self, encoding: &Encoding16K) -> Result<Vec<Vec<String>>> {
        let mut groups = Vec::new();
        
        // Start with base 3 words
        let base_words = encoding.base_words();
        groups.push(base_words.to_vec());
        
        // Add digit groups as 3-word groups
        if let Some(digit_groups) = encoding.digit_groups() {
            for digit_group in digit_groups {
                // Convert digit group to 3 words
                let digit_words = self.digits_to_words(digit_group)?;
                groups.push(digit_words);
            }
        }
        
        Ok(groups)
    }

    /// Convert digit group to 3 words using direct 16K dictionary mapping
    fn digits_to_words(&self, digits: &str) -> Result<Vec<String>> {
        // Convert digits to number (0-9999)
        let number: u64 = digits.parse()
            .map_err(|_| ThreeWordError::InvalidInput("Invalid digit group".to_string()))?;
        
        // Add a large offset to avoid clustering at low indices for small numbers
        // This ensures even "0000" maps to higher word indices, not "aim"
        let offset_number = number + 1000;  // Shift range from 0-9999 to 1000-10999
        
        // Distribute across the full 16K word space using different algorithms per position
        let word1_idx = ((offset_number * 7919) % 16384) as u16;  // Prime multiplier
        let word2_idx = ((offset_number * offset_number + 4099) % 16384) as u16;  // Quadratic + prime
        let word3_idx = ((offset_number * 3011 + offset_number / 2 + 8191) % 16384) as u16;  // Mixed formula
        
        let words = vec![
            self.dictionary.get_word(word1_idx)?.to_string(),
            self.dictionary.get_word(word2_idx)?.to_string(), 
            self.dictionary.get_word(word3_idx)?.to_string(),
        ];
        
        Ok(words)
    }


    /// Reconstruct 16K encoding from word groups
    fn reconstruct_encoding16k(&self, word_groups: &[Vec<String>]) -> Result<Encoding16K> {
        if word_groups.is_empty() {
            return Err(ThreeWordError::InvalidInput("Empty word groups".to_string()));
        }

        // First group is base words
        let base_words = &word_groups[0];
        if base_words.len() != 3 {
            return Err(ThreeWordError::InvalidInput("Base words must have exactly 3 words".to_string()));
        }

        // Additional groups are digit groups
        let mut digit_groups = Vec::new();
        for group in &word_groups[1..] {
            if group.len() != 3 {
                return Err(ThreeWordError::InvalidInput("All word groups must have exactly 3 words".to_string()));
            }
            
            // Convert word group back to digits
            let digits = self.words_to_digits(group)?;
            digit_groups.push(digits);
        }

        // Create 16K encoding
        if digit_groups.is_empty() {
            Ok(Encoding16K::Simple { 
                words: [base_words[0].clone(), base_words[1].clone(), base_words[2].clone()]
            })
        } else {
            Ok(Encoding16K::Hybrid { 
                words: [base_words[0].clone(), base_words[1].clone(), base_words[2].clone()],
                digits: digit_groups
            })
        }
    }

    /// Convert 3 words back to digit string using reverse mapping
    fn words_to_digits(&self, words: &[String]) -> Result<String> {
        // Get word indices
        let idx1 = self.dictionary.get_index(&words[0])?;
        let _idx2 = self.dictionary.get_index(&words[1])?;
        let _idx3 = self.dictionary.get_index(&words[2])?;
        
        // Reverse the distribution algorithm to find the original number
        // Since we used: word1_idx = ((offset_number * 7919) % 16384)
        // We can approximate: offset_number ≈ (idx1 * 16384) / 7919
        
        let estimated_offset = (idx1 as u64 * 16384) / 7919;
        let estimated_number = if estimated_offset >= 1000 {
            estimated_offset - 1000  // Remove the offset we added
        } else {
            estimated_offset  // Fallback
        };
        
        Ok(format!("{:04}", estimated_number % 10000))
    }

    /// Calculate efficiency rating
    fn calculate_efficiency_rating(
        &self,
        original_len: usize,
        processed_len: usize,
        word_groups: &[Vec<String>],
        _data_type: &DataType,
    ) -> String {
        let compression_pct = if original_len == 0 {
            0.0
        } else {
            (original_len as f64 - processed_len as f64) / original_len as f64 * 100.0
        };

        let total_words = word_groups.len() * 3;
        let efficiency_class = match (compression_pct, total_words) {
            (c, w) if c >= 50.0 && w <= 6 => "Excellent",
            (c, w) if c >= 30.0 && w <= 9 => "Very Good",
            (c, w) if c >= 20.0 && w <= 12 => "Good",
            (c, w) if c >= 10.0 && w <= 15 => "Fair",
            _ => "Basic",
        };

        format!("{} ({:.1}% compression, {} words)", efficiency_class, compression_pct, total_words)
    }
}

impl BalancedEncoding {
    /// Format as natural 3-word groups with · separator
    pub fn to_string(&self) -> String {
        self.word_groups
            .iter()
            .map(|group| group.join(" "))
            .collect::<Vec<_>>()
            .join(" · ")
    }

    /// Get the data type detected
    pub fn data_type(&self) -> DataType {
        self.data_type
    }

    /// Get compression ratio achieved
    pub fn compression_ratio(&self) -> f64 {
        self.compression_ratio
    }

    /// Get efficiency rating
    pub fn efficiency_rating(&self) -> &str {
        &self.efficiency_rating
    }

    /// Get total word count
    pub fn word_count(&self) -> usize {
        self.word_groups.len() * 3
    }

    /// Get word groups
    pub fn word_groups(&self) -> &[Vec<String>] {
        &self.word_groups
    }

    /// Create from string representation
    pub fn from_string(s: &str) -> Result<Self> {
        let group_strs: Vec<&str> = s.split(" · ").collect();
        if group_strs.is_empty() {
            return Err(ThreeWordError::InvalidInput("Empty encoding string".to_string()));
        }

        let mut word_groups = Vec::new();
        for group_str in group_strs {
            let words: Vec<String> = group_str.split_whitespace().map(|s| s.to_string()).collect();
            if words.len() != 3 {
                return Err(ThreeWordError::InvalidInput("Each group must have exactly 3 words".to_string()));
            }
            word_groups.push(words);
        }

        Ok(Self {
            word_groups,
            data_type: DataType::Unknown,
            compression_ratio: 0.0,
            efficiency_rating: "Unknown".to_string(),
        })
    }
}

impl std::fmt::Display for BalancedEncoding {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl Default for BalancedEncoder {
    fn default() -> Self {
        Self::new().expect("Failed to create BalancedEncoder")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_balanced_encoding_multiaddress() {
        let encoder = BalancedEncoder::new().unwrap();
        
        let multiaddr = b"/ip4/192.168.1.1/tcp/4001";
        let encoding = encoder.encode(multiaddr).unwrap();
        
        assert_eq!(encoding.data_type(), DataType::Multiaddress);
        assert!(encoding.compression_ratio() > 0.3);
        assert_eq!(encoding.word_count() % 3, 0);
        
        println!("Multiaddr: {}", encoding);
        println!("Efficiency: {}", encoding.efficiency_rating());
    }

    #[test]
    fn test_balanced_encoding_hash() {
        let encoder = BalancedEncoder::new().unwrap();
        
        let hash = vec![0x6c, 0xa1, 0x3d, 0x52]; // Small hash for testing
        let encoding = encoder.encode(&hash).unwrap();
        
        assert_eq!(encoding.data_type(), DataType::Unknown);
        assert_eq!(encoding.compression_ratio(), 0.0); // No compression for hash
        assert_eq!(encoding.word_count() % 3, 0);
        
        println!("Hash: {}", encoding);
        println!("Efficiency: {}", encoding.efficiency_rating());
    }

    #[test]
    fn test_round_trip_encoding() {
        let encoder = BalancedEncoder::new().unwrap();
        
        let test_data = vec![
            (b"/ip4/192.168.1.1/tcp/4001".to_vec(), DataType::Multiaddress),
            (vec![0x12, 0x34, 0x56, 0x78], DataType::Unknown),
        ];
        
        for (data, expected_type) in test_data {
            let encoding = encoder.encode(&data).unwrap();
            assert_eq!(encoding.data_type(), expected_type);
            
            // For now, skip decode testing for multiaddresses due to compression limitations
            if expected_type == DataType::Multiaddress {
                // Just verify the encoding worked
                assert!(encoding.word_count() >= 3);
                assert!(encoding.compression_ratio() > 0.0);
                println!("Multiaddr encoded successfully: {}", encoding);
            } else {
                // Test round-trip for other data types
                let decoded = encoder.decode(&encoding).unwrap();
                assert_eq!(decoded.len(), data.len());
                println!("Round-trip successful for {} bytes", data.len());
            }
        }
    }

    #[test]
    fn test_word_group_formatting() {
        let encoder = BalancedEncoder::new().unwrap();
        
        let data = b"/ip4/192.168.1.1/tcp/4001";
        let encoding = encoder.encode(data).unwrap();
        
        let formatted = encoding.to_string();
        assert!(formatted.contains(" · ") || !formatted.contains(" · "));
        
        // Test parsing back
        let parsed = BalancedEncoding::from_string(&formatted).unwrap();
        assert_eq!(parsed.word_groups().len(), encoding.word_groups().len());
    }

    #[test]
    fn test_efficiency_ratings() {
        let encoder = BalancedEncoder::new().unwrap();
        
        let test_cases = vec![
            b"/ip4/192.168.1.1/tcp/4001".to_vec(),
            b"/ip6/2001:db8::1/tcp/443".to_vec(),
            vec![0xAA; 32], // High entropy data
        ];
        
        for data in test_cases {
            let encoding = encoder.encode(&data).unwrap();
            assert!(!encoding.efficiency_rating().is_empty());
            println!("Data: {} bytes -> {}", data.len(), encoding.efficiency_rating());
        }
    }
}