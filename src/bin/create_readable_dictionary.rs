#!/usr/bin/env rust
//! Human-Readable Dictionary Creator for Three-Word Networking
//!
//! This tool creates a 65K dictionary focused on common, memorable English words
//! that are easy to pronounce, spell, and remember for voice communication.
//!
//! Priority: Common everyday words > Technical terms > Obscure words
//!
//! Usage: cargo run --bin create_readable_dictionary

use std::collections::HashSet;
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Creating Human-Readable 65K Dictionary");
    println!("=====================================\n");

    // Start with high-quality base words
    let mut dictionary = HashSet::new();
    
    // Add core common English words
    add_core_vocabulary(&mut dictionary);
    println!("Added {} core vocabulary words", dictionary.len());
    
    // Add descriptive adjectives
    add_descriptive_words(&mut dictionary);
    println!("Added descriptive words, total: {}", dictionary.len());
    
    // Add common verbs and their forms
    add_verbs_and_forms(&mut dictionary);
    println!("Added verbs and forms, total: {}", dictionary.len());
    
    // Add common nouns
    add_common_nouns(&mut dictionary);
    println!("Added common nouns, total: {}", dictionary.len());
    
    // Add numbers as words
    add_number_words(&mut dictionary);
    println!("Added number words, total: {}", dictionary.len());
    
    // Add colors, animals, foods
    add_categories(&mut dictionary);
    println!("Added categories, total: {}", dictionary.len());
    
    // Generate additional readable words if needed
    if dictionary.len() < 65536 {
        let needed = 65536 - dictionary.len();
        println!("Need {} more words, generating...", needed);
        add_systematic_words(&mut dictionary, needed);
    }
    
    // Convert to sorted vector
    let mut words: Vec<String> = dictionary.into_iter().collect();
    words.sort();
    words.truncate(65536);
    
    // Validate all words are readable
    words.retain(|word| is_readable_word(word));
    
    // Pad to exact count if needed
    while words.len() < 65536 {
        let word = generate_simple_word(words.len());
        if !words.contains(&word) && is_readable_word(&word) {
            words.push(word);
        }
    }
    
    words.sort();
    
    // Final validation
    if words.len() != 65536 {
        return Err(format!("Failed to create exactly 65536 words, got {}", words.len()).into());
    }
    
    // Save the readable dictionary
    let output_path = "data/readable_word_list_65k.txt";
    let content = words.join("\n");
    fs::write(output_path, content)?;
    
    println!("\nReadable dictionary created successfully!");
    println!("File: {}", output_path);
    println!("Words: {}", words.len());
    
    // Show some sample words
    println!("\nSample words:");
    for i in (0..words.len()).step_by(words.len() / 10) {
        println!("  {}", words[i]);
    }
    
    Ok(())
}

fn add_core_vocabulary(dict: &mut HashSet<String>) {
    let core_words = [
        // Basic pronouns and articles
        "that", "this", "them", "they", "their", "there", "these", "those",
        "what", "when", "where", "which", "while", "with", "will", "would",
        "your", "yours", "about", "above", "after", "again", "along", "also",
        "among", "around", "back", "been", "before", "being", "below",
        "between", "both", "came", "come", "could", "down", "each", "even",
        "every", "first", "from", "have", "here", "into", "just", "know",
        "last", "like", "look", "made", "make", "many", "more", "most",
        "much", "must", "need", "never", "next", "only", "other", "over",
        "said", "same", "should", "since", "some", "still", "such", "take",
        "than", "then", "time", "today", "took", "turn", "used", "very",
        "want", "ways", "well", "went", "were", "will", "with", "work",
        "year", "years", "open", "close", "start", "stop", "help", "find",
        "give", "part", "place", "right", "small", "large", "good", "great",
        "high", "long", "new", "old", "own", "same", "big", "little",
        "early", "last", "late", "left", "next", "right", "sure", "true",
        
        // Simple action words
        "add", "ask", "call", "care", "feel", "get", "give", "go", "had",
        "has", "have", "hear", "help", "hold", "keep", "know", "learn",
        "leave", "let", "live", "look", "love", "make", "may", "mean",
        "move", "need", "put", "read", "run", "say", "see", "seem", "show",
        "take", "talk", "tell", "think", "try", "turn", "use", "wait",
        "walk", "want", "watch", "way", "work", "write",
    ];
    
    for &word in &core_words {
        if word.len() >= 3 && word.len() <= 8 {
            dict.insert(word.to_string());
        }
    }
}

