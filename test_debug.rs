use four_word_networking::FourWordAdaptiveEncoder;

fn main() {
    let encoder = FourWordAdaptiveEncoder::new().expect("Failed to create encoder");
    
    let test_address = "[2001:4860:4860::8888]:53";
    println\!("Testing address: {}", test_address);
    
    // Encode
    match encoder.encode(test_address) {
        Ok(encoded) => {
            println\!("Encoded: '{}'", encoded);
            println\!("Word count: {}", encoded.split_whitespace().count());
            
            // Try to decode
            match encoder.decode(&encoded) {
                Ok(decoded) => {
                    println\!("Decoded: '{}'", decoded);
                    if decoded == test_address {
                        println\!("✓ Roundtrip successful\!");
                    } else {
                        println\!("✗ Roundtrip failed\!");
                        println\!("  Expected: {}", test_address);
                        println\!("  Got:      {}", decoded);
                    }
                },
                Err(e) => {
                    println\!("Decoding error: {:?}", e);
                }
            }
        },
        Err(e) => {
            println\!("Encoding error: {:?}", e);
        }
    }
}
EOF < /dev/null