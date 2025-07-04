//! Three-Word Networking
//!
//! Convert complex network multiaddresses into memorable three-word combinations 
//! for human-friendly peer discovery and sharing.
//!
//! ## Features
//!
//! - **Human-Readable**: Converts complex multiaddrs like `/ip6/2001:db8::1/udp/9000/quic` 
//!   into memorable addresses like `ocean.thunder.falcon`
//! - **Voice-Friendly**: Easy to share over phone calls or voice chat
//! - **Error-Resistant**: Much less prone to typos than long technical addresses
//! - **Deterministic**: Same multiaddr always produces the same three-word address
//! - **Massive Scale**: Supports 68.7 billion base combinations, extensible to 4.5 quadrillion
//! - **Universal**: Works with any valid multiaddr format
//!
//! ## Example
//!
//! ```rust
//! use three_word_networking::{WordEncoder, ThreeWordAddress};
//!
//! let encoder = WordEncoder::new();
//! let multiaddr = "/ip6/2001:db8::1/udp/9000/quic";
//! 
//! // Convert to three words
//! let words = encoder.encode_multiaddr_string(multiaddr)?;
//! println!("Address: {} -> {}", multiaddr, words);
//! 
//! // Validate the three-word address
//! let parsed = ThreeWordAddress::from_string(&words.to_string())?;
//! assert!(parsed.validate(&encoder).is_ok());
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```

pub mod words;
pub mod error;
pub mod multiaddr_parser;
pub mod universal;
pub mod dictionary16k;
pub mod encoder16k;
pub mod compression;
pub mod balanced_encoder;
pub mod ultra_compression;
pub mod ultra_compact_encoder;

pub use words::{ThreeWordAddress, WordDictionary, WordEncoder};
pub use error::{ThreeWordError, Result};
pub use multiaddr_parser::{ParsedMultiaddr, IpType, Protocol};
pub use universal::{UniversalEncoder, UniversalEncoding, EncodingStrategy};
pub use balanced_encoder::{BalancedEncoder, BalancedEncoding};
pub use ultra_compact_encoder::{UltraCompactEncoder, UltraCompactEncoding};

/// Version of the three-word networking library
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Address space information
pub struct AddressSpace;

impl AddressSpace {
    /// Get the total number of base three-word combinations
    pub fn base_combinations() -> u64 {
        ThreeWordAddress::base_combinations()
    }
    
    /// Get the total number of extended combinations (with numeric suffixes)
    pub fn total_combinations() -> u64 {
        ThreeWordAddress::address_space_size()
    }
    
    /// Get human-readable description of the address space
    pub fn description() -> String {
        ThreeWordAddress::address_space_description()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_functionality() {
        let encoder = WordEncoder::new();
        let multiaddr = "/ip6/2001:db8::1/udp/9000/quic";
        
        let words = encoder.encode_multiaddr_string(multiaddr).unwrap();
        assert!(words.validate(&encoder).is_ok());
        
        // Test deterministic encoding
        let words2 = encoder.encode_multiaddr_string(multiaddr).unwrap();
        assert_eq!(words, words2);
    }
    
    #[test]
    fn test_address_space() {
        assert!(AddressSpace::base_combinations() > 0);
        assert!(AddressSpace::total_combinations() >= AddressSpace::base_combinations());
        assert!(!AddressSpace::description().is_empty());
    }
}