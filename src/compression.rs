//! Advanced compression module for IP addresses and ports
//!
//! This module implements sophisticated compression techniques to reduce
//! IP addresses and ports to fit within the 42-bit limit of four words.

use crate::error::FourWordError;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

/// Maximum bits available in four words (3 × 14 bits)
const MAX_BITS: usize = 42;

/// Address type prefixes (variable length encoding)
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AddressType {
    // IPv4 types (3-bit prefix)
    Ipv4Localhost = 0b000,  // 127.0.0.0/8
    Ipv4Private192 = 0b001, // 192.168.0.0/16
    Ipv4Private10 = 0b010,  // 10.0.0.0/8
    Ipv4Private172 = 0b011, // 172.16.0.0/12
    Ipv4Public = 0b100,     // All other IPv4
    // IPv6 types (4-bit prefix, starts with 0b11)
    Ipv6Localhost = 0b1100,   // ::1
    Ipv6LinkLocal = 0b1101,   // fe80::/10
    Ipv6UniqueLocal = 0b1110, // fc00::/7
    Ipv6Public = 0b1111,      // All other IPv6
}

/// Common port encoding (frequency-based)
#[derive(Debug, Clone)]
pub struct PortCompressor {
    // Top 16 most common ports get 4-bit encoding
    common_ports: Vec<(u16, u8)>,
    // Next 240 ports get 8-bit encoding
    frequent_ports: Vec<(u16, u8)>,
}

impl PortCompressor {
    pub fn new() -> Self {
        Self {
            // Most common ports (4-bit encoding: 0x0-0xF)
            common_ports: vec![
                (80, 0x0),    // HTTP
                (443, 0x1),   // HTTPS
                (22, 0x2),    // SSH
                (21, 0x3),    // FTP
                (25, 0x4),    // SMTP
                (53, 0x5),    // DNS
                (8080, 0x6),  // HTTP alt
                (3306, 0x7),  // MySQL
                (5432, 0x8),  // PostgreSQL
                (6379, 0x9),  // Redis
                (27017, 0xA), // MongoDB
                (8443, 0xB),  // HTTPS alt
                (3000, 0xC),  // Dev server
                (5000, 0xD),  // Dev server
                (8000, 0xE),  // Dev server
                (9000, 0xF),  // Various
            ],
            // Next most frequent ports (8-bit encoding: 0x10-0xFF)
            frequent_ports: vec![
                (23, 0x10),   // Telnet
                (110, 0x11),  // POP3
                (143, 0x12),  // IMAP
                (445, 0x13),  // SMB
                (1433, 0x14), // MSSQL
                (1521, 0x15), // Oracle
                (2049, 0x16), // NFS
                (3389, 0x17), // RDP
                (5900, 0x18), // VNC
                (8081, 0x19), // HTTP alt
                (8082, 0x1A), // HTTP alt
                (8083, 0x1B), // HTTP alt
                (8888, 0x1C), // HTTP alt
                (9090, 0x1D), // Web admin
                (9200, 0x1E), // Elasticsearch
                (11211, 0x1F), // Memcached
                              // ... could add more up to 0xFF
            ],
        }
    }

    pub fn compress(&self, port: Option<u16>) -> (Vec<u8>, usize) {
        match port {
            None => (vec![], 0), // No port = 0 bits
            Some(p) => {
                // Check common ports (4 bits)
                if let Some((_, code)) = self.common_ports.iter().find(|(port, _)| *port == p) {
                    (vec![*code], 4)
                }
                // Check frequent ports (8 bits)
                else if let Some((_, code)) =
                    self.frequent_ports.iter().find(|(port, _)| *port == p)
                {
                    (vec![*code], 8)
                }
                // Full port (16 bits)
                else {
                    (vec![(p >> 8) as u8, (p & 0xFF) as u8], 16)
                }
            }
        }
    }

