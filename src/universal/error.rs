//! Error types for the Universal Word Encoding System

use thiserror::Error;

/// Errors that can occur during encoding operations
#[derive(Error, Debug, Clone, PartialEq, Eq)]
pub enum EncodingError {
    /// Data is too small to encode (empty)
    #[error("Data is too small to encode: {0} bytes (minimum 1 byte)")]
    DataTooSmall(usize),

    /// Data is too large to encode (> 32 bytes)
    #[error("Data is too large to encode: {0} bytes (maximum 32 bytes)")]
    DataTooLarge(usize),

    /// Invalid multiaddress format
    #[error("Invalid multiaddress format: {0}")]
    InvalidMultiaddr(String),

    /// Dictionary not loaded or corrupted
    #[error("Dictionary not loaded or corrupted")]
    DictionaryNotLoaded,

    /// Invalid cryptocurrency address format
    #[error("Invalid cryptocurrency address format: {0}")]
    InvalidCryptoAddress(String),

    /// Hash computation failed
    #[error("Hash computation failed: {0}")]
    HashError(String),
}

/// Errors that can occur during decoding operations
#[derive(Error, Debug, Clone, PartialEq, Eq)]
pub enum DecodingError {
    /// Invalid word not found in dictionary
    #[error("Invalid word not found in dictionary: '{0}'")]
    InvalidWord(String),

    /// Insufficient story views for holographic decoding
    #[error(
        "Insufficient story views for holographic decoding: got {got}, need at least {needed}"
    )]
    InsufficientViews { got: usize, needed: usize },

    /// Checksum mismatch during decoding
    #[error("Checksum mismatch during decoding")]
    ChecksumMismatch,

    /// Invalid encoding format
    #[error("Invalid encoding format: {0}")]
    InvalidFormat(String),

    /// Dictionary lookup failed
    #[error("Dictionary lookup failed for word: '{0}'")]
    DictionaryLookupFailed(String),

    /// Fractal decoding failed
    #[error("Fractal decoding failed: {0}")]
    FractalDecodingFailed(String),

    /// Holographic constraint solving failed
    #[error("Holographic constraint solving failed: {0}")]
    HolographicSolvingFailed(String),
}

/// Convenience type alias for encoding results
pub type EncodingResult<T> = Result<T, EncodingError>;

/// Convenience type alias for decoding results
pub type DecodingResult<T> = Result<T, DecodingError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encoding_error_display() {
        let err = EncodingError::DataTooLarge(64);
        assert_eq!(
            err.to_string(),
            "Data is too large to encode: 64 bytes (maximum 32 bytes)"
        );

        let err = EncodingError::DataTooSmall(0);
        assert_eq!(
            err.to_string(),
            "Data is too small to encode: 0 bytes (minimum 1 byte)"
        );
    }

    #[test]
    fn test_decoding_error_display() {
        let err = DecodingError::InvalidWord("badword".to_string());
        assert_eq!(
            err.to_string(),
            "Invalid word not found in dictionary: 'badword'"
        );

        let err = DecodingError::InsufficientViews { got: 2, needed: 3 };
        assert_eq!(
            err.to_string(),
            "Insufficient story views for holographic decoding: got 2, need at least 3"
        );
    }
}
