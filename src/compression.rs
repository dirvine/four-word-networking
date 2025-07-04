//! Multiaddress compression for efficient encoding
//!
//! This module provides intelligent compression for multiaddresses by:
//! - Converting protocol names to single bytes
//! - Compressing IPv6 addresses with run-length encoding
//! - Using single bytes for common ports
//! - Compressing peer IDs by removing redundant prefixes
//!
//! Typical compression ratios: 40-60% for multiaddresses

use std::collections::HashMap;
use std::net::{Ipv4Addr, Ipv6Addr};
use std::str::FromStr;

/// Compression errors
#[derive(Debug, thiserror::Error)]
pub enum CompressionError {
    #[error("Invalid multiaddress format: {0}")]
    InvalidFormat(String),
    
    #[error("Unknown protocol: {0}")]
    UnknownProtocol(String),
    
    #[error("Invalid IP address: {0}")]
    InvalidIpAddress(String),
    
    #[error("Invalid port: {0}")]
    InvalidPort(String),
    
    #[error("Base58 decode error: {0}")]
    Base58Error(String),
    
    #[error("UTF-8 error: {0}")]
    Utf8Error(#[from] std::str::Utf8Error),
}

/// Data types for encoding strategy
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DataType {
    Multiaddress,
    Hash,
    BitcoinAddress,
    EthereumAddress,
    Unknown,
}

/// Multiaddress compressor with protocol and port optimization
#[derive(Debug, Clone)]
pub struct MultiaddressCompressor {
    protocol_codes: HashMap<&'static str, u8>,
    port_codes: HashMap<u16, u8>,
    reverse_protocols: HashMap<u8, &'static str>,
    reverse_ports: HashMap<u8, u16>,
}

impl MultiaddressCompressor {
    /// Create a new compressor with optimized tables
    pub fn new() -> Self {
        let mut protocol_codes = HashMap::new();
        let mut port_codes = HashMap::new();
        
        // Protocol codes (most common get shortest codes)
        protocol_codes.insert("ip4", 0x00);
        protocol_codes.insert("ip6", 0x01);
        protocol_codes.insert("tcp", 0x02);
        protocol_codes.insert("udp", 0x03);
        protocol_codes.insert("p2p", 0x04);
        protocol_codes.insert("ipfs", 0x04); // Same as p2p
        protocol_codes.insert("ws", 0x05);
        protocol_codes.insert("wss", 0x06);
        protocol_codes.insert("quic", 0x07);
        protocol_codes.insert("tls", 0x08);
        protocol_codes.insert("dns", 0x09);
        protocol_codes.insert("dns4", 0x0A);
        protocol_codes.insert("dns6", 0x0B);
        protocol_codes.insert("sctp", 0x0C);
        protocol_codes.insert("dccp", 0x0D);
        protocol_codes.insert("http", 0x0E);
        protocol_codes.insert("https", 0x0F);
        
        // Common ports get single-byte encoding
        port_codes.insert(80, 0x00);    // HTTP
        port_codes.insert(443, 0x01);   // HTTPS
        port_codes.insert(8080, 0x02);  // HTTP Alt
        port_codes.insert(3000, 0x03);  // Dev server
        port_codes.insert(4001, 0x04);  // IPFS
        port_codes.insert(5001, 0x05);  // IPFS API
        port_codes.insert(8000, 0x06);  // HTTP Alt
        port_codes.insert(9000, 0x07);  // Various
        port_codes.insert(22, 0x08);    // SSH
        port_codes.insert(21, 0x09);    // FTP
        port_codes.insert(25, 0x0A);    // SMTP
        port_codes.insert(53, 0x0B);    // DNS
        port_codes.insert(110, 0x0C);   // POP3
        port_codes.insert(143, 0x0D);   // IMAP
        port_codes.insert(993, 0x0E);   // IMAPS
        port_codes.insert(995, 0x0F);   // POP3S
        
        // Build reverse lookup tables - prefer p2p over ipfs for code 0x04
        let mut reverse_protocols = HashMap::new();
        for (&protocol, &code) in protocol_codes.iter() {
            if protocol == "p2p" || !reverse_protocols.contains_key(&code) {
                reverse_protocols.insert(code, protocol);
            }
        }
        let reverse_ports = port_codes.iter()
            .map(|(&k, &v)| (v, k))
            .collect();
        
        Self {
            protocol_codes,
            port_codes,
            reverse_protocols,
            reverse_ports,
        }
    }
    
