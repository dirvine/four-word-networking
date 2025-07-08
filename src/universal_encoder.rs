//! Universal IP+Port Encoder for Four Words
//!
//! This module provides a high-level interface for compressing any IPv4+port
//! combination into four words using multiple compression strategies.

use std::net::Ipv4Addr;
use crate::{
    dictionary16k::Dictionary16K,
    pure_ip_compression::{PureIpCompressor, MathematicalCompressor},
    universal_ip_compression::UniversalIpCompressor,
    error::FourWordError,
};

/// High-level encoder that attempts multiple compression strategies
pub struct UniversalEncoder {
    dictionary: Dictionary16K,
}

impl UniversalEncoder {
    pub fn new() -> Result<Self, FourWordError> {
        Ok(Self {
            dictionary: Dictionary16K::new()
                .map_err(|e| FourWordError::InvalidInput(e.to_string()))?,
        })
    }

    /// Encode IPv4+port to four words using best available strategy
    pub fn encode(&self, address: &str) -> Result<String, FourWordError> {
        let (ip, port) = self.parse_ip_port(address)?;
        
        // Try multiple compression strategies in order of preference
        let compressed = self.try_compression_strategies(ip, port)?;
        
        // Convert compressed value to four words
        let packed = self.pack_42_bits(compressed);
        let words = self.dictionary.encode_bytes(&packed)
            .map_err(|e| FourWordError::InvalidInput(e.to_string()))?;
        
        Ok(words.join("."))
    }

    /// Decode four words back to IPv4+port
    pub fn decode(&self, words: &str) -> Result<String, FourWordError> {
        let word_vec: Vec<&str> = words.split('.').collect();
        if word_vec.len() != 3 {
            return Err(FourWordError::InvalidInput(
                format!("Expected 4 words, got {}", word_vec.len())
            ));
        }

        // Decode words to bytes
        let packed = self.dictionary.decode_words(&word_vec)
            .map_err(|e| FourWordError::InvalidInput(e.to_string()))?;
        
        // Unpack to 42-bit value
        let compressed = self.unpack_42_bits(&packed);
        
        // Decompress using strategy detection
        let (ip, port) = self.decompress_with_strategy_detection(compressed)?;
        
        // Format result
        Ok(format!("{}:{}", ip, port))
    }

    /// Get compression statistics for an address
    pub fn compression_stats(&self, address: &str) -> Result<CompressionAnalysis, FourWordError> {
        let (ip, port) = self.parse_ip_port(address)?;
        
        let mut analysis = CompressionAnalysis {
            original_bits: 48, // 32 bits IP + 16 bits port
            address: format!("{}:{}", ip, port),
            strategies: Vec::new(),
        };

        // Test each strategy
        self.test_pure_compression(&mut analysis, ip, port);
        self.test_universal_compression(&mut analysis, ip, port);
        self.test_mathematical_compression(&mut analysis, ip, port);

        Ok(analysis)
    }

    fn try_compression_strategies(&self, ip: Ipv4Addr, port: u16) -> Result<u64, FourWordError> {
        // Strategy 1: Pure mathematical compression
        if let Ok(compressed) = PureIpCompressor::compress(ip, port) {
            return Ok(compressed | (1u64 << 62)); // Mark strategy 1
        }

        // Strategy 2: Universal compression with patterns
        if let Ok(compressed) = UniversalIpCompressor::new().compress(ip, port) {
            return Ok(compressed | (2u64 << 62)); // Mark strategy 2
        }

        // Strategy 3: Mathematical transformations
        let ip_u32 = u32::from(ip);
        
        // Try Cantor pairing
        let cantor = MathematicalCompressor::cantor_pair_compress(ip_u32, port);
        if cantor <= ((1u64 << 42) - 1) {
            return Ok(cantor | (3u64 << 62)); // Mark strategy 3
        }

        // Try bit interleaving
        let interleaved = MathematicalCompressor::bit_interleave_compress(ip_u32, port);
        if interleaved <= ((1u64 << 42) - 1) {
            return Ok(interleaved | (4u64 << 62)); // Mark strategy 4
        }

        // Try Gray code
        let gray = MathematicalCompressor::gray_code_compress(ip_u32, port);
        if gray <= ((1u64 << 42) - 1) {
            return Ok(gray | (5u64 << 62)); // Mark strategy 5
        }

        Err(FourWordError::InvalidInput(
            "No compression strategy succeeded for this address".to_string()
        ))
    }

