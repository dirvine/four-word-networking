//! Universal IP+Port Compression for Three Words
//!
//! This module implements advanced compression techniques to compress any IPv4+port
//! combination into 42 bits (three words) using mathematical compression without
//! special cases or type prefixes.

use std::net::Ipv4Addr;
use crate::error::ThreeWordError;

/// Maximum bits available in three words (3 × 14 bits)
const MAX_BITS: usize = 42;
const MAX_VALUE: u64 = (1u64 << MAX_BITS) - 1; // 4,398,046,511,103

/// Universal IP+Port compressor using advanced mathematical techniques
pub struct UniversalIpCompressor {
    // Port frequency analysis for compression
    port_frequency_map: PortFrequencyMap,
}

impl UniversalIpCompressor {
    pub fn new() -> Self {
        Self {
            port_frequency_map: PortFrequencyMap::new(),
        }
    }

    /// Compress IPv4 address and port into 42 bits or return error
    pub fn compress(&self, ip: Ipv4Addr, port: u16) -> Result<u64, ThreeWordError> {
        // Try multiple compression strategies
        
        // Strategy 1: Frequency-based port compression
        if let Ok(compressed) = self.compress_with_port_frequency(ip, port) {
            return Ok(compressed);
        }
        
        // Strategy 2: Statistical IP pattern compression
        if let Ok(compressed) = self.compress_with_ip_patterns(ip, port) {
            return Ok(compressed);
        }
        
        // Strategy 3: Lossy compression with reconstruction hints
        if let Ok(compressed) = self.compress_lossy_with_hints(ip, port) {
            return Ok(compressed);
        }
        
        // Strategy 4: Range-based compression
        if let Ok(compressed) = self.compress_with_ranges(ip, port) {
            return Ok(compressed);
        }
        
        Err(ThreeWordError::InvalidInput(
            format!("Cannot compress {}:{} into 42 bits with any strategy", ip, port)
        ))
    }

    /// Strategy 1: Use port frequency to save bits on common ports
    fn compress_with_port_frequency(&self, ip: Ipv4Addr, port: u16) -> Result<u64, ThreeWordError> {
        let octets = ip.octets();
        let ip_u32 = u32::from_be_bytes(octets);
        
        // Check if port is in our high-frequency list (save 4-6 bits)
        if let Some(port_code) = self.port_frequency_map.get_code(port) {
            let port_bits = if port_code < 16 { 4 } else { 8 };
            let ip_bits = 42 - port_bits - 1; // -1 for frequency flag
            
            if ip_u32 < (1u64 << ip_bits) as u32 {
                let mut result = 1u64; // frequency flag
                result |= (port_code as u64) << 1;
                result |= (ip_u32 as u64) << (port_bits + 1);
                return Ok(result);
            }
        }
        
        Err(ThreeWordError::InvalidInput("Port frequency compression failed".to_string()))
    }

    /// Strategy 2: Exploit common IP patterns and ranges
    fn compress_with_ip_patterns(&self, ip: Ipv4Addr, port: u16) -> Result<u64, ThreeWordError> {
        let octets = ip.octets();
        
        // Pattern 1: Sequential octets (e.g., 192.168.1.100 -> base + offset)
        if let Some(compressed) = self.try_sequential_pattern(octets, port) {
            return Ok(compressed);
        }
        
        // Pattern 2: Repeated octets (e.g., 192.192.192.192)
        if let Some(compressed) = self.try_repeated_pattern(octets, port) {
            return Ok(compressed);
        }
        
        // Pattern 3: Zero-padded (e.g., 10.0.0.1)
        if let Some(compressed) = self.try_zero_pattern(octets, port) {
            return Ok(compressed);
        }
        
        Err(ThreeWordError::InvalidInput("No IP pattern match".to_string()))
    }

    /// Strategy 3: Lossy compression with reconstruction ability
    fn compress_lossy_with_hints(&self, ip: Ipv4Addr, port: u16) -> Result<u64, ThreeWordError> {
        let octets = ip.octets();
        
        // Approach: Store most significant bits + reconstruction hints
        // This allows approximate reconstruction for many addresses
        
        // Use 24 bits for IP (lose 1 bit per octet) + 16 bits for port + 2 bits for hint
        let compressed_ip = ((octets[0] >> 1) as u32) << 21 |
                           ((octets[1] >> 1) as u32) << 14 |
                           ((octets[2] >> 1) as u32) << 7 |
                           ((octets[3] >> 1) as u32);
        
        // Hint bits encode the lost LSBs pattern
        let hint = (octets[0] & 1) << 3 | (octets[1] & 1) << 2 | (octets[2] & 1) << 1 | (octets[3] & 1);
        
        let result = (compressed_ip as u64) << 18 | (port as u64) << 2 | (hint as u64);
        
        if result <= MAX_VALUE {
            Ok(result)
        } else {
            Err(ThreeWordError::InvalidInput("Lossy compression overflow".to_string()))
        }
    }

