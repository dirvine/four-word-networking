use three_word_networking::ThreeWordAdaptiveEncoder;
use three_word_networking::three_word_ipv6_encoder::ThreeWordIpv6Encoder;

fn main() {
    println!("Testing specific words from issue report");
    println!("=======================================\n");

    // Test with the exact words from the issue
    let issue_words = ["Kaufhof", "Dingley", "Inno", "Roupe", "Stimuli", "Bugger"];

    println!("Issue words (with capitals): {issue_words:?}");

    // Test 1: Direct IPv6 encoder with lowercase
    println!("\nTest 1: Direct IPv6 encoder with lowercase words");
    let encoder = ThreeWordIpv6Encoder::new().unwrap();
    let lowercase_words: Vec<String> = issue_words.iter().map(|w| w.to_lowercase()).collect();
    let lowercase_refs: Vec<&str> = lowercase_words.iter().map(|s| s.as_str()).collect();
    println!("Lowercase words: {lowercase_refs:?}");

    match encoder.decode_from_words(&lowercase_refs) {
        Ok((ip, port)) => {
            println!("Decoded: [{ip}]:{port}");
            println!("IP segments: {:?}", ip.segments());
        }
        Err(e) => {
            println!("Error: {e}");
        }
    }

    // Test 2: ThreeWordAdaptiveEncoder with the formatted string
    println!("\nTest 2: ThreeWordAdaptiveEncoder with dash-separated string");
    let adaptive_encoder = ThreeWordAdaptiveEncoder::new().unwrap();
    let word_string = issue_words.join("-");
    println!("Word string: {word_string}");

    match adaptive_encoder.decode(&word_string) {
        Ok(decoded) => {
            println!("Decoded: {decoded}");
        }
        Err(e) => {
            println!("Error: {e}");
        }
    }

    // Test 3: Let's encode ::1:443 and see what we get
    println!("\nTest 3: Encoding [::1]:443");
    let encoded = adaptive_encoder.encode("[::1]:443").unwrap();
    println!("Encoded: {encoded}");

    // Test 4: Decode what we just encoded
    println!("\nTest 4: Decoding what we just encoded");
    let decoded = adaptive_encoder.decode(&encoded).unwrap();
    println!("Decoded: {decoded}");

    // Test 5: Check if case sensitivity is the issue
    println!("\nTest 5: Manual case testing");
    let test_cases = vec![
        "kaufhof-dingley-inno-roupe-stimuli-bugger", // all lowercase
        "Kaufhof-Dingley-Inno-Roupe-Stimuli-Bugger", // all title case
        "KAUFHOF-DINGLEY-INNO-ROUPE-STIMULI-BUGGER", // all uppercase
    ];

    for test in test_cases {
        println!("\nTesting: {test}");
        match adaptive_encoder.decode(test) {
            Ok(decoded) => println!("  Decoded: {decoded}"),
            Err(e) => println!("  Error: {e}"),
        }
    }
}
