//! Universal Word Encoding System
//!
//! This module provides a universal encoding system that can encode ANY data 
//! from network addresses up to 32-byte hashes into human-memorable word sequences,
//! with 100% accurate encode/decode capability.

pub mod encoder;
pub mod dictionaries;
pub mod simple;
pub mod fractal;
pub mod holographic;
pub mod error;

#[cfg(test)]
pub mod real_world_tests;

#[cfg(test)]
pub mod demo_tests;

#[cfg(test)]
pub mod exhaustive_tests;

#[cfg(test)]
pub mod fast_exhaustive_tests;

#[cfg(test)]
pub mod ultra_tests;

pub use encoder::UniversalEncoder;
pub use dictionaries::Dictionaries;
pub use error::{EncodingError, DecodingError};

/// Encoding strategies based on data size
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EncodingStrategy {
    /// Simple encoding for data <= 8 bytes: 4 words only
    Simple,
    /// Fractal encoding for data 9-20 bytes: 3 base words + zoom levels
    Fractal,
    /// Holographic encoding for data 21-32 bytes: Multiple story views
    Holographic,
}

impl EncodingStrategy {
    /// Determine the appropriate encoding strategy based on data length
    pub fn for_data_length(len: usize) -> Result<Self, EncodingError> {
        match len {
            0 => Err(EncodingError::DataTooSmall(len)),
            1..=8 => Ok(EncodingStrategy::Simple),
            9..=20 => Ok(EncodingStrategy::Fractal),
            21..=32 => Ok(EncodingStrategy::Holographic),
            _ => Err(EncodingError::DataTooLarge(len)),
        }
    }
}

/// Unified encoding format that can represent any encoding strategy result
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UniversalEncoding {
    /// Simple encoding with 4 words
    Simple(simple::ThreeWords),
    /// Fractal encoding with base words and zoom levels
    Fractal(fractal::FractalEncoding),
    /// Holographic encoding with multiple story views
    Holographic(holographic::HolographicEncoding),
}

impl UniversalEncoding {
    /// Get all words from this encoding (for testing pronunciation, etc.)
    pub fn all_words(&self) -> Vec<&str> {
        match self {
            UniversalEncoding::Simple(words) => words.all_words(),
            UniversalEncoding::Fractal(encoding) => encoding.all_words(),
            UniversalEncoding::Holographic(encoding) => encoding.all_words(),
        }
    }
    
    /// Convert to human-readable string representation
    pub fn to_string(&self) -> String {
        match self {
            UniversalEncoding::Simple(words) => words.to_string(),
            UniversalEncoding::Fractal(encoding) => encoding.to_string(),
            UniversalEncoding::Holographic(encoding) => encoding.to_string(),
        }
    }
}

/// Result type for encoding operations
pub type EncodingResult<T> = Result<T, EncodingError>;

/// Result type for decoding operations
pub type DecodingResult<T> = Result<T, DecodingError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encoding_strategy_selection() {
        // Test edge cases
        assert!(EncodingStrategy::for_data_length(0).is_err());
        assert!(EncodingStrategy::for_data_length(33).is_err());
        
        // Test simple encoding range
        assert_eq!(EncodingStrategy::for_data_length(1).unwrap(), EncodingStrategy::Simple);
        assert_eq!(EncodingStrategy::for_data_length(8).unwrap(), EncodingStrategy::Simple);
        
        // Test fractal encoding range
        assert_eq!(EncodingStrategy::for_data_length(9).unwrap(), EncodingStrategy::Fractal);
        assert_eq!(EncodingStrategy::for_data_length(20).unwrap(), EncodingStrategy::Fractal);
        
        // Test holographic encoding range
        assert_eq!(EncodingStrategy::for_data_length(21).unwrap(), EncodingStrategy::Holographic);
        assert_eq!(EncodingStrategy::for_data_length(32).unwrap(), EncodingStrategy::Holographic);
    }
}