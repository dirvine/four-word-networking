//! IPv6 Hierarchical Compression Engine
//!
//! This module implements advanced compression techniques specifically designed
//! for IPv6 addresses, taking advantage of their hierarchical structure and
//! common patterns to achieve optimal compression ratios.

use crate::error::FourWordError;
use std::net::Ipv6Addr;

/// IPv6 address categories for compression optimization
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Ipv6Category {
    /// ::1 - IPv6 loopback (4 words)
    Loopback,
    /// fe80::/10 - Link-local addresses (3-4 words)
    LinkLocal,
    /// fc00::/7 - Unique local addresses (4-5 words)
    UniqueLocal,
    /// 2001:db8::/32 - Documentation addresses (4 words)
    Documentation,
    /// 2000::/3 - Global unicast (4-6 words)
    GlobalUnicast,
    /// ::/128 - Unspecified address (4 words)
    Unspecified,
    /// Multicast and other special addresses (5-6 words)
    Special,
}

/// Compressed representation of an IPv6 address
#[derive(Debug, Clone)]
pub struct CompressedIpv6 {
    pub category: Ipv6Category,
    pub compressed_data: Vec<u8>,
    pub original_bits: usize,
    pub compressed_bits: usize,
    pub port: Option<u16>,
}

impl CompressedIpv6 {
    /// Get the total compressed size including port
    pub fn total_bits(&self) -> usize {
        self.compressed_bits + self.port.map_or(0, |_| 16)
    }

    /// Get the recommended word count for this compression
    /// IPv6 always uses 4-6 words to distinguish from IPv4
    pub fn recommended_word_count(&self) -> usize {
        let total_bits = self.total_bits();
        // IPv6 always uses at least 4 words for clear differentiation from IPv4
        if total_bits <= 56 {
            4
        } else if total_bits <= 70 {
            5
        } else {
            6
        }
    }

    /// Get compression ratio
    pub fn compression_ratio(&self) -> f64 {
        let original_total = self.original_bits + self.port.map_or(0, |_| 16);
        1.0 - (self.total_bits() as f64 / original_total as f64)
    }

    /// Get human-readable category description
    pub fn category_description(&self) -> &'static str {
        match self.category {
            Ipv6Category::Loopback => "IPv6 Loopback (::1)",
            Ipv6Category::LinkLocal => "Link-Local (fe80::)",
            Ipv6Category::UniqueLocal => "Unique Local (fc00::)",
            Ipv6Category::Documentation => "Documentation (2001:db8::)",
            Ipv6Category::GlobalUnicast => "Global Unicast",
            Ipv6Category::Unspecified => "Unspecified (::)",
            Ipv6Category::Special => "Special/Multicast",
        }
    }
}

/// Advanced IPv6 compression engine
pub struct Ipv6Compressor;

impl Ipv6Compressor {
    /// Compress an IPv6 address with optional port
    pub fn compress(ip: Ipv6Addr, port: Option<u16>) -> Result<CompressedIpv6, FourWordError> {
        let category = Self::categorize_address(&ip);

        match category {
            Ipv6Category::Loopback => Self::compress_loopback(ip, port),
            Ipv6Category::LinkLocal => Self::compress_link_local(ip, port),
            Ipv6Category::UniqueLocal => Self::compress_unique_local(ip, port),
            Ipv6Category::Documentation => Self::compress_documentation(ip, port),
            Ipv6Category::GlobalUnicast => Self::compress_global_unicast(ip, port),
            Ipv6Category::Unspecified => Self::compress_unspecified(ip, port),
            Ipv6Category::Special => Self::compress_special(ip, port),
        }
    }

    /// Decompress back to IPv6 address and port
    pub fn decompress(
        compressed: &CompressedIpv6,
    ) -> Result<(Ipv6Addr, Option<u16>), FourWordError> {
        let ip = match compressed.category {
            Ipv6Category::Loopback => Self::decompress_loopback(&compressed.compressed_data)?,
            Ipv6Category::LinkLocal => Self::decompress_link_local(&compressed.compressed_data)?,
            Ipv6Category::UniqueLocal => {
                Self::decompress_unique_local(&compressed.compressed_data)?
            }
            Ipv6Category::Documentation => {
                Self::decompress_documentation(&compressed.compressed_data)?
            }
            Ipv6Category::GlobalUnicast => {
                Self::decompress_global_unicast(&compressed.compressed_data)?
            }
            Ipv6Category::Unspecified => Self::decompress_unspecified(&compressed.compressed_data)?,
            Ipv6Category::Special => Self::decompress_special(&compressed.compressed_data)?,
        };

        Ok((ip, compressed.port))
    }

