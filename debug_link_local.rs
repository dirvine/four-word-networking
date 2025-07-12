use std::net::Ipv6Addr;
use three_word_networking::ipv6_compression::{Ipv6Compressor, Ipv6Category};

fn main() {
    // Test the specific failing case: fe80::e00:0:0:1
    let ip = Ipv6Addr::new(0xfe80, 0, 0, 0, 0x0e00, 0, 0, 1);
    println!("Original IP: {}", ip);
    
    // Compress it
    let compressed = Ipv6Compressor::compress(ip, None).unwrap();
    println!("Category: {:?}", compressed.category);
    println!("Compressed data: {:?}", compressed.compressed_data);
    println!("Compressed data (hex): {:02x?}", compressed.compressed_data);
    
    // Decompress it
    let (decompressed, _) = Ipv6Compressor::decompress(&compressed).unwrap();
    println!("Decompressed IP: {}", decompressed);
    
    // Check if they match
    println!("Match: {}", ip == decompressed);
}