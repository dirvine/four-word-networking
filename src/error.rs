//! Error types for three-word networking

use thiserror::Error;

/// Result type for three-word networking operations
pub type Result<T> = std::result::Result<T, ThreeWordError>;

/// Error types for three-word networking
#[derive(Error, Debug)]
pub enum ThreeWordError {
    #[error("Invalid multiaddr format: {0}")]
    InvalidMultiaddr(String),
    
    #[error("Invalid three-word address format: {0}")]
    InvalidThreeWordAddress(String),
    
    #[error("Word not found in dictionary: {0}")]
    WordNotFound(String),
    
    #[error("Dictionary position out of range: {0}")]
    PositionOutOfRange(usize),
    
    #[error("Numeric suffix out of range: {0}")]
    NumericSuffixOutOfRange(u32),
    
    #[error("Registry lookup not yet implemented: {0}")]
    RegistryLookupNotImplemented(String),
    
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
}