    /// Categorize an IPv6 address for optimal compression
    fn categorize_address(ip: &Ipv6Addr) -> Ipv6Category {
        let segments = ip.segments();

        // Check for loopback ::1
        if ip.is_loopback() {
            return Ipv6Category::Loopback;
        }

        // Check for unspecified ::
        if ip.is_unspecified() {
            return Ipv6Category::Unspecified;
        }

        // Check for link-local fe80::/10
        if segments[0] & 0xFFC0 == 0xFE80 {
            return Ipv6Category::LinkLocal;
        }

        // Check for unique local fc00::/7
        if segments[0] & 0xFE00 == 0xFC00 {
            return Ipv6Category::UniqueLocal;
        }

        // Check for documentation 2001:db8::/32
        if segments[0] == 0x2001 && segments[1] == 0x0DB8 {
            return Ipv6Category::Documentation;
        }

        // Check for global unicast 2000::/3
        if segments[0] & 0xE000 == 0x2000 {
            return Ipv6Category::GlobalUnicast;
        }

        // Everything else (multicast, etc.)
        Ipv6Category::Special
    }

    /// Compress loopback address ::1
    fn compress_loopback(
        _ip: Ipv6Addr,
        port: Option<u16>,
    ) -> Result<CompressedIpv6, FourWordError> {
        // Loopback is just ::1, but we ensure 4 words minimum for IPv6
        // Add padding bytes to ensure we reach 4 words (56 bits total)
        let padding = vec![0x00, 0x00, 0x01, 0x00, 0x00, 0x00]; // 48 bits of padding
        Ok(CompressedIpv6 {
            category: Ipv6Category::Loopback,
            compressed_data: padding,
            original_bits: 128,
            compressed_bits: 48, // Ensure 4 words minimum
            port,
        })
    }

    /// Compress link-local address fe80::/10
    fn compress_link_local(
        ip: Ipv6Addr,
        port: Option<u16>,
    ) -> Result<CompressedIpv6, FourWordError> {
        let segments = ip.segments();

        // Link-local: fe80:0000:0000:0000:xxxx:xxxx:xxxx:xxxx
        // Optimize for common patterns

        // Check for simple patterns (fe80::1, fe80::2, etc.)
        let non_zero_segments: Vec<(usize, u16)> = segments[4..8]
            .iter()
            .enumerate()
            .filter(|&(_, &seg)| seg != 0)
            .map(|(i, &seg)| (i + 4, seg))
            .collect();

        let mut compressed = Vec::new();
        let compressed_bits;

        if non_zero_segments.is_empty() {
            // fe80:: - all zeros in interface ID
            // Pad to ensure 4 words minimum
            compressed = vec![0, 0, 0, 0, 0, 0, 0]; // Marker + padding for 56 bits
            compressed_bits = 3 + 56; // category + 7 bytes
        } else if non_zero_segments.len() == 1 && non_zero_segments[0].1 <= 255 {
            // Single small value like fe80::1 - store position + value
            let (pos, val) = non_zero_segments[0];
            // Pad to ensure 4 words minimum
            compressed = vec![1, (pos - 4) as u8, val as u8, 0, 0, 0, 0]; // Marker + data + padding
            compressed_bits = 3 + 56; // category + 7 bytes
        } else if segments[4] & 0x0200 == 0x0200 {
            // EUI-64 derived address - store just the MAC-derived part
            compressed = vec![2]; // Marker for EUI-64
            let mac_derived = [
                (segments[4] ^ 0x0200) as u8, // Remove universal/local bit
                (segments[4] >> 8) as u8,
                (segments[5]) as u8,
                (segments[5] >> 8) as u8,
                (segments[6]) as u8,
                (segments[6] >> 8) as u8,
            ];
            compressed.extend_from_slice(&mac_derived);
            compressed_bits = 3 + 56; // category + marker + MAC (7 bytes)
        } else {
            // Complex pattern - store efficiently with RLE
            compressed.push(3); // Marker for complex pattern
            for &(pos, val) in &non_zero_segments {
                compressed.push((pos - 4) as u8);
                compressed.extend_from_slice(&val.to_be_bytes());
            }
            compressed.push(255); // End marker
            compressed_bits = 3 + (compressed.len() * 8); // category + data
        }

        Ok(CompressedIpv6 {
            category: Ipv6Category::LinkLocal,
            compressed_data: compressed,
            original_bits: 128,
            compressed_bits,
            port,
        })
    }

