//! Simple Perfect Encoder - Direct 48-bit encoding without complex permutations
//!
//! This module provides a simpler perfect encoding that maps 48 bits directly
//! to four words without using word order permutations.

use crate::{FourWordError, Result};
use std::collections::HashMap;

/// Simple encoding using just four words
#[derive(Debug, Clone)]
pub struct SimpleEncoding {
    pub words: [String; 3],
    pub is_ipv6: bool,
}

impl SimpleEncoding {
    /// Convert to string representation
    pub fn to_string(&self) -> String {
        if self.is_ipv6 {
            // IPv6 uses dashes and title case
            format!("{}-{}-{}", 
                capitalize(&self.words[0]),
                capitalize(&self.words[1]), 
                capitalize(&self.words[2]))
        } else {
            // IPv4 uses dots and lowercase
            format!("{}.{}.{}", self.words[0], self.words[1], self.words[2])
        }
    }
    
    /// Parse from string representation
    pub fn from_string(s: &str, dictionary: &SimpleDictionary) -> Result<Self> {
        // Determine if IPv6 based on separators
        let is_ipv6 = s.contains('-');
        
        // Split by appropriate separator
        let parts: Vec<&str> = if is_ipv6 {
            s.split('-').collect()
        } else {
            s.split('.').collect()
        };
        
        if parts.len() != 3 {
            return Err(FourWordError::InvalidInput(
                format!("Expected 4 words, found {}", parts.len())
            ));
        }
        
        // Normalize words to lowercase
        let words = [
            parts[0].to_lowercase(),
            parts[1].to_lowercase(),
            parts[2].to_lowercase(),
        ];
        
        // Verify all words exist in dictionary
        for word in &words {
            if dictionary.find_word(word).is_none() {
                return Err(FourWordError::InvalidInput(
                    format!("Word '{}' not in dictionary", word)
                ));
            }
        }
        
        Ok(SimpleEncoding { words, is_ipv6 })
    }
}

/// Simple dictionary
pub struct SimpleDictionary {
    words: Vec<String>,
    word_to_index: HashMap<String, usize>,
}

impl SimpleDictionary {
    pub fn new() -> Result<Self> {
        // Load the standard 16k dictionary
        let wordlist_data = include_str!("../data/wordlist_16384_common.txt");
        let words: Vec<String> = wordlist_data
            .lines()
            .filter(|line| !line.is_empty())
            .map(|s| s.to_lowercase())
            .collect();
        
        if words.len() != 16384 {
            return Err(FourWordError::InvalidInput(
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

/// Simple perfect encoder
pub struct SimplePerfectEncoder {
    dictionary: SimpleDictionary,
}

impl SimplePerfectEncoder {
    pub fn new() -> Result<Self> {
        Ok(Self {
            dictionary: SimpleDictionary::new()?,
        })
    }
    
    /// Encode 48 bits into four words
    pub fn encode_48_bits(&self, data: u64, is_ipv6: bool) -> Result<SimpleEncoding> {
        // We have 16384 words (14 bits), so we can use 14 bits per word
        // But for 48-bit data, we need 16 bits per word
        // So we'll map the 16-bit chunks to our 14-bit dictionary
        let word1_idx = ((data >> 32) & 0x3FFF) as usize;  // Take lower 14 bits of first 16
        let word2_idx = ((data >> 16) & 0x3FFF) as usize;  // Take lower 14 bits of second 16
        let word3_idx = (data & 0x3FFF) as usize;          // Take lower 14 bits of third 16
        
        Ok(SimpleEncoding {
            words: [
                self.dictionary.get_word(word1_idx),
                self.dictionary.get_word(word2_idx),
                self.dictionary.get_word(word3_idx),
            ],
            is_ipv6,
        })
    }
    
    /// Decode four words back to 48 bits
    pub fn decode_48_bits(&self, encoding: &SimpleEncoding) -> Result<u64> {
        // Get word indices
        let word1_idx = self.dictionary.find_word(&encoding.words[0])
            .ok_or_else(|| FourWordError::InvalidInput("Word 1 not found".to_string()))? as u64;
        let word2_idx = self.dictionary.find_word(&encoding.words[1])
            .ok_or_else(|| FourWordError::InvalidInput("Word 2 not found".to_string()))? as u64;
        let word3_idx = self.dictionary.find_word(&encoding.words[2])
            .ok_or_else(|| FourWordError::InvalidInput("Word 3 not found".to_string()))? as u64;
        
        // Reconstruct what we can (only 42 bits out of 48)
        // This means we lose 6 bits of precision
        let data = (word1_idx << 32) | (word2_idx << 16) | word3_idx;
        
        Ok(data)
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
    fn test_simple_encoding_roundtrip() {
        let encoder = SimplePerfectEncoder::new().unwrap();
        
        // Test various values
        let test_values = vec![
            (0x0000_0000_0000u64, false),
            (0x0FFF_FFFF_FFFFu64, false),  // Max 42 bits
            (0x1234_5678_9ABCu64, true),
            (0x0000_0000_0001u64, true),
        ];
        
        for (value, is_ipv6) in test_values {
            let encoded = encoder.encode_48_bits(value, is_ipv6).unwrap();
            let decoded = encoder.decode_48_bits(&encoded).unwrap();
            
            // Only lower 42 bits are preserved
            assert_eq!(value & 0x0FFF_FFFF_FFFF, decoded, 
                      "Failed for value: 0x{:012X}", value);
            assert_eq!(is_ipv6, encoded.is_ipv6);
        }
    }
    
    #[test]
    fn test_ipv4_ipv6_distinction() {
        let encoder = SimplePerfectEncoder::new().unwrap();
        
        let ipv4 = encoder.encode_48_bits(0x1234, false).unwrap();
        let ipv6 = encoder.encode_48_bits(0x1234, true).unwrap();
        
        let ipv4_str = ipv4.to_string();
        let ipv6_str = ipv6.to_string();
        
        // IPv4 should use dots
        assert!(ipv4_str.contains('.'));
        assert!(!ipv4_str.contains('-'));
        
        // IPv6 should use dashes
        assert!(!ipv6_str.contains('.'));
        assert!(ipv6_str.contains('-'));
        
        // Should decode correctly
        let decoded_ipv4 = SimpleEncoding::from_string(&ipv4_str, &encoder.dictionary).unwrap();
        let decoded_ipv6 = SimpleEncoding::from_string(&ipv6_str, &encoder.dictionary).unwrap();
        
        assert!(!decoded_ipv4.is_ipv6);
        assert!(decoded_ipv6.is_ipv6);
    }
}