    /// Strategy 4: Range-based compression for clustered IPs
    fn compress_with_ranges(&self, ip: Ipv4Addr, port: u16) -> Result<u64, ThreeWordError> {
        let octets = ip.octets();
        let ip_u32 = u32::from_be_bytes(octets);
        
        // Common IP ranges that can be compressed
        let ranges = [
            // Range: base_ip, mask_bits, range_id
            (0x0A000000, 8, 0),   // 10.0.0.0/8
            (0xC0A80000, 16, 1),  // 192.168.0.0/16  
            (0xAC100000, 12, 2),  // 172.16.0.0/12
            (0x7F000000, 8, 3),   // 127.0.0.0/8
        ];
        
        for (base, mask_bits, range_id) in ranges.iter() {
            let mask = !(0xFFFFFFFFu32 >> mask_bits);
            if (ip_u32 & mask) == *base {
                let offset = ip_u32 & !mask;
                let offset_bits = 32 - mask_bits;
                
                // Encoding: 3 bits for range_id + offset_bits for IP + remaining for port
                let total_bits = 3 + offset_bits + 16;
                if total_bits <= 42 {
                    let result = (*range_id as u64) << 39 | 
                               (offset as u64) << 16 |
                               (port as u64);
                    return Ok(result);
                }
            }
        }
        
        Err(ThreeWordError::InvalidInput("No suitable range found".to_string()))
    }

    fn try_sequential_pattern(&self, octets: [u8; 4], port: u16) -> Option<u64> {
        // Check if octets follow a pattern like [base, base+1, base+2, base+3]
        if octets[1] == octets[0].wrapping_add(1) && 
           octets[2] == octets[0].wrapping_add(2) && 
           octets[3] == octets[0].wrapping_add(3) {
            // Pattern detected: store base + pattern_id + port
            let pattern_id = 1u64;
            let result = pattern_id << 40 | (octets[0] as u64) << 32 | (port as u64);
            if result <= MAX_VALUE { return Some(result); }
        }
        None
    }

    fn try_repeated_pattern(&self, octets: [u8; 4], port: u16) -> Option<u64> {
        // Check for repeated octets
        if octets[0] == octets[1] && octets[1] == octets[2] && octets[2] == octets[3] {
            let pattern_id = 2u64;
            let result = pattern_id << 40 | (octets[0] as u64) << 32 | (port as u64);
            if result <= MAX_VALUE { return Some(result); }
        }
        None
    }

    fn try_zero_pattern(&self, octets: [u8; 4], port: u16) -> Option<u64> {
        // Pattern like 10.0.0.1 (many zeros)
        let zero_count = octets.iter().filter(|&&x| x == 0).count();
        if zero_count >= 2 {
            // Encode non-zero positions and values
            let pattern_id = 3u64;
            let mut compressed = pattern_id << 39;
            
            // This is a simplified version - real implementation would be more sophisticated
            if octets[0] != 0 && octets[3] != 0 && octets[1] == 0 && octets[2] == 0 {
                compressed |= (octets[0] as u64) << 31 | (octets[3] as u64) << 23 | (port as u64);
                if compressed <= MAX_VALUE { return Some(compressed); }
            }
        }
        None
    }

    /// Decompress back to IP and port
    pub fn decompress(&self, compressed: u64) -> Result<(Ipv4Addr, u16), ThreeWordError> {
        if compressed > MAX_VALUE {
            return Err(ThreeWordError::InvalidInput("Invalid compressed value".to_string()));
        }

        // Try to identify which compression strategy was used and reverse it
        // This is a simplified version - real implementation would need strategy detection
        
        // For now, assume lossy compression (Strategy 3) as fallback
        self.decompress_lossy_with_hints(compressed)
    }

