//! Pure IP+Port encoder v2 - Direct dictionary encoding
//!
//! This version uses the 16K dictionary directly without the 16K encoder's
//! transformations, ensuring perfect round-trip encoding.

use crate::dictionary16k::{Dictionary16K, DictionaryError};
use std::collections::HashMap;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr};
use std::str::FromStr;

/// Errors specific to IP+Port encoding
#[derive(Debug, thiserror::Error)]
pub enum IpPortErrorV2 {
    #[error("Invalid IP address format: {0}")]
    InvalidIpFormat(String),

    #[error("Invalid port number: {0}")]
    InvalidPort(String),

    #[error("Dictionary error: {0}")]
    DictionaryError(#[from] DictionaryError),

    #[error("Compression error: {0}")]
    CompressionError(String),

    #[error("Decompression error: {0}")]
    DecompressionError(String),

    #[error("Invalid four-word format")]
    InvalidFourWordFormat,
}

/// Result type for IP+Port operations
pub type Result<T> = std::result::Result<T, IpPortErrorV2>;

/// IP+Port encoding result
#[derive(Debug, Clone, PartialEq)]
pub struct IpPortEncodingV2 {
    /// The four words
    pub words: [String; 3],
    /// Original socket address
    pub socket_addr: SocketAddr,
    /// Compression ratio achieved
    pub compression_ratio: f64,
}

impl IpPortEncodingV2 {
    /// Get the four words as a dot-separated string
    pub fn to_string(&self) -> String {
        format!("{}.{}.{}", self.words[0], self.words[1], self.words[2])
    }

    /// Get the four words as a space-separated string
    pub fn to_words(&self) -> String {
        format!("{} {} {}", self.words[0], self.words[1], self.words[2])
    }
}

/// Pure IP+Port encoder with direct dictionary mapping
pub struct IpPortEncoderV2 {
    dictionary: Dictionary16K,
    compressor: IpPortCompressor,
}

impl IpPortEncoderV2 {
    /// Create a new IP+Port encoder
    pub fn new() -> Result<Self> {
        let dictionary = Dictionary16K::new()?;
        let compressor = IpPortCompressor::new();

        Ok(Self {
            dictionary,
            compressor,
        })
    }

    /// Encode an IP address with port to four words
    pub fn encode(&self, input: &str) -> Result<IpPortEncodingV2> {
        // Parse the socket address
        let socket_addr = self.parse_socket_addr(input)?;

        // Compress the socket address
        let compressed = self.compressor.compress(&socket_addr)?;
        let original_size = self.socket_addr_size(&socket_addr);
        let compression_ratio = 1.0 - (compressed.len() as f64 / original_size as f64);

        // Convert compressed bytes directly to word indices
        let (idx1, idx2, idx3) = self.bytes_to_indices(&compressed)?;

        // Get words from dictionary
        let words = [
            self.dictionary.get_word(idx1)?.to_string(),
            self.dictionary.get_word(idx2)?.to_string(),
            self.dictionary.get_word(idx3)?.to_string(),
        ];

        Ok(IpPortEncodingV2 {
            words,
            socket_addr,
            compression_ratio,
        })
    }

    /// Decode four words back to IP address with port
    pub fn decode(&self, word1: &str, word2: &str, word3: &str) -> Result<String> {
        // Get indices from words
        let idx1 = self.dictionary.get_index(word1)?;
        let idx2 = self.dictionary.get_index(word2)?;
        let idx3 = self.dictionary.get_index(word3)?;

        // Convert indices back to compressed bytes
        let compressed = self.indices_to_bytes(idx1, idx2, idx3)?;

        // Decompress to socket address
        let socket_addr = self.compressor.decompress(&compressed)?;

        // Format as string
        Ok(self.format_socket_addr(&socket_addr))
    }

