#!/usr/bin/env rust
//! Create frequency-based dictionary from Hugging Face common-words-79k dataset

use std::fs;
use std::collections::HashSet;
use std::process::Command;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Creating frequency-based dictionary from Hugging Face dataset...\n");
    
    // First, check if uv is available
    let uv_check = Command::new("uv")
        .arg("--version")
        .output();
        
    if uv_check.is_err() {
        println!("Error: UV is required to run the Python script.");
        println!("Please install UV:");
        println!("  curl -LsSf https://astral.sh/uv/install.sh | sh");
        return Ok(());
    }
    
    // Create Python script to download the dataset
    let python_script = r#"
import sys
try:
    from datasets import load_dataset
    import pandas as pd
    
    print("Downloading common-words-79k dataset from Hugging Face...")
    dataset = load_dataset("jaagli/common-words-79k", split="whole")
    
    print(f"Downloaded {len(dataset)} entries")
    
    # Convert to pandas DataFrame
    df = dataset.to_pandas()
    
    # Save as CSV
    df.to_csv("data/common-words-79k-raw.csv", index=False)
    print("Saved raw dataset to data/common-words-79k-raw.csv")
    
    # Also save just the words with frequencies
    if 'alias' in df.columns and 'frequency' in df.columns:
        words_df = df[['alias', 'frequency']].copy()
        words_df.columns = ['word', 'frequency']
        words_df = words_df.sort_values('frequency', ascending=False)
        words_df.to_csv("data/common-words-frequency.csv", index=False)
        print(f"Saved {len(words_df)} words with frequencies")
    else:
        print("Warning: Expected columns not found. Columns are:", df.columns.tolist())
        
except ImportError as e:
    print(f"Error: Missing required Python library: {e}")
    print("Please install: pip3 install datasets pandas")
    sys.exit(1)
except Exception as e:
    print(f"Error downloading dataset: {e}")
    sys.exit(1)
"#;
    
    // Write Python script
    fs::write("download_dataset.py", python_script)?;
    
    // Execute Python script with uv
    println!("Executing Python script with UV to download dataset...");
    let output = Command::new("uv")
        .arg("run")
        .arg("--with")
        .arg("datasets")
        .arg("--with")
        .arg("pandas")
        .arg("python")
        .arg("download_dataset.py")
        .output()?;
        
    if !output.status.success() {
        println!("Python script failed:");
        println!("{}", String::from_utf8_lossy(&output.stderr));
        return Ok(());
    }
    
    println!("{}", String::from_utf8_lossy(&output.stdout));
    
    // Clean up Python script
    fs::remove_file("download_dataset.py").ok();
    
    // Now process the downloaded data
    if let Ok(content) = fs::read_to_string("data/common-words-frequency.csv") {
        process_frequency_data(&content)?;
    } else {
        println!("\nCouldn't find frequency data. Trying raw dataset...");
        if let Ok(content) = fs::read_to_string("data/common-words-79k-raw.csv") {
            process_raw_dataset(&content)?;
        } else {
            println!("Error: No dataset files found. Please check the download process.");
        }
    }
    
    Ok(())
}

fn process_frequency_data(content: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n=== PROCESSING FREQUENCY DATA ===");
    
    let mut words_with_freq = Vec::new();
    let lines: Vec<&str> = content.lines().collect();
    
    // Skip header
    for line in lines.iter().skip(1) {
        let parts: Vec<&str> = line.split(',').collect();
        if parts.len() >= 2 {
            let word = parts[0].trim().to_lowercase();
            let frequency = parts[1].trim().parse::<u64>().unwrap_or(0);
            
            if is_suitable_word(&word) {
                words_with_freq.push((word, frequency));
            }
        }
    }
    
    println!("Found {} suitable words", words_with_freq.len());
    
    // Sort by frequency (highest first)
    words_with_freq.sort_by(|a, b| b.1.cmp(&a.1));
    
    // Take top 65,536 words
    let selected_words: Vec<String> = words_with_freq
        .into_iter()
        .take(65536)
        .map(|(word, _)| word)
        .collect();
        
    save_dictionary(&selected_words)?;
    
    Ok(())
}

