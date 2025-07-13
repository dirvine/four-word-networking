use std::net::Ipv6Addr;
use std::str::FromStr;

// Copy the relevant structures and functions from ipv6_compression.rs
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Ipv6Category {
    Loopback,
    LinkLocal,
    UniqueLocal,
    Documentation,
    GlobalUnicast,
    Unspecified,
    Special,
}

#[derive(Debug, Clone)]
pub struct CompressedIpv6 {
    pub category: Ipv6Category,
    pub compressed_data: Vec<u8>,
    pub original_bits: usize,
    pub compressed_bits: usize,
    pub port: Option<u16>,
}

pub struct Ipv6Compressor;

impl Ipv6Compressor {
    pub fn new() -> Self {
        Self
    }

    pub fn compress(&self, ip: Ipv6Addr, port: Option<u16>) -> Result<CompressedIpv6, String> {
        let category = Self::categorize_address(&ip);
        println!("Category: {:?}", category);

        match category {
            Ipv6Category::Documentation => Self::compress_documentation(ip, port),
            _ => Err("Not documentation".to_string()),
        }
    }

    fn categorize_address(ip: &Ipv6Addr) -> Ipv6Category {
        let segments = ip.segments();
        println!("Categorizing: segments = {:x?}", segments);

        // Check for documentation 2001:db8::/32
        if segments[0] == 0x2001 && segments[1] == 0x0DB8 {
            println!("Matched documentation pattern");
            return Ipv6Category::Documentation;
        }

        println!("Did not match documentation pattern");
        Ipv6Category::Special
    }

    fn compress_documentation(
        ip: Ipv6Addr,
        port: Option<u16>,
    ) -> Result<CompressedIpv6, String> {
        let segments = ip.segments();
        println!("Compressing documentation address: {:x?}", segments);

        let mut compressed = Vec::new();
        
        // Always store at least segments 2-3 (after the 2001:db8 prefix)
        compressed.extend_from_slice(&segments[2].to_be_bytes());
        compressed.extend_from_slice(&segments[3].to_be_bytes());
        println!("After storing segments 2-3: {:?}", compressed);
        
        // Check for non-zero segments in the interface ID (segments 4-7)
        let non_zero_interface: Vec<(usize, u16)> = segments[4..8]
            .iter()
            .enumerate()
            .filter(|&(_, &seg)| seg != 0)
            .map(|(i, &seg)| (i + 4, seg))
            .collect();
        
        println!("Non-zero interface segments: {:?}", non_zero_interface);
        println!("Is empty: {}", non_zero_interface.is_empty());
        
        if non_zero_interface.is_empty() {
            // No interface ID, just the prefix - marker 0
            println!("Using marker 0 (empty interface)");
            compressed.insert(0, 0);
        } else if non_zero_interface.len() == 1 && non_zero_interface[0].1 <= 255 {
            // Single small value in interface ID - marker 1
            println!("Using marker 1 (single small value)");
            compressed.insert(0, 1);
            let (pos, val) = non_zero_interface[0];
            compressed.push((pos - 4) as u8);
            compressed.push(val as u8);
        } else {
            // Complex interface ID - marker 2, store all non-zero segments
            println!("Using marker 2 (complex interface)");
            compressed.insert(0, 2);
            for &(pos, val) in &non_zero_interface {
                compressed.push((pos - 4) as u8);
                compressed.extend_from_slice(&val.to_be_bytes());
            }
            compressed.push(255); // End marker
        }

        let compressed_bits = 3 + (compressed.len() * 8); // category + data
        println!("Final compressed data: {:?}", compressed);

        Ok(CompressedIpv6 {
            category: Ipv6Category::Documentation,
            compressed_data: compressed,
            original_bits: 128,
            compressed_bits,
            port,
        })
    }
}

fn main() {
    let ip = Ipv6Addr::from_str("2001:db8:85a3::8a2e:370:7334").unwrap();
    let port = Some(8080);
    
    println!("Testing compression of: {}", ip);
    
    let compressor = Ipv6Compressor::new();
    match compressor.compress(ip, port) {
        Ok(compressed) => {
            println!("Compression successful!");
            println!("Category: {:?}", compressed.category);
            println!("Compressed data: {:?}", compressed.compressed_data);
            println!("Original bits: {}", compressed.original_bits);
            println!("Compressed bits: {}", compressed.compressed_bits);
        }
        Err(e) => {
            println!("Compression failed: {}", e);
        }
    }
}