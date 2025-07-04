//! Dictionary builder for 16,384 word dictionary
//!
//! This tool combines multiple high-quality wordlists to create exactly
//! 16,384 unique, memorable words for the Universal Word Encoding system.

use std::collections::{HashMap, HashSet};
use std::fs;
use std::error::Error;

const TARGET_WORD_COUNT: usize = 16_384;

fn main() -> Result<(), Box<dyn Error>> {
    println!("🏗️  Building 16,384 word dictionary...");
    println!("=====================================");
    
    let mut word_candidates = HashSet::new();
    let mut word_sources = HashMap::new();
    
    // Load EFF wordlist (highest priority - very curated)
    println!("\n📚 Loading EFF large wordlist...");
    load_eff_wordlist(&mut word_candidates, &mut word_sources)?;
    
    // Load BIP39 wordlist (crypto standard)
    println!("₿ Loading BIP39 wordlist...");
    load_bip39_wordlist(&mut word_candidates, &mut word_sources)?;
    
    // Load Diceware 8k
    println!("🎲 Loading Diceware 8k wordlist...");
    load_diceware_wordlist(&mut word_candidates, &mut word_sources)?;
    
    // Load English words
    println!("🔤 Loading English words...");
    load_english_wordlist(&mut word_candidates, &mut word_sources)?;
    
    // Load supplementary words
    println!("📝 Loading supplementary words...");
    load_supplementary_wordlist(&mut word_candidates, &mut word_sources)?;
    
    println!("\n📊 Initial word collection:");
    println!("  Total unique candidates: {}", word_candidates.len());
    
    // Filter and validate words
    let mut validated_words = Vec::new();
    for word in word_candidates {
        if is_valid_word(&word) {
            validated_words.push(word);
        }
    }
    
    println!("  Valid words after filtering: {}", validated_words.len());
    
    // Sort by quality/priority
    validated_words.sort_by(|a, b| {
        // Prioritize EFF words, then BIP39, then others
        let a_source = word_sources.get(a).map(|s| s.as_str()).unwrap_or("unknown");
        let b_source = word_sources.get(b).map(|s| s.as_str()).unwrap_or("unknown");
        
        let a_priority = match a_source {
            "eff" => 0,
            "bip39" => 1,
            "diceware" => 2,
            "supplementary" => 3,
            "english" => 4,
            _ => 5,
        };
        
        let b_priority = match b_source {
            "eff" => 0,
            "bip39" => 1,
            "diceware" => 2,
            "supplementary" => 3,
            "english" => 4,
            _ => 5,
        };
        
        // Primary sort by priority, secondary by length (prefer shorter), tertiary alphabetical
        a_priority.cmp(&b_priority)
            .then_with(|| a.len().cmp(&b.len()))
            .then_with(|| a.cmp(b))
    });
    
    // Select exactly 16,384 words
    if validated_words.len() >= TARGET_WORD_COUNT {
        validated_words.truncate(TARGET_WORD_COUNT);
    } else {
        return Err(format!(
            "Not enough valid words! Have {}, need {}",
            validated_words.len(),
            TARGET_WORD_COUNT
        ).into());
    }
    
    // Validate final list
    validate_final_wordlist(&validated_words)?;
    
    // Write to file
    fs::create_dir_all("data")?;
    let wordlist_content = validated_words.join("\n");
    fs::write("data/wordlist_16384.txt", wordlist_content)?;
    
    // Generate statistics
    print_statistics(&validated_words, &word_sources);
    
    println!("\n✅ Successfully built 16,384 word dictionary!");
    println!("   Saved to: data/wordlist_16384.txt");
    
    Ok(())
}

fn load_eff_wordlist(
    words: &mut HashSet<String>, 
    sources: &mut HashMap<String, String>
) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string("wordlists/eff_large_wordlist.txt")?;
    let mut count = 0;
    
    for line in content.lines() {
        if let Some(word) = line.split_whitespace().nth(1) {
            let normalized = normalize_word(word);
            if words.insert(normalized.clone()) {
                sources.insert(normalized, "eff".to_string());
                count += 1;
            }
        }
    }
    
    println!("  Loaded {} EFF words", count);
    Ok(())
}

fn load_bip39_wordlist(
    words: &mut HashSet<String>, 
    sources: &mut HashMap<String, String>
) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string("wordlists/bip39_english.txt")?;
    let mut count = 0;
    
    for line in content.lines() {
        let normalized = normalize_word(line.trim());
        if words.insert(normalized.clone()) {
            sources.insert(normalized, "bip39".to_string());
            count += 1;
        }
    }
    
    println!("  Loaded {} BIP39 words", count);
    Ok(())
}

