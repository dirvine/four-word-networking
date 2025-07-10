//! 16,384 word dictionary for Universal Word Encoding
//!
//! This module provides a high-quality dictionary of exactly 16,384 words
//! selected from multiple sources including EFF, BIP39, and curated wordlists.
//! Each word can be represented with exactly 14 bits (2^14 = 16,384).

use std::collections::HashMap;
use std::ptr;
use std::sync::Once;

/// Number of words in the dictionary (2^14)
pub const WORD_COUNT: usize = 16_384;

/// Number of bits needed per word
pub const BITS_PER_WORD: usize = 14;

/// Total bits covered by 3 words
pub const BITS_FOUR_WORDS: usize = 42;

/// Maximum word index (0-based)
pub const MAX_WORD_INDEX: u16 = (WORD_COUNT - 1) as u16;

/// Error types for dictionary operations
#[derive(Debug, thiserror::Error)]
pub enum DictionaryError {
    #[error("Invalid word index: {0} (max: {1})")]
    InvalidIndex(u16, u16),

    #[error("Unknown word: {0}")]
    UnknownWord(String),

    #[error("Dictionary not initialized")]
    NotInitialized,

    #[error("Invalid word count: expected {expected}, got {actual}")]
    InvalidWordCount { expected: usize, actual: usize },
}

/// High-performance dictionary using embedded wordlist
#[derive(Debug, Clone)]
pub struct Dictionary16K {
    words: Vec<String>,
    word_to_index: HashMap<String, u16>,
}

impl Dictionary16K {
    /// Create a new dictionary instance
    pub fn new() -> Result<Self, DictionaryError> {
        // Load the pre-built word list (common English words prioritized)
        let wordlist_data = include_str!("../data/wordlist_16384_common.txt");
        let words: Vec<String> = wordlist_data
            .lines()
            .map(|s| s.trim().to_string())
            .collect();

        // Validate word count
        if words.len() != WORD_COUNT {
            return Err(DictionaryError::InvalidWordCount {
                expected: WORD_COUNT,
                actual: words.len(),
            });
        }

        // Build reverse lookup map
        let word_to_index = words
            .iter()
            .enumerate()
            .map(|(i, word)| (word.clone(), i as u16))
            .collect();

        Ok(Self {
            words,
            word_to_index,
        })
    }

    /// Get word by index (0-based)
    pub fn get_word(&self, index: u16) -> Result<&str, DictionaryError> {
        if index > MAX_WORD_INDEX {
            return Err(DictionaryError::InvalidIndex(index, MAX_WORD_INDEX));
        }

        self.words
            .get(index as usize)
            .map(|s| s.as_str())
            .ok_or(DictionaryError::InvalidIndex(index, MAX_WORD_INDEX))
    }

    /// Get index by word
    pub fn get_index(&self, word: &str) -> Result<u16, DictionaryError> {
        self.word_to_index
            .get(word)
            .copied()
            .ok_or_else(|| DictionaryError::UnknownWord(word.to_string()))
    }

    /// Get total word count
    pub fn len(&self) -> usize {
        self.words.len()
    }

    /// Check if dictionary is empty (should never be true)
    pub fn is_empty(&self) -> bool {
        self.words.is_empty()
    }

    /// Get a sample of words for display
    pub fn sample_words(&self, count: usize) -> Vec<&str> {
        let step = self.words.len() / count.min(self.words.len());
        self.words
            .iter()
            .step_by(step.max(1))
            .take(count)
            .map(|s| s.as_str())
            .collect()
    }

    /// Check if a word exists in the dictionary
    pub fn contains_word(&self, word: &str) -> bool {
        self.word_to_index.contains_key(word)
    }

    /// Get dictionary statistics
    pub fn stats(&self) -> DictionaryStats {
        let mut length_distribution = HashMap::new();
        let mut total_chars = 0;

        for word in &self.words {
            let len = word.len();
            *length_distribution.entry(len).or_insert(0) += 1;
            total_chars += len;
        }

        let min_length = self.words.iter().map(|w| w.len()).min().unwrap_or(0);
        let max_length = self.words.iter().map(|w| w.len()).max().unwrap_or(0);
        let avg_length = total_chars as f64 / self.words.len() as f64;

        DictionaryStats {
            total_words: self.words.len(),
            min_length,
            max_length,
            avg_length,
            length_distribution,
        }
    }

