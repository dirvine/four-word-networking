#!/usr/bin/env rust
//! Validate and fix dictionary to ensure all words meet our criteria

use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Validating and fixing dictionary...");
    
    // Load current dictionary
    let content = fs::read_to_string("data/natural_readable_word_list_65k.txt")?;
    let words: Vec<String> = content.lines().map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).collect();
    
    println!("Loaded {} words", words.len());
    
    // Validate each word
    let mut valid_words = Vec::new();
    let mut invalid_words = Vec::new();
    
    for word in words {
        if is_valid_word(&word) {
            valid_words.push(word);
        } else {
            invalid_words.push(word.clone());
            println!("Invalid word: '{}' (len: {}, reason: {})", word, word.len(), get_invalid_reason(&word));
        }
    }
    
    println!("Valid words: {}", valid_words.len());
    println!("Invalid words: {}", invalid_words.len());
    
    // If we need more words to reach 65,536
    while valid_words.len() < 65536 {
        let needed = 65536 - valid_words.len();
        println!("Need {} more words", needed);
        
        // Generate simple valid words
        let filler_words = generate_filler_words(needed);
        for filler in filler_words {
            if is_valid_word(&filler) && !valid_words.contains(&filler) {
                valid_words.push(filler);
                if valid_words.len() >= 65536 {
                    break;
                }
            }
        }
    }
    
    // Sort and truncate to exactly 65,536
    valid_words.sort();
    valid_words.dedup();
    valid_words.truncate(65536);
    
    println!("Final valid word count: {}", valid_words.len());
    
    // Save the cleaned dictionary
    let output = valid_words.join("\n");
    fs::write("data/natural_readable_word_list_65k.txt", output)?;
    
    println!("Saved cleaned dictionary");
    
    // Show some examples
    println!("\nFirst 10 words:");
    for (i, word) in valid_words.iter().take(10).enumerate() {
        println!("  {}: {} (len: {})", i, word, word.len());
    }
    
    println!("\nLast 10 words:");
    let len = valid_words.len();
    for (i, word) in valid_words.iter().skip(len - 10).enumerate() {
        println!("  {}: {} (len: {})", len - 10 + i, word, word.len());
    }
    
    Ok(())
}

fn is_valid_word(word: &str) -> bool {
    // Must be 3-10 characters
    if word.len() < 3 || word.len() > 10 {
        return false;
    }
    
    // Must contain only lowercase ASCII letters
    if !word.chars().all(|c| c.is_ascii_lowercase()) {
        return false;
    }
    
    // Must have at least one vowel
    if !word.chars().any(|c| "aeiou".contains(c)) {
        return false;
    }
    
    // No excessive consonant clusters
    let mut consonant_streak = 0;
    for ch in word.chars() {
        if "bcdfghjklmnpqrstvwxyz".contains(ch) {
            consonant_streak += 1;
            if consonant_streak > 4 {
                return false;
            }
        } else {
            consonant_streak = 0;
        }
    }
    
    true
}

fn get_invalid_reason(word: &str) -> String {
    if word.len() < 3 {
        return "too short".to_string();
    }
    if word.len() > 10 {
        return "too long".to_string();
    }
    if !word.chars().all(|c| c.is_ascii_lowercase()) {
        return "non-lowercase chars".to_string();
    }
    if !word.chars().any(|c| "aeiou".contains(c)) {
        return "no vowels".to_string();
    }
    
    let mut consonant_streak = 0;
    for ch in word.chars() {
        if "bcdfghjklmnpqrstvwxyz".contains(ch) {
            consonant_streak += 1;
            if consonant_streak > 4 {
                return "too many consonants".to_string();
            }
        } else {
            consonant_streak = 0;
        }
    }
    
    "unknown".to_string()
}

fn generate_filler_words(needed: usize) -> Vec<String> {
    let mut words = Vec::new();
    
    // Generate simple 3-letter combinations
    for a in 'a'..='z' {
        for b in 'a'..='z' {
            for c in 'a'..='z' {
                if words.len() >= needed {
                    break;
                }
                let word = format!("{}{}{}", a, b, c);
                if is_valid_word(&word) {
                    words.push(word);
                }
            }
            if words.len() >= needed {
                break;
            }
        }
        if words.len() >= needed {
            break;
        }
    }
    
    // If still need more, generate 4-letter combinations
    if words.len() < needed {
        for a in 'a'..='z' {
            for b in 'a'..='z' {
                for c in 'a'..='z' {
                    for d in 'a'..='z' {
                        if words.len() >= needed {
                            break;
                        }
                        let word = format!("{}{}{}{}", a, b, c, d);
                        if is_valid_word(&word) {
                            words.push(word);
                        }
                    }
                    if words.len() >= needed {
                        break;
                    }
                }
                if words.len() >= needed {
                    break;
                }
            }
            if words.len() >= needed {
                break;
            }
        }
    }
    
    words
}