//! Main universal encoder that coordinates all encoding strategies

use crate::universal::{
    EncodingStrategy, UniversalEncoding,
    dictionaries::Dictionaries,
    simple::SimpleEncoder,
    fractal::FractalEncoder,
    holographic::HolographicEncoder,
    error::{EncodingError, EncodingResult, DecodingResult},
};

/// Main universal encoder that can encode any data up to 32 bytes
#[derive(Debug, Clone)]
pub struct UniversalEncoder {
    dictionaries: Dictionaries,
    simple_encoder: SimpleEncoder,
    fractal_encoder: FractalEncoder,
    holographic_encoder: HolographicEncoder,
}

impl UniversalEncoder {
    /// Create a new universal encoder
    pub fn new() -> EncodingResult<Self> {
        let dictionaries = Dictionaries::new()?;
        let simple_encoder = SimpleEncoder::new(dictionaries.clone());
        let fractal_encoder = FractalEncoder::new(dictionaries.clone());
        let holographic_encoder = HolographicEncoder::new(dictionaries.clone());
        
        Ok(UniversalEncoder {
            dictionaries,
            simple_encoder,
            fractal_encoder,
            holographic_encoder,
        })
    }
    
    /// Encode arbitrary data using the appropriate strategy
    pub fn encode(&self, data: &[u8]) -> EncodingResult<UniversalEncoding> {
        let strategy = EncodingStrategy::for_data_length(data.len())?;
        
        match strategy {
            EncodingStrategy::Simple => {
                let encoded = self.simple_encoder.encode(data)?;
                Ok(UniversalEncoding::Simple(encoded))
            }
            EncodingStrategy::Fractal => {
                let encoded = self.fractal_encoder.encode(data)?;
                Ok(UniversalEncoding::Fractal(encoded))
            }
            EncodingStrategy::Holographic => {
                let encoded = self.holographic_encoder.encode(data)?;
                Ok(UniversalEncoding::Holographic(encoded))
            }
        }
    }
    
    /// Decode universal encoding back to original data
    pub fn decode(&self, encoding: &UniversalEncoding) -> DecodingResult<Vec<u8>> {
        match encoding {
            UniversalEncoding::Simple(simple) => {
                self.simple_encoder.decode(simple)
            }
            UniversalEncoding::Fractal(fractal) => {
                self.fractal_encoder.decode(fractal)
            }
            UniversalEncoding::Holographic(holographic) => {
                self.holographic_encoder.decode(holographic)
            }
        }
    }
    
    /// Encode network multiaddress
    pub fn encode_multiaddr(&self, multiaddr: &str) -> EncodingResult<UniversalEncoding> {
        let bytes = self.parse_multiaddr(multiaddr)?;
        self.encode(&bytes)
    }
    
    /// Encode Bitcoin address
    pub fn encode_bitcoin(&self, address: &str) -> EncodingResult<UniversalEncoding> {
        let bytes = self.parse_bitcoin_address(address)?;
        self.encode(&bytes)
    }
    
    /// Encode Ethereum address
    pub fn encode_ethereum(&self, address: &str) -> EncodingResult<UniversalEncoding> {
        let bytes = self.parse_ethereum_address(address)?;
        self.encode(&bytes)
    }
    
    /// Encode SHA-256 hash
    pub fn encode_sha256(&self, hash: &[u8; 32]) -> EncodingResult<UniversalEncoding> {
        self.encode(hash)
    }
    
    /// Parse multiaddress to bytes (placeholder implementation)
    fn parse_multiaddr(&self, multiaddr: &str) -> EncodingResult<Vec<u8>> {
        // This is a placeholder - in a real implementation we'd properly parse multiaddrs
        if !multiaddr.starts_with('/') {
            return Err(EncodingError::InvalidMultiaddr(multiaddr.to_string()));
        }
        
        // For now, hash the multiaddr string to get consistent bytes
        let bytes = multiaddr.as_bytes();
        if bytes.len() <= 32 {
            Ok(bytes.to_vec())
        } else {
            // Use a simple hash for longer multiaddrs
            Ok(self.simple_hash(bytes))
        }
    }
    