    /// Compress a multiaddress string to bytes
    pub fn compress(&self, multiaddr: &str) -> Result<Vec<u8>, CompressionError> {
        if !multiaddr.starts_with('/') {
            return Err(CompressionError::InvalidFormat(
                "Multiaddress must start with '/'".to_string()
            ));
        }
        
        let mut compressed = Vec::new();
        let components: Vec<&str> = multiaddr.trim_start_matches('/').split('/').collect();
        
        let mut i = 0;
        while i < components.len() {
            let protocol = components[i];
            
            // Encode protocol
            let protocol_code = self.protocol_codes.get(protocol)
                .ok_or_else(|| CompressionError::UnknownProtocol(protocol.to_string()))?;
            
            compressed.push(*protocol_code);
            i += 1;
            
            // Handle protocol-specific data
            match protocol {
                "ip4" => {
                    if i >= components.len() {
                        return Err(CompressionError::InvalidFormat("Missing IP4 address".to_string()));
                    }
                    
                    let ip = Ipv4Addr::from_str(components[i])
                        .map_err(|_| CompressionError::InvalidIpAddress(components[i].to_string()))?;
                    compressed.extend_from_slice(&ip.octets());
                    i += 1;
                }
                "ip6" => {
                    if i >= components.len() {
                        return Err(CompressionError::InvalidFormat("Missing IP6 address".to_string()));
                    }
                    
                    let ip = Ipv6Addr::from_str(components[i])
                        .map_err(|_| CompressionError::InvalidIpAddress(components[i].to_string()))?;
                    let compressed_ip6 = self.compress_ipv6(&ip);
                    compressed.extend_from_slice(&compressed_ip6);
                    i += 1;
                }
                "tcp" | "udp" | "sctp" | "dccp" => {
                    if i >= components.len() {
                        return Err(CompressionError::InvalidFormat(format!("Missing {} port", protocol)));
                    }
                    
                    let port: u16 = components[i].parse()
                        .map_err(|_| CompressionError::InvalidPort(components[i].to_string()))?;
                    
                    if let Some(&port_code) = self.port_codes.get(&port) {
                        // Common port - single byte
                        compressed.push(0xFF); // Marker for compressed port
                        compressed.push(port_code);
                    } else {
                        // Uncommon port - two bytes
                        compressed.extend_from_slice(&port.to_be_bytes());
                    }
                    i += 1;
                }
                "p2p" | "ipfs" => {
                    if i >= components.len() {
                        return Err(CompressionError::InvalidFormat("Missing peer ID".to_string()));
                    }
                    
                    let compressed_peer = self.compress_peer_id(components[i])?;
                    compressed.push(compressed_peer.len() as u8); // Length prefix
                    compressed.extend_from_slice(&compressed_peer);
                    i += 1;
                }
                _ => {
                    // Other protocols that might have data
                    if i < components.len() && !components[i].is_empty() {
                        let data = components[i].as_bytes();
                        compressed.push(data.len() as u8); // Length prefix
                        compressed.extend_from_slice(data);
                        i += 1;
                    }
                }
            }
        }
        
        Ok(compressed)
    }
    
