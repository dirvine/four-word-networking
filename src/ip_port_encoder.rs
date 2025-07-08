//! Pure IP address and port encoding into four-word addresses.
//! 
//! This module provides efficient encoding of IP addresses (IPv4/IPv6) with optional ports
//! into memorable four-word combinations, without the overhead of multiaddr parsing.

use crate::{dictionary16k::Dictionary16K, error::FourWordError};
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

/// Maximum size of compressed data (5 bytes to fit in 4 words with 16k dictionary)
const MAX_COMPRESSED_SIZE: usize = 5;

/// Common ports that get single-byte encoding
const COMMON_PORTS: &[(u16, u8)] = &[
    (80, 0x01),   // HTTP
    (443, 0x02),  // HTTPS
    (22, 0x03),   // SSH
    (21, 0x04),   // FTP
    (25, 0x05),   // SMTP
    (53, 0x06),   // DNS
    (8080, 0x07), // HTTP alt
    (8443, 0x08), // HTTPS alt
    (3000, 0x09), // Dev server
    (5000, 0x0A), // Dev server
    (8000, 0x0B), // Dev server
    (9000, 0x0C), // Various
    (3306, 0x0D), // MySQL
    (5432, 0x0E), // PostgreSQL
    (6379, 0x0F), // Redis
    (27017, 0x10), // MongoDB
];

/// Represents a parsed IP address with optional port
#[derive(Debug, Clone, PartialEq)]
pub struct IpPortAddress {
    pub ip: IpAddr,
    pub port: Option<u16>,
}

impl IpPortAddress {
    /// Parse from string formats like "192.168.1.1:8080" or "\[2001:db8::1\]:443"
    pub fn parse(input: &str) -> Result<Self, FourWordError> {
        // Handle IPv6 with port: [addr]:port
        if input.starts_with('[') {
            if let Some(close_idx) = input.find(']') {
                let addr_part = &input[1..close_idx];
                let port_part = if close_idx + 1 < input.len() && &input[close_idx + 1..close_idx + 2] == ":" {
                    Some(&input[close_idx + 2..])
                } else {
                    None
                };
                
                let ip = addr_part.parse::<Ipv6Addr>()
                    .map_err(|_| FourWordError::InvalidInput(format!("Invalid IPv6 address: {}", addr_part)))?;
                
                let port = if let Some(port_str) = port_part {
                    Some(port_str.parse::<u16>()
                        .map_err(|_| FourWordError::InvalidInput(format!("Invalid port: {}", port_str)))?)
                } else {
                    None
                };
                
                return Ok(Self {
                    ip: IpAddr::V6(ip),
                    port,
                });
            }
        }
        
        // Handle IPv4 with port or IPv6 without brackets
        if let Some(last_colon) = input.rfind(':') {
            // Check if this is part of IPv6 (multiple colons) or IPv4:port
            let colon_count = input.matches(':').count();
            
            if colon_count == 1 {
                // IPv4:port
                let addr_part = &input[..last_colon];
                let port_part = &input[last_colon + 1..];
                
                let ip = addr_part.parse::<Ipv4Addr>()
                    .map_err(|_| FourWordError::InvalidInput(format!("Invalid IPv4 address: {}", addr_part)))?;
                
                let port = port_part.parse::<u16>()
                    .map_err(|_| FourWordError::InvalidInput(format!("Invalid port: {}", port_part)))?;
                
                return Ok(Self {
                    ip: IpAddr::V4(ip),
                    port: Some(port),
                });
            }
        }
        
        // Try parsing as standalone IP
        let ip = input.parse::<IpAddr>()
            .map_err(|_| FourWordError::InvalidInput(format!("Invalid IP address: {}", input)))?;
        
        Ok(Self { ip, port: None })
    }
    
    /// Convert back to string representation
    pub fn to_string(&self) -> String {
        match (&self.ip, self.port) {
            (IpAddr::V4(addr), Some(port)) => format!("{}:{}", addr, port),
            (IpAddr::V4(addr), None) => addr.to_string(),
            (IpAddr::V6(addr), Some(port)) => format!("[{}]:{}", addr, port),
            (IpAddr::V6(addr), None) => addr.to_string(),
        }
    }
}

/// Compressed representation of IP+port (max 5 bytes)
#[derive(Debug, Clone, PartialEq)]
pub struct CompressedIpPort {
    data: Vec<u8>,
}