    pub fn decompress(&self, data: &[u8], bits: usize) -> Result<Option<u16>, FourWordError> {
        match bits {
            0 => Ok(None),
            4 => {
                let code = data[0] & 0x0F;
                self.common_ports
                    .iter()
                    .find(|(_, c)| *c == code)
                    .map(|(port, _)| Some(*port))
                    .ok_or_else(|| {
                        FourWordError::InvalidInput("Invalid common port code".to_string())
                    })
            }
            8 => {
                let code = data[0];
                // First check if it's a common port with full byte
                if code <= 0x0F {
                    self.common_ports
                        .iter()
                        .find(|(_, c)| *c == code)
                        .map(|(port, _)| Some(*port))
                        .ok_or_else(|| FourWordError::InvalidInput("Invalid port code".to_string()))
                } else {
                    self.frequent_ports
                        .iter()
                        .find(|(_, c)| *c == code)
                        .map(|(port, _)| Some(*port))
                        .ok_or_else(|| {
                            FourWordError::InvalidInput("Invalid frequent port code".to_string())
                        })
                }
            }
            16 => {
                if data.len() >= 2 {
                    Ok(Some(((data[0] as u16) << 8) | (data[1] as u16)))
                } else {
                    Err(FourWordError::InvalidInput(
                        "Insufficient data for port".to_string(),
                    ))
                }
            }
            _ => Err(FourWordError::InvalidInput(format!(
                "Invalid port bit count: {}",
                bits
            ))),
        }
    }
}

/// Main compression engine
pub struct IpCompressor {
    port_compressor: PortCompressor,
}

impl IpCompressor {
    pub fn new() -> Self {
        Self {
            port_compressor: PortCompressor::new(),
        }
    }

    /// Compress an IP address with optional port into minimal bits
    pub fn compress(
        &self,
        ip: &IpAddr,
        port: Option<u16>,
    ) -> Result<CompressedAddress, FourWordError> {
        let (addr_type, addr_data, addr_bits) = self.compress_address(ip)?;
        let (port_data, port_bits) = self.port_compressor.compress(port);

        let total_bits = addr_bits + port_bits;
        if total_bits > MAX_BITS {
            return Err(FourWordError::InvalidInput(format!(
                "Compressed size {} bits exceeds maximum {} bits",
                total_bits, MAX_BITS
            )));
        }

        Ok(CompressedAddress {
            addr_type,
            addr_data,
            addr_bits,
            port_data,
            port_bits,
            total_bits,
        })
    }

    /// Compress IP address based on type and pattern
    fn compress_address(
        &self,
        ip: &IpAddr,
    ) -> Result<(AddressType, Vec<u8>, usize), FourWordError> {
        match ip {
            IpAddr::V4(ipv4) => self.compress_ipv4(ipv4),
            IpAddr::V6(ipv6) => self.compress_ipv6(ipv6),
        }
    }

    fn compress_ipv4(
        &self,
        ipv4: &Ipv4Addr,
    ) -> Result<(AddressType, Vec<u8>, usize), FourWordError> {
        let octets = ipv4.octets();

        // Localhost: 127.x.x.x (3-bit type + 8 bits for last octet = 11 bits)
        if octets[0] == 127 {
            Ok((AddressType::Ipv4Localhost, vec![octets[3]], 11))
        }
        // Private 192.168.x.x (3-bit type + 16 bits = 19 bits)
        else if octets[0] == 192 && octets[1] == 168 {
            Ok((AddressType::Ipv4Private192, vec![octets[2], octets[3]], 19))
        }
        // Private 10.x.x.x (3-bit type + 24 bits = 27 bits)
        else if octets[0] == 10 {
            Ok((
                AddressType::Ipv4Private10,
                vec![octets[1], octets[2], octets[3]],
                27,
            ))
        }
        // Private 172.16-31.x.x (3-bit type + 4 bits for range + 16 bits = 23 bits)
        else if octets[0] == 172 && octets[1] >= 16 && octets[1] <= 31 {
            let range_bits = octets[1] - 16; // 0-15 fits in 4 bits
            Ok((
                AddressType::Ipv4Private172,
                vec![range_bits, octets[2], octets[3]],
                23,
            ))
        }
        // Public IPv4 (3-bit type + 32 bits = 35 bits)
        else {
            Ok((AddressType::Ipv4Public, octets.to_vec(), 35))
        }
    }

