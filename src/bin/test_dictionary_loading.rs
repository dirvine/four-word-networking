#!/usr/bin/env rust
//! Test which dictionary is actually being loaded

use three_word_networking::dictionary65k::Dictionary65K;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Testing dictionary loading...");
    
    let dict = Dictionary65K::new()?;
    
    // Check the first 10 words
    println!("First 10 words:");
    for i in 0..10 {
        let word = dict.get_word(i)?;
        println!("  {}: {}", i, word);
    }
    
    // Check if specific old words exist
    let old_words = ["cry", "dec", "des", "abc", "cot"];
    println!("\nChecking for old words:");
    for word in &old_words {
        match dict.get_index(word) {
            Ok(index) => println!("  '{}' found at index {}", word, index),
            Err(_) => println!("  '{}' NOT found (good!)", word),
        }
    }
    
    // Check if specific new words exist
    let new_words = ["aaa", "aaberg", "aachen", "aah", "aaker"];
    println!("\nChecking for new words:");
    for word in &new_words {
        match dict.get_index(word) {
            Ok(index) => println!("  '{}' found at index {}", word, index),
            Err(_) => println!("  '{}' NOT found", word),
        }
    }
    
    Ok(())
}