impl CompressedIpPort {
    /// Compress an IP+port address into minimal bytes
    pub fn compress(addr: &IpPortAddress) -> Result<Self, FourWordError> {
        let mut data = Vec::with_capacity(MAX_COMPRESSED_SIZE);
        
        match &addr.ip {
            IpAddr::V4(ipv4) => {
                let octets = ipv4.octets();
                
                // Check for localhost patterns
                if octets[0] == 127 {
                    // Localhost: header + last octet + port
                    data.push(0x80); // Localhost marker
                    data.push(octets[3]); // Last octet (usually 1)
                    
                    if let Some(port) = addr.port {
                        Self::compress_port(&mut data, port);
                    }
                }
                // Check for private networks
                else if octets[0] == 192 && octets[1] == 168 {
                    // 192.168.x.x: header + last 2 octets + port
                    data.push(0x81); // Private network marker
                    data.push(octets[2]);
                    data.push(octets[3]);
                    
                    if let Some(port) = addr.port {
                        Self::compress_port(&mut data, port);
                    }
                }
                else if octets[0] == 10 {
                    // 10.x.x.x: header + last 3 octets + port
                    data.push(0x82); // 10.x network marker
                    data.push(octets[1]);
                    data.push(octets[2]);
                    data.push(octets[3]);
                    
                    if let Some(port) = addr.port {
                        // For 10.x networks, we're at 4 bytes, so only common ports fit
                        if let Some(&(_, code)) = COMMON_PORTS.iter().find(|&&(p, _)| p == port) {
                            data.push(code);
                        } else {
                            return Err(FourWordError::InvalidInput(
                                "10.x.x.x addresses with non-common ports exceed 5 bytes".to_string()
                            ));
                        }
                    }
                }
                else if octets[0] == 172 && octets[1] >= 16 && octets[1] <= 31 {
                    // 172.16-31.x.x: header + partial second octet + last 2 octets + port
                    data.push(0x83); // 172.16-31 network marker
                    data.push((octets[1] - 16) << 4 | (octets[2] >> 4)); // Pack 4 bits + upper 4 bits
                    data.push((octets[2] << 4) | (octets[3] >> 4)); // Pack lower 4 bits + upper 4 bits
                    data.push(octets[3] << 4); // Pack lower 4 bits
                    
                    if let Some(port) = addr.port {
                        // We're at 4 bytes, check for common port
                        if let Some(&(_, code)) = COMMON_PORTS.iter().find(|&&(p, _)| p == port) {
                            // Check if port code fits in 4 bits
                            if code > 0x0F {
                                // Port code doesn't fit in 4 bits
                                return Err(FourWordError::InvalidInput(
                                    "172.16-31.x.x addresses require smaller port codes".to_string()
                                ));
                            }
                            // Pack port code into remaining 4 bits
                            data[3] |= code;
                        } else {
                            // Non-common port doesn't fit
                            return Err(FourWordError::InvalidInput(
                                "172.16-31.x.x addresses with non-common ports exceed 5 bytes".to_string()
                            ));
                        }
                    }
                }
                else {
                    // Public IP: header + full 4 octets
                    data.push(0x84); // Public IPv4 marker
                    data.extend_from_slice(&octets);
                    
                    if addr.port.is_some() {
                        // Public IPs with ports don't fit in 5 bytes
                        return Err(FourWordError::InvalidInput(
                            "Public IPv4 addresses with ports exceed 5 bytes".to_string()
                        ));
                    }
                }
            }
            IpAddr::V6(ipv6) => {
                // IPv6 localhost
                if ipv6.is_loopback() {
                    data.push(0x90); // IPv6 localhost marker
                    if let Some(port) = addr.port {
                        Self::compress_port(&mut data, port);
                    }
                }
                // IPv6 link-local (fe80::/10)
                else if ipv6.segments()[0] & 0xFFC0 == 0xFE80 {
                    // Store marker + interface ID (last 64 bits)
                    data.push(0x91); // IPv6 link-local marker
                    // This would need 8 more bytes - doesn't fit
                    return Err(FourWordError::InvalidInput(
                        "IPv6 link-local addresses exceed 5 bytes".to_string()
                    ));
                }
                else {
                    // Full IPv6 doesn't fit in 5 bytes
                    return Err(FourWordError::InvalidInput(
                        "Full IPv6 addresses exceed 5 bytes".to_string()
                    ));
                }
            }
        }
        
        if data.len() > MAX_COMPRESSED_SIZE {
            return Err(FourWordError::InvalidInput(
                format!("Compressed size {} exceeds maximum {}", data.len(), MAX_COMPRESSED_SIZE)
            ));
        }
        
        Ok(Self { data })
    }
    