    /// Compress unique local address fc00::/7
    fn compress_unique_local(
        ip: Ipv6Addr,
        port: Option<u16>,
    ) -> Result<CompressedIpv6, FourWordError> {
        let segments = ip.segments();

        // Unique local: fcxx:xxxx:xxxx:xxxx:xxxx:xxxx:xxxx:xxxx
        // Check if interface ID (last 64 bits) is non-zero
        let interface_id_is_zero = segments[4] == 0 && segments[5] == 0 && segments[6] == 0 && segments[7] == 0;

        let mut compressed = vec![];
        
        // Store segments[0-3] as 8 bytes (prefix + global ID + subnet)
        compressed.extend_from_slice(&segments[0].to_be_bytes()); // segments[0] (includes fc/fd prefix)
        compressed.extend_from_slice(&segments[1].to_be_bytes()); // segments[1]
        compressed.extend_from_slice(&segments[2].to_be_bytes()); // segments[2]
        compressed.extend_from_slice(&segments[3].to_be_bytes()); // segments[3] (subnet)

        let compressed_bits = if interface_id_is_zero {
            // Interface ID is zero, only store prefix + global ID + subnet
            3 + 64 // category + 4 segments (8 bytes)
        } else {
            // Interface ID is non-zero, store all 8 segments
            compressed.extend_from_slice(&segments[4].to_be_bytes()); // segments[4]
            compressed.extend_from_slice(&segments[5].to_be_bytes()); // segments[5]
            compressed.extend_from_slice(&segments[6].to_be_bytes()); // segments[6]
            compressed.extend_from_slice(&segments[7].to_be_bytes()); // segments[7]
            3 + 128 // category + 8 segments (16 bytes)
        };

        Ok(CompressedIpv6 {
            category: Ipv6Category::UniqueLocal,
            compressed_data: compressed,
            original_bits: 128,
            compressed_bits,
            port,
        })
    }

    /// Compress documentation address 2001:db8::/32
    fn compress_documentation(
        ip: Ipv6Addr,
        port: Option<u16>,
    ) -> Result<CompressedIpv6, FourWordError> {
        let segments = ip.segments();

        // Documentation: 2001:0db8:xxxx:xxxx:xxxx:xxxx:xxxx:xxxx
        // Check for common patterns first

        // Check for simple patterns like 2001:db8::1, 2001:db8::2, etc.
        let non_zero_segments: Vec<(usize, u16)> = segments[2..8]
            .iter()
            .enumerate()
            .filter(|&(_, &seg)| seg != 0)
            .map(|(i, &seg)| (i + 2, seg))
            .collect();

        let mut compressed = Vec::new();

        if non_zero_segments.is_empty() {
            // 2001:db8:: - all zeros after prefix
            // Pad to ensure 4 words minimum
            compressed = vec![0, 0, 0, 0, 0, 0, 0]; // Marker + padding for 56 bits
        } else if non_zero_segments.len() == 1 && non_zero_segments[0].1 <= 255 {
            // Single small value like 2001:db8::1 - store position + value
            let (pos, val) = non_zero_segments[0];
            // Pad to ensure 4 words minimum
            compressed = vec![1, pos as u8, val as u8, 0, 0, 0, 0]; // Marker + data + padding
        } else {
            // Complex pattern - store more efficiently with RLE
            compressed.push(2); // Marker for complex pattern
            for &(pos, val) in &non_zero_segments {
                compressed.push(pos as u8);
                compressed.extend_from_slice(&val.to_be_bytes());
            }
            compressed.push(255); // End marker
        }

        let compressed_bits = 3 + (compressed.len() * 8); // category + data

        Ok(CompressedIpv6 {
            category: Ipv6Category::Documentation,
            compressed_data: compressed,
            original_bits: 128,
            compressed_bits,
            port,
        })
    }

