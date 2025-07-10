use std::net::Ipv6Addr;
use three_word_networking::ipv6_compression::Ipv6Category;
use three_word_networking::three_word_ipv6_encoder::{
    Ipv6ThreeWordGroupEncoding, ThreeWordGroup, ThreeWordIpv6Encoder,
};

fn main() {
    println!("Tracing the decode issue");
    println!("========================\n");

    let encoder = ThreeWordIpv6Encoder::new().unwrap();

    // The problematic words
    let words = vec!["kaufhof", "dingley", "inno", "roupe", "stimuli", "bugger"];

    // Create the encoding structure manually
    let groups = [
        ThreeWordGroup {
            words: [
                words[0].to_string(),
                words[1].to_string(),
                words[2].to_string(),
            ],
        },
        ThreeWordGroup {
            words: [
                words[3].to_string(),
                words[4].to_string(),
                words[5].to_string(),
            ],
        },
    ];

    let encoding = Ipv6ThreeWordGroupEncoding::SixWords {
        groups: groups.clone(),
        original_ip: Ipv6Addr::UNSPECIFIED, // dummy
        original_port: 0,                   // dummy
        category: Ipv6Category::Loopback,   // This should be loopback for ::1
    };

    println!("Decoding with Loopback category:");
    match encoder.decode(&encoding) {
        Ok((ip, port)) => {
            println!("Decoded: [{ip}]:{port}");
            println!("IP segments: {:?}", ip.segments());
        }
        Err(e) => {
            println!("Error: {e}");
        }
    }

    // Now let's try with different categories to see what happens
    let categories = [
        Ipv6Category::Loopback,
        Ipv6Category::Unspecified,
        Ipv6Category::LinkLocal,
        Ipv6Category::GlobalUnicast,
    ];

    println!("\nTrying different categories:");
    for cat in &categories {
        let test_encoding = Ipv6ThreeWordGroupEncoding::SixWords {
            groups: groups.clone(),
            original_ip: Ipv6Addr::UNSPECIFIED,
            original_port: 0,
            category: *cat,
        };

        match encoder.decode(&test_encoding) {
            Ok((ip, port)) => {
                println!("{cat:?}: [{ip}]:{port}");
            }
            Err(e) => {
                println!("{cat:?}: Error - {e}");
            }
        }
    }

    // Let's also check what happens when we encode ::1
    println!("\nEncoding [::1]:443:");
    let encoded = encoder.encode(Ipv6Addr::LOCALHOST, 443).unwrap();
    println!("Encoded words: {:?}", encoded.all_words());

    // And decode it back
    let (decoded_ip, decoded_port) = encoder.decode(&encoded).unwrap();
    println!("Decoded back: [{decoded_ip}]:{decoded_port}");

    // Compare the words
    println!("\nComparing words:");
    println!("Issue words:   {words:?}");
    println!("Encoded words: {:?}", encoded.all_words());
    println!("Match: {}", words == encoded.all_words());
}