    /// Add port to compressed data
    fn compress_port(data: &mut Vec<u8>, port: u16) {
        // Try common port encoding first
        if let Some(&(_, code)) = COMMON_PORTS.iter().find(|&&(p, _)| p == port) {
            data.push(code);
        } else {
            // Full port as 2 bytes
            data.push((port >> 8) as u8);
            data.push((port & 0xFF) as u8);
        }
    }
    
    /// Decompress back to IP+port
    pub fn decompress(&self) -> Result<IpPortAddress, FourWordError> {
        if self.data.is_empty() {
            return Err(FourWordError::InvalidInput("Empty compressed data".to_string()));
        }
        
        let header = self.data[0];
        let mut idx = 1;
        
        match header {
            0x80 => {
                // Localhost: 127.0.0.x
                let last_octet = if idx < self.data.len() {
                    self.data[idx]
                } else {
                    1 // Default to 127.0.0.1
                };
                idx += 1;
                
                let ip = IpAddr::V4(Ipv4Addr::new(127, 0, 0, last_octet));
                let port = self.decompress_port(&mut idx)?;
                
                Ok(IpPortAddress { ip, port })
            }
            0x81 => {
                // 192.168.x.x
                if idx + 1 >= self.data.len() {
                    return Err(FourWordError::InvalidInput("Incomplete 192.168.x.x address".to_string()));
                }
                
                let ip = IpAddr::V4(Ipv4Addr::new(192, 168, self.data[idx], self.data[idx + 1]));
                idx += 2;
                
                let port = self.decompress_port(&mut idx)?;
                Ok(IpPortAddress { ip, port })
            }
            0x82 => {
                // 10.x.x.x
                if idx + 2 >= self.data.len() {
                    return Err(FourWordError::InvalidInput("Incomplete 10.x.x.x address".to_string()));
                }
                
                let ip = IpAddr::V4(Ipv4Addr::new(10, self.data[idx], self.data[idx + 1], self.data[idx + 2]));
                idx += 3;
                
                let port = self.decompress_port(&mut idx)?;
                Ok(IpPortAddress { ip, port })
            }
            0x83 => {
                // 172.16-31.x.x (packed format)
                if idx + 2 >= self.data.len() {
                    return Err(FourWordError::InvalidInput("Incomplete 172.16-31.x.x address".to_string()));
                }
                
                let second_octet = 16 + (self.data[idx] >> 4);
                let third_octet = ((self.data[idx] & 0x0F) << 4) | (self.data[idx + 1] >> 4);
                let fourth_octet = ((self.data[idx + 1] & 0x0F) << 4) | (self.data[idx + 2] >> 4);
                idx += 3;
                
                let ip = IpAddr::V4(Ipv4Addr::new(172, second_octet, third_octet, fourth_octet));
                
                // Check for packed port in remaining 4 bits
                let port_code = self.data[idx - 1] & 0x0F;
                let port = if port_code > 0 {
                    COMMON_PORTS.iter().find(|&&(_, c)| c == port_code).map(|&(p, _)| p)
                } else {
                    None
                };
                
                Ok(IpPortAddress { ip, port })
            }
            0x84 => {
                // Public IPv4
                if idx + 3 >= self.data.len() {
                    return Err(FourWordError::InvalidInput("Incomplete public IPv4 address".to_string()));
                }
                
                let ip = IpAddr::V4(Ipv4Addr::new(
                    self.data[idx],
                    self.data[idx + 1],
                    self.data[idx + 2],
                    self.data[idx + 3],
                ));
                idx += 4;
                
                let port = self.decompress_port(&mut idx)?;
                Ok(IpPortAddress { ip, port })
            }
            0x90 => {
                // IPv6 localhost
                let ip = IpAddr::V6(Ipv6Addr::LOCALHOST);
                let port = self.decompress_port(&mut idx)?;
                Ok(IpPortAddress { ip, port })
            }
            _ => Err(FourWordError::InvalidInput(format!("Unknown compression header: 0x{:02X}", header))),
        }
    }
    
