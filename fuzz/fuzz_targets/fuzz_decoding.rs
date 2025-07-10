#![no_main]

use libfuzzer_sys::fuzz_target;
use three_word_networking::FourWordAdaptiveEncoder;

fuzz_target!(|data: &[u8]| {
    // Try to interpret input as a string that might be encoded words
    if let Ok(input_str) = std::str::from_utf8(data) {
        // Skip empty inputs
        if input_str.is_empty() {
            return;
        }
        
        // Try to create encoder
        if let Ok(encoder) = FourWordAdaptiveEncoder::new() {
            // Try to decode the input as words
            if let Ok(decoded) = encoder.decode(input_str) {
                // If decoding succeeds, try to encode it back
                let _ = encoder.encode(decoded.as_str());
            }
            
            // Try validation
            let _ = encoder.is_valid_words(input_str);
        }
    }
});