    /// Parse Bitcoin address to bytes (placeholder implementation)
    fn parse_bitcoin_address(&self, address: &str) -> EncodingResult<Vec<u8>> {
        // This is a placeholder - in a real implementation we'd properly decode Base58
        if address.len() < 26 || address.len() > 35 {
            return Err(EncodingError::InvalidCryptoAddress(address.to_string()));
        }
        
        // For now, hash the address string to get consistent 25 bytes
        let mut bytes = self.simple_hash(address.as_bytes());
        bytes.truncate(25);
        Ok(bytes)
    }
    
    /// Parse Ethereum address to bytes (placeholder implementation)
    fn parse_ethereum_address(&self, address: &str) -> EncodingResult<Vec<u8>> {
        // This is a placeholder - in a real implementation we'd properly decode hex
        if !address.starts_with("0x") || address.len() != 42 {
            return Err(EncodingError::InvalidCryptoAddress(address.to_string()));
        }
        
        // For now, hash the address string to get consistent 20 bytes
        let mut bytes = self.simple_hash(address.as_bytes());
        bytes.truncate(20);
        Ok(bytes)
    }
    
    /// Simple hash function for consistent byte generation
    fn simple_hash(&self, data: &[u8]) -> Vec<u8> {
        // Simple hash based on FNV-1a algorithm
        let mut hash = 2166136261u64;
        for &byte in data {
            hash ^= byte as u64;
            hash = hash.wrapping_mul(16777619);
        }
        
        // Extend to 32 bytes
        let mut result = Vec::new();
        for i in 0..4 {
            let part = hash.wrapping_add(i as u64).to_be_bytes();
            result.extend_from_slice(&part);
        }
        
        result
    }
    
    /// Get encoding strategy for given data length
    pub fn strategy_for_length(&self, len: usize) -> EncodingResult<EncodingStrategy> {
        EncodingStrategy::for_data_length(len)
    }
    
    /// Get dictionary reference for advanced usage
    pub fn dictionaries(&self) -> &Dictionaries {
        &self.dictionaries
    }
}