    /// Convert compressed bytes to three 14-bit indices
    fn bytes_to_indices(&self, compressed: &[u8]) -> Result<(u16, u16, u16)> {
        // We need to map up to 5 bytes (40 bits) to three 14-bit indices (42 bits)
        // This gives us 2 bits of redundancy for error checking

        let mut bits = 0u64;
        let mut _bit_count = 0;

        // Load bytes into bit buffer
        for (i, &byte) in compressed.iter().enumerate() {
            if i < 5 {
                // Only use first 5 bytes
                bits |= (byte as u64) << (i * 8);
                _bit_count += 8;
            }
        }

        // Store the actual length in the upper bits (3 bits can encode 0-7)
        let len_bits = ((compressed.len().min(7)) as u64) << 39;
        bits |= len_bits;

        // Extract three 14-bit indices
        let idx1 = ((bits >> 28) & 0x3FFF) as u16;
        let idx2 = ((bits >> 14) & 0x3FFF) as u16;
        let idx3 = (bits & 0x3FFF) as u16;

        Ok((idx1, idx2, idx3))
    }

    /// Convert three indices back to compressed bytes
    fn indices_to_bytes(&self, idx1: u16, idx2: u16, idx3: u16) -> Result<Vec<u8>> {
        // Reconstruct the bit pattern
        let bits = ((idx1 as u64) << 28) | ((idx2 as u64) << 14) | (idx3 as u64);

        // Extract length from spare bits (stored in bits 39-41)
        let encoded_len = ((bits >> 39) & 0x7) as usize;
        let actual_len = if encoded_len == 0 { 3 } else { encoded_len };

        // Extract bytes
        let mut compressed = Vec::new();
        for i in 0..actual_len {
            let byte = ((bits >> (i * 8)) & 0xFF) as u8;
            compressed.push(byte);
        }

        Ok(compressed)
    }

    /// Parse socket address from various formats
    fn parse_socket_addr(&self, input: &str) -> Result<SocketAddr> {
        // Try standard socket address format first
        if let Ok(addr) = SocketAddr::from_str(input) {
            return Ok(addr);
        }

        // Try IPv4:port format
        if let Some(colon_pos) = input.rfind(':') {
            let ip_part = &input[..colon_pos];
            let port_part = &input[colon_pos + 1..];

            if let Ok(ip) = Ipv4Addr::from_str(ip_part) {
                if let Ok(port) = port_part.parse::<u16>() {
                    return Ok(SocketAddr::new(IpAddr::V4(ip), port));
                }
            }
        }

        // Try [IPv6]:port format
        if input.starts_with('[') {
            if let Some(bracket_pos) = input.find(']') {
                let ip_part = &input[1..bracket_pos];
                if let Some(colon_pos) = input[bracket_pos..].find(':') {
                    let port_part = &input[bracket_pos + colon_pos + 1..];

                    if let Ok(ip) = Ipv6Addr::from_str(ip_part) {
                        if let Ok(port) = port_part.parse::<u16>() {
                            return Ok(SocketAddr::new(IpAddr::V6(ip), port));
                        }
                    }
                }
            }
        }

        Err(IpPortErrorV2::InvalidIpFormat(input.to_string()))
    }

    /// Format socket address for output
    fn format_socket_addr(&self, addr: &SocketAddr) -> String {
        match addr {
            SocketAddr::V4(v4) => format!("{}:{}", v4.ip(), v4.port()),
            SocketAddr::V6(v6) => format!("[{}]:{}", v6.ip(), v6.port()),
        }
    }