fn add_descriptive_words(dict: &mut HashSet<String>) {
    let adjectives = [
        // Colors
        "red", "blue", "green", "yellow", "black", "white", "brown", "gray",
        "pink", "purple", "orange", "silver", "gold", "dark", "light",
        
        // Sizes
        "big", "small", "large", "tiny", "huge", "giant", "little", "mini",
        "wide", "narrow", "thick", "thin", "tall", "short", "deep", "shallow",
        
        // Qualities
        "good", "bad", "nice", "great", "fine", "best", "better", "worse",
        "easy", "hard", "simple", "complex", "clear", "clean", "dirty",
        "fresh", "old", "new", "young", "hot", "cold", "warm", "cool",
        "dry", "wet", "soft", "hard", "smooth", "rough", "quiet", "loud",
        "fast", "slow", "quick", "happy", "sad", "angry", "calm", "busy",
        "free", "full", "empty", "rich", "poor", "safe", "strong", "weak",
        "smart", "wise", "kind", "mean", "nice", "rude", "funny", "serious",
        "brave", "scared", "lucky", "careful", "wild", "tame", "real", "fake",
    ];
    
    for &word in &adjectives {
        if word.len() >= 3 && word.len() <= 8 {
            dict.insert(word.to_string());
        }
    }
}

fn add_verbs_and_forms(dict: &mut HashSet<String>) {
    let base_verbs = [
        "ask", "add", "buy", "call", "come", "cook", "cut", "do", "draw",
        "drive", "eat", "fall", "feel", "find", "fly", "get", "give", "go",
        "grow", "have", "hear", "help", "hide", "hold", "jump", "keep",
        "know", "learn", "leave", "live", "look", "love", "make", "meet",
        "move", "need", "open", "play", "pull", "push", "put", "read",
        "ride", "run", "say", "see", "sell", "send", "show", "sing", "sit",
        "sleep", "speak", "stand", "start", "stay", "stop", "swim", "take",
        "talk", "teach", "tell", "think", "throw", "touch", "travel", "try",
        "turn", "use", "visit", "wait", "wake", "walk", "want", "wash",
        "watch", "wear", "win", "work", "write",
    ];
    
    for &verb in &base_verbs {
        if verb.len() >= 3 && verb.len() <= 8 {
            dict.insert(verb.to_string());
            
            // Add common forms
            dict.insert(format!("{}s", verb));
            dict.insert(format!("{}ed", verb));
            dict.insert(format!("{}ing", verb));
            
            // Special cases
            match verb {
                "run" => {
                    dict.insert("runs".to_string());
                    dict.insert("running".to_string());
                    dict.insert("runner".to_string());
                },
                "swim" => {
                    dict.insert("swims".to_string());
                    dict.insert("swimming".to_string());
                    dict.insert("swimmer".to_string());
                },
                "drive" => {
                    dict.insert("drives".to_string());
                    dict.insert("driving".to_string());
                    dict.insert("driver".to_string());
                },
                "teach" => {
                    dict.insert("teaches".to_string());
                    dict.insert("teaching".to_string());
                    dict.insert("teacher".to_string());
                },
                "work" => {
                    dict.insert("works".to_string());
                    dict.insert("working".to_string());
                    dict.insert("worker".to_string());
                },
                "play" => {
                    dict.insert("plays".to_string());
                    dict.insert("playing".to_string());
                    dict.insert("player".to_string());
                },
                _ => {}
            }
        }
    }
}