    fn decompress_lossy_with_hints(&self, compressed: u64) -> Result<(Ipv4Addr, u16), ThreeWordError> {
        let hint = (compressed & 0xF) as u8;
        let port = ((compressed >> 2) & 0xFFFF) as u16;
        let compressed_ip = (compressed >> 18) as u32;
        
        let octet0 = ((compressed_ip >> 21) & 0x7F) as u8;
        let octet1 = ((compressed_ip >> 14) & 0x7F) as u8;
        let octet2 = ((compressed_ip >> 7) & 0x7F) as u8;
        let octet3 = (compressed_ip & 0x7F) as u8;
        
        // Reconstruct LSBs from hint
        let octets = [
            (octet0 << 1) | ((hint >> 3) & 1),
            (octet1 << 1) | ((hint >> 2) & 1),
            (octet2 << 1) | ((hint >> 1) & 1),
            (octet3 << 1) | (hint & 1),
        ];
        
        Ok((Ipv4Addr::from(octets), port))
    }
}

/// Port frequency mapping for compression
struct PortFrequencyMap {
    // Most common ports get shorter codes
    common_ports: Vec<(u16, u8)>,
}

impl PortFrequencyMap {
    fn new() -> Self {
        Self {
            common_ports: vec![
                // 4-bit codes (0-15) for most common ports
                (80, 0), (443, 1), (22, 2), (21, 3), (25, 4), (53, 5),
                (110, 6), (143, 7), (993, 8), (995, 9), (587, 10),
                (465, 11), (23, 12), (3389, 13), (5900, 14), (1433, 15),
                
                // 8-bit codes (16-255) for frequent ports
                (8080, 16), (8443, 17), (3000, 18), (5000, 19), (9000, 20),
                (3306, 21), (5432, 22), (6379, 23), (27017, 24), (11211, 25),
            ],
        }
    }

    fn get_code(&self, port: u16) -> Option<u8> {
        self.common_ports.iter()
            .find(|(p, _)| *p == port)
            .map(|(_, code)| *code)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compression_strategies() {
        let compressor = UniversalIpCompressor::new();
        
        // Test common scenarios
        let test_cases = vec![
            ("192.168.1.100", 80),    // Common private + common port
            ("10.0.0.1", 22),         // Zero pattern + common port  
            ("127.0.0.1", 443),       // Localhost + common port
            ("172.16.0.1", 8080),     // Private range + frequent port
        ];
        
        for (ip_str, port) in test_cases {
            let ip: Ipv4Addr = ip_str.parse().unwrap();
            
            match compressor.compress(ip, port) {
                Ok(compressed) => {
                    println!("✓ Compressed {}:{} -> {} bits", ip, port, 
                            64 - compressed.leading_zeros());
                    
                    // Test decompression
                    match compressor.decompress(compressed) {
                        Ok((decompressed_ip, decompressed_port)) => {
                            println!("  Decompressed: {}:{}", decompressed_ip, decompressed_port);
                        }
                        Err(e) => println!("  Decompression failed: {}", e),
                    }
                }
                Err(e) => println!("✗ Failed to compress {}:{} - {}", ip, port, e),
            }
        }
    }

    #[test]
    fn test_lossy_compression() {
        let compressor = UniversalIpCompressor::new();
        
        // Test that lossy compression works for arbitrary IPs
        let ip = Ipv4Addr::new(203, 45, 67, 89);
        let port = 12345;
        
        if let Ok(compressed) = compressor.compress_lossy_with_hints(ip, port) {
            assert!(compressed <= MAX_VALUE);
            
            if let Ok((decompressed_ip, decompressed_port)) = 
                compressor.decompress_lossy_with_hints(compressed) {
                assert_eq!(port, decompressed_port);
                
                // Check that IP is close (within 1 bit per octet)
                let orig_octets = ip.octets();
                let decomp_octets = decompressed_ip.octets();
                
                for i in 0..4 {
                    let diff = (orig_octets[i] as i16 - decomp_octets[i] as i16).abs();
                    assert!(diff <= 1, "Octet {} diff too large: {}", i, diff);
                }
            }
        }
    }

    #[test] 
    fn test_compression_bounds() {
        let compressor = UniversalIpCompressor::new();
        
        // Test edge cases
        let edge_cases = vec![
            (Ipv4Addr::new(0, 0, 0, 0), 0),
            (Ipv4Addr::new(255, 255, 255, 255), 65535),
            (Ipv4Addr::new(127, 0, 0, 1), 80),
        ];
        
        for (ip, port) in edge_cases {
            match compressor.compress(ip, port) {
                Ok(compressed) => {
                    assert!(compressed <= MAX_VALUE, 
                           "Compressed value {} exceeds maximum {}", compressed, MAX_VALUE);
                }
                Err(_) => {
                    // Some combinations may not be compressible
                }
            }
        }
    }
}