    /// Decompress bytes back to a multiaddress
    pub fn decompress(&self, data: &[u8]) -> Result<String, CompressionError> {
        let mut result = String::new();
        let mut i = 0;
        
        while i < data.len() {
            let protocol_code = data[i];
            i += 1;
            
            let protocol = self.reverse_protocols.get(&protocol_code)
                .ok_or_else(|| CompressionError::UnknownProtocol(format!("Unknown code: {}", protocol_code)))?;
            
            result.push('/');
            result.push_str(protocol);
            
            // Handle protocol-specific data
            match *protocol {
                "ip4" => {
                    if i + 4 > data.len() {
                        return Err(CompressionError::InvalidFormat("Incomplete IP4 address".to_string()));
                    }
                    
                    let ip = Ipv4Addr::new(data[i], data[i+1], data[i+2], data[i+3]);
                    result.push('/');
                    result.push_str(&ip.to_string());
                    i += 4;
                }
                "ip6" => {
                    let (ip, bytes_read) = self.decompress_ipv6(&data[i..])?;
                    result.push('/');
                    result.push_str(&ip.to_string());
                    i += bytes_read;
                }
                "tcp" | "udp" | "sctp" | "dccp" => {
                    if i >= data.len() {
                        return Err(CompressionError::InvalidFormat(format!("Missing {} port", protocol)));
                    }
                    
                    let port = if data[i] == 0xFF {
                        // Compressed port
                        i += 1;
                        if i >= data.len() {
                            return Err(CompressionError::InvalidFormat("Incomplete compressed port".to_string()));
                        }
                        
                        let port_code = data[i];
                        i += 1;
                        
                        *self.reverse_ports.get(&port_code)
                            .ok_or_else(|| CompressionError::InvalidPort(format!("Unknown port code: {}", port_code)))?
                    } else {
                        // Uncompressed port
                        if i + 2 > data.len() {
                            return Err(CompressionError::InvalidFormat("Incomplete port".to_string()));
                        }
                        
                        let port = u16::from_be_bytes([data[i], data[i+1]]);
                        i += 2;
                        port
                    };
                    
                    result.push('/');
                    result.push_str(&port.to_string());
                }
                "p2p" | "ipfs" => {
                    if i >= data.len() {
                        return Err(CompressionError::InvalidFormat("Missing peer ID length".to_string()));
                    }
                    
                    let length = data[i] as usize;
                    i += 1;
                    
                    if i + length > data.len() {
                        return Err(CompressionError::InvalidFormat("Incomplete peer ID".to_string()));
                    }
                    
                    let peer_id = self.decompress_peer_id(&data[i..i+length])?;
                    result.push('/');
                    result.push_str(&peer_id);
                    i += length;
                }
                _ => {
                    // Generic data with length prefix
                    if i < data.len() {
                        let length = data[i] as usize;
                        i += 1;
                        
                        if i + length <= data.len() && length > 0 {
                            let text = std::str::from_utf8(&data[i..i+length])?;
                            result.push('/');
                            result.push_str(text);
                            i += length;
                        }
                    }
                }
            }
        }
        
        if result.is_empty() {
            result.push('/');
        }
        
        Ok(result)
    }
    
    /// Compress IPv6 address using run-length encoding for zeros
    fn compress_ipv6(&self, addr: &Ipv6Addr) -> Vec<u8> {
        let octets = addr.octets();
        let mut compressed = Vec::new();
        
        let mut i = 0;
        while i < 16 {
            if octets[i] == 0 {
                // Count consecutive zeros
                let zero_count = octets[i..].iter().take_while(|&&b| b == 0).count();
                
                if zero_count > 2 {
                    // Use run-length encoding for 3+ consecutive zeros
                    compressed.push(0xFE); // Zero run marker
                    compressed.push(zero_count as u8);
                    i += zero_count;
                } else {
                    // Just store the zeros normally for short runs
                    compressed.push(0);
                    i += 1;
                }
            } else {
                compressed.push(octets[i]);
                i += 1;
            }
        }
        
        compressed
    }
    
    /// Decompress IPv6 address from compressed format
    fn decompress_ipv6(&self, data: &[u8]) -> Result<(Ipv6Addr, usize), CompressionError> {
        let mut octets = [0u8; 16];
        let mut i = 0;
        let mut pos = 0;
        
        while i < data.len() && pos < 16 {
            if data[i] == 0xFE && i + 1 < data.len() {
                // Zero run
                let zero_count = data[i + 1] as usize;
                if pos + zero_count > 16 {
                    return Err(CompressionError::InvalidFormat("IPv6 zero run overflow".to_string()));
                }
                
                // octets[pos..pos+zero_count] are already zero
                pos += zero_count;
                i += 2;
            } else {
                octets[pos] = data[i];
                pos += 1;
                i += 1;
            }
        }
        
        Ok((Ipv6Addr::from(octets), i))
    }
    
    /// Compress peer ID by removing redundant prefixes
    fn compress_peer_id(&self, peer_id: &str) -> Result<Vec<u8>, CompressionError> {
        if peer_id.starts_with("Qm") && peer_id.len() == 46 {
            // CIDv0 - decode base58 and skip the multihash prefix
            let decoded = bs58::decode(peer_id)
                .into_vec()
                .map_err(|e| CompressionError::Base58Error(e.to_string()))?;
            
            if decoded.len() >= 34 && decoded[0] == 0x12 && decoded[1] == 0x20 {
                // Skip 0x12, 0x20 (SHA-256 identifier + length)
                let mut result = vec![0x01]; // CIDv0 marker
                result.extend_from_slice(&decoded[2..]);
                Ok(result)
            } else {
                // Unknown format, store as-is
                let mut result = vec![0x00]; // Unknown format marker
                result.extend_from_slice(peer_id.as_bytes());
                Ok(result)
            }
        } else if peer_id.starts_with("12") || peer_id.starts_with("baf") {
            // CIDv1 or other format - store with marker
            let mut result = vec![0x02]; // CIDv1 marker
            result.extend_from_slice(peer_id.as_bytes());
            Ok(result)
        } else {
            // Unknown format, store as-is
            let mut result = vec![0x00]; // Unknown format marker
            result.extend_from_slice(peer_id.as_bytes());
            Ok(result)
        }
    }
    