fn add_common_nouns(dict: &mut HashSet<String>) {
    let nouns = [
        // Body parts
        "arm", "back", "ear", "eye", "face", "foot", "hair", "hand", "head",
        "leg", "mouth", "neck", "nose", "skin", "tooth", "body", "heart",
        
        // Family
        "baby", "boy", "child", "dad", "family", "father", "girl", "man",
        "mom", "mother", "parent", "person", "woman", "son", "daughter",
        "brother", "sister", "friend", "people",
        
        // Home and objects
        "bag", "ball", "bed", "bike", "book", "box", "car", "chair", "cup",
        "desk", "door", "game", "glass", "house", "key", "light", "map",
        "paper", "pen", "phone", "picture", "room", "shoe", "table", "toy",
        "tree", "wall", "window", "yard", "garden", "kitchen", "bathroom",
        
        // Nature
        "air", "animal", "bird", "cat", "dog", "fish", "flower", "grass",
        "horse", "leaf", "moon", "plant", "rain", "river", "rock", "sand",
        "sea", "sky", "snow", "star", "sun", "tree", "water", "wind",
        "cloud", "earth", "fire", "hill", "lake", "mountain", "ocean",
        
        // Food
        "apple", "bread", "cake", "cheese", "chicken", "coffee", "egg",
        "fish", "food", "fruit", "ice", "meat", "milk", "orange", "pizza",
        "rice", "soup", "sugar", "tea", "water", "wine", "banana", "butter",
        
        // Time
        "day", "hour", "minute", "month", "morning", "night", "second",
        "time", "today", "week", "year", "Monday", "Tuesday", "Wednesday",
        "Thursday", "Friday", "Saturday", "Sunday", "January", "February",
        "March", "April", "June", "July", "August", "September", "October",
        "November", "December",
        
        // Abstract concepts
        "idea", "love", "name", "number", "part", "place", "story", "thing",
        "way", "word", "world", "life", "music", "sound", "voice", "color",
        "shape", "size", "space", "change", "chance", "choice", "peace",
        "power", "problem", "reason", "result", "truth", "value",
    ];
    
    for &word in &nouns {
        if word.len() >= 3 && word.len() <= 8 {
            dict.insert(word.to_string());
            
            // Add plurals where appropriate
            if !word.ends_with('s') {
                let plural = if word.ends_with('y') {
                    format!("{}ies", &word[..word.len()-1])
                } else {
                    format!("{}s", word)
                };
                if plural.len() <= 8 {
                    dict.insert(plural);
                }
            }
        }
    }
}

fn add_number_words(dict: &mut HashSet<String>) {
    let numbers = [
        "zero", "one", "two", "three", "four", "five", "six", "seven",
        "eight", "nine", "ten", "eleven", "twelve", "thirteen", "fourteen",
        "fifteen", "sixteen", "seventeen", "eighteen", "nineteen", "twenty",
        "thirty", "forty", "fifty", "sixty", "seventy", "eighty", "ninety",
        "hundred", "thousand", "million", "first", "second", "third",
        "fourth", "fifth", "sixth", "seventh", "eighth", "ninth", "tenth",
    ];
    
    for &word in &numbers {
        if word.len() >= 3 && word.len() <= 8 {
            dict.insert(word.to_string());
        }
    }
}