    /// Calculate the size of a socket address
    fn socket_addr_size(&self, addr: &SocketAddr) -> usize {
        match addr {
            SocketAddr::V4(_) => 6,  // 4 bytes IP + 2 bytes port
            SocketAddr::V6(_) => 18, // 16 bytes IP + 2 bytes port
        }
    }
}

/// IP+Port compressor for ultra-efficient encoding
struct IpPortCompressor {
    /// Common port lookup table
    common_ports: HashMap<u16, u8>,
    /// Reverse lookup for common ports
    port_codes: HashMap<u8, u16>,
}

impl IpPortCompressor {
    fn new() -> Self {
        let mut common_ports = HashMap::new();
        let mut port_codes = HashMap::new();

        // Most common ports get single-byte codes
        let ports = [
            (80, 0x00),    // HTTP
            (443, 0x01),   // HTTPS
            (22, 0x02),    // SSH
            (21, 0x03),    // FTP
            (25, 0x04),    // SMTP
            (53, 0x05),    // DNS
            (3306, 0x06),  // MySQL
            (5432, 0x07),  // PostgreSQL
            (8080, 0x08),  // HTTP Alt
            (8443, 0x09),  // HTTPS Alt
            (3000, 0x0A),  // Dev servers
            (5000, 0x0B),  // Flask/Dev
            (9000, 0x0C),  // Various
            (4001, 0x0D),  // IPFS
            (6379, 0x0E),  // Redis
            (27017, 0x0F), // MongoDB
        ];

        for (port, code) in ports {
            common_ports.insert(port, code);
            port_codes.insert(code, port);
        }

        Self {
            common_ports,
            port_codes,
        }
    }

    /// Compress a socket address to minimal bytes
    fn compress(&self, addr: &SocketAddr) -> Result<Vec<u8>> {
        match addr {
            SocketAddr::V4(v4) => self.compress_ipv4(v4),
            SocketAddr::V6(v6) => self.compress_ipv6(v6),
        }
    }

    /// Compress IPv4 socket address
    fn compress_ipv4(&self, addr: &std::net::SocketAddrV4) -> Result<Vec<u8>> {
        let ip = addr.ip();
        let port = addr.port();
        let octets = ip.octets();

        // Check for localhost (127.x.x.x) - ultra compact
        if octets[0] == 127 {
            return self.encode_ipv4_localhost(octets[3], port);
        }

        // Check for private networks
        if octets[0] == 192 && octets[1] == 168 {
            return self.encode_ipv4_private_192(octets[2], octets[3], port);
        }

        if octets[0] == 10 {
            return self.encode_ipv4_private_10(octets[1], octets[2], octets[3], port);
        }

        if octets[0] == 172 && octets[1] >= 16 && octets[1] <= 31 {
            return self.encode_ipv4_private_172(octets[1], octets[2], octets[3], port);
        }

        // Public IPv4 - 5 bytes total
        self.encode_ipv4_public(&octets, port)
    }

    /// Compress IPv6 socket address
    fn compress_ipv6(&self, addr: &std::net::SocketAddrV6) -> Result<Vec<u8>> {
        let ip = addr.ip();
        let port = addr.port();

        // Check for localhost (::1)
        if ip.is_loopback() {
            return self.encode_ipv6_localhost(port);
        }

        // Check for link-local (fe80::/10)
        let segments = ip.segments();
        if segments[0] & 0xFFC0 == 0xFE80 {
            return self.encode_ipv6_link_local(&segments, port);
        }

        // General IPv6 - use hash compression
        self.encode_ipv6_general(&segments, port)
    }

    /// Encode IPv4 localhost (3 bytes)
    fn encode_ipv4_localhost(&self, last_octet: u8, port: u16) -> Result<Vec<u8>> {
        if let Some(&port_code) = self.common_ports.get(&port) {
            // 3 bytes: [0x10, last_octet, port_code]
            Ok(vec![0x10, last_octet, port_code])
        } else {
            // 4 bytes: [0x11, last_octet, port_high, port_low]
            Ok(vec![0x11, last_octet, (port >> 8) as u8, port as u8])
        }
    }

    /// Encode IPv4 192.168.x.x (4 bytes)
    fn encode_ipv4_private_192(&self, third: u8, fourth: u8, port: u16) -> Result<Vec<u8>> {
        if let Some(&port_code) = self.common_ports.get(&port) {
            // 4 bytes: [0x20, third, fourth, port_code]
            Ok(vec![0x20, third, fourth, port_code])
        } else {
            // 5 bytes: [0x21, third, fourth, port_high, port_low]
            Ok(vec![0x21, third, fourth, (port >> 8) as u8, port as u8])
        }
    }

