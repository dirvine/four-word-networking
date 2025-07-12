//! Error types for three-word networking

use thiserror::Error;

/// Result type for three-word networking operations
pub type Result<T> = std::result::Result<T, FourWordError>;

/// Error types for three-word networking
#[derive(Error, Debug)]
pub enum FourWordError {
    #[error("Invalid word address format: {0}")]
    InvalidFourWordAddress(String),

    #[error("Word not found in dictionary: {0}")]
    WordNotFound(String),

    #[error("Dictionary position out of range: {0}")]
    PositionOutOfRange(usize),

    #[error("Numeric suffix out of range: {0}")]
    NumericSuffixOutOfRange(u32),

    #[error("Invalid input: {0}")]
    InvalidInput(String),

    #[error("Compression error: {0}")]
    CompressionError(String),

    #[error("Decompression error: {0}")]
    DecompressionError(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("Encoding error: {0}")]
    EncodingError(String),

    #[error("Decoding error: {0}")]
    DecodingError(String),

    #[error("Dictionary error: {0}")]
    DictionaryError(String),
}