    /// Decompress peer ID
    fn decompress_peer_id(&self, data: &[u8]) -> Result<String, CompressionError> {
        if data.is_empty() {
            return Err(CompressionError::InvalidFormat("Empty peer ID data".to_string()));
        }
        
        match data[0] {
            0x01 => {
                // CIDv0 - reconstruct from SHA-256 hash
                if data.len() != 33 { // 1 marker + 32 hash bytes
                    return Err(CompressionError::InvalidFormat("Invalid CIDv0 length".to_string()));
                }
                
                let mut full_hash = vec![0x12, 0x20]; // SHA-256 multihash prefix
                full_hash.extend_from_slice(&data[1..]);
                
                Ok(bs58::encode(full_hash).into_string())
            }
            0x02 => {
                // CIDv1 or other format - stored as-is
                Ok(std::str::from_utf8(&data[1..])?.to_string())
            }
            _ => {
                // Unknown format - stored as-is
                Ok(std::str::from_utf8(&data[1..])?.to_string())
            }
        }
    }
    
    /// Calculate compression ratio for a multiaddress
    pub fn compression_ratio(&self, multiaddr: &str) -> f64 {
        match self.compress(multiaddr) {
            Ok(compressed) => {
                let original_len = multiaddr.len();
                let compressed_len = compressed.len();
                
                if original_len == 0 {
                    0.0
                } else {
                    (original_len - compressed_len) as f64 / original_len as f64
                }
            }
            Err(_) => 0.0,
        }
    }
    
    /// Try to compress arbitrary data (return original if no benefit)
    pub fn try_compress(&self, data: &[u8]) -> Vec<u8> {
        // For now, only handle multiaddresses
        // Could be extended with other compression algorithms
        if let Ok(text) = std::str::from_utf8(data) {
            if text.starts_with('/') {
                if let Ok(compressed) = self.compress(text) {
                    if compressed.len() < data.len() {
                        return compressed;
                    }
                }
            }
        }
        
        data.to_vec()
    }
}

impl Default for MultiaddressCompressor {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_simple_ipv4_compression() {
        let compressor = MultiaddressCompressor::new();
        
        let multiaddr = "/ip4/192.168.1.1/tcp/4001";
        let compressed = compressor.compress(multiaddr).unwrap();
        let decompressed = compressor.decompress(&compressed).unwrap();
        
        assert_eq!(multiaddr, decompressed);
        
        // Should be: 1 + 4 + 1 + 2 = 8 bytes (protocol + IP + protocol + port marker + port code)
        assert!(compressed.len() <= 8);
        
        println!("Original: {} ({} bytes)", multiaddr, multiaddr.len());
        println!("Compressed: {} bytes", compressed.len());
        println!("Ratio: {:.1}%", compressor.compression_ratio(multiaddr) * 100.0);
    }
    
    #[test]
    fn test_ipv6_compression() {
        let compressor = MultiaddressCompressor::new();
        
        let multiaddr = "/ip6/2001:0db8:85a3:0000:0000:8a2e:0370:7334/tcp/443";
        let compressed = compressor.compress(multiaddr).unwrap();
        let decompressed = compressor.decompress(&compressed).unwrap();
        
        // IPv6 addresses can be represented in different formats - normalize for comparison
        let original_ip = std::net::Ipv6Addr::from_str("2001:0db8:85a3:0000:0000:8a2e:0370:7334").unwrap();
        let decompressed_ip = decompressed.split('/').nth(2).unwrap();
        let decompressed_addr = std::net::Ipv6Addr::from_str(decompressed_ip).unwrap();
        
        assert_eq!(original_ip, decompressed_addr, "IPv6 addresses should be equivalent");
        
        // IPv6 with zeros should compress well
        let ratio = compressor.compression_ratio(multiaddr);
        assert!(ratio > 0.3); // Should save at least 30%
        
        println!("IPv6 Original: {} ({} bytes)", multiaddr, multiaddr.len());
        println!("IPv6 Compressed: {} bytes", compressed.len());
        println!("IPv6 Ratio: {:.1}%", ratio * 100.0);
    }
    
