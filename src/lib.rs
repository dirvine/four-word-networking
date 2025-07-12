//! Three-Word Networking
//!
//! Convert network IP addresses into memorable word combinations
//! for human-friendly networking.
//!
//! ## Features
//!
//! - **Perfect IPv4**: Converts IPv4 addresses like `192.168.1.1:443`
//!   into exactly 3 memorable words like `lehr.delfs.enrages` with 100% perfect reconstruction
//! - **Adaptive IPv6**: Converts IPv6 addresses into 6 or 9 words (groups of 3) using intelligent compression
//! - **Voice-Friendly**: Easy to share over phone calls or voice chat
//! - **Error-Resistant**: Much less prone to typos than long technical addresses
//! - **Deterministic**: Same IP address always produces the same word combination
//! - **Visual Distinction**: IPv4 uses dots, IPv6 uses dashes for clear differentiation
//! - **Universal**: Works with any valid IP address format
//!
//! ## Example
//!
//! ```rust
//! use three_word_networking::ThreeWordAdaptiveEncoder;
//!
//! let encoder = ThreeWordAdaptiveEncoder::new()?;
//! let address = "192.168.1.1:443";
//!
//! // Convert to three words (perfect reconstruction for IPv4)
//! let words = encoder.encode(address)?;
//! println!("Address: {} -> {}", address, words);
//! // Output: Address: 192.168.1.1:443 -> lehr.delfs.enrages
//!
//! // Decode back to exact address
//! let decoded = encoder.decode(&words)?;
//! assert_eq!(address, decoded);
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```

pub mod compression;
pub mod dictionary65k;
pub mod error;
pub mod three_word_encoder;
pub mod three_word_ipv6_encoder;
pub mod unified_three_word_encoder;
// Experimental modules removed
pub mod ipv6_compression;
pub mod ipv6_pattern_feistel;
pub mod ipv6_perfect_patterns;
pub mod pure_ip_compression;
pub mod three_word_adaptive_encoder;
// Ultra modules removed - used outdated 3-word system
pub mod universal_ip_compression;

#[cfg(test)]
mod property_tests;

pub use error::{FourWordError, Result};
// Experimental modules removed - use FourWordAdaptiveEncoder instead
pub use ipv6_compression::{CompressedIpv6, Ipv6Category, Ipv6Compressor};
pub use ipv6_pattern_feistel::{IPv6PatternFeistel, IPv6PatternId};
pub use ipv6_perfect_patterns::{IPv6Pattern, IPv6PatternDetector};
pub use pure_ip_compression::{MathematicalCompressor, PureIpCompressor};
pub use three_word_adaptive_encoder::ThreeWordAdaptiveEncoder;
// UltraCompactEncoder removed - used outdated 3-word system
pub use three_word_encoder::{ThreeWordEncoder, ThreeWordEncoding};
pub use three_word_ipv6_encoder::{
    Ipv6ThreeWordGroupEncoding, ThreeWordGroup, ThreeWordIpv6Encoder,
};
pub use unified_three_word_encoder::{UnifiedThreeWordEncoder, UnifiedThreeWordEncoding};
pub use universal_ip_compression::UniversalIpCompressor;

/// Version of the three-word networking library
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_functionality() {
        let encoder = ThreeWordAdaptiveEncoder::new().unwrap();
        let address = "192.168.1.1:443";

        let words = encoder.encode(address).unwrap();
        let decoded = encoder.decode(&words).unwrap();
        assert_eq!(address, decoded);
    }
}