    /// Encode bytes into four words using the 16k dictionary
    /// For up to 5 bytes (40 bits), which fits in 4 words (42 bits)
    pub fn encode_bytes(&self, data: &[u8]) -> Result<Vec<String>, DictionaryError> {
        if data.is_empty() || data.len() > 5 {
            return Err(DictionaryError::InvalidWordCount {
                expected: 5,
                actual: data.len(),
            });
        }

        // We have exactly 42 bits to work with (4 words × 14 bits)
        // We'll use a different approach:
        // - For lengths 1-4: Use 2 bits for length, 40 bits for data
        // - For length 5: We need all 40 bits for data, so encode length differently

        let value = if data.len() <= 4 {
            // Standard encoding: 2 bits length + up to 32 bits data
            let len_encoded = (data.len() - 1) as u64;
            let mut val = len_encoded << 40;

            // Pack data (shift left to fill from MSB)
            for (i, &byte) in data.iter().enumerate() {
                val |= (byte as u64) << (32 - i * 8);
            }
            val
        } else {
            // Length 5: Special encoding
            // Set top 2 bits to 11 (3) to indicate length 5
            let mut val = 3u64 << 40;

            // For 5 bytes, we need all 40 bits
            // Pack all 5 bytes into the lower 40 bits
            for (i, &byte) in data.iter().enumerate() {
                val |= (byte as u64) << ((4 - i) * 8);
            }
            val
        };

        // Extract three 14-bit indices from the 42-bit value
        // We have 42 bits total: [41:28] [27:14] [13:0]
        let idx1 = ((value >> 28) & 0x3FFF) as u16;
        let idx2 = ((value >> 14) & 0x3FFF) as u16;
        let idx3 = (value & 0x3FFF) as u16;

        Ok(vec![
            self.get_word(idx1)?.to_string(),
            self.get_word(idx2)?.to_string(),
            self.get_word(idx3)?.to_string(),
        ])
    }

    /// Decode four words back to bytes
    pub fn decode_words(&self, words: &[&str]) -> Result<Vec<u8>, DictionaryError> {
        if words.len() != 3 {
            return Err(DictionaryError::InvalidWordCount {
                expected: 3,
                actual: words.len(),
            });
        }

        // Get indices for each word
        let idx1 = self.get_index(words[0])?;
        let idx2 = self.get_index(words[1])?;
        let idx3 = self.get_index(words[2])?;

        // Reconstruct the value (4 words × 14 bits = 42 bits)
        let value = ((idx1 as u64) << 28) | ((idx2 as u64) << 14) | (idx3 as u64);

        // Extract length from the uppermost 2 bits of our 42-bit value
        let len_encoded = (idx1 >> 12) & 0x3; // Top 2 bits of first word
        let len = if len_encoded == 3 {
            5
        } else {
            (len_encoded + 1) as usize
        };

        // Extract data based on length
        let mut result = Vec::with_capacity(len);

        if len <= 4 {
            // Standard decoding: data starts at bit 32
            let data_bits = value & 0xFFFFFFFFFF; // Lower 40 bits

            // Extract bytes from the data (data was shifted to MSB)
            for i in 0..len {
                let shift = 32 - i * 8;
                let byte = ((data_bits >> shift) & 0xFF) as u8;
                result.push(byte);
            }
        } else {
            // Length 5: All 40 lower bits are data
            let data_bits = value & 0xFFFFFFFFFF;

            for i in 0..5 {
                let shift = (4 - i) * 8;
                let byte = ((data_bits >> shift) & 0xFF) as u8;
                result.push(byte);
            }
        }

        Ok(result)
    }
}

/// Dictionary statistics
#[derive(Debug, Clone)]
pub struct DictionaryStats {
    pub total_words: usize,
    pub min_length: usize,
    pub max_length: usize,
    pub avg_length: f64,
    pub length_distribution: HashMap<usize, usize>,
}

/// Global dictionary instance (lazy-loaded)
static INIT_DICTIONARY: Once = Once::new();
static mut GLOBAL_DICTIONARY: *const Dictionary16K = ptr::null();

/// Get global dictionary instance
pub fn get_global_dictionary() -> Result<&'static Dictionary16K, DictionaryError> {
    unsafe {
        INIT_DICTIONARY.call_once(|| {
            match Dictionary16K::new() {
                Ok(dict) => {
                    let boxed = Box::new(dict);
                    GLOBAL_DICTIONARY = Box::into_raw(boxed);
                }
                Err(_) => {
                    // Keep null pointer to indicate failure
                }
            }
        });

        if GLOBAL_DICTIONARY.is_null() {
            Err(DictionaryError::NotInitialized)
        } else {
            Ok(&*GLOBAL_DICTIONARY)
        }
    }
}

/// Utility functions for encoding/decoding with the dictionary
pub mod utils {
    use super::*;