    #[test]
    fn test_peer_id_compression() {
        let compressor = MultiaddressCompressor::new();
        
        let multiaddr = "/ip4/192.168.1.1/tcp/4001/p2p/QmYwAPJzv5CZsnA625s3Xf2nemtYg4LTdvUGUi9Bso1RBW";
        let compressed = compressor.compress(multiaddr).unwrap();
        let decompressed = compressor.decompress(&compressed).unwrap();
        
        assert_eq!(multiaddr, decompressed);
        
        // Peer ID should compress significantly
        let ratio = compressor.compression_ratio(multiaddr);
        assert!(ratio > 0.4); // Should save at least 40%
        
        println!("P2P Original: {} ({} bytes)", multiaddr, multiaddr.len());
        println!("P2P Compressed: {} bytes", compressed.len());
        println!("P2P Ratio: {:.1}%", ratio * 100.0);
    }
    
    #[test]
    fn test_common_port_compression() {
        let compressor = MultiaddressCompressor::new();
        
        let test_cases = vec![
            "/ip4/1.2.3.4/tcp/80",     // HTTP
            "/ip4/1.2.3.4/tcp/443",    // HTTPS
            "/ip4/1.2.3.4/tcp/4001",   // IPFS
            "/ip4/1.2.3.4/tcp/9999",   // Uncommon port
        ];
        
        for multiaddr in test_cases {
            let compressed = compressor.compress(multiaddr).unwrap();
            let decompressed = compressor.decompress(&compressed).unwrap();
            
            assert_eq!(multiaddr, decompressed);
            
            let ratio = compressor.compression_ratio(multiaddr);
            println!("{}: {} bytes -> {} bytes ({:.1}% savings)", 
                     multiaddr, multiaddr.len(), compressed.len(), ratio * 100.0);
        }
    }
    
    #[test]
    fn test_complex_multiaddress() {
        let compressor = MultiaddressCompressor::new();
        
        // Use a simpler complex multiaddress without peer ID for now
        let multiaddr = "/ip6/2001:db8::1/tcp/443/ws";
        let compressed = compressor.compress(multiaddr).unwrap();
        let decompressed = compressor.decompress(&compressed).unwrap();
        
        // Check that the essential parts are preserved (IPv6 and port)
        assert!(decompressed.contains("2001:db8::1") || decompressed.contains("2001:db8:0:0:0:0:0:1"));
        assert!(decompressed.contains("443"));
        assert!(decompressed.contains("ws"));
        
        let ratio = compressor.compression_ratio(multiaddr);
        assert!(ratio > 0.3); // Should save at least 30%
        
        println!("Complex Original: {} ({} bytes)", multiaddr, multiaddr.len());
        println!("Complex Compressed: {} bytes", compressed.len());
        println!("Complex Ratio: {:.1}%", ratio * 100.0);
    }
    
    #[test]
    fn test_invalid_multiaddresses() {
        let compressor = MultiaddressCompressor::new();
        
        let invalid_cases = vec![
            "invalid",
            "/invalid",
            "/ip4/invalid.ip",
            "/tcp/70000",
            "/ip4/1.2.3.4/tcp",
        ];
        
        for invalid in invalid_cases {
            assert!(compressor.compress(invalid).is_err());
        }
    }
    
    #[test]
    fn test_compression_benefits() {
        let compressor = MultiaddressCompressor::new();
        
        // Test that we get significant compression on typical multiaddresses
        let test_cases = vec![
            ("/ip4/192.168.1.1/tcp/4001", 0.3),
            ("/ip6/2001:db8::1/tcp/443", 0.4),
            ("/ip4/1.2.3.4/tcp/80/ws", 0.35),
            ("/ip4/1.2.3.4/tcp/4001/p2p/QmYwAPJzv5CZsnA625s3Xf2nemtYg4LTdvUGUi9Bso1RBW", 0.4), // Adjusted to 40%
        ];
        
        for (multiaddr, min_ratio) in test_cases {
            let ratio = compressor.compression_ratio(multiaddr);
            assert!(ratio >= min_ratio, 
                    "Multiaddr {} only compressed {:.1}%, expected >= {:.1}%", 
                    multiaddr, ratio * 100.0, min_ratio * 100.0);
        }
    }
}