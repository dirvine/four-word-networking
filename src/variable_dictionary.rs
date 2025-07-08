//! Variable-Length Dictionary System
//!
//! This module provides a dictionary system that can encode data into 3-6 words
//! based on the size requirements. This enables "Three-Word Networking" for IPv4
//! while supporting IPv6 with smart scaling to 4-6 words as needed.

use crate::{
    dictionary16k::Dictionary16K,
    error::ThreeWordError,
};

/// Number of bits per word in our 16K dictionary
const BITS_PER_WORD: usize = 14; // log2(16384) = 14 bits

/// Maximum bits that can be encoded in each word count
const MAX_BITS_3_WORDS: usize = 3 * BITS_PER_WORD; // 42 bits
const MAX_BITS_4_WORDS: usize = 4 * BITS_PER_WORD; // 56 bits  
const MAX_BITS_5_WORDS: usize = 5 * BITS_PER_WORD; // 70 bits
const MAX_BITS_6_WORDS: usize = 6 * BITS_PER_WORD; // 84 bits

/// Adaptive dictionary that selects optimal word count based on data size
pub struct VariableDictionary {
    dictionary: Dictionary16K,
}

impl VariableDictionary {
    pub fn new() -> Result<Self, ThreeWordError> {
        Ok(Self {
            dictionary: Dictionary16K::new()
                .map_err(|e| ThreeWordError::InvalidInput(e.to_string()))?,
        })
    }

    /// Encode data into the minimum number of words needed (3-6 words)
    pub fn encode_adaptive(&self, data: &[u8]) -> Result<AdaptiveEncoding, ThreeWordError> {
        let bit_count = data.len() * 8;
        
        // Determine minimum word count needed
        let word_count = if bit_count <= MAX_BITS_3_WORDS {
            3
        } else if bit_count <= MAX_BITS_4_WORDS {
            4
        } else if bit_count <= MAX_BITS_5_WORDS {
            5
        } else if bit_count <= MAX_BITS_6_WORDS {
            6
        } else {
            return Err(ThreeWordError::InvalidInput(
                format!("Data requires {} bits, exceeds maximum {} bits (6 words)", 
                       bit_count, MAX_BITS_6_WORDS)
            ));
        };

        // Encode with the determined word count
        let words = self.encode_fixed_length(data, word_count)?;
        
        Ok(AdaptiveEncoding {
            words,
            word_count,
            original_bits: bit_count,
            efficiency: 1.0 - (bit_count as f64 / (word_count * BITS_PER_WORD) as f64),
        })
    }

    /// Encode data into exactly the specified number of words
    pub fn encode_fixed_length(&self, data: &[u8], word_count: usize) -> Result<Vec<String>, ThreeWordError> {
        if word_count < 3 || word_count > 6 {
            return Err(ThreeWordError::InvalidInput(
                format!("Word count must be 3-6, got {}", word_count)
            ));
        }

        let max_bits = word_count * BITS_PER_WORD;
        let bit_count = data.len() * 8;

        if bit_count > max_bits {
            return Err(ThreeWordError::InvalidInput(
                format!("Data requires {} bits, but {} words can only hold {} bits", 
                       bit_count, word_count, max_bits)
            ));
        }

        // Pack data into the target bit space
        let packed_data = self.pack_to_target_bits(data, max_bits)?;
        
        // Convert packed data to bit value for variable-length encoding
        let bit_value = self.bytes_to_bits(&packed_data, max_bits);
        
        // Split the bits across the requested number of words
        let mut words = Vec::new();
        for i in 0..word_count {
            let shift = (word_count - 1 - i) * BITS_PER_WORD;
            let word_index = ((bit_value >> shift) & 0x3FFF) as u16;
            let word = self.dictionary.get_word(word_index)
                .map_err(|e| ThreeWordError::InvalidInput(e.to_string()))?;
            words.push(word.to_string());
        }

        Ok(words)
    }

