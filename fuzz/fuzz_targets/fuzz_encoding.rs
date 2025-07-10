#![no_main]

use libfuzzer_sys::fuzz_target;
use three_word_networking::FourWordAdaptiveEncoder;

fuzz_target!(|data: &[u8]| {
    // Try to interpret input as a string
    if let Ok(input_str) = std::str::from_utf8(data) {
        // Skip empty or very short inputs
        if input_str.len() < 3 {
            return;
        }
        
        // Try to create encoder
        if let Ok(encoder) = FourWordAdaptiveEncoder::new() {
            // Try to encode the input as an address
            let _ = encoder.encode(input_str);
            
            // If it looks like it could be words, try to decode it
            if input_str.contains('.') || input_str.contains('-') {
                let _ = encoder.decode(input_str);
            }
        }
    }
});
