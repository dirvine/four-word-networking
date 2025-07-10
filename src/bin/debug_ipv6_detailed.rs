use std::net::Ipv6Addr;
use three_word_networking::ipv6_compression::{Ipv6Category, Ipv6Compressor};
use three_word_networking::three_word_ipv6_encoder::ThreeWordIpv6Encoder;

fn main() {
    println!("Detailed IPv6 encoding/decoding debug");
    println!("=====================================\n");

    let encoder = ThreeWordIpv6Encoder::new().unwrap();
    let ip = Ipv6Addr::LOCALHOST; // ::1
    let port = 443;

    println!("Original: [{ip}]:{port}");
    println!("IP segments: {:?}", ip.segments());

    // First, let's check compression
    let compressed = Ipv6Compressor::compress(ip, Some(port)).unwrap();
    println!("\nCompression details:");
    println!("Category: {:?}", compressed.category);
    println!("Compressed data: {:?}", compressed.compressed_data);
    println!("Compressed bits: {}", compressed.compressed_bits);
    println!("Total bits: {}", compressed.total_bits());

    // Now encode
    let encoded = encoder.encode(ip, port).unwrap();
    println!("\nEncoded: {}", encoded.to_string());
    println!("Word count: {}", encoded.word_count());

    // Let's manually trace through encode_six_words
    println!("\nManual encode trace:");

    // The encoding should pack data like this:
    // packed[0] = category as u8 (Loopback = 0)
    // packed[1..10] = compressed data (padded)
    // packed[10..12] = port

    let mut packed = [0u8; 12];
    packed[0] = Ipv6Category::Loopback as u8;
    println!("Category byte: {:#x}", packed[0]);

    // Copy compressed data
    let data_len = compressed.compressed_data.len().min(10);
    packed[1..1 + data_len].copy_from_slice(&compressed.compressed_data[..data_len]);
    println!("Compressed data in packed: {:?}", &packed[1..1 + data_len]);

    // Port
    packed[10] = (port >> 8) as u8;
    packed[11] = (port & 0xFF) as u8;
    println!("Port bytes: [{:#x}, {:#x}]", packed[10], packed[11]);
    println!("Full packed data: {packed:?}");

    // Now let's decode
    println!("\nDecoding:");
    let (decoded_ip, decoded_port) = encoder.decode(&encoded).unwrap();
    println!("Decoded: [{decoded_ip}]:{decoded_port}");
    println!("Decoded IP segments: {:?}", decoded_ip.segments());

    // Let's trace through decode_six_words manually
    println!("\nManual decode trace:");

    // Get the encoded words
    let all_words = encoded.all_words();
    let words_str: Vec<&str> = all_words.iter().map(|s| s.as_str()).collect();
    println!("Words to decode: {words_str:?}");

    // The decoder should:
    // 1. Unpack groups to bytes
    // 2. Extract port from bytes[10..12]
    // 3. Extract category from bytes[0]
    // 4. Reconstruct based on category

    // Let's check what category is being detected during decode
    match &encoded {
        three_word_networking::three_word_ipv6_encoder::Ipv6ThreeWordGroupEncoding::SixWords {
            category,
            ..
        } => {
            println!("Category in encoding: {category:?}");
        }
        _ => println!("Not six words?"),
    }
}