    /// Decompress port from data
    fn decompress_port(&self, idx: &mut usize) -> Result<Option<u16>, FourWordError> {
        if *idx >= self.data.len() {
            return Ok(None);
        }
        
        let first_byte = self.data[*idx];
        
        // Check if it's a common port code
        if let Some(&(port, _)) = COMMON_PORTS.iter().find(|&&(_, code)| code == first_byte) {
            *idx += 1;
            return Ok(Some(port));
        }
        
        // Otherwise it's a full 2-byte port
        if *idx + 1 >= self.data.len() {
            return Err(FourWordError::InvalidInput("Incomplete port data".to_string()));
        }
        
        let port = ((first_byte as u16) << 8) | (self.data[*idx + 1] as u16);
        *idx += 2;
        
        Ok(Some(port))
    }
}

/// Main encoder for IP+port to four-word addresses
pub struct IpPortEncoder {
    dictionary: Dictionary16K,
}

impl IpPortEncoder {
    /// Create a new encoder with the 16k dictionary
    pub fn new() -> Result<Self, FourWordError> {
        Ok(Self {
            dictionary: Dictionary16K::new()
                .map_err(|e| FourWordError::InvalidInput(e.to_string()))?,
        })
    }
    
    /// Encode an IP+port address into four words
    pub fn encode(&self, input: &str) -> Result<String, FourWordError> {
        let addr = IpPortAddress::parse(input)?;
        let compressed = CompressedIpPort::compress(&addr)?;
        
        // Convert compressed bytes to four words using 16k dictionary
        let words = self.dictionary.encode_bytes(&compressed.data)
            .map_err(|e| FourWordError::InvalidInput(e.to_string()))?;
        Ok(words.join("."))
    }
    