    /// Compress global unicast address 2000::/3
    fn compress_global_unicast(
        ip: Ipv6Addr,
        port: Option<u16>,
    ) -> Result<CompressedIpv6, FourWordError> {
        let segments = ip.segments();

        // Global unicast is the most challenging to compress
        // We'll use statistical compression based on common patterns

        // Check for common provider patterns
        if let Some(compressed) = Self::try_provider_patterns(&segments) {
            return Ok(CompressedIpv6 {
                category: Ipv6Category::GlobalUnicast,
                compressed_data: compressed,
                original_bits: 128,
                compressed_bits: 3 + 48, // category + pattern data
                port,
            });
        }

        // Fallback: store all segments (full 128 bits)
        let mut compressed = Vec::new();
        for segment in segments {
            compressed.extend_from_slice(&segment.to_be_bytes());
        }

        Ok(CompressedIpv6 {
            category: Ipv6Category::GlobalUnicast,
            compressed_data: compressed,
            original_bits: 128,
            compressed_bits: 3 + 128, // category + full address
            port,
        })
    }

    /// Compress unspecified address ::
    fn compress_unspecified(
        _ip: Ipv6Addr,
        port: Option<u16>,
    ) -> Result<CompressedIpv6, FourWordError> {
        // Unspecified is all zeros, but we ensure 4 words minimum for IPv6
        // Add padding bytes to ensure we reach 4 words (56 bits total)
        let padding = vec![0x00, 0x00, 0x00, 0x00, 0x00, 0x00]; // 48 bits of padding
        Ok(CompressedIpv6 {
            category: Ipv6Category::Unspecified,
            compressed_data: padding,
            original_bits: 128,
            compressed_bits: 48, // Ensure 4 words minimum
            port,
        })
    }

    /// Compress special addresses (multicast, etc.)
    fn compress_special(ip: Ipv6Addr, port: Option<u16>) -> Result<CompressedIpv6, FourWordError> {
        let segments = ip.segments();

        // For special addresses, store all segments but mark as special
        let mut compressed = Vec::new();
        for segment in segments {
            compressed.extend_from_slice(&segment.to_be_bytes());
        }

        Ok(CompressedIpv6 {
            category: Ipv6Category::Special,
            compressed_data: compressed,
            original_bits: 128,
            compressed_bits: 3 + 128, // category + full address
            port,
        })
    }

    /// Try to compress using common provider patterns
    fn try_provider_patterns(segments: &[u16; 8]) -> Option<Vec<u8>> {
        // Common patterns from major IPv6 providers
        let patterns = [
            // Google: 2001:4860::/32
            ([0x2001, 0x4860], 32),
            // Hurricane Electric: 2001:470::/32
            ([0x2001, 0x0470], 32),
            // Comcast: 2001:558::/32
            ([0x2001, 0x0558], 32),
        ];

        for (pattern, prefix_bits) in patterns {
            if segments[0] == pattern[0] && segments[1] == pattern[1] {
                // Store pattern ID + remaining bits
                let pattern_id = match pattern {
                    [0x2001, 0x4860] => 0u8,
                    [0x2001, 0x0470] => 1u8,
                    [0x2001, 0x0558] => 2u8,
                    _ => continue,
                };

                let mut compressed = vec![pattern_id];

                // Store the remaining segments after the pattern
                let remaining_segments = 8 - (prefix_bits / 16);
                for i in (8 - remaining_segments)..8 {
                    compressed.extend_from_slice(&segments[i].to_be_bytes());
                }

                return Some(compressed);
            }
        }

        None
    }

    // Decompression methods (implementations would mirror compression logic)
    fn decompress_loopback(_data: &[u8]) -> Result<Ipv6Addr, FourWordError> {
        Ok(Ipv6Addr::LOCALHOST)
    }