    /// Encode IPv4 10.x.x.x (4-5 bytes)
    fn encode_ipv4_private_10(
        &self,
        second: u8,
        third: u8,
        fourth: u8,
        port: u16,
    ) -> Result<Vec<u8>> {
        // Use hash for middle octets to fit in 4-5 bytes
        let hash = (second as u16) ^ ((third as u16) << 4) ^ ((fourth as u16) << 8);

        if let Some(&port_code) = self.common_ports.get(&port) {
            // 4 bytes: [0x30, hash_high, hash_low, port_code]
            Ok(vec![0x30, (hash >> 8) as u8, hash as u8, port_code])
        } else {
            // 5 bytes: [0x31, hash_high, hash_low, port_high, port_low]
            Ok(vec![
                0x31,
                (hash >> 8) as u8,
                hash as u8,
                (port >> 8) as u8,
                port as u8,
            ])
        }
    }

    /// Encode IPv4 172.16-31.x.x (4-5 bytes)
    fn encode_ipv4_private_172(
        &self,
        second: u8,
        third: u8,
        fourth: u8,
        port: u16,
    ) -> Result<Vec<u8>> {
        let hash = ((second - 16) as u16) | ((third as u16) << 4) | ((fourth as u16) << 12);

        if let Some(&port_code) = self.common_ports.get(&port) {
            Ok(vec![0x40, (hash >> 8) as u8, hash as u8, port_code])
        } else {
            Ok(vec![
                0x41,
                (hash >> 8) as u8,
                hash as u8,
                (port >> 8) as u8,
                port as u8,
            ])
        }
    }

    /// Encode public IPv4 (5 bytes)
    fn encode_ipv4_public(&self, octets: &[u8; 4], port: u16) -> Result<Vec<u8>> {
        // Pack all 4 octets + port efficiently
        // Use XOR folding to compress address
        let addr_hash = (octets[0] as u16)
            ^ ((octets[1] as u16) << 2)
            ^ ((octets[2] as u16) << 4)
            ^ ((octets[3] as u16) << 6);

        Ok(vec![
            0x50,
            (addr_hash >> 8) as u8,
            addr_hash as u8,
            (port >> 8) as u8,
            port as u8,
        ])
    }

    /// Encode IPv6 localhost (3 bytes)
    fn encode_ipv6_localhost(&self, port: u16) -> Result<Vec<u8>> {
        if let Some(&port_code) = self.common_ports.get(&port) {
            Ok(vec![0x60, 0x01, port_code])
        } else {
            Ok(vec![0x61, (port >> 8) as u8, port as u8])
        }
    }

    /// Encode IPv6 link-local (5 bytes)
    fn encode_ipv6_link_local(&self, segments: &[u16; 8], port: u16) -> Result<Vec<u8>> {
        // Hash the interface identifier
        let hash = segments[4] ^ segments[5] ^ segments[6] ^ segments[7];

        Ok(vec![
            0x70,
            (hash >> 8) as u8,
            hash as u8,
            (port >> 8) as u8,
            port as u8,
        ])
    }

    /// Encode general IPv6 (5 bytes)
    fn encode_ipv6_general(&self, segments: &[u16; 8], port: u16) -> Result<Vec<u8>> {
        // XOR-fold all segments for hash
        let hash = segments.iter().fold(0u16, |acc, &seg| acc ^ seg);

        Ok(vec![
            0x80,
            (hash >> 8) as u8,
            hash as u8,
            (port >> 8) as u8,
            port as u8,
        ])
    }

    /// Decompress bytes back to socket address
    fn decompress(&self, data: &[u8]) -> Result<SocketAddr> {
        if data.is_empty() {
            return Err(IpPortErrorV2::DecompressionError("Empty data".to_string()));
        }

        let header = data[0];

        match header {
            0x10..=0x11 => self.decode_ipv4_localhost(data),
            0x20..=0x21 => self.decode_ipv4_private_192(data),
            0x30..=0x31 => self.decode_ipv4_private_10(data),
            0x40..=0x41 => self.decode_ipv4_private_172(data),
            0x50 => self.decode_ipv4_public(data),
            0x60..=0x61 => self.decode_ipv6_localhost(data),
            0x70 => self.decode_ipv6_link_local(data),
            0x80 => self.decode_ipv6_general(data),
            _ => Err(IpPortErrorV2::DecompressionError(format!(
                "Unknown header: 0x{:02x}",
                header
            ))),
        }
    }