impl Default for UniversalEncoder {
    fn default() -> Self {
        Self::new().expect("Failed to create default UniversalEncoder")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_universal_encoder_creation() {
        let encoder = UniversalEncoder::new().unwrap();
        
        // Test that all encoders are properly initialized
        assert!(encoder.dictionaries.actors.len() == 4096);
        assert!(encoder.dictionaries.actions.len() == 4096);
        assert!(encoder.dictionaries.objects.len() == 4096);
        assert!(encoder.dictionaries.modifiers.len() == 4096);
    }
    
    #[test]
    fn test_encode_decode_all_strategies() {
        let encoder = UniversalEncoder::new().unwrap();
        
        // Test simple encoding (≤8 bytes)
        let simple_data = vec![0x01, 0x02, 0x03, 0x04];
        let simple_encoded = encoder.encode(&simple_data).unwrap();
        let simple_decoded = encoder.decode(&simple_encoded).unwrap();
        assert!(!simple_decoded.is_empty());
        
        // Test fractal encoding (9-20 bytes)
        let fractal_data = vec![0x01; 15];
        let fractal_encoded = encoder.encode(&fractal_data).unwrap();
        let fractal_decoded = encoder.decode(&fractal_encoded).unwrap();
        assert!(!fractal_decoded.is_empty());
        
        // Test holographic encoding (21-32 bytes)
        let holographic_data = vec![0x01; 25];
        let holographic_encoded = encoder.encode(&holographic_data).unwrap();
        let holographic_decoded = encoder.decode(&holographic_encoded).unwrap();
        assert!(!holographic_decoded.is_empty());
    }
    
    #[test]
    fn test_strategy_selection() {
        let encoder = UniversalEncoder::new().unwrap();
        
        // Test strategy selection for different data lengths
        assert_eq!(encoder.strategy_for_length(4).unwrap(), EncodingStrategy::Simple);
        assert_eq!(encoder.strategy_for_length(12).unwrap(), EncodingStrategy::Fractal);
        assert_eq!(encoder.strategy_for_length(28).unwrap(), EncodingStrategy::Holographic);
        
        // Test bounds
        assert!(encoder.strategy_for_length(0).is_err());
        assert!(encoder.strategy_for_length(33).is_err());
    }
    
    #[test]
    fn test_deterministic_encoding() {
        let encoder = UniversalEncoder::new().unwrap();
        
        let test_cases = vec![
            vec![0x01; 4],   // Simple
            vec![0x02; 12],  // Fractal
            vec![0x03; 24],  // Holographic
        ];
        
        for data in test_cases {
            let encoded1 = encoder.encode(&data).unwrap();
            let encoded2 = encoder.encode(&data).unwrap();
            assert_eq!(encoded1, encoded2);
        }
    }
    
    #[test]
    fn test_multiaddr_encoding() {
        let encoder = UniversalEncoder::new().unwrap();
        
        let multiaddrs = vec![
            "/ip4/127.0.0.1/tcp/8080",
            "/ip6/::1/tcp/8080",
            "/dns4/example.com/tcp/443",
        ];
        
        for multiaddr in multiaddrs {
            let encoded = encoder.encode_multiaddr(multiaddr).unwrap();
            let decoded = encoder.decode(&encoded).unwrap();
            assert!(!decoded.is_empty());
        }
    }
    
    #[test]
    fn test_bitcoin_address_encoding() {
        let encoder = UniversalEncoder::new().unwrap();
        
        let bitcoin_addresses = vec![
            "1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa",
            "3J98t1WpEZ73CNmQviecrnyiWrnqRhWNLy",
        ];
        
        for address in bitcoin_addresses {
            let encoded = encoder.encode_bitcoin(address).unwrap();
            let decoded = encoder.decode(&encoded).unwrap();
            assert!(!decoded.is_empty());
        }
    }
    
    #[test]
    fn test_ethereum_address_encoding() {
        let encoder = UniversalEncoder::new().unwrap();
        
        let ethereum_addresses = vec![
            "0x742d35Cc6634C0532925a3b8D200dC2e36de85f2",
            "0x8ba1f109551bD432803012645Hac136c11f7ab42",
        ];
        
        for address in ethereum_addresses {
            let encoded = encoder.encode_ethereum(address).unwrap();
            let decoded = encoder.decode(&encoded).unwrap();
            assert!(!decoded.is_empty());
        }
    }
    
    #[test]
    fn test_sha256_encoding() {
        let encoder = UniversalEncoder::new().unwrap();
        
        let hash = [0x12u8; 32];
        let encoded = encoder.encode_sha256(&hash).unwrap();
        let decoded = encoder.decode(&encoded).unwrap();
        assert!(!decoded.is_empty());
    }
    
    #[test]
    fn test_invalid_addresses() {
        let encoder = UniversalEncoder::new().unwrap();
        
        // Invalid multiaddr
        assert!(encoder.encode_multiaddr("not-a-multiaddr").is_err());
        
        // Invalid Bitcoin address
        assert!(encoder.encode_bitcoin("invalid").is_err());
        
        // Invalid Ethereum address
        assert!(encoder.encode_ethereum("invalid").is_err());
    }
    
    #[test]
    fn test_encoding_bounds() {
        let encoder = UniversalEncoder::new().unwrap();
        
        // Test empty data
        assert!(encoder.encode(&[]).is_err());
        
        // Test oversized data
        let oversized = vec![0u8; 33];
        assert!(encoder.encode(&oversized).is_err());
    }
}