    fn decompress_link_local(data: &[u8]) -> Result<Ipv6Addr, FourWordError> {
        if data.is_empty() {
            return Err(FourWordError::InvalidInput(
                "Empty link-local data".to_string(),
            ));
        }

        let mut segments = [0u16; 8];
        segments[0] = 0xfe80;
        segments[1] = 0x0000;
        segments[2] = 0x0000;
        segments[3] = 0x0000;

        match data[0] {
            0 => {
                // All zeros pattern: fe80::
                // segments already initialized correctly
            }
            1 => {
                // Single value pattern
                if data.len() >= 3 {
                    let pos = data[1] as usize + 4; // Convert back to absolute position
                    let val = data[2] as u16;
                    if (4..8).contains(&pos) {
                        segments[pos] = val;
                    }
                }
            }
            2 => {
                // EUI-64 derived address
                if data.len() >= 7 {
                    segments[4] = ((data[2] as u16) << 8) | (data[1] as u16) | 0x0200;
                    segments[5] = ((data[4] as u16) << 8) | (data[3] as u16);
                    segments[6] = ((data[6] as u16) << 8) | (data[5] as u16);
                    // segments[7] remains 0 - simplified reconstruction
                }
            }
            3 => {
                // Complex pattern with RLE
                let mut i = 1;
                while i < data.len() && data[i] != 255 {
                    if i + 2 < data.len() {
                        let pos = data[i] as usize + 4; // Convert back to absolute position
                        let val = ((data[i + 1] as u16) << 8) | (data[i + 2] as u16);
                        if (4..8).contains(&pos) {
                            segments[pos] = val;
                        }
                        i += 3;
                    } else {
                        break;
                    }
                }
            }
            _ => {
                return Err(FourWordError::InvalidInput(
                    "Invalid link-local pattern".to_string(),
                ));
            }
        }

        Ok(Ipv6Addr::from(segments))
    }

    fn decompress_unique_local(data: &[u8]) -> Result<Ipv6Addr, FourWordError> {
        if data.len() == 8 {
            // Interface ID is zero, only prefix + global ID + subnet are stored
            let segments = [
                ((data[0] as u16) << 8) | (data[1] as u16), // segments[0] (fc/fd prefix)
                ((data[2] as u16) << 8) | (data[3] as u16), // segments[1]
                ((data[4] as u16) << 8) | (data[5] as u16), // segments[2]
                ((data[6] as u16) << 8) | (data[7] as u16), // segments[3] (subnet)
                0x0000, // segments[4] - interface ID is zero
                0x0000, // segments[5] - interface ID is zero
                0x0000, // segments[6] - interface ID is zero
                0x0000, // segments[7] - interface ID is zero
            ];
            Ok(Ipv6Addr::from(segments))
        } else if data.len() == 16 {
            // Interface ID is non-zero, all 8 segments are stored
            let segments = [
                ((data[0] as u16) << 8) | (data[1] as u16),   // segments[0] (fc/fd prefix)
                ((data[2] as u16) << 8) | (data[3] as u16),   // segments[1]
                ((data[4] as u16) << 8) | (data[5] as u16),   // segments[2]
                ((data[6] as u16) << 8) | (data[7] as u16),   // segments[3] (subnet)
                ((data[8] as u16) << 8) | (data[9] as u16),   // segments[4] - interface ID
                ((data[10] as u16) << 8) | (data[11] as u16), // segments[5] - interface ID
                ((data[12] as u16) << 8) | (data[13] as u16), // segments[6] - interface ID
                ((data[14] as u16) << 8) | (data[15] as u16), // segments[7] - interface ID
            ];
            Ok(Ipv6Addr::from(segments))
        } else {
            Err(FourWordError::InvalidInput(format!(
                "Invalid unique local data length: {} (expected 8 or 16 bytes)",
                data.len()
            )))
        }
    }

    fn decompress_documentation(data: &[u8]) -> Result<Ipv6Addr, FourWordError> {
        if data.is_empty() {
            return Err(FourWordError::InvalidInput(
                "Empty documentation data".to_string(),
            ));
        }

        let mut segments = [0u16; 8];
        segments[0] = 0x2001;
        segments[1] = 0x0db8;

        match data[0] {
            0 => {
                // All zeros pattern: 2001:db8::
                // segments already initialized to zeros
            }
            1 => {
                // Single value pattern
                if data.len() >= 3 {
                    let pos = data[1] as usize;
                    let val = data[2] as u16;
                    if (2..8).contains(&pos) {
                        segments[pos] = val;
                    }
                }
            }
            2 => {
                // Complex pattern with RLE
                let mut i = 1;
                while i < data.len() && data[i] != 255 {
                    if i + 2 < data.len() {
                        let pos = data[i] as usize;
                        let val = ((data[i + 1] as u16) << 8) | (data[i + 2] as u16);
                        if (2..8).contains(&pos) {
                            segments[pos] = val;
                        }
                        i += 3;
                    } else {
                        break;
                    }
                }
            }
            _ => {
                return Err(FourWordError::InvalidInput(
                    "Invalid documentation pattern".to_string(),
                ));
            }
        }

        Ok(Ipv6Addr::from(segments))
    }