fn load_diceware_wordlist(
    words: &mut HashSet<String>, 
    sources: &mut HashMap<String, String>
) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string("wordlists/diceware8k.txt")?;
    let mut count = 0;
    
    for line in content.lines() {
        if line.starts_with('#') || line.trim().is_empty() {
            continue;
        }
        
        // Simple format: one word per line
        let normalized = normalize_word(line.trim());
        if !normalized.is_empty() && words.insert(normalized.clone()) {
            sources.insert(normalized, "diceware".to_string());
            count += 1;
        }
    }
    
    println!("  Loaded {} Diceware words", count);
    Ok(())
}

fn load_english_wordlist(
    words: &mut HashSet<String>, 
    sources: &mut HashMap<String, String>
) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string("wordlists/english_words.txt")?;
    let mut count = 0;
    
    for line in content.lines() {
        let normalized = normalize_word(line.trim());
        if !normalized.is_empty() && words.insert(normalized.clone()) {
            sources.insert(normalized, "english".to_string());
            count += 1;
        }
    }
    
    println!("  Loaded {} English words", count);
    Ok(())
}

fn load_supplementary_wordlist(
    words: &mut HashSet<String>, 
    sources: &mut HashMap<String, String>
) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string("wordlists/supplementary.txt")?;
    let mut count = 0;
    
    for line in content.lines() {
        let normalized = normalize_word(line.trim());
        if !normalized.is_empty() && words.insert(normalized.clone()) {
            sources.insert(normalized, "supplementary".to_string());
            count += 1;
        }
    }
    
    println!("  Loaded {} supplementary words", count);
    Ok(())
}

fn normalize_word(word: &str) -> String {
    word.trim()
        .to_lowercase()
        .chars()
        .filter(|c| c.is_ascii_alphabetic())
        .collect()
}

fn is_valid_word(word: &str) -> bool {
    // Length requirements (allow 2-letter words to get more candidates)
    if word.len() < 2 || word.len() > 15 {
        return false;
    }
    
    // Only ASCII letters
    if !word.chars().all(|c| c.is_ascii_alphabetic()) {
        return false;
    }
    
    // Must start with letter
    if !word.chars().next().unwrap().is_ascii_alphabetic() {
        return false;
    }
    
    // Avoid clearly offensive words (basic filter)
    let offensive_patterns = ["xxx"];
    for pattern in &offensive_patterns {
        if word.contains(pattern) {
            return false;
        }
    }
    
    // Skip single letters
    if word.len() == 1 {
        return false;
    }
    
    true
}

fn validate_final_wordlist(words: &[String]) -> Result<(), Box<dyn Error>> {
    println!("\n🔍 Validating final wordlist...");
    
    // Check count
    if words.len() != TARGET_WORD_COUNT {
        return Err(format!("Expected {} words, got {}", TARGET_WORD_COUNT, words.len()).into());
    }
    
    // Check for duplicates
    let unique: HashSet<_> = words.iter().collect();
    if unique.len() != words.len() {
        return Err("Duplicate words found!".into());
    }
    
    // Check for exact duplicates and very similar words (first 3 chars)
    let mut three_char_prefixes = HashMap::new();
    for word in words {
        let prefix = word.chars().take(3).collect::<String>();
        three_char_prefixes.entry(prefix).or_insert_with(Vec::new).push(word.clone());
    }
    
    // Allow some prefix collisions but warn about high counts
    let mut high_collision_count = 0;
    for (prefix, words_with_prefix) in &three_char_prefixes {
        if words_with_prefix.len() > 10 {
            high_collision_count += 1;
            if high_collision_count <= 5 {
                println!("  ⚠️  High collision prefix '{}': {} words", prefix, words_with_prefix.len());
            }
        }
    }
    
    // Check word lengths
    for word in words {
        if word.len() < 2 || word.len() > 15 {
            return Err(format!("Invalid word length: {}", word).into());
        }
    }
    
    println!("  ✅ Exact word count: {}", words.len());
    println!("  ✅ All words unique");
    println!("  ✅ 3-char prefix distribution checked");
    println!("  ✅ All words 2-15 characters");
    
    Ok(())
}

fn print_statistics(words: &[String], sources: &HashMap<String, String>) {
    println!("\n📈 Dictionary Statistics:");
    println!("=========================");
    
    // Length distribution
    let mut length_counts = HashMap::new();
    for word in words {
        *length_counts.entry(word.len()).or_insert(0) += 1;
    }
    
    println!("\n📏 Length Distribution:");
    for len in 3..=12 {
        if let Some(count) = length_counts.get(&len) {
            println!("  {} chars: {} words", len, count);
        }
    }
    
    // Source distribution
    let mut source_counts = HashMap::new();
    for word in words {
        if let Some(source) = sources.get(word) {
            *source_counts.entry(source.clone()).or_insert(0) += 1;
        }
    }
    
    println!("\n📚 Source Distribution:");
    for (source, count) in source_counts {
        println!("  {}: {} words", source, count);
    }
    
    // Sample words
    println!("\n🔤 Sample Words:");
    for i in (0..words.len()).step_by(words.len() / 10) {
        print!("  {}", words[i]);
        if i + words.len() / 10 < words.len() {
            print!(",");
        }
    }
    println!();
}