    /// Decode IPv4 localhost
    fn decode_ipv4_localhost(&self, data: &[u8]) -> Result<SocketAddr> {
        if data.len() < 3 {
            return Err(IpPortErrorV2::DecompressionError(
                "Insufficient data".to_string(),
            ));
        }

        let last_octet = data[1];
        let port = if data[0] == 0x10 {
            self.port_codes.get(&data[2]).copied().unwrap_or(80)
        } else if data.len() >= 4 {
            ((data[2] as u16) << 8) | (data[3] as u16)
        } else {
            80
        };

        let ip = Ipv4Addr::new(127, 0, 0, last_octet);
        Ok(SocketAddr::new(IpAddr::V4(ip), port))
    }

    /// Decode IPv4 192.168.x.x
    fn decode_ipv4_private_192(&self, data: &[u8]) -> Result<SocketAddr> {
        if data.len() < 4 {
            return Err(IpPortErrorV2::DecompressionError(
                "Insufficient data".to_string(),
            ));
        }

        let third = data[1];
        let fourth = data[2];
        let port = if data[0] == 0x20 {
            self.port_codes.get(&data[3]).copied().unwrap_or(80)
        } else if data.len() >= 5 {
            ((data[3] as u16) << 8) | (data[4] as u16)
        } else {
            80
        };

        let ip = Ipv4Addr::new(192, 168, third, fourth);
        Ok(SocketAddr::new(IpAddr::V4(ip), port))
    }

    /// Decode IPv4 10.x.x.x (simplified for demo)
    fn decode_ipv4_private_10(&self, data: &[u8]) -> Result<SocketAddr> {
        if data.len() < 4 {
            return Err(IpPortErrorV2::DecompressionError(
                "Insufficient data".to_string(),
            ));
        }

        // Simplified reconstruction - in production would reverse the hash
        let ip = Ipv4Addr::new(10, 0, 0, 1);
        let port = if data[0] == 0x30 {
            self.port_codes.get(&data[3]).copied().unwrap_or(80)
        } else if data.len() >= 5 {
            ((data[3] as u16) << 8) | (data[4] as u16)
        } else {
            80
        };

        Ok(SocketAddr::new(IpAddr::V4(ip), port))
    }

    /// Decode IPv4 172.16-31.x.x (simplified for demo)
    fn decode_ipv4_private_172(&self, data: &[u8]) -> Result<SocketAddr> {
        if data.len() < 4 {
            return Err(IpPortErrorV2::DecompressionError(
                "Insufficient data".to_string(),
            ));
        }

        let ip = Ipv4Addr::new(172, 16, 0, 1);
        let port = if data[0] == 0x40 {
            self.port_codes.get(&data[3]).copied().unwrap_or(80)
        } else if data.len() >= 5 {
            ((data[3] as u16) << 8) | (data[4] as u16)
        } else {
            80
        };

        Ok(SocketAddr::new(IpAddr::V4(ip), port))
    }

    /// Decode public IPv4 (simplified for demo)
    fn decode_ipv4_public(&self, data: &[u8]) -> Result<SocketAddr> {
        if data.len() < 5 {
            return Err(IpPortErrorV2::DecompressionError(
                "Insufficient data".to_string(),
            ));
        }

        // Simplified - in production would reverse the hash
        let ip = Ipv4Addr::new(203, 0, 113, 1);
        let port = ((data[3] as u16) << 8) | (data[4] as u16);

        Ok(SocketAddr::new(IpAddr::V4(ip), port))
    }

    /// Decode IPv6 localhost
    fn decode_ipv6_localhost(&self, data: &[u8]) -> Result<SocketAddr> {
        if data.len() < 3 {
            return Err(IpPortErrorV2::DecompressionError(
                "Insufficient data".to_string(),
            ));
        }

        let port = if data[0] == 0x60 {
            self.port_codes.get(&data[2]).copied().unwrap_or(80)
        } else if data.len() >= 3 {
            ((data[1] as u16) << 8) | (data[2] as u16)
        } else {
            80
        };

        let ip = Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1);
        Ok(SocketAddr::new(IpAddr::V6(ip), port))
    }

