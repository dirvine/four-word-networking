#!/usr/bin/env rust
//! Validate the new 65K dictionary quality and characteristics

use std::collections::HashMap;
use three_word_networking::dictionary65k::Dictionary65K;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("65K Dictionary Quality Validation");
    println!("================================");

    let dict = Dictionary65K::new()?;
    let stats = dict.stats();

    println!("\nBasic Statistics:");
    println!("  Total words: {}", stats.total_words);
    println!("  Average length: {:.1} characters", stats.avg_length);
    println!(
        "  Length range: {}-{} characters",
        stats.min_length, stats.max_length
    );

    println!("\nLength Distribution:");
    let mut lengths: Vec<_> = stats.length_distribution.iter().collect();
    lengths.sort_by_key(|&(len, _)| len);
    for (len, count) in lengths {
        let percentage = (*count as f64 / stats.total_words as f64) * 100.0;
        println!("  {len} chars: {count:5} words ({percentage:4.1}%)");
    }

    // Check for duplicates
    println!("\nDuplicate Analysis:");
    let mut word_counts = HashMap::new();
    for i in 0..65536 {
        let word = dict.get_word(i as u16)?;
        *word_counts.entry(word.to_string()).or_insert(0) += 1;
    }

    let duplicates: Vec<_> = word_counts
        .iter()
        .filter(|&(_, count)| *count > 1)
        .collect();

    if duplicates.is_empty() {
        println!("  ✅ No duplicate words found");
    } else {
        println!("  ❌ Found {} duplicate words:", duplicates.len());
        for (word, count) in duplicates.iter().take(10) {
            println!("    '{word}' appears {count} times");
        }
    }

    // Character analysis
    println!("\nCharacter Analysis:");
    let mut char_counts = HashMap::new();
    let mut has_uppercase = false;
    let mut has_numbers = false;
    let mut has_special = false;

    for i in 0..65536 {
        let word = dict.get_word(i as u16)?;
        for ch in word.chars() {
            *char_counts.entry(ch).or_insert(0) += 1;
            if ch.is_uppercase() {
                has_uppercase = true;
            }
            if ch.is_numeric() {
                has_numbers = true;
            }
            if !ch.is_alphanumeric() {
                has_special = true;
            }
        }
    }

    println!(
        "  Contains uppercase: {}",
        if has_uppercase { "Yes" } else { "No" }
    );
    println!(
        "  Contains numbers: {}",
        if has_numbers { "Yes" } else { "No" }
    );
    println!(
        "  Contains special chars: {}",
        if has_special { "Yes" } else { "No" }
    );

    // Sample words by length
    println!("\nSample Words by Length:");
    for len in 3..=7 {
        let samples: Vec<_> = (0..=65535u16)
            .map(|i| dict.get_word(i).unwrap())
            .filter(|word| word.len() == len)
            .take(5)
            .collect();

        if !samples.is_empty() {
            println!("  {} chars: {}", len, samples.join(", "));
        }
    }

    // Voice-friendliness check (basic)
    println!("\nVoice-Friendliness Analysis:");

    let mut similar_pairs = 0;
    for i in 0..std::cmp::min(1000, 65536) {
        let word1 = dict.get_word(i as u16)?;
        for j in i + 1..std::cmp::min(i + 100, 65536) {
            let word2 = dict.get_word(j as u16)?;
            if sounds_similar(word1, word2) {
                similar_pairs += 1;
            }
        }
    }

    println!("  Potentially confusing pairs in sample: {similar_pairs}");

    // Encoding quality test
    println!("\nEncoding Quality Test:");
    test_encoding_distribution(&dict)?;

    println!("\n✅ Dictionary validation complete!");
    Ok(())
}

fn sounds_similar(word1: &str, word2: &str) -> bool {
    // Very basic phonetic similarity check
    if word1.len() != word2.len() {
        return false;
    }

    let mut differences = 0;
    for (c1, c2) in word1.chars().zip(word2.chars()) {
        if c1 != c2 {
            differences += 1;
            if differences > 1 {
                return false;
            }

            // Check if it's a potentially confusing substitution
            let confusing_pairs = [
                ('b', 'd'),
                ('p', 'b'),
                ('m', 'n'),
                ('f', 'v'),
                ('s', 'z'),
                ('c', 'k'),
                ('i', 'e'),
                ('o', 'u'),
            ];

            let is_confusing = confusing_pairs
                .iter()
                .any(|&(a, b)| (c1 == a && c2 == b) || (c1 == b && c2 == a));

            if !is_confusing {
                return false;
            }
        }
    }

    differences == 1
}

fn test_encoding_distribution(dict: &Dictionary65K) -> Result<(), Box<dyn std::error::Error>> {
    use std::collections::HashSet;

    // Test that different indices produce different words
    let mut word_set = HashSet::new();
    let sample_size = 1000;

    for i in 0..sample_size {
        let word = dict.get_word(i as u16)?;
        word_set.insert(word.to_string());
    }

    println!(
        "  Sample {} indices -> {} unique words",
        sample_size,
        word_set.len()
    );

    if word_set.len() == sample_size {
        println!("  ✅ Perfect uniqueness in sample");
    } else {
        println!("  ⚠️  Some words repeated in sample");
    }

    Ok(())
}