    fn compress_ipv6(
        &self,
        ipv6: &Ipv6Addr,
    ) -> Result<(AddressType, Vec<u8>, usize), FourWordError> {
        let segments = ipv6.segments();

        // Localhost ::1 (4-bit type only = 4 bits)
        if ipv6.is_loopback() {
            Ok((AddressType::Ipv6Localhost, vec![], 4))
        }
        // Link-local fe80::/10 (4-bit type + interface ID = variable)
        else if segments[0] & 0xFFC0 == 0xFE80 {
            // For link-local, we could store just the interface ID (last 64 bits)
            // But that's still 64 bits, too large for our system
            // Instead, we'll store a hash or truncated version
            let interface_id = ((segments[6] as u32) << 16) | (segments[7] as u32);
            Ok((
                AddressType::Ipv6LinkLocal,
                vec![
                    (interface_id >> 24) as u8,
                    (interface_id >> 16) as u8,
                    (interface_id >> 8) as u8,
                    interface_id as u8,
                ],
                36,
            )) // 4-bit type + 32-bit truncated interface ID
        }
        // Unique local fc00::/7 (4-bit type + subnet ID = variable)
        else if segments[0] & 0xFE00 == 0xFC00 {
            // Store first 48 bits (prefix + global ID + partial subnet)
            Ok((
                AddressType::Ipv6UniqueLocal,
                vec![
                    (segments[0] >> 8) as u8,
                    segments[0] as u8,
                    (segments[1] >> 8) as u8,
                    segments[1] as u8,
                    (segments[2] >> 8) as u8,
                    segments[2] as u8,
                ],
                52,
            )) // 4-bit type + 48 bits
        }
        // Public IPv6 - too large to fit
        else {
            Err(FourWordError::InvalidInput(
                "Public IPv6 addresses cannot be compressed to fit in 42 bits".to_string(),
            ))
        }
    }

    /// Decompress back to IP address and port
    pub fn decompress(
        &self,
        compressed: &CompressedAddress,
    ) -> Result<(IpAddr, Option<u16>), FourWordError> {
        let ip = self.decompress_address(compressed.addr_type, &compressed.addr_data)?;
        let port = self
            .port_compressor
            .decompress(&compressed.port_data, compressed.port_bits)?;
        Ok((ip, port))
    }

    fn decompress_address(
        &self,
        addr_type: AddressType,
        data: &[u8],
    ) -> Result<IpAddr, FourWordError> {
        match addr_type {
            AddressType::Ipv4Localhost => {
                if data.len() >= 1 {
                    Ok(IpAddr::V4(Ipv4Addr::new(127, 0, 0, data[0])))
                } else {
                    Ok(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)))
                }
            }
            AddressType::Ipv4Private192 => {
                if data.len() >= 2 {
                    Ok(IpAddr::V4(Ipv4Addr::new(192, 168, data[0], data[1])))
                } else {
                    Err(FourWordError::InvalidInput(
                        "Insufficient data for 192.168.x.x".to_string(),
                    ))
                }
            }
            AddressType::Ipv4Private10 => {
                if data.len() >= 3 {
                    Ok(IpAddr::V4(Ipv4Addr::new(10, data[0], data[1], data[2])))
                } else {
                    Err(FourWordError::InvalidInput(
                        "Insufficient data for 10.x.x.x".to_string(),
                    ))
                }
            }
            AddressType::Ipv4Private172 => {
                if data.len() >= 3 {
                    let second_octet = 16 + data[0]; // Restore range 16-31
                    Ok(IpAddr::V4(Ipv4Addr::new(
                        172,
                        second_octet,
                        data[1],
                        data[2],
                    )))
                } else {
                    Err(FourWordError::InvalidInput(
                        "Insufficient data for 172.16-31.x.x".to_string(),
                    ))
                }
            }
            AddressType::Ipv4Public => {
                if data.len() >= 4 {
                    Ok(IpAddr::V4(Ipv4Addr::new(
                        data[0], data[1], data[2], data[3],
                    )))
                } else {
                    Err(FourWordError::InvalidInput(
                        "Insufficient data for public IPv4".to_string(),
                    ))
                }
            }
            AddressType::Ipv6Localhost => Ok(IpAddr::V6(Ipv6Addr::LOCALHOST)),
            AddressType::Ipv6LinkLocal => {
                if data.len() >= 4 {
                    // Reconstruct a link-local address with the interface ID
                    let interface_id = ((data[0] as u32) << 24)
                        | ((data[1] as u32) << 16)
                        | ((data[2] as u32) << 8)
                        | (data[3] as u32);
                    Ok(IpAddr::V6(Ipv6Addr::new(
                        0xfe80,
                        0,
                        0,
                        0,
                        0,
                        0,
                        (interface_id >> 16) as u16,
                        (interface_id & 0xFFFF) as u16,
                    )))
                } else {
                    Err(FourWordError::InvalidInput(
                        "Insufficient data for link-local IPv6".to_string(),
                    ))
                }
            }
            AddressType::Ipv6UniqueLocal => {
                if data.len() >= 6 {
                    let seg0 = ((data[0] as u16) << 8) | (data[1] as u16);
                    let seg1 = ((data[2] as u16) << 8) | (data[3] as u16);
                    let seg2 = ((data[4] as u16) << 8) | (data[5] as u16);
                    Ok(IpAddr::V6(Ipv6Addr::new(seg0, seg1, seg2, 0, 0, 0, 0, 0)))
                } else {
                    Err(FourWordError::InvalidInput(
                        "Insufficient data for unique local IPv6".to_string(),
                    ))
                }
            }
            AddressType::Ipv6Public => Err(FourWordError::InvalidInput(
                "Public IPv6 decompression not supported".to_string(),
            )),
        }
    }
}

