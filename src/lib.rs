//! Four-Word Networking
//!
//! Convert network IP addresses into memorable word combinations 
//! for human-friendly networking.
//!
//! ## Features
//!
//! - **Perfect IPv4**: Converts IPv4 addresses like `192.168.1.1:443` 
//!   into exactly 4 memorable words like `paper.broaden.smith.bully` with 100% perfect reconstruction
//! - **Adaptive IPv6**: Converts IPv6 addresses into 4-6 words using intelligent compression
//! - **Voice-Friendly**: Easy to share over phone calls or voice chat
//! - **Error-Resistant**: Much less prone to typos than long technical addresses
//! - **Deterministic**: Same IP address always produces the same word combination
//! - **Visual Distinction**: IPv4 uses dots, IPv6 uses dashes for clear differentiation
//! - **Universal**: Works with any valid IP address format
//!
//! ## Example
//!
//! ```rust
//! use four_word_networking::FourWordAdaptiveEncoder;
//!
//! let encoder = FourWordAdaptiveEncoder::new()?;
//! let address = "192.168.1.1:443";
//! 
//! // Convert to four words (perfect reconstruction for IPv4)
//! let words = encoder.encode(address)?;
//! println!("Address: {} -> {}", address, words);
//! // Output: Address: 192.168.1.1:443 -> paper.broaden.smith.bully
//! 
//! // Decode back to exact address
//! let decoded = encoder.decode(&words)?;
//! assert_eq!(address, decoded);
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```

pub mod error;
pub mod universal;
pub mod dictionary16k;
pub mod encoder16k;
pub mod compression;
// pub mod balanced_encoder; // Temporarily disabled - needs update for new compression API
pub mod ultra_compression;
pub mod ultra_compact_encoder;
pub mod ip_port_encoder;
pub mod ip_port_encoder_v2;
pub mod compressed_encoder;
pub mod universal_ip_compression;
pub mod pure_ip_compression;
pub mod universal_encoder;
pub mod variable_dictionary;
pub mod ipv6_compression;
pub mod adaptive_encoder;
pub mod api;
pub mod perfect_encoder;
pub mod ipv4_perfect_codec;
pub mod ipv6_perfect_codec;
pub mod perfect_adaptive_encoder;
pub mod simple_perfect_encoder;
pub mod simple_adaptive_encoder;
pub mod four_word_encoder;

pub use error::{ThreeWordError, Result};
pub use universal::{UniversalEncoding, EncodingStrategy};
pub use ip_port_encoder::{IpPortEncoder, IpPortAddress};
// pub use balanced_encoder::{BalancedEncoder, BalancedEncoding};
pub use ultra_compact_encoder::{UltraCompactEncoder, UltraCompactEncoding};
pub use ip_port_encoder_v2::{IpPortEncoderV2, IpPortEncodingV2, IpPortErrorV2};
pub use compressed_encoder::{CompressedEncoder, CompressionStats};
pub use universal_ip_compression::UniversalIpCompressor;
pub use pure_ip_compression::{PureIpCompressor, MathematicalCompressor};
pub use universal_encoder::{UniversalEncoder, CompressionAnalysis, StrategyResult};
pub use variable_dictionary::{VariableDictionary, AdaptiveEncoding, CapacityInfo};
pub use ipv6_compression::{Ipv6Compressor, CompressedIpv6, Ipv6Category};
pub use adaptive_encoder::{AdaptiveEncoder, AdaptiveResult, AddressType, CompressionAnalysis as AdaptiveAnalysis};
pub use api::{ThreeWordNetworking, AddressInput, EncodingInfo, AddressType as ApiAddressType};
pub use perfect_adaptive_encoder::PerfectAdaptiveEncoder;
pub use simple_adaptive_encoder::SimpleAdaptiveEncoder;
pub use four_word_encoder::FourWordAdaptiveEncoder;

/// Version of the three-word networking library
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_functionality() {
        let encoder = FourWordAdaptiveEncoder::new().unwrap();
        let address = "192.168.1.1:443";
        
        let words = encoder.encode(address).unwrap();
        let decoded = encoder.decode(&words).unwrap();
        assert_eq!(address, decoded);
    }
}