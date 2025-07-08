use four_word_networking::encoder16k::UniversalEncoder16K;

fn main() {
    let encoder = UniversalEncoder16K::new().expect("Failed to create encoder");
    
    println!("Testing raw 16K encoding without semantic classification:");
    println!("=========================================================");
    
    let test_cases = vec![
        // Raw IP addresses as bytes (no multiaddr semantics)
        (vec![192, 168, 1, 1], "IPv4 as raw bytes"),
        (vec![192, 168, 1, 1, 0, 80], "IPv4 + port as raw bytes"),
        
        // Tiny hashes
        (vec![0xde, 0xad, 0xbe, 0xef], "4-byte hash"),
        (vec![0xca, 0xfe, 0xba, 0xbe, 0xde, 0xad], "6-byte hash"),
        
        // Short random data
        (vec![1, 2, 3], "3 bytes"),
        (vec![1, 2, 3, 4, 5], "5 bytes"),
        
        // Compressed multiaddr equivalent (what compression might give us)
        (vec![0x04, 192, 168, 1, 1, 0x02, 0, 80], "Compressed multiaddr simulation"),
    ];
    
    for (data, description) in test_cases {
        println!("\n📊 Testing: {} ({} bytes)", description, data.len());
        println!("   Data: {:?}", data);
        
        match encoder.encode(&data) {
            Ok(encoded) => {
                let word_count = match &encoded {
                    four_word_networking::encoder16k::Encoding16K::Simple { words } => 3,
                    four_word_networking::encoder16k::Encoding16K::Hybrid { words, digits } => 3 + digits.len() * 3,
                };
                
                println!("   Encoded: {}", encoded);
                println!("   Total words: {}", word_count);
                
                if word_count == 3 {
                    println!("   ✅ Perfect! Achieved 3-word encoding");
                } else {
                    println!("   📈 Uses {} words", word_count);
                }
                
                // Show efficiency info
                let efficiency = encoder.efficiency_info(&data);
                println!("   Efficiency: {} ({})", efficiency.efficiency_rating, efficiency.description);
            }
            Err(e) => {
                println!("   ❌ Error: {}", e);
            }
        }
    }
    
    println!("\n🎯 Key Insight:");
    println!("   Without semantic overhead, small data (≤5 bytes) achieves perfect 3-word encoding!");
    println!("   The balanced encoder adds semantic classification which increases word count.");
}