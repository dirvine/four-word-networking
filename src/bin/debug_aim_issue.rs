use three_word_networking::{
    ultra_compact_encoder::UltraCompactEncoder,
    ultra_compression::UltraCompressor,
    encoder16k::UniversalEncoder16K,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔍 DEBUGGING THE 'AIM' WORD REPETITION ISSUE");
    println!("============================================");
    
    let ultra_encoder = UltraCompactEncoder::new()?;
    let compressor = UltraCompressor::new();
    let encoder16k = UniversalEncoder16K::new()?;
    
    let test_cases = vec![
        "/ip4/127.0.0.1/tcp/4001",
        "/ip4/192.168.1.1/tcp/80",
        "/ip4/192.168.1.100/tcp/8080",
        "/ip6/::1/tcp/4001",
        "/ip4/10.0.0.1/udp/53",
    ];
    
    for multiaddr in test_cases {
        println!("\n🔬 Analyzing: {}", multiaddr);
        
        // Step 1: Show ultra-compression output
        match compressor.ultra_compress(multiaddr) {
            Ok(compressed) => {
                println!("   1️⃣ Ultra-compressed: {:?} ({} bytes)", compressed, compressed.len());
                let hex: String = compressed.iter().map(|b| format!("{:02x}", b)).collect();
                println!("      Hex: {}", hex);
                
                // Step 2: Show how 16K encoder processes this data
                match encoder16k.encode(&compressed) {
                    Ok(encoded16k) => {
                        println!("   2️⃣ 16K encoded: {}", encoded16k);
                        
                        // Show the raw indices if it's simple encoding
                        let mut padded = [0u8; 6];
                        padded[..compressed.len().min(6)].copy_from_slice(&compressed[..compressed.len().min(6)]);
                        let value = u64::from_be_bytes([0, 0, padded[0], padded[1], padded[2], padded[3], padded[4], padded[5]]);
                        
                        let idx1 = ((value >> 28) & 0x3FFF) as u16;
                        let idx2 = ((value >> 14) & 0x3FFF) as u16; 
                        let idx3 = (value & 0x3FFF) as u16;
                        
                        println!("      Raw indices: {} {} {} (third={} -> 'aim' at index 0)", idx1, idx2, idx3, idx3);
                        
                        // Check if idx3 is 0
                        if idx3 == 0 {
                            println!("      🚨 PROBLEM: Third index is 0, which maps to 'aim'");
                        }
                    }
                    Err(e) => println!("   2️⃣ 16K encoding error: {}", e),
                }
            }
            Err(e) => println!("   1️⃣ Ultra-compression error: {}", e),
        }
        
        // Step 3: Show final ultra-compact result
        match ultra_encoder.encode(multiaddr) {
            Ok(result) => {
                let words = result.to_words();
                println!("   3️⃣ Final result: {}", words);
                if words.ends_with("aim") {
                    println!("      ❌ CONFIRMED: Ends with 'aim'");
                } else {
                    println!("      ✅ OK: Does not end with 'aim'");
                }
            }
            Err(e) => println!("   3️⃣ Ultra-compact error: {}", e),
        }
    }
    
    println!("\n💡 ROOT CAUSE ANALYSIS:");
    println!("The ultra-compression creates very small compressed values.");
    println!("When these are processed by the 16K encoder's bit-splitting logic,");
    println!("the third 14-bit segment (bits 13-0) often becomes 0,");
    println!("which maps to dictionary index 0 = 'aim'.");
    
    Ok(())
}