use four_word_networking::UltraCompactEncoder;
use std::collections::HashMap;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🧪 VALIDATING THE 'AIM' FIX WITH DIVERSE INPUTS");
    println!("===============================================");
    
    let encoder = UltraCompactEncoder::new()?;
    
    // Test many different multiaddresses
    let test_cases = vec![
        "/ip4/127.0.0.1/tcp/4001",
        "/ip4/127.0.0.1/tcp/8080", 
        "/ip4/127.0.0.1/udp/53",
        "/ip4/192.168.1.1/tcp/80",
        "/ip4/192.168.1.1/tcp/443",
        "/ip4/192.168.1.100/tcp/8080",
        "/ip4/192.168.0.1/tcp/22",
        "/ip4/10.0.0.1/tcp/22",
        "/ip4/10.0.0.1/udp/53",
        "/ip4/10.0.1.1/tcp/3000",
        "/ip6/::1/tcp/4001",
        "/ip6/::1/tcp/443",
        "/ip6/::1/udp/53",
        "/ip4/172.16.0.1/tcp/80",
        "/ip4/172.16.1.1/tcp/443",
        "/ip4/203.0.113.1/tcp/80",    // Public IP
        "/ip4/203.0.113.1/tcp/443",
        "/ip4/8.8.8.8/udp/53",        // Google DNS
        "/ip4/1.1.1.1/udp/53",        // Cloudflare DNS
        "/ip4/203.0.113.100/tcp/8080",
    ];
    
    let mut word_counts = HashMap::new();
    let mut aim_count = 0;
    let mut total_encodings = 0;
    
    println!("Testing {} multiaddresses...\n", test_cases.len());
    
    for multiaddr in test_cases {
        match encoder.encode(multiaddr) {
            Ok(encoded) => {
                let words = encoded.to_words();
                total_encodings += 1;
                
                // Count word occurrences
                for word in words.split_whitespace() {
                    *word_counts.entry(word.to_string()).or_insert(0) += 1;
                    if word == "aim" {
                        aim_count += 1;
                        println!("❌ Found 'aim': {} -> {}", multiaddr, words);
                    }
                }
                
                println!("✅ {} -> {}", multiaddr, words);
            }
            Err(e) => println!("❌ Error: {} -> {}", multiaddr, e),
        }
    }
    
    println!("\n📊 ANALYSIS RESULTS:");
    println!("Total encodings: {}", total_encodings);
    println!("Occurrences of 'aim': {} ({:.1}%)", aim_count, aim_count as f64 / (total_encodings * 3) as f64 * 100.0);
    
    // Show most frequent words
    let mut word_freq: Vec<_> = word_counts.iter().collect();
    word_freq.sort_by(|a, b| b.1.cmp(a.1));
    
    println!("\nTop 10 most frequent words:");
    for (word, count) in word_freq.iter().take(10) {
        let percentage = **count as f64 / (total_encodings * 3) as f64 * 100.0;
        println!("  '{}': {} times ({:.1}%)", word, count, percentage);
    }
    
    if aim_count == 0 {
        println!("\n🎉 SUCCESS: No 'aim' repetitions found!");
        println!("✅ The fix completely eliminated the clustering issue.");
    } else {
        println!("\n⚠️  Warning: Still found {} 'aim' occurrences", aim_count);
    }
    
    // Check for new clustering patterns
    let max_frequency = word_freq.first().map(|(_, count)| **count).unwrap_or(0);
    let excessive_clustering_threshold = (total_encodings * 3) / 20; // More than 5% is excessive
    
    if max_frequency > excessive_clustering_threshold {
        let (word, count) = word_freq.first().unwrap();
        println!("\n⚠️  New clustering detected: '{}' appears {} times ({:.1}%)", 
            word, count, **count as f64 / (total_encodings * 3) as f64 * 100.0);
    } else {
        println!("\n✅ Excellent distribution: No excessive clustering detected");
    }
    
    Ok(())
}