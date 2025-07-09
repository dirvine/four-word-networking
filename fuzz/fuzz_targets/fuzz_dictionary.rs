#![no_main]

use libfuzzer_sys::fuzz_target;
use four_word_networking::*;
use std::collections::HashSet;

fuzz_target!(|data: &[u8]| {
    // Test dictionary operations
    if data.len() < 2 {
        return;
    }
    
    // Test word index access
    let word_index = u16::from_be_bytes([data[0], data[1]]) % 16384;
    
    // Test dictionary access
    if let Ok(dict) = FourWordDictionary::new() {
        // Test word retrieval
        if let Ok(word) = dict.get_word(word_index) {
            // Test word validation
            validate_word_quality(&word);
            
            // Test word-to-index conversion
            let _ = dict.get_index(&word);
        }
        
        // Test batch operations
        if data.len() >= 8 {
            let indices = [
                u16::from_be_bytes([data[0], data[1]]) % 16384,
                u16::from_be_bytes([data[2], data[3]]) % 16384,
                u16::from_be_bytes([data[4], data[5]]) % 16384,
                u16::from_be_bytes([data[6], data[7]]) % 16384,
            ];
            
            // Test encoding from indices
            if let Ok(words) = dict.encode_from_indices(&indices) {
                // Test decoding back to indices
                let _ = dict.decode_to_indices(&words);
            }
        }
    }
    
    // Test word list processing
    if let Ok(input_str) = std::str::from_utf8(data) {
        if !input_str.is_empty() {
            test_word_list_processing(input_str);
        }
    }
});

/// Test word quality validation
fn validate_word_quality(word: &str) -> bool {
    // Length check
    if word.len() < 3 || word.len() > 12 {
        return false;
    }
    
    // Character validation
    if !word.chars().all(|c| c.is_ascii_lowercase() || c == '-') {
        return false;
    }
    
    // No leading/trailing hyphens
    if word.starts_with('-') || word.ends_with('-') {
        return false;
    }
    
    // No consecutive hyphens
    if word.contains("--") {
        return false;
    }
    
    // Must contain at least one vowel
    let vowels = ['a', 'e', 'i', 'o', 'u'];
    if !word.chars().any(|c| vowels.contains(&c)) {
        return false;
    }
    
    // Must not be too similar to numbers
    let number_like = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "ten"];
    if number_like.contains(&word) {
        return false;
    }
    
    true
}

/// Test word list processing
fn test_word_list_processing(input: &str) {
    // Test splitting into words
    let words: Vec<&str> = input.split_whitespace().collect();
    
    // Test each word
    for word in words {
        if word.len() >= 3 && word.len() <= 12 {
            validate_word_quality(word);
        }
    }
    
    // Test deduplication
    let unique_words: HashSet<&str> = words.into_iter().collect();
    
    // Test sorting
    let mut sorted_words: Vec<&str> = unique_words.into_iter().collect();
    sorted_words.sort();
    
    // Test filtering
    let filtered_words: Vec<&str> = sorted_words
        .into_iter()
        .filter(|word| validate_word_quality(word))
        .collect();
    
    // Test selection of best words
    let selected_words: Vec<&str> = filtered_words
        .into_iter()
        .take(1000)
        .collect();
    
    // Test word scoring
    for word in selected_words {
        let _score = calculate_word_score(word);
    }
}

/// Calculate word quality score
fn calculate_word_score(word: &str) -> f64 {
    let mut score = 0.0;
    
    // Length score (prefer 4-6 characters)
    match word.len() {
        4..=6 => score += 1.0,
        7..=8 => score += 0.8,
        3 | 9..=10 => score += 0.6,
        _ => score += 0.2,
    }
    
    // Vowel ratio score
    let vowels = ['a', 'e', 'i', 'o', 'u'];
    let vowel_count = word.chars().filter(|c| vowels.contains(c)).count();
    let vowel_ratio = vowel_count as f64 / word.len() as f64;
    
    if vowel_ratio >= 0.2 && vowel_ratio <= 0.6 {
        score += 0.5;
    }
    
    // Consonant clusters (penalize)
    let consonant_clusters = count_consonant_clusters(word);
    if consonant_clusters <= 2 {
        score += 0.3;
    }
    
    // Common prefixes/suffixes
    let common_prefixes = ["un", "re", "pre", "dis", "in", "im"];
    let common_suffixes = ["ing", "ed", "er", "est", "ly", "tion"];
    
    for prefix in common_prefixes {
        if word.starts_with(prefix) {
            score += 0.2;
            break;
        }
    }
    
    for suffix in common_suffixes {
        if word.ends_with(suffix) {
            score += 0.2;
            break;
        }
    }
    
    score
}

/// Count consonant clusters in a word
fn count_consonant_clusters(word: &str) -> usize {
    let vowels = ['a', 'e', 'i', 'o', 'u'];
    let chars: Vec<char> = word.chars().collect();
    let mut clusters = 0;
    let mut in_cluster = false;
    
    for ch in chars {
        if vowels.contains(&ch) {
            in_cluster = false;
        } else if !in_cluster {
            clusters += 1;
            in_cluster = true;
        }
    }
    
    clusters
}

/// Test dictionary consistency
fn test_dictionary_consistency() {
    if let Ok(dict) = FourWordDictionary::new() {
        // Test that all indices map to valid words
        for i in 0..16384 {
            if let Ok(word) = dict.get_word(i) {
                // Test that word maps back to same index
                if let Ok(index) = dict.get_index(&word) {
                    assert_eq!(i, index);
                }
            }
        }
    }
}

/// Test edge cases
fn test_edge_cases() {
    if let Ok(dict) = FourWordDictionary::new() {
        // Test boundary indices
        let boundary_indices = [0, 1, 16383, 16384, 32767, 65535];
        
        for index in boundary_indices {
            let _ = dict.get_word(index);
        }
        
        // Test empty and invalid words
        let invalid_words = ["", "a", "aa", "verylongwordthatistoobig", "word-", "-word", "word--word"];
        
        for word in invalid_words {
            let _ = dict.get_index(word);
        }
    }
}