    /// Decode words back to original data
    pub fn decode_adaptive(&self, words: &[String]) -> Result<Vec<u8>, ThreeWordError> {
        if words.len() < 3 || words.len() > 6 {
            return Err(ThreeWordError::InvalidInput(
                format!("Word count must be 3-6, got {}", words.len())
            ));
        }

        // Convert words back to indices and reconstruct the bit value
        let mut bit_value = 0u128;
        for (i, word) in words.iter().enumerate() {
            let index = self.dictionary.get_index(word)
                .map_err(|e| ThreeWordError::InvalidInput(e.to_string()))?;
            let shift = (words.len() - 1 - i) * BITS_PER_WORD;
            bit_value |= (index as u128) << shift;
        }

        // Convert bit value back to bytes
        let max_bits = words.len() * BITS_PER_WORD;
        let max_bytes = (max_bits + 7) / 8;
        let mut packed_data = Vec::new();
        
        for i in 0..max_bytes {
            let shift = (max_bytes - 1 - i) * 8;
            let byte = (bit_value >> shift) as u8;
            packed_data.push(byte);
        }

        // Unpack from the target bit space
        self.unpack_from_target_bits(&packed_data, max_bits)
    }

    /// Pack data to fit exactly in the target bit space
    fn pack_to_target_bits(&self, data: &[u8], target_bits: usize) -> Result<Vec<u8>, ThreeWordError> {
        let target_bytes = (target_bits + 7) / 8; // Round up to nearest byte
        let mut packed = vec![0u8; target_bytes];
        
        // Copy data, padding with zeros if necessary
        let copy_len = data.len().min(packed.len());
        packed[..copy_len].copy_from_slice(&data[..copy_len]);
        
        Ok(packed)
    }

    /// Unpack data from the target bit space back to original size
    fn unpack_from_target_bits(&self, packed_data: &[u8], _target_bits: usize) -> Result<Vec<u8>, ThreeWordError> {
        // For now, return the packed data as-is
        // In a more sophisticated implementation, we would store the original length
        // and trim to the exact original size
        Ok(packed_data.to_vec())
    }

    /// Convert bytes to a single bit value for encoding
    fn bytes_to_bits(&self, data: &[u8], max_bits: usize) -> u128 {
        let max_bytes = (max_bits + 7) / 8;
        let mut value = 0u128;
        
        // u128 can safely handle up to 16 bytes, but we limit to our max (84 bits = ~11 bytes)
        for (i, &byte) in data.iter().take(max_bytes.min(16)).enumerate() {
            let shift_amount = (max_bytes - 1 - i) * 8;
            if shift_amount < 128 {  // Safety check for u128
                value |= (byte as u128) << shift_amount;
            }
        }
        
        value
    }

    /// Get the optimal word count for a given bit count
    pub fn optimal_word_count(bit_count: usize) -> usize {
        if bit_count <= MAX_BITS_3_WORDS {
            3
        } else if bit_count <= MAX_BITS_4_WORDS {
            4
        } else if bit_count <= MAX_BITS_5_WORDS {
            5
        } else if bit_count <= MAX_BITS_6_WORDS {
            6
        } else {
            // More than 6 words needed - not supported
            7 // Indicates error condition
        }
    }

    /// Get capacity information for different word counts
    pub fn capacity_info() -> CapacityInfo {
        CapacityInfo {
            three_words: MAX_BITS_3_WORDS,
            four_words: MAX_BITS_4_WORDS,
            five_words: MAX_BITS_5_WORDS,
            six_words: MAX_BITS_6_WORDS,
        }
    }
}

/// Result of adaptive encoding showing word count and efficiency
#[derive(Debug, Clone)]
pub struct AdaptiveEncoding {
    pub words: Vec<String>,
    pub word_count: usize,
    pub original_bits: usize,
    pub efficiency: f64, // How much of the word space is used (higher is better)
}

impl AdaptiveEncoding {
    /// Get the encoded result as a dot-separated string
    pub fn to_string(&self) -> String {
        self.words.join(".")
    }

