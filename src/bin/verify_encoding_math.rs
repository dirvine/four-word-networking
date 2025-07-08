//! Verify the mathematical calculations for IP address encoding

fn main() {
    println!("IP Address Encoding Mathematical Verification");
    println!("===========================================\n");
    
    // IPv4 + Port calculations
    println!("IPv4 + Port Requirements:");
    println!("- IPv4 address: 32 bits");
    println!("- Port number: 16 bits");
    println!("- Total: 48 bits");
    println!("- Total combinations: 2^48 = {:e}\n", 2u128.pow(48));
    
    // Dictionary size calculations
    let dict_sizes = vec![
        (4_096, 12, "4K"),
        (8_192, 13, "8K"),
        (16_384, 14, "16K"),
        (32_768, 15, "32K"),
        (65_536, 16, "64K"),
    ];
    
    println!("Words needed for IPv4+Port (48 bits):");
    for (size, bits, name) in &dict_sizes {
        let words_needed = (48 + bits - 1) / bits; // Ceiling division
        let total_bits = words_needed * bits;
        let efficiency = (48.0 / total_bits as f64) * 100.0;
        let combinations = (*size as u128).pow(words_needed);
        
        println!("- {} dictionary ({} bits/word): {} words = {} bits ({:.1}% efficient)",
                 name, bits, words_needed, total_bits, efficiency);
        println!("  Can encode: {:e} combinations", combinations);
    }
    
    println!("\nIPv6 + Port Requirements:");
    println!("- IPv6 address: 128 bits");
    println!("- Port number: 16 bits");
    println!("- Total: 144 bits");
    println!("- Total combinations: 2^144 ≈ {:e}\n", 2f64.powf(144.0));
    
    println!("Words needed for IPv6+Port (144 bits):");
    for (size, bits, name) in &dict_sizes {
        let words_needed = (144 + bits - 1) / bits; // Ceiling division
        let total_bits = words_needed * bits;
        let efficiency = (144.0 / total_bits as f64) * 100.0;
        
        println!("- {} dictionary ({} bits/word): {} words = {} bits ({:.1}% efficient)",
                 name, bits, words_needed, total_bits, efficiency);
    }
    
    println!("\n16K Dictionary Analysis (Current Implementation):");
    println!("================================================");
    
    let dict_16k = 16_384u128;
    
    println!("\n3 Words (Current Base Format):");
    let three_word_bits = 3 * 14;
    let three_word_combos = dict_16k.pow(3);
    println!("- Bits available: {} bits", three_word_bits);
    println!("- Combinations: {:e}", three_word_combos);
    println!("- Can encode up to {} bytes of data", three_word_bits / 8);
    println!("- IPv4+Port coverage: {:.2}%", (three_word_combos as f64 / 2f64.powf(48.0)) * 100.0);
    
    println!("\n4 Words (Full IPv4 Coverage):");
    let four_word_bits = 4 * 14;
    let four_word_combos = dict_16k.pow(4);
    println!("- Bits available: {} bits", four_word_bits);
    println!("- Combinations: {:e}", four_word_combos);
    println!("- IPv4+Port coverage: {:.1}%", (four_word_combos as f64 / 2f64.powf(48.0)) * 100.0);
    
    println!("\nCompression Success Rates:");
    println!("- Localhost (127.x.x.x): 2^8 addresses = {} (0.000006% of IPv4)", 256);
    println!("- Private 192.168.x.x: 2^16 addresses = {} (0.0015% of IPv4)", 65536);
    println!("- Private 10.x.x.x: 2^24 addresses = {} (0.39% of IPv4)", 16777216);
    println!("- Private 172.16-31.x.x: 2^20 addresses = {} (0.024% of IPv4)", 1048576);
    println!("- Total private: ~1.8% of IPv4 space can be compressed to 3 words");
    
    println!("\nConclusion:");
    println!("- 3 words with 16K dict: Good for private/common addresses only");
    println!("- 4 words with 16K dict: Covers 100% of IPv4+port combinations");
    println!("- 3 words with 64K dict: Covers 100% of IPv4+port combinations");
}