//! IPv4 Perfect Codec - 100% reconstruction for all IPv4+port combinations
//!
//! This module implements perfect encoding/decoding of IPv4 addresses with ports
//! using the multi-dimensional encoding system.

use std::net::Ipv4Addr;
use crate::{
    perfect_encoder::{PerfectEncoder, MultiDimEncoding}, Result,
};

/// IPv4 codec that guarantees perfect reconstruction
pub struct IPv4PerfectCodec {
    encoder: PerfectEncoder,
}

impl IPv4PerfectCodec {
    pub fn new() -> Result<Self> {
        Ok(Self {
            encoder: PerfectEncoder::new()?,
        })
    }
    
    /// Encode IPv4 address + port into four words with perfect reconstruction
    pub fn encode(&self, ip: Ipv4Addr, port: u16) -> Result<MultiDimEncoding> {
        // Convert IPv4 + port to 48 bits
        let ip_bits = u32::from_be_bytes(ip.octets()) as u64;
        let port_bits = port as u64;
        let combined = (ip_bits << 16) | port_bits;
        
        // Use perfect encoder
        self.encoder.encode_48_bits(combined)
    }
    
    /// Decode four words back to exact IPv4 address + port
    pub fn decode(&self, encoding: &MultiDimEncoding) -> Result<(Ipv4Addr, u16)> {
        // Decode to 48 bits
        let combined = self.encoder.decode_48_bits(encoding)?;
        
        // Extract IP and port
        let ip_bits = ((combined >> 16) & 0xFFFFFFFF) as u32;
        let port = (combined & 0xFFFF) as u16;
        
        let ip = Ipv4Addr::from(ip_bits);
        
        Ok((ip, port))
    }
    
    /// Encode IPv4 to string format
    pub fn encode_to_string(&self, ip: Ipv4Addr, port: u16) -> Result<String> {
        let encoding = self.encode(ip, port)?;
        Ok(encoding.to_string())
    }
    
    /// Decode from string format
    pub fn decode_from_string(&self, s: &str) -> Result<(Ipv4Addr, u16)> {
        let dictionary = crate::perfect_encoder::PerfectDictionary::new()?;
        let encoding = MultiDimEncoding::from_string(s, &dictionary)?;
        self.decode(&encoding)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_ipv4_perfect_roundtrip() {
        let codec = IPv4PerfectCodec::new().unwrap();
        
        // Test various IPv4 addresses and ports
        let test_cases = vec![
            (Ipv4Addr::new(192, 168, 1, 1), 443),
            (Ipv4Addr::new(10, 0, 0, 1), 22),
            (Ipv4Addr::new(8, 8, 8, 8), 53),
            (Ipv4Addr::new(255, 255, 255, 255), 65535),
            (Ipv4Addr::new(0, 0, 0, 0), 0),
            (Ipv4Addr::new(127, 0, 0, 1), 8080),
        ];
        
        for (ip, port) in test_cases {
            let encoded = codec.encode(ip, port).unwrap();
            let (decoded_ip, decoded_port) = codec.decode(&encoded).unwrap();
            
            assert_eq!(ip, decoded_ip, "IP mismatch for {}", ip);
            assert_eq!(port, decoded_port, "Port mismatch for {}:{}", ip, port);
        }
    }
    
    #[test]
    fn test_ipv4_string_format() {
        let codec = IPv4PerfectCodec::new().unwrap();
        
        let ip = Ipv4Addr::new(192, 168, 1, 1);
        let port = 443;
        
        let encoded_str = codec.encode_to_string(ip, port).unwrap();
        println!("Encoded {}:{} as: {}", ip, port, encoded_str);
        
        // Should be able to decode back
        let (decoded_ip, decoded_port) = codec.decode_from_string(&encoded_str).unwrap();
        assert_eq!(ip, decoded_ip);
        assert_eq!(port, decoded_port);
    }
    
    #[test]
    fn test_all_ports_for_ip() {
        let codec = IPv4PerfectCodec::new().unwrap();
        let ip = Ipv4Addr::new(192, 168, 1, 1);
        
        // Test a range of ports
        for port in (0..=65535).step_by(1000) {
            let encoded = codec.encode(ip, port).unwrap();
            let (decoded_ip, decoded_port) = codec.decode(&encoded).unwrap();
            
            assert_eq!(ip, decoded_ip, "IP mismatch at port {}", port);
            assert_eq!(port, decoded_port, "Port mismatch at port {}", port);
        }
    }
}