    /// Get a human-readable summary
    pub fn summary(&self) -> String {
        format!(
            "{} words ({} bits capacity, {:.1}% efficient)",
            self.word_count,
            self.word_count * BITS_PER_WORD,
            self.efficiency * 100.0
        )
    }

    /// Check if this is the optimal encoding (3 words for IPv4)
    pub fn is_optimal(&self) -> bool {
        self.word_count == 3
    }

    /// Get word count category
    pub fn category(&self) -> String {
        match self.word_count {
            3 => "Optimal (IPv4)".to_string(),
            4 => "Excellent (Common IPv6)".to_string(),
            5 => "Good (Standard IPv6)".to_string(),
            6 => "Complete (Full IPv6)".to_string(),
            _ => "Unknown".to_string(),
        }
    }
}

/// Information about the capacity of different word counts
#[derive(Debug)]
pub struct CapacityInfo {
    pub three_words: usize,  // 42 bits
    pub four_words: usize,   // 56 bits
    pub five_words: usize,   // 70 bits
    pub six_words: usize,    // 84 bits
}

impl CapacityInfo {
    pub fn describe(&self) -> String {
        format!(
            "Word Capacity:\n\
            3 words: {} bits (IPv4 + port)\n\
            4 words: {} bits (Common IPv6)\n\
            5 words: {} bits (Standard IPv6)\n\
            6 words: {} bits (Full IPv6 + port)",
            self.three_words,
            self.four_words, 
            self.five_words,
            self.six_words
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_adaptive_encoding() {
        let dict = VariableDictionary::new().unwrap();
        
        // Test small data (should use 3 words)
        let small_data = vec![0x12, 0x34, 0x56]; // 24 bits
        let encoding = dict.encode_adaptive(&small_data).unwrap();
        assert_eq!(encoding.word_count, 3);
        assert!(encoding.is_optimal());
        
        // Test larger data (should use more words)
        let large_data = vec![0u8; 7]; // 56 bits - fits exactly in 4 words
        let encoding = dict.encode_adaptive(&large_data).unwrap();
        assert_eq!(encoding.word_count, 4);
        assert!(!encoding.is_optimal());
    }

    #[test]
    fn test_fixed_length_encoding() {
        let dict = VariableDictionary::new().unwrap();
        
        // Test encoding small data into 4 words (should work with padding)
        let data = vec![0x12, 0x34];
        let words = dict.encode_fixed_length(&data, 4).unwrap();
        assert_eq!(words.len(), 4);
        
        // Test round-trip
        let decoded = dict.decode_adaptive(&words).unwrap();
        // Note: decoded might be longer due to padding, but should start with original data
        assert!(decoded.starts_with(&data));
    }

    #[test]
    fn test_word_count_selection() {
        assert_eq!(VariableDictionary::optimal_word_count(30), 3);  // IPv4
        assert_eq!(VariableDictionary::optimal_word_count(50), 4);  // Needs 4 words
        assert_eq!(VariableDictionary::optimal_word_count(65), 5);  // Needs 5 words
        assert_eq!(VariableDictionary::optimal_word_count(75), 6);  // Needs 6 words
    }

    #[test]
    fn test_capacity_info() {
        let info = VariableDictionary::capacity_info();
        assert_eq!(info.three_words, 42);
        assert_eq!(info.four_words, 56);
        assert_eq!(info.five_words, 70);
        assert_eq!(info.six_words, 84);
    }

    #[test]
    fn test_encoding_categories() {
        let dict = VariableDictionary::new().unwrap();
        
        let data3 = vec![0u8; 5]; // Should use 3 words
        let enc3 = dict.encode_adaptive(&data3).unwrap();
        assert_eq!(enc3.category(), "Optimal (IPv4)");
        
        let data4 = vec![0u8; 7]; // Should use 4 words  
        let enc4 = dict.encode_adaptive(&data4).unwrap();
        assert_eq!(enc4.category(), "Excellent (Common IPv6)");
    }
}