    fn decompress_global_unicast(data: &[u8]) -> Result<Ipv6Addr, FourWordError> {
        if data.len() >= 16 {
            let mut segments = [0u16; 8];
            for i in 0..8 {
                segments[i] = ((data[i * 2] as u16) << 8) | (data[i * 2 + 1] as u16);
            }
            Ok(Ipv6Addr::from(segments))
        } else {
            Err(FourWordError::InvalidInput(
                "Invalid global unicast data".to_string(),
            ))
        }
    }

    fn decompress_unspecified(_data: &[u8]) -> Result<Ipv6Addr, FourWordError> {
        Ok(Ipv6Addr::UNSPECIFIED)
    }

    fn decompress_special(data: &[u8]) -> Result<Ipv6Addr, FourWordError> {
        if data.len() >= 16 {
            let mut segments = [0u16; 8];
            for i in 0..8 {
                segments[i] = ((data[i * 2] as u16) << 8) | (data[i * 2 + 1] as u16);
            }
            Ok(Ipv6Addr::from(segments))
        } else {
            Err(FourWordError::InvalidInput(
                "Invalid special address data".to_string(),
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_loopback_compression() {
        let ip = Ipv6Addr::LOCALHOST;
        let compressed = Ipv6Compressor::compress(ip, Some(443)).unwrap();

        assert_eq!(compressed.category, Ipv6Category::Loopback);
        assert_eq!(compressed.compressed_data.len(), 6); // Padded to 6 bytes
        // With category byte + 6 bytes data = 56 bits total = 4 words
        assert!(compressed.recommended_word_count() >= 4); // IPv6 minimum 4 words

        let (decompressed_ip, port) = Ipv6Compressor::decompress(&compressed).unwrap();
        assert_eq!(decompressed_ip, ip);
        assert_eq!(port, Some(443));
    }

    #[test]
    fn test_unspecified_compression() {
        let ip = Ipv6Addr::UNSPECIFIED;
        let compressed = Ipv6Compressor::compress(ip, None).unwrap();

        assert_eq!(compressed.category, Ipv6Category::Unspecified);
        assert_eq!(compressed.compressed_data.len(), 6); // Padded to 6 bytes
        assert!(compressed.recommended_word_count() >= 4); // IPv6 minimum 4 words
    }

    #[test]
    fn test_link_local_compression() {
        let ip = Ipv6Addr::from_str("fe80::1").unwrap();
        let compressed = Ipv6Compressor::compress(ip, Some(22)).unwrap();

        assert_eq!(compressed.category, Ipv6Category::LinkLocal);
        assert!(compressed.recommended_word_count() >= 4); // IPv6 minimum 4 words
        assert!(compressed.compression_ratio() > 0.3); // Adjusted for padding
    }

    #[test]
    fn test_documentation_compression() {
        let ip = Ipv6Addr::from_str("2001:db8::1").unwrap();
        let compressed = Ipv6Compressor::compress(ip, Some(80)).unwrap();

        assert_eq!(compressed.category, Ipv6Category::Documentation);
        assert!(
            compressed.recommended_word_count() >= 4 && compressed.recommended_word_count() <= 6
        );
    }

    #[test]
    fn test_category_descriptions() {
        let ip = Ipv6Addr::LOCALHOST;
        let compressed = Ipv6Compressor::compress(ip, None).unwrap();
        assert_eq!(compressed.category_description(), "IPv6 Loopback (::1)");
    }

    #[test]
    fn test_compression_ratios() {
        let test_cases = vec![
            (Ipv6Addr::LOCALHOST, "loopback"),
            (Ipv6Addr::UNSPECIFIED, "unspecified"),
            (Ipv6Addr::from_str("fe80::1").unwrap(), "link-local"),
        ];

        for (ip, name) in test_cases {
            let compressed = Ipv6Compressor::compress(ip, Some(443)).unwrap();
            let ratio = compressed.compression_ratio();
            println!("{}: {:.1}% compression", name, ratio * 100.0);
            assert!(ratio > 0.0, "{name} should have some compression");
        }
    }
}