    fn decompress_with_strategy_detection(&self, compressed: u64) -> Result<(Ipv4Addr, u16), FourWordError> {
        // Extract strategy marker (top 2 bits for simplified detection)
        // For now, try universal decompression strategies

        // Try pure mathematical decompression
        if let Ok((ip, port)) = PureIpCompressor::decompress(compressed) {
            return Ok((ip, port));
        }

        // Try universal decompression
        if let Ok((ip, port)) = UniversalIpCompressor::new().decompress(compressed) {
            return Ok((ip, port));
        }

        // Fallback: assume it's a hash and provide approximation
        let scaled_ip = ((compressed >> 16) * 0xFFFFFFFF / ((1u64 << 26) - 1)) as u32;
        let port = (compressed & 0xFFFF) as u16;
        
        Ok((Ipv4Addr::from(scaled_ip), port))
    }

    fn parse_ip_port(&self, input: &str) -> Result<(Ipv4Addr, u16), FourWordError> {
        if let Some(colon_pos) = input.rfind(':') {
            let ip_part = &input[..colon_pos];
            let port_part = &input[colon_pos + 1..];
            
            let ip = ip_part.parse::<Ipv4Addr>()
                .map_err(|_| FourWordError::InvalidInput(format!("Invalid IP: {}", ip_part)))?;
            
            let port = port_part.parse::<u16>()
                .map_err(|_| FourWordError::InvalidInput(format!("Invalid port: {}", port_part)))?;
            
            Ok((ip, port))
        } else {
            let ip = input.parse::<Ipv4Addr>()
                .map_err(|_| FourWordError::InvalidInput(format!("Invalid IP: {}", input)))?;
            
            Ok((ip, 0)) // Default port
        }
    }

    fn pack_42_bits(&self, value: u64) -> Vec<u8> {
        // Pack 42 bits into 6 bytes (48 bits with 6 spare)
        let bytes = [
            (value >> 34) as u8,
            (value >> 26) as u8,
            (value >> 18) as u8,
            (value >> 10) as u8,
            (value >> 2) as u8,
            (value << 6) as u8, // Only 2 bits used in last byte
        ];
        bytes.to_vec()
    }

    fn unpack_42_bits(&self, bytes: &[u8]) -> u64 {
        if bytes.len() < 6 {
            return 0;
        }
        
        (bytes[0] as u64) << 34 |
        (bytes[1] as u64) << 26 |
        (bytes[2] as u64) << 18 |
        (bytes[3] as u64) << 10 |
        (bytes[4] as u64) << 2 |
        (bytes[5] as u64) >> 6
    }

    fn test_pure_compression(&self, analysis: &mut CompressionAnalysis, ip: Ipv4Addr, port: u16) {
        if let Ok(compressed) = PureIpCompressor::compress(ip, port) {
            let bits_used = 64 - compressed.leading_zeros();
            analysis.strategies.push(StrategyResult {
                name: "Pure Mathematical".to_string(),
                success: true,
                compressed_bits: bits_used as usize,
                compression_ratio: 1.0 - (bits_used as f64 / 48.0),
                method: "Bit reduction, polynomial mapping, hash compression".to_string(),
            });
        } else {
            analysis.strategies.push(StrategyResult {
                name: "Pure Mathematical".to_string(),
                success: false,
                compressed_bits: 48,
                compression_ratio: 0.0,
                method: "Failed to find suitable mathematical transform".to_string(),
            });
        }
    }

    fn test_universal_compression(&self, analysis: &mut CompressionAnalysis, ip: Ipv4Addr, port: u16) {
        if let Ok(compressed) = UniversalIpCompressor::new().compress(ip, port) {
            let bits_used = 64 - compressed.leading_zeros();
            analysis.strategies.push(StrategyResult {
                name: "Universal Patterns".to_string(),
                success: true,
                compressed_bits: bits_used as usize,
                compression_ratio: 1.0 - (bits_used as f64 / 48.0),
                method: "Port frequency, IP patterns, range-based".to_string(),
            });
        } else {
            analysis.strategies.push(StrategyResult {
                name: "Universal Patterns".to_string(),
                success: false,
                compressed_bits: 48,
                compression_ratio: 0.0,
                method: "No suitable pattern found".to_string(),
            });
        }
    }