    /// Decode four words back to IP+port address
    pub fn decode(&self, words: &str) -> Result<String, FourWordError> {
        let word_vec: Vec<&str> = words.split('.').collect();
        if word_vec.len() != 3 {
            return Err(FourWordError::InvalidInput(
                format!("Expected 4 words, got {}", word_vec.len())
            ));
        }
        
        let bytes = self.dictionary.decode_words(&word_vec)
            .map_err(|e| FourWordError::InvalidInput(e.to_string()))?;
        let compressed = CompressedIpPort { data: bytes };
        let addr = compressed.decompress()?;
        
        Ok(addr.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_ip_port_parsing() {
        // IPv4 with port
        let addr = IpPortAddress::parse("192.168.1.100:8080").unwrap();
        assert_eq!(addr.ip, IpAddr::V4(Ipv4Addr::new(192, 168, 1, 100)));
        assert_eq!(addr.port, Some(8080));
        
        // IPv4 without port
        let addr = IpPortAddress::parse("10.0.0.1").unwrap();
        assert_eq!(addr.ip, IpAddr::V4(Ipv4Addr::new(10, 0, 0, 1)));
        assert_eq!(addr.port, None);
        
        // IPv6 with port
        let addr = IpPortAddress::parse("[2001:db8::1]:443").unwrap();
        assert_eq!(addr.port, Some(443));
        
        // IPv6 without port
        let addr = IpPortAddress::parse("::1").unwrap();
        assert_eq!(addr.ip, IpAddr::V6(Ipv6Addr::LOCALHOST));
        assert_eq!(addr.port, None);
    }
    
    #[test]
    fn test_localhost_compression() {
        let addr = IpPortAddress {
            ip: IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
            port: Some(8080),
        };
        
        let compressed = CompressedIpPort::compress(&addr).unwrap();
        assert_eq!(compressed.data.len(), 3); // header + last octet + common port
        
        let decompressed = compressed.decompress().unwrap();
        assert_eq!(decompressed, addr);
    }
    
    #[test]
    fn test_private_network_compression() {
        // 192.168.x.x
        let addr = IpPortAddress {
            ip: IpAddr::V4(Ipv4Addr::new(192, 168, 1, 100)),
            port: Some(443),
        };
        
        let compressed = CompressedIpPort::compress(&addr).unwrap();
        assert_eq!(compressed.data.len(), 4); // header + 2 octets + common port
        
        let decompressed = compressed.decompress().unwrap();
        assert_eq!(decompressed, addr);
        
        // 10.x.x.x
        let addr = IpPortAddress {
            ip: IpAddr::V4(Ipv4Addr::new(10, 20, 30, 40)),
            port: Some(22),
        };
        
        let compressed = CompressedIpPort::compress(&addr).unwrap();
        assert_eq!(compressed.data.len(), 5); // header + 3 octets + common port
        
        let decompressed = compressed.decompress().unwrap();
        assert_eq!(decompressed, addr);
    }
    
    #[test]
    fn test_ipv6_localhost_compression() {
        let addr = IpPortAddress {
            ip: IpAddr::V6(Ipv6Addr::LOCALHOST),
            port: Some(443),
        };
        
        let compressed = CompressedIpPort::compress(&addr).unwrap();
        assert_eq!(compressed.data.len(), 2); // header + common port
        
        let decompressed = compressed.decompress().unwrap();
        assert_eq!(decompressed, addr);
    }
    
    #[test]
    fn test_encoder_integration() {
        let encoder = IpPortEncoder::new().unwrap();
        
        // Test various addresses
        let test_cases = vec![
            "127.0.0.1:8080",
            "192.168.1.1:443",
            "10.0.0.1:22",
            "[::1]:443",
        ];
        
        for addr in test_cases {
            let encoded = encoder.encode(addr).unwrap();
            let words: Vec<&str> = encoded.split('.').collect();
            assert_eq!(words.len(), 3);
            
            let decoded = encoder.decode(&encoded).unwrap();
            assert_eq!(decoded, addr);
        }
    }
    
    #[test]
    fn test_error_cases() {
        let encoder = IpPortEncoder::new().unwrap();
        
        // Test invalid addresses
        assert!(encoder.encode("invalid").is_err());
        assert!(encoder.encode("256.256.256.256").is_err());
        assert!(encoder.encode("192.168.1.1:99999").is_err());
        
        // Test addresses that exceed 5 bytes when compressed
        assert!(encoder.encode("8.8.8.8:1234").is_err()); // Public IP with non-common port
        
        // 172.20 is in the private range 172.16-31, and port 9999 is not common
        // So it should fail because non-common ports don't fit in the packed format
        assert!(encoder.encode("172.20.1.1:9999").is_err());
        
        assert!(encoder.encode("10.0.0.1:1234").is_err()); // 10.x with non-common port
    }
    
    #[test]
    fn test_common_ports() {
        let encoder = IpPortEncoder::new().unwrap();
        
        // Test all common ports with different IP types
        let common_ports = vec![80, 443, 22, 21, 25, 53, 8080, 8443, 3000, 5000, 8000, 9000];
        
        for port in common_ports {
            // Localhost
            let addr = format!("127.0.0.1:{}", port);
            let encoded = encoder.encode(&addr).unwrap();
            let decoded = encoder.decode(&encoded).unwrap();
            assert_eq!(decoded, addr);
            
            // Private network
            let addr = format!("192.168.1.1:{}", port);
            let encoded = encoder.encode(&addr).unwrap();
            let decoded = encoder.decode(&encoded).unwrap();
            assert_eq!(decoded, addr);
        }
    }
    
    #[test]
    fn test_edge_cases() {
        let encoder = IpPortEncoder::new().unwrap();
        
        // Test edge cases for each IP type
        let test_cases = vec![
            "127.0.0.255:80",    // Localhost with max last octet
            "192.168.255.255:443", // Max private network address
            "10.255.255.255:22",   // Max 10.x address
            "172.16.0.0:80",       // Min 172.16-31 range with common port (0x01)
            "172.31.255.255:53",   // Max 172.16-31 range with common port (0x06)
        ];
        
        for addr in test_cases {
            let encoded = encoder.encode(addr).unwrap();
            let decoded = encoder.decode(&encoded).unwrap();
            assert_eq!(decoded, addr);
        }
    }
    
    #[test]
    fn test_ipv6_support() {
        let encoder = IpPortEncoder::new().unwrap();
        
        // Currently only localhost is supported for IPv6
        let encoded = encoder.encode("[::1]:80").unwrap();
        let decoded = encoder.decode(&encoded).unwrap();
        assert_eq!(decoded, "[::1]:80");
        
        // Other IPv6 addresses should fail
        assert!(encoder.encode("[2001:db8::1]:80").is_err());
        assert!(encoder.encode("[fe80::1]:80").is_err());
    }
}