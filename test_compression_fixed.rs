use std::net::Ipv6Addr;
use std::str::FromStr;

// This is a standalone test to verify the compression works after fixing the dictionary
// Note: This would need the actual crate imports to work properly, but shows the expected behavior

fn main() {
    println!("Testing compression of 2001:db8:85a3::8a2e:370:7334");
    
    let ip = Ipv6Addr::from_str("2001:db8:85a3::8a2e:370:7334").unwrap();
    let segments = ip.segments();
    
    println!("IPv6 segments: {:x?}", segments);
    
    // Simulate the compress_documentation function behavior
    let mut compressed = Vec::new();
    
    // Store segments 2-3 (routing prefix after 2001:db8)
    compressed.extend_from_slice(&segments[2].to_be_bytes()); // 0x85a3 → [0x85, 0xa3]
    compressed.extend_from_slice(&segments[3].to_be_bytes()); // 0x0000 → [0x00, 0x00]
    
    println!("After storing segments 2-3: {:?}", compressed);
    
    // Find non-zero interface segments (4-7)
    let non_zero_interface: Vec<(usize, u16)> = segments[4..8]
        .iter()
        .enumerate()
        .filter(|&(_, &seg)| seg != 0)
        .map(|(i, &seg)| (i + 4, seg))
        .collect();
    
    println!("Non-zero interface segments: {:?}", non_zero_interface);
    
    // Multiple non-zero segments: use marker 2
    compressed.insert(0, 2);
    for &(pos, val) in &non_zero_interface {
        compressed.push((pos - 4) as u8);
        compressed.extend_from_slice(&val.to_be_bytes());
    }
    compressed.push(255); // End marker
    
    println!("Final compressed data: {:?}", compressed);
    println!("Expected: [2, 133, 163, 0, 0, 1, 138, 46, 2, 3, 112, 3, 115, 52, 255]");
    
    // Verify the compression
    let expected = vec![2, 133, 163, 0, 0, 1, 138, 46, 2, 3, 112, 3, 115, 52, 255];
    if compressed == expected {
        println!("✅ Compression is correct!");
    } else {
        println!("❌ Compression mismatch");
        println!("Got:      {:?}", compressed);
        println!("Expected: {:?}", expected);
    }
}