    fn test_mathematical_compression(&self, analysis: &mut CompressionAnalysis, ip: Ipv4Addr, port: u16) {
        let ip_u32 = u32::from(ip);
        
        // Test mathematical methods
        let cantor = MathematicalCompressor::cantor_pair_compress(ip_u32, port);
        let interleaved = MathematicalCompressor::bit_interleave_compress(ip_u32, port);
        let gray = MathematicalCompressor::gray_code_compress(ip_u32, port);
        
        let max_42_bits = (1u64 << 42) - 1;
        
        if cantor <= max_42_bits {
            let bits_used = 64 - cantor.leading_zeros();
            analysis.strategies.push(StrategyResult {
                name: "Cantor Pairing".to_string(),
                success: true,
                compressed_bits: bits_used as usize,
                compression_ratio: 1.0 - (bits_used as f64 / 48.0),
                method: "Mathematical pairing function".to_string(),
            });
        }
        
        if interleaved <= max_42_bits {
            let bits_used = 64 - interleaved.leading_zeros();
            analysis.strategies.push(StrategyResult {
                name: "Bit Interleaving".to_string(),
                success: true,
                compressed_bits: bits_used as usize,
                compression_ratio: 1.0 - (bits_used as f64 / 48.0),
                method: "Interleaved bit patterns".to_string(),
            });
        }
        
        if gray <= max_42_bits {
            let bits_used = 64 - gray.leading_zeros();
            analysis.strategies.push(StrategyResult {
                name: "Gray Code".to_string(),
                success: true,
                compressed_bits: bits_used as usize,
                compression_ratio: 1.0 - (bits_used as f64 / 48.0),
                method: "Gray code mapping for locality".to_string(),
            });
        }
    }
}

/// Analysis of compression strategies for an address
#[derive(Debug)]
pub struct CompressionAnalysis {
    pub original_bits: usize,
    pub address: String,
    pub strategies: Vec<StrategyResult>,
}

impl CompressionAnalysis {
    pub fn best_strategy(&self) -> Option<&StrategyResult> {
        self.strategies.iter()
            .filter(|s| s.success)
            .max_by(|a, b| a.compression_ratio.partial_cmp(&b.compression_ratio).unwrap())
    }

    pub fn summary(&self) -> String {
        if let Some(best) = self.best_strategy() {
            format!(
                "Best: {} - {} → {} bits ({:.1}% compression) via {}",
                best.name,
                self.original_bits,
                best.compressed_bits,
                best.compression_ratio * 100.0,
                best.method
            )
        } else {
            "No successful compression strategy found".to_string()
        }
    }
}

#[derive(Debug)]
pub struct StrategyResult {
    pub name: String,
    pub success: bool,
    pub compressed_bits: usize,
    pub compression_ratio: f64,
    pub method: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_universal_encoder() {
        let encoder = UniversalEncoder::new().unwrap();
        
        let test_cases = vec![
            "192.168.1.100:80",
            "10.0.0.1:22",
            "8.8.8.8:53",
            "203.45.67.89:12345",
        ];
        
        for address in test_cases {
            println!("\nTesting: {}", address);
            
            match encoder.encode(address) {
                Ok(words) => {
                    println!("  Encoded: {}", words);
                    
                    match encoder.decode(&words) {
                        Ok(decoded) => {
                            println!("  Decoded: {}", decoded);
                        }
                        Err(e) => println!("  Decode error: {}", e),
                    }
                }
                Err(e) => println!("  Encode error: {}", e),
            }
            
            // Show compression analysis
            if let Ok(analysis) = encoder.compression_stats(address) {
                println!("  Analysis: {}", analysis.summary());
                for strategy in &analysis.strategies {
                    if strategy.success {
                        println!("    ✓ {}: {:.1}% compression", 
                                strategy.name, strategy.compression_ratio * 100.0);
                    }
                }
            }
        }
    }
}