fn add_categories(dict: &mut HashSet<String>) {
    let animals = [
        "ant", "bear", "bee", "bird", "cat", "cow", "deer", "dog", "duck",
        "eagle", "fish", "fox", "frog", "goat", "horse", "lion", "monkey",
        "mouse", "owl", "pig", "rabbit", "shark", "sheep", "snake", "tiger",
        "turtle", "whale", "wolf", "zebra", "chicken", "elephant", "giraffe",
    ];
    
    let foods = [
        "apple", "banana", "bread", "burger", "cake", "carrot", "cheese",
        "cherry", "cookie", "corn", "grape", "honey", "lemon", "mango",
        "orange", "pasta", "peach", "pear", "pizza", "potato", "rice",
        "salad", "soup", "tomato", "yogurt",
    ];
    
    let objects = [
        "ball", "book", "bottle", "brush", "button", "camera", "candle",
        "clock", "coin", "comb", "computer", "fork", "guitar", "hammer",
        "knife", "ladder", "lamp", "mirror", "needle", "paint", "pencil",
        "piano", "plate", "radio", "rope", "scissors", "spoon", "stamp",
        "stick", "string", "tool", "watch", "wheel",
    ];
    
    for category in [&animals[..], &foods[..], &objects[..]].iter() {
        for &word in *category {
            if word.len() >= 3 && word.len() <= 8 {
                dict.insert(word.to_string());
            }
        }
    }
}


fn is_readable_word(word: &str) -> bool {
    // Must be lowercase ASCII letters only
    if !word.chars().all(|c| c.is_ascii_lowercase()) {
        return false;
    }
    
    // Must have reasonable length
    if word.len() < 3 || word.len() > 8 {
        return false;
    }
    
    // Must have at least one vowel
    let vowel_count = word.chars().filter(|&c| "aeiou".contains(c)).count();
    if vowel_count == 0 {
        return false;
    }
    
    // Avoid difficult consonant clusters
    let difficult_patterns = ["xth", "nth", "rh", "ght", "pht", "sch"];
    for pattern in &difficult_patterns {
        if word.contains(pattern) {
            return false;
        }
    }
    
    // Avoid very repetitive patterns
    if word.len() >= 4 {
        let chars: Vec<char> = word.chars().collect();
        let mut repeats = 0;
        for i in 0..chars.len()-1 {
            if chars[i] == chars[i+1] {
                repeats += 1;
            }
        }
        if repeats > 1 {
            return false;
        }
    }
    
    true
}

fn add_systematic_words(dict: &mut HashSet<String>, needed: usize) {
    let consonants = ['b', 'c', 'd', 'f', 'g', 'h', 'j', 'k', 'l', 'm', 'n', 'p', 'r', 's', 't', 'v', 'w', 'y', 'z'];
    let vowels = ['a', 'e', 'i', 'o', 'u'];
    
    let mut count = 0;
    
    // Generate 4-letter CVCV pattern words
    for c1 in &consonants {
        for v1 in &vowels {
            for c2 in &consonants {
                for v2 in &vowels {
                    if count >= needed { return; }
                    let word = format!("{}{}{}{}", c1, v1, c2, v2);
                    if !dict.contains(&word) && is_readable_word(&word) {
                        dict.insert(word);
                        count += 1;
                    }
                }
            }
        }
    }
    
    // Generate 5-letter CVCVC pattern words
    for c1 in &consonants[..10] { // Limit to prevent timeout
        for v1 in &vowels {
            for c2 in &consonants[..10] {
                for v2 in &vowels {
                    for c3 in &consonants[..10] {
                        if count >= needed { return; }
                        let word = format!("{}{}{}{}{}", c1, v1, c2, v2, c3);
                        if !dict.contains(&word) && is_readable_word(&word) {
                            dict.insert(word);
                            count += 1;
                        }
                    }
                }
            }
        }
    }
}

fn generate_simple_word(index: usize) -> String {
    let consonants = ['b', 'c', 'd', 'f', 'g', 'h', 'j', 'k', 'l', 'm', 'n', 'p', 'r', 's', 't', 'v', 'w', 'y', 'z'];
    let vowels = ['a', 'e', 'i', 'o', 'u'];
    
    // Generate 4-letter words with consonant-vowel-consonant-vowel pattern
    let c1 = consonants[index % consonants.len()];
    let v1 = vowels[(index / consonants.len()) % vowels.len()];
    let c2 = consonants[(index / (consonants.len() * vowels.len())) % consonants.len()];
    let v2 = vowels[(index / (consonants.len() * vowels.len() * consonants.len())) % vowels.len()];
    
    format!("{}{}{}{}", c1, v1, c2, v2)
}