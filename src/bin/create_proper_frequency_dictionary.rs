#!/usr/bin/env rust
//! Create a properly frequency-ordered dictionary from existing processed data

use std::fs;
use std::collections::HashSet;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Creating properly ordered frequency-based dictionary...\n");
    
    // Read the processed CSV file
    let csv_content = fs::read_to_string("data/common-words-processed.csv")?;
    let mut words_with_freq: Vec<(String, u64)> = Vec::new();
    
    // Parse CSV (skip header)
    for (i, line) in csv_content.lines().enumerate() {
        if i == 0 { continue; } // Skip header
        
        let parts: Vec<&str> = line.split(',').collect();
        if parts.len() >= 2 {
            let word = parts[0].trim().to_lowercase();
            let frequency = parts[1].trim().parse::<u64>().unwrap_or(0);
            
            // Apply same filters as before
            if is_suitable_word(&word) {
                words_with_freq.push((word, frequency));
            }
        }
    }
    
    println!("Found {} suitable words from Hugging Face dataset", words_with_freq.len());
    
    // Sort by frequency (highest first) - THIS IS THE KEY!
    words_with_freq.sort_by(|a, b| b.1.cmp(&a.1));
    
    // Take the most frequent words
    let mut selected_words: Vec<String> = words_with_freq
        .into_iter()
        .map(|(word, _freq)| word)
        .collect();
    
    // If we don't have enough, add high-quality filler words
    if selected_words.len() < 65536 {
        println!("Adding {} high-quality filler words...", 65536 - selected_words.len());
        add_quality_filler_words(&mut selected_words);
    }
    
    // Ensure exactly 65,536 unique words
    let mut seen = HashSet::new();
    selected_words.retain(|word| seen.insert(word.clone()));
    selected_words.truncate(65536);
    
    // Final padding if needed
    while selected_words.len() < 65536 {
        let filler = format!("word{:05}", selected_words.len());
        if seen.insert(filler.clone()) {
            selected_words.push(filler);
        }
    }
    
    // Save the dictionary
    let output = selected_words.join("\n");
    fs::write("data/proper_frequency_word_list_65k.txt", &output)?;
    
    println!("\n✓ Created properly frequency-ordered dictionary with {} words", selected_words.len());
    
    // Show examples
    println!("\nFirst 50 words (highest frequency):");
    for (i, word) in selected_words.iter().take(50).enumerate() {
        print!("{:12} ", word);
        if (i + 1) % 5 == 0 { println!(); }
    }
    
    println!("\n\nExample three-word addresses with new dictionary:");
    println!("- {}.{}.{}", selected_words[0], selected_words[100], selected_words[1000]);
    println!("- {}.{}.{}", selected_words[10], selected_words[500], selected_words[5000]);
    println!("- {}.{}.{}", selected_words[50], selected_words[1000], selected_words[10000]);
    
    Ok(())
}

fn is_suitable_word(word: &str) -> bool {
    // Length check (3+ characters as requested)
    if word.len() < 3 {
        return false;
    }
    
    // No multi-word phrases
    if word.contains('_') || word.contains(' ') || word.contains('-') {
        return false;
    }
    
    // Only lowercase letters
    if !word.chars().all(|c| c.is_ascii_lowercase()) {
        return false;
    }
    
    // Must have at least one vowel
    if !word.chars().any(|c| "aeiou".contains(c)) {
        return false;
    }
    
    // No excessive consonants
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
    
    // Not offensive
    !is_offensive(word)
}

fn is_offensive(word: &str) -> bool {
    let offensive_terms = [
        "fuck", "shit", "piss", "cock", "cunt", "bitch", "bastard", "damn", "hell",
        "ass", "arse", "dick", "pussy", "slut", "whore", "fag", "dyke",
        "nigger", "nigga", "retard", "rape", "nazi", "hitler",
    ];
    
    offensive_terms.iter().any(|&term| word.contains(term))
}

fn add_quality_filler_words(words: &mut Vec<String>) {
    let quality_words = vec![
        // Most common English words
        "the", "and", "that", "have", "with", "this", "will", "your", "from", "they",
        "would", "there", "their", "what", "about", "which", "when", "make", "like", "time",
        "very", "just", "know", "take", "people", "into", "year", "good", "some", "could",
        "them", "see", "other", "than", "then", "now", "look", "only", "come", "its",
        "over", "think", "also", "back", "after", "use", "two", "how", "work", "first",
        "well", "way", "even", "new", "want", "because", "any", "these", "give", "day",
        
        // Common verbs with forms
        "working", "playing", "running", "walking", "talking", "thinking", "looking", "making", "taking", "giving",
        "helping", "starting", "stopping", "opening", "closing", "writing", "reading", "speaking", "listening", "watching",
        "learning", "teaching", "building", "creating", "connecting", "managing", "handling", "controlling", "changing", "moving",
        "inserting", "processing", "developing", "programming", "designing", "analyzing", "computing", "networking",
        
        // Common nouns
        "person", "people", "world", "life", "hand", "part", "child", "eye", "woman", "place",
        "work", "week", "case", "point", "government", "company", "number", "group", "problem", "fact",
        
        // Technology terms
        "computer", "system", "program", "software", "hardware", "network", "internet", "website", "server", "database",
        "application", "technology", "digital", "online", "device", "mobile", "platform", "security", "access", "account",
    ];
    
    for word in quality_words {
        if !words.iter().any(|w| w == word) {
            words.push(word.to_string());
        }
    }
}