use std::net::Ipv6Addr;
use three_word_networking::ipv6_compression::*;

fn main() {
    println!("Debugging fc00:: compression bug\n");
    
    let test_addresses = vec![
        "fc00::",
        "fc01::",
        "fc00::1",
        "fd00::",
        "fd00::1",
        "fc00:1234:5678:9abc::",
        "fd00:abcd:ef01:2345::",
        "fc00:1234:5678:9abc:1111:2222:3333:4444",
        "fd00::abcd:1234",
    ];
    
    for addr_str in test_addresses {
        println!("Testing: {}", addr_str);
        
        let ip: Ipv6Addr = addr_str.parse().unwrap();
        let segments = ip.segments();
        println!("  Original segments: {:04x?}", segments);
        
        // Compress
        let compressed = Ipv6Compressor::compress(ip, None).unwrap();
        println!("  Category: {:?}", compressed.category);
        println!("  Compressed data: {:02x?}", compressed.compressed_data);
        
        // Decompress
        let (decompressed, _) = Ipv6Compressor::decompress(&compressed).unwrap();
        let decompressed_segments = decompressed.segments();
        println!("  Decompressed segments: {:04x?}", decompressed_segments);
        println!("  Decompressed: {}", decompressed);
        
        if ip != decompressed {
            println!("  ❌ MISMATCH!");
            println!("     Expected: {}", ip);
            println!("     Got:      {}", decompressed);
        } else {
            println!("  ✅ Match");
        }
        
        println!();
    }
}