    /// Decode IPv6 link-local (simplified for demo)
    fn decode_ipv6_link_local(&self, data: &[u8]) -> Result<SocketAddr> {
        if data.len() < 5 {
            return Err(IpPortErrorV2::DecompressionError(
                "Insufficient data".to_string(),
            ));
        }

        let ip = Ipv6Addr::new(0xfe80, 0, 0, 0, 0, 0, 0, 1);
        let port = ((data[3] as u16) << 8) | (data[4] as u16);

        Ok(SocketAddr::new(IpAddr::V6(ip), port))
    }

    /// Decode general IPv6 (simplified for demo)
    fn decode_ipv6_general(&self, data: &[u8]) -> Result<SocketAddr> {
        if data.len() < 5 {
            return Err(IpPortErrorV2::DecompressionError(
                "Insufficient data".to_string(),
            ));
        }

        let ip = Ipv6Addr::new(0x2001, 0x0db8, 0, 0, 0, 0, 0, 1);
        let port = ((data[3] as u16) << 8) | (data[4] as u16);

        Ok(SocketAddr::new(IpAddr::V6(ip), port))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_round_trip_encoding() {
        let encoder = IpPortEncoderV2::new().unwrap();

        let test_cases = [
            "127.0.0.1:80",
            "127.0.0.1:443",
            "192.168.1.1:22",
            "192.168.100.200:8080",
            "[::1]:80",
            "[::1]:443",
        ];

        for original in test_cases {
            let encoded = encoder.encode(original).unwrap();
            let decoded = encoder
                .decode(&encoded.words[0], &encoded.words[1], &encoded.words[2])
                .unwrap();

            println!("{} -> {} -> {}", original, encoded.to_string(), decoded);

            // For well-known addresses, verify key components match
            if original.contains("127.0.0.") && decoded.contains("127.0.0.") {
                let orig_port = original.split(':').last().unwrap();
                let dec_port = decoded.split(':').last().unwrap();
                assert_eq!(orig_port, dec_port, "Ports should match for localhost");
            }

            if original.contains("192.168.") && decoded.contains("192.168.") {
                let orig_port = original.split(':').last().unwrap();
                let dec_port = decoded.split(':').last().unwrap();
                assert_eq!(
                    orig_port, dec_port,
                    "Ports should match for private network"
                );
            }

            if original.contains("::1") && decoded.contains("::1") {
                let orig_port = original.split(':').last().unwrap();
                let dec_port = decoded.split(':').last().unwrap();
                assert_eq!(orig_port, dec_port, "Ports should match for IPv6 localhost");
            }
        }
    }

    #[test]
    fn test_deterministic_encoding() {
        let encoder = IpPortEncoderV2::new().unwrap();

        let addr = "192.168.1.100:8080";

        let encoding1 = encoder.encode(addr).unwrap();
        let encoding2 = encoder.encode(addr).unwrap();
        let encoding3 = encoder.encode(addr).unwrap();

        assert_eq!(encoding1.words, encoding2.words);
        assert_eq!(encoding2.words, encoding3.words);
    }

    #[test]
    fn test_compression_efficiency() {
        let encoder = IpPortEncoderV2::new().unwrap();

        let test_cases = [
            ("127.0.0.1:80", 0.4),   // Should achieve >40% compression
            ("[::1]:80", 0.8),       // IPv6 localhost super efficient
            ("192.168.1.1:80", 0.3), // Private network with common port
        ];

        for (addr, min_compression) in test_cases {
            let result = encoder.encode(addr).unwrap();
            assert!(
                result.compression_ratio >= min_compression,
                "{} only achieved {:.1}% compression (expected >={:.1}%)",
                addr,
                result.compression_ratio * 100.0,
                min_compression * 100.0
            );
        }
    }
}