/// Compressed address representation
#[derive(Debug, Clone)]
pub struct CompressedAddress {
    pub addr_type: AddressType,
    pub addr_data: Vec<u8>,
    pub addr_bits: usize,
    pub port_data: Vec<u8>,
    pub port_bits: usize,
    pub total_bits: usize,
}

impl CompressedAddress {
    /// Pack into a bit stream for encoding
    pub fn pack(&self) -> Vec<u8> {
        let mut bits = BitWriter::new();

        // Write address type prefix
        match self.addr_type {
            // 3-bit prefixes
            AddressType::Ipv4Localhost
            | AddressType::Ipv4Private192
            | AddressType::Ipv4Private10
            | AddressType::Ipv4Private172
            | AddressType::Ipv4Public => {
                bits.write_bits(self.addr_type as u32, 3);
            }
            // 4-bit prefixes
            AddressType::Ipv6Localhost
            | AddressType::Ipv6LinkLocal
            | AddressType::Ipv6UniqueLocal
            | AddressType::Ipv6Public => {
                bits.write_bits(self.addr_type as u32, 4);
            }
        }

        // Write address data with appropriate bit sizes
        match self.addr_type {
            AddressType::Ipv4Localhost => {
                // 8 bits for last octet
                if !self.addr_data.is_empty() {
                    bits.write_bits(self.addr_data[0] as u32, 8);
                }
            }
            AddressType::Ipv4Private192 => {
                // 8 bits + 8 bits for last two octets
                for byte in &self.addr_data {
                    bits.write_bits(*byte as u32, 8);
                }
            }
            AddressType::Ipv4Private10 => {
                // 8 bits + 8 bits + 8 bits for last three octets
                for byte in &self.addr_data {
                    bits.write_bits(*byte as u32, 8);
                }
            }
            AddressType::Ipv4Private172 => {
                // 4 bits for range + 8 bits + 8 bits for last two octets
                if self.addr_data.len() >= 3 {
                    bits.write_bits(self.addr_data[0] as u32, 4); // range_bits (4 bits)
                    bits.write_bits(self.addr_data[1] as u32, 8); // third octet
                    bits.write_bits(self.addr_data[2] as u32, 8); // fourth octet
                }
            }
            AddressType::Ipv4Public => {
                // 8 bits × 4 for all octets
                for byte in &self.addr_data {
                    bits.write_bits(*byte as u32, 8);
                }
            }
            AddressType::Ipv6Localhost => {
                // No additional data
            }
            AddressType::Ipv6LinkLocal => {
                // 8 bits × 4 for interface ID
                for byte in &self.addr_data {
                    bits.write_bits(*byte as u32, 8);
                }
            }
            AddressType::Ipv6UniqueLocal => {
                // 8 bits × 6 for prefix data
                for byte in &self.addr_data {
                    bits.write_bits(*byte as u32, 8);
                }
            }
            AddressType::Ipv6Public => {
                // Should not reach here as it's rejected during compression
            }
        }

        // Write port flag and data
        if self.port_bits > 0 {
            bits.write_bits(1, 1); // Has port
            if self.port_bits == 4 {
                bits.write_bits(0, 1); // Common port marker
                bits.write_bits(self.port_data[0] as u32, 4);
            } else if self.port_bits == 8 {
                bits.write_bits(1, 1); // Frequent port marker
                bits.write_bits(0, 1); // Not full port
                bits.write_bits(self.port_data[0] as u32, 8);
            } else {
                bits.write_bits(1, 1); // Frequent port marker
                bits.write_bits(1, 1); // Full port marker
                bits.write_bits(
                    ((self.port_data[0] as u32) << 8) | (self.port_data[1] as u32),
                    16,
                );
            }
        } else {
            bits.write_bits(0, 1); // No port
        }

        bits.finish()
    }