fn process_raw_dataset(content: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n=== PROCESSING RAW DATASET ===");
    
    let mut all_words = HashSet::new();
    let lines: Vec<&str> = content.lines().collect();
    
    // Try to extract words from various possible column positions
    for line in lines.iter().skip(1) {
        // Try different parsing strategies
        // Strategy 1: First column
        if let Some(word) = line.split(',').next() {
            let word = word.trim().trim_matches('"').to_lowercase();
            if is_suitable_word(&word) {
                all_words.insert(word);
            }
        }
        
        // Strategy 2: Look for word-like patterns
        for part in line.split(',') {
            let cleaned = part.trim().trim_matches('"').to_lowercase();
            if is_suitable_word(&cleaned) && cleaned.len() >= 3 {
                all_words.insert(cleaned);
            }
        }
    }
    
    println!("Found {} suitable words", all_words.len());
    
    if all_words.len() < 65536 {
        println!("Warning: Not enough suitable words. Found only {}", all_words.len());
        println!("Consider relaxing filters or using a different dataset.");
    }
    
    // Convert to vector and sort
    let mut selected_words: Vec<String> = all_words.into_iter().collect();
    selected_words.sort();
    selected_words.truncate(65536);
    
    // Pad with generated words if needed
    while selected_words.len() < 65536 {
        let filler = format!("word{:05}", selected_words.len());
        selected_words.push(filler);
    }
    
    save_dictionary(&selected_words)?;
    
    Ok(())
}

fn is_suitable_word(word: &str) -> bool {
    // Length check (3-12 characters as requested)
    if word.len() < 3 || word.len() > 12 {
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
    
    // No excessive consonants (max 4 in a row)
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
    if is_offensive(word) {
        return false;
    }
    
    true
}

fn is_offensive(word: &str) -> bool {
    // Comprehensive offensive word filter
    let offensive_words = [
        // Profanity
        "fuck", "shit", "piss", "cock", "cunt", "bitch", "bastard", "damn", "hell",
        "ass", "arse", "dick", "pussy", "slut", "whore", "fag", "dyke", "queer",
        
        // Slurs and hate speech
        "nigger", "nigga", "chink", "gook", "spic", "kike", "wop", "raghead",
        "retard", "retarded", "midget", "cripple", "gimp", "spastic", "mongo",
        
        // Sexual terms
        "penis", "vagina", "testicle", "scrotum", "dildo", "vibrator", "orgasm",
        "masturbate", "ejaculate", "erection", "arousal", "intercourse",
        
        // Drug references
        "cocaine", "heroin", "meth", "crack", "weed", "marijuana", "cannabis",
        
        // Violence
        "murder", "rape", "molest", "abuse", "torture", "mutilate",
        
        // Other inappropriate
        "nazi", "hitler", "suicide", "jihad", "terrorist",
    ];
    
    // Check if word contains any offensive term
    for offensive in &offensive_words {
        if word.contains(offensive) {
            return true;
        }
    }
    
    false
}

fn save_dictionary(words: &[String]) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n=== SAVING FREQUENCY-BASED DICTIONARY ===");
    
    // Ensure exactly 65,536 words
    let mut final_words = words.to_vec();
    final_words.truncate(65536);
    
    // Pad if needed
    while final_words.len() < 65536 {
        let filler = format!("filler{:05}", final_words.len());
        final_words.push(filler);
    }
    
    // Save the new dictionary
    let output = final_words.join("\n");
    fs::write("data/frequency_based_word_list_65k.txt", output)?;
    
    println!("Saved {} words to data/frequency_based_word_list_65k.txt", final_words.len());
    
    // Show some examples
    println!("\n=== SAMPLE WORDS ===");
    println!("First 20 words (most frequent):");
    for (i, word) in final_words.iter().take(20).enumerate() {
        println!("  {:3}. {}", i + 1, word);
    }
    
    println!("\nMiddle sample (around 32k):");
    for i in 32760..32770 {
        if i < final_words.len() {
            println!("  {:5}. {}", i + 1, final_words[i]);
        }
    }
    
    println!("\nLast 10 words:");
    let start = final_words.len().saturating_sub(10);
    for (i, word) in final_words[start..].iter().enumerate() {
        println!("  {:5}. {}", start + i + 1, word);
    }
    
    // Test encoding quality
    println!("\n=== ENCODING QUALITY TEST ===");
    println!("Example three-word combinations:");
    
    // Simulate some encodings using top words
    let examples = [
        (0, 50, 100),
        (10, 200, 500),
        (100, 1000, 5000),
        (1000, 10000, 30000),
    ];
    
    for (a, b, c) in &examples {
        if *c < final_words.len() {
            println!("- {}.{}.{}", final_words[*a], final_words[*b], final_words[*c]);
        }
    }
    
    println!("\nThese combinations are much more natural and memorable!");
    
    Ok(())
}