    /// Convert a 14-bit value to a word
    pub fn index_to_word(index: u16) -> Result<&'static str, DictionaryError> {
        let dict = get_global_dictionary()?;
        dict.get_word(index)
    }

    /// Convert a word to a 14-bit value
    pub fn word_to_index(word: &str) -> Result<u16, DictionaryError> {
        let dict = get_global_dictionary()?;
        dict.get_index(word)
    }

    /// Convert three 14-bit indices to words
    pub fn indices_to_words(indices: [u16; 3]) -> Result<[String; 3], DictionaryError> {
        let dict = get_global_dictionary()?;
        Ok([
            dict.get_word(indices[0])?.to_string(),
            dict.get_word(indices[1])?.to_string(),
            dict.get_word(indices[2])?.to_string(),
        ])
    }

    /// Convert four words to 14-bit indices
    pub fn words_to_indices(words: [&str; 3]) -> Result<[u16; 3], DictionaryError> {
        let dict = get_global_dictionary()?;
        Ok([
            dict.get_index(words[0])?,
            dict.get_index(words[1])?,
            dict.get_index(words[2])?,
        ])
    }

    /// Check if all words in a slice are valid
    pub fn validate_words(words: &[&str]) -> Result<(), DictionaryError> {
        let dict = get_global_dictionary()?;
        for word in words {
            dict.get_index(word)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dictionary_creation() {
        let dict = Dictionary16K::new().unwrap();
        assert_eq!(dict.len(), WORD_COUNT);
        assert!(!dict.is_empty());
    }

    #[test]
    fn test_word_lookup() {
        let dict = Dictionary16K::new().unwrap();

        // Test first word
        let first_word = dict.get_word(0).unwrap();
        assert!(!first_word.is_empty());

        // Test last word
        let last_word = dict.get_word(MAX_WORD_INDEX).unwrap();
        assert!(!last_word.is_empty());

        // Test round-trip
        let index = dict.get_index(first_word).unwrap();
        assert_eq!(index, 0);

        let word = dict.get_word(index).unwrap();
        assert_eq!(word, first_word);
    }

    #[test]
    fn test_invalid_operations() {
        let dict = Dictionary16K::new().unwrap();

        // Test invalid index
        assert!(dict.get_word(WORD_COUNT as u16).is_err());

        // Test unknown word
        assert!(dict.get_index("xyzzy123nonexistent").is_err());
    }

    #[test]
    fn test_constants() {
        assert_eq!(WORD_COUNT, 16_384);
        assert_eq!(BITS_PER_WORD, 14);
        assert_eq!(BITS_FOUR_WORDS, 42);
        assert_eq!(MAX_WORD_INDEX, 16_383);
    }

    #[test]
    fn test_global_dictionary() {
        let dict1 = get_global_dictionary().unwrap();
        let dict2 = get_global_dictionary().unwrap();

        // Should be the same instance
        assert_eq!(dict1.len(), dict2.len());
        assert_eq!(dict1.get_word(0).unwrap(), dict2.get_word(0).unwrap());
    }

    #[test]
    fn test_utility_functions() {
        // Test index to word
        let word = utils::index_to_word(0).unwrap();
        assert!(!word.is_empty());

        // Test word to index
        let index = utils::word_to_index(word).unwrap();
        assert_eq!(index, 0);

        // Test four words conversion
        let indices = [0, 1, 2];
        let words = utils::indices_to_words(indices).unwrap();
        assert_eq!(words.len(), 3);

        let words_refs: [&str; 3] = [&words[0], &words[1], &words[2]];
        let recovered_indices = utils::words_to_indices(words_refs).unwrap();
        assert_eq!(recovered_indices, indices);
    }

    #[test]
    fn test_dictionary_stats() {
        let dict = Dictionary16K::new().unwrap();
        let stats = dict.stats();

        assert_eq!(stats.total_words, WORD_COUNT);
        assert!(stats.min_length >= 2);
        assert!(stats.max_length <= 15);
        assert!(stats.avg_length > 2.0 && stats.avg_length < 15.0);
        assert!(!stats.length_distribution.is_empty());
    }

    #[test]
    fn test_sample_words() {
        let dict = Dictionary16K::new().unwrap();
        let samples = dict.sample_words(10);

        assert_eq!(samples.len(), 10);

        // All samples should be valid words
        for word in samples {
            assert!(dict.contains_word(word));
        }
    }

    #[test]
    fn test_word_validation() {
        let dict = Dictionary16K::new().unwrap();

        // Get some valid words
        let valid_words = dict.sample_words(5);
        let word_refs: Vec<&str> = valid_words.iter().copied().collect();

        // Should validate successfully
        assert!(utils::validate_words(&word_refs).is_ok());

        // Add an invalid word
        let mut invalid_words = word_refs;
        invalid_words.push("invalidword123");

        // Should fail validation
        assert!(utils::validate_words(&invalid_words).is_err());
    }
}