    /// Unpack from a bit stream
    pub fn unpack(
        data: &[u8],
        compressor: &IpCompressor,
    ) -> Result<(IpAddr, Option<u16>), FourWordError> {
        let mut bits = BitReader::new(data);

        // Read address type prefix
        let first_3_bits = bits.read_bits(3)? as u8;
        let (addr_type, _type_bits) = if first_3_bits < 0b110 {
            // IPv4 type (3-bit prefix)
            (
                match first_3_bits {
                    0b000 => AddressType::Ipv4Localhost,
                    0b001 => AddressType::Ipv4Private192,
                    0b010 => AddressType::Ipv4Private10,
                    0b011 => AddressType::Ipv4Private172,
                    0b100 => AddressType::Ipv4Public,
                    _ => unreachable!(),
                },
                3,
            )
        } else {
            // IPv6 type (4-bit prefix)
            let fourth_bit = bits.read_bits(1)? as u8;
            let ipv6_type = (first_3_bits << 1) | fourth_bit;
            (
                match ipv6_type {
                    0b1100 => AddressType::Ipv6Localhost,
                    0b1101 => AddressType::Ipv6LinkLocal,
                    0b1110 => AddressType::Ipv6UniqueLocal,
                    0b1111 => AddressType::Ipv6Public,
                    _ => unreachable!(),
                },
                4,
            )
        };

        // Read address data based on type
        let addr_data = match addr_type {
            AddressType::Ipv4Localhost => vec![bits.read_bits(8)? as u8],
            AddressType::Ipv4Private192 => vec![bits.read_bits(8)? as u8, bits.read_bits(8)? as u8],
            AddressType::Ipv4Private10 => vec![
                bits.read_bits(8)? as u8,
                bits.read_bits(8)? as u8,
                bits.read_bits(8)? as u8,
            ],
            AddressType::Ipv4Private172 => vec![
                bits.read_bits(4)? as u8,
                bits.read_bits(8)? as u8,
                bits.read_bits(8)? as u8,
            ],
            AddressType::Ipv4Public => vec![
                bits.read_bits(8)? as u8,
                bits.read_bits(8)? as u8,
                bits.read_bits(8)? as u8,
                bits.read_bits(8)? as u8,
            ],
            AddressType::Ipv6Localhost => vec![],
            AddressType::Ipv6LinkLocal => vec![
                bits.read_bits(8)? as u8,
                bits.read_bits(8)? as u8,
                bits.read_bits(8)? as u8,
                bits.read_bits(8)? as u8,
            ],
            AddressType::Ipv6UniqueLocal => vec![
                bits.read_bits(8)? as u8,
                bits.read_bits(8)? as u8,
                bits.read_bits(8)? as u8,
                bits.read_bits(8)? as u8,
                bits.read_bits(8)? as u8,
                bits.read_bits(8)? as u8,
            ],
            AddressType::Ipv6Public => {
                return Err(FourWordError::InvalidInput(
                    "Public IPv6 not supported".to_string(),
                ));
            }
        };

        // Read port if present
        let port = if bits.read_bits(1)? == 1 {
            if bits.read_bits(1)? == 0 {
                // Common port (4 bits)
                let code = bits.read_bits(4)? as u8;
                compressor.port_compressor.decompress(&[code], 4)?
            } else if bits.read_bits(1)? == 0 {
                // Frequent port (8 bits)
                let code = bits.read_bits(8)? as u8;
                compressor.port_compressor.decompress(&[code], 8)?
            } else {
                // Full port (16 bits)
                let port_value = bits.read_bits(16)?;
                Some(port_value as u16)
            }
        } else {
            None
        };

        let ip = compressor.decompress_address(addr_type, &addr_data)?;
        Ok((ip, port))
    }
}

/// Bit-level writer for packing data
struct BitWriter {
    data: Vec<u8>,
    current_byte: u8,
    bits_in_current: usize,
}

impl BitWriter {
    fn new() -> Self {
        Self {
            data: Vec::new(),
            current_byte: 0,
            bits_in_current: 0,
        }
    }

    fn write_bits(&mut self, value: u32, num_bits: usize) {
        let value = value;
        let mut bits_to_write = num_bits;

        while bits_to_write > 0 {
            let bits_available = 8 - self.bits_in_current;
            let bits_this_round = bits_to_write.min(bits_available);

            let mask = (1 << bits_this_round) - 1;
            let bits = ((value >> (bits_to_write - bits_this_round)) & mask) as u8;

            self.current_byte = (self.current_byte << bits_this_round) | bits;
            self.bits_in_current += bits_this_round;

            if self.bits_in_current == 8 {
                self.data.push(self.current_byte);
                self.current_byte = 0;
                self.bits_in_current = 0;
            }

            bits_to_write -= bits_this_round;
        }
    }

