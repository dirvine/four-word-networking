use std::net::Ipv6Addr;
use three_word_networking::three_word_ipv6_encoder::ThreeWordIpv6Encoder;

fn main() {
    println!("Debugging IPv6 encoding/decoding issue");
    println!("======================================\n");

    let encoder = ThreeWordIpv6Encoder::new().unwrap();
    let ip = Ipv6Addr::LOCALHOST; // ::1
    let port = 443;

    println!("Original: [{ip}]:{port}");

    // Encode
    let encoded = encoder.encode(ip, port).unwrap();
    println!("\nEncoded: {}", encoded.to_string());
    println!("Word count: {}", encoded.word_count());
    println!("All words: {:?}", encoded.all_words());

    // Decode
    let (decoded_ip, decoded_port) = encoder.decode(&encoded).unwrap();
    println!("\nDecoded: [{decoded_ip}]:{decoded_port}");

    // Compare
    println!("\nComparison:");
    println!("Original IP:  {:?}", ip.segments());
    println!("Decoded IP:   {:?}", decoded_ip.segments());
    println!("Match: {}", ip == decoded_ip);

    // Let's also test decoding the specific words mentioned
    println!("\n\nTesting specific words from issue:");
    let words = ["Kaufhof", "Dingley", "Inno", "Roupe", "Stimuli", "Bugger"];
    let result = encoder.decode_from_words(&words.iter().copied().collect::<Vec<_>>());
    match result {
        Ok((ip, port)) => {
            println!("Decoded from words: [{ip}]:{port}");
            println!("IP segments: {:?}", ip.segments());
        }
        Err(e) => {
            println!("Error decoding: {e}");
        }
    }
}