    fn finish(mut self) -> Vec<u8> {
        if self.bits_in_current > 0 {
            // Pad the remaining bits to complete the byte
            self.current_byte <<= 8 - self.bits_in_current;
            self.data.push(self.current_byte);
        }
        self.data
    }

    /// Get the total number of bits written
    #[allow(dead_code)]
    fn bit_count(&self) -> usize {
        self.data.len() * 8 + self.bits_in_current
    }
}

/// Bit-level reader for unpacking data
struct BitReader<'a> {
    data: &'a [u8],
    byte_index: usize,
    bit_index: usize,
}

impl<'a> BitReader<'a> {
    fn new(data: &'a [u8]) -> Self {
        Self {
            data,
            byte_index: 0,
            bit_index: 0,
        }
    }

    fn read_bits(&mut self, num_bits: usize) -> Result<u32, FourWordError> {
        let mut result = 0u32;
        let mut bits_read = 0;

        while bits_read < num_bits {
            if self.byte_index >= self.data.len() {
                return Err(FourWordError::InvalidInput(
                    "Insufficient data for bit reading".to_string(),
                ));
            }

            let bits_available = 8 - self.bit_index;
            let bits_to_read = (num_bits - bits_read).min(bits_available);

            let mask = ((1 << bits_to_read) - 1) as u8;
            let bits = (self.data[self.byte_index] >> (bits_available - bits_to_read)) & mask;

            result = (result << bits_to_read) | (bits as u32);
            bits_read += bits_to_read;

            self.bit_index += bits_to_read;
            if self.bit_index == 8 {
                self.byte_index += 1;
                self.bit_index = 0;
            }
        }

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ipv4_compression() {
        let compressor = IpCompressor::new();

        // Test localhost
        let ip = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
        let compressed = compressor.compress(&ip, Some(80)).unwrap();
        assert!(compressed.total_bits <= 15); // 11 bits for IP + 4 bits for common port

        // Test private network
        let ip = IpAddr::V4(Ipv4Addr::new(192, 168, 1, 100));
        let compressed = compressor.compress(&ip, Some(443)).unwrap();
        assert!(compressed.total_bits <= 23); // 19 bits for IP + 4 bits for common port

        // Round trip test
        let (decompressed_ip, decompressed_port) = compressor.decompress(&compressed).unwrap();
        assert_eq!(decompressed_ip, ip);
        assert_eq!(decompressed_port, Some(443));
    }

    #[test]
    fn test_ipv6_compression() {
        let compressor = IpCompressor::new();

        // Test localhost
        let ip = IpAddr::V6(Ipv6Addr::LOCALHOST);
        let compressed = compressor.compress(&ip, Some(22)).unwrap();
        assert!(compressed.total_bits <= 8); // 4 bits for type + 4 bits for common port

        // Round trip test
        let (decompressed_ip, decompressed_port) = compressor.decompress(&compressed).unwrap();
        assert_eq!(decompressed_ip, ip);
        assert_eq!(decompressed_port, Some(22));
    }

    #[test]
    fn test_port_compression() {
        let compressor = PortCompressor::new();

        // Common port
        let (data, bits) = compressor.compress(Some(80));
        assert_eq!(bits, 4);
        assert_eq!(compressor.decompress(&data, bits).unwrap(), Some(80));

        // Frequent port
        let (data, bits) = compressor.compress(Some(3389));
        assert_eq!(bits, 8);
        assert_eq!(compressor.decompress(&data, bits).unwrap(), Some(3389));

        // Arbitrary port
        let (data, bits) = compressor.compress(Some(12345));
        assert_eq!(bits, 16);
        assert_eq!(compressor.decompress(&data, bits).unwrap(), Some(12345));
    }

    #[test]
    fn test_bit_packing() {
        let compressor = IpCompressor::new();

        let ip = IpAddr::V4(Ipv4Addr::new(10, 20, 30, 40));
        let compressed = compressor.compress(&ip, Some(8080)).unwrap();

        // Pack and unpack
        let packed = compressed.pack();
        assert!(packed.len() <= 6); // Should fit in 6 bytes max

        let (unpacked_ip, unpacked_port) = CompressedAddress::unpack(&packed, &compressor).unwrap();
        assert_eq!(unpacked_ip, ip);
        assert_eq!(unpacked_port, Some(8080));
    }
}
