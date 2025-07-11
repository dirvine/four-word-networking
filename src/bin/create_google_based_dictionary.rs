#!/usr/bin/env rust
//! Create dictionary based on Google's 10,000 most common English words
//! supplemented with natural word forms and additional common words

use std::fs;
use std::collections::HashSet;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Creating Google-based frequency dictionary...\n");
    
    let mut words = Vec::new();
    let mut seen = HashSet::new();
    
    // First, load Google's 10,000 most common words
    if let Ok(google_content) = fs::read_to_string("data/google-10000-english.txt") {
        for line in google_content.lines() {
            let word = line.trim().to_lowercase();
            if is_suitable_word(&word) && seen.insert(word.clone()) {
                words.push(word);
            }
        }
        println!("Loaded {} words from Google 10k list", words.len());
    }
    
    // Add natural forms of common verbs from the Google list
    let common_verbs = vec![
        "be", "have", "do", "say", "get", "make", "go", "know", "take", "see",
        "come", "think", "look", "want", "give", "use", "find", "tell", "ask", "work",
        "seem", "feel", "try", "leave", "call", "keep", "help", "talk", "turn", "start",
        "show", "hear", "play", "run", "move", "like", "live", "believe", "hold", "bring",
        "happen", "write", "provide", "sit", "stand", "lose", "pay", "meet", "include", "continue",
        "set", "learn", "change", "lead", "understand", "watch", "follow", "stop", "create", "speak",
        "read", "allow", "add", "spend", "grow", "open", "walk", "win", "offer", "remember",
        "love", "consider", "appear", "buy", "wait", "serve", "die", "send", "expect", "build",
        "stay", "fall", "cut", "reach", "kill", "remain", "suggest", "raise", "pass", "sell",
        "require", "report", "decide", "pull", "push", "save", "teach", "catch", "choose", "develop",
        "drive", "deal", "carry", "break", "hope", "join", "produce", "eat", "cover", "receive",
        "draw", "return", "agree", "support", "hit", "plan", "drop", "accept", "contain", "bear",
        "search", "connect", "process", "manage", "insert", "install", "debug", "program", "compute", "network"
    ];
    
    // Add -ing forms
    for verb in &common_verbs {
        let ing_form = format!("{}ing", verb);
        // Handle special cases
        let ing_form = match *verb {
            "be" => "being".to_string(),
            "have" => "having".to_string(),
            "make" => "making".to_string(),
            "take" => "taking".to_string(),
            "come" => "coming".to_string(),
            "give" => "giving".to_string(),
            "use" => "using".to_string(),
            "write" => "writing".to_string(),
            "provide" => "providing".to_string(),
            "live" => "living".to_string(),
            "believe" => "believing".to_string(),
            "continue" => "continuing".to_string(),
            "change" => "changing".to_string(),
            "create" => "creating".to_string(),
            "die" => "dying".to_string(),
            "lie" => "lying".to_string(),
            "tie" => "tying".to_string(),
            "hope" => "hoping".to_string(),
            "save" => "saving".to_string(),
            "drive" => "driving".to_string(),
            "receive" => "receiving".to_string(),
            "choose" => "choosing".to_string(),
            "lose" => "losing".to_string(),
            "serve" => "serving".to_string(),
            "leave" => "leaving".to_string(),
            "move" => "moving".to_string(),
            "love" => "loving".to_string(),
            "raise" => "raising".to_string(),
            "close" => "closing".to_string(),
            _ => {
                if verb.ends_with("e") && !verb.ends_with("ee") && !verb.ends_with("ie") && !verb.ends_with("oe") {
                    format!("{}ing", &verb[..verb.len()-1])
                } else if verb.ends_with("y") && verb.len() > 2 && !"aeiou".contains(verb.chars().nth(verb.len()-2).unwrap()) {
                    format!("{}ing", &verb[..verb.len()-1])
                } else {
                    ing_form
                }
            }
        };
        
        if is_suitable_word(&ing_form) && seen.insert(ing_form.clone()) {
            words.push(ing_form);
        }
    }
    
    // Add -ed forms
    for verb in &common_verbs {
        let ed_forms: Vec<String> = match *verb {
            "be" => vec!["was", "were", "been"].iter().map(|s| s.to_string()).collect(),
            "have" => vec!["had".to_string()],
            "do" => vec!["did", "done"].iter().map(|s| s.to_string()).collect(),
            "say" => vec!["said".to_string()],
            "get" => vec!["got", "gotten"].iter().map(|s| s.to_string()).collect(),
            "make" => vec!["made".to_string()],
            "go" => vec!["went", "gone"].iter().map(|s| s.to_string()).collect(),
            "know" => vec!["knew", "known"].iter().map(|s| s.to_string()).collect(),
            "take" => vec!["took", "taken"].iter().map(|s| s.to_string()).collect(),
            "see" => vec!["saw", "seen"].iter().map(|s| s.to_string()).collect(),
            "come" => vec!["came".to_string()],
            "think" => vec!["thought".to_string()],
            "look" => vec!["looked".to_string()],
            "want" => vec!["wanted".to_string()],
            "give" => vec!["gave", "given"].iter().map(|s| s.to_string()).collect(),
            "use" => vec!["used".to_string()],
            "find" => vec!["found".to_string()],
            "tell" => vec!["told".to_string()],
            "ask" => vec!["asked".to_string()],
            "work" => vec!["worked".to_string()],
            "seem" => vec!["seemed".to_string()],
            "feel" => vec!["felt".to_string()],
            "try" => vec!["tried".to_string()],
            "leave" => vec!["left".to_string()],
            "call" => vec!["called".to_string()],
            _ => vec![format!("{}ed", verb)]
        };
        
        for form in ed_forms {
            if is_suitable_word(&form) && seen.insert(form.clone()) {
                words.push(form);
            }
        }
    }
    
    // Add -s forms for nouns
    let common_nouns = vec![
        "time", "person", "year", "way", "day", "thing", "man", "world", "life", "hand",
        "part", "child", "eye", "woman", "place", "work", "week", "case", "point", "company",
        "number", "group", "problem", "fact", "house", "car", "phone", "book", "room", "name",
        "home", "water", "mother", "father", "friend", "door", "road", "hour", "minute", "mile"
    ];
    
    for noun in &common_nouns {
        let plural = match *noun {
            "person" => "people".to_string(),
            "child" => "children".to_string(),
            "man" => "men".to_string(),
            "woman" => "women".to_string(),
            "life" => "lives".to_string(),
            _ => format!("{}s", noun)
        };
        
        if is_suitable_word(&plural) && seen.insert(plural.clone()) {
            words.push(plural);
        }
    }
    
    // Add remaining words from words_alpha.txt that are reasonably common
    if words.len() < 65536 {
        if let Ok(alpha_content) = fs::read_to_string("data/words_alpha.txt") {
            let alpha_words: Vec<String> = alpha_content
                .lines()
                .map(|s| s.trim().to_lowercase())
                .filter(|word| is_suitable_word(word) && is_common_enough(word))
                .collect();
            
            // Add words we haven't seen yet
            for word in alpha_words {
                if seen.insert(word.clone()) {
                    words.push(word);
                    if words.len() >= 65536 {
                        break;
                    }
                }
            }
        }
    }
    
    // Ensure exactly 65,536 words
    words.truncate(65536);
    
    // Final padding if needed
    while words.len() < 65536 {
        let filler = format!("word{:05}", words.len());
        if seen.insert(filler.clone()) {
            words.push(filler);
        }
    }
    
    // Save the dictionary
    let output = words.join("\n");
    fs::write("data/google_based_word_list_65k.txt", &output)?;
    
    println!("\n✓ Created Google-based frequency dictionary with {} words", words.len());
    
    // Show examples
    println!("\nFirst 50 words (from Google's most common):");
    for (i, word) in words.iter().take(50).enumerate() {
        print!("{:10} ", word);
        if (i + 1) % 5 == 0 { println!(); }
    }
    
    println!("\n\nExample three-word addresses:");
    println!("- {}.{}.{}", words[0], words[10], words[100]);
    println!("- {}.{}.{}", words[5], words[50], words[500]);
    println!("- {}.{}.{}", words[15], words[150], words[1500]);
    
    // Check for requested words
    println!("\nChecking for requested -ing forms:");
    for check_word in ["inserting", "connecting", "processing", "managing", "searching", "installing", "debugging"] {
        if words.contains(&check_word.to_string()) {
            println!("✓ Found '{}'", check_word);
        } else {
            println!("✗ Missing '{}'", check_word);
        }
    }
    
    Ok(())
}

fn is_suitable_word(word: &str) -> bool {
    // Allow 2+ character words
    if word.len() < 2 {
        return false;
    }
    
    // Only lowercase letters
    if !word.chars().all(|c| c.is_ascii_lowercase()) {
        return false;
    }
    
    // Skip offensive words
    let offensive = ["fuck", "shit", "damn", "hell", "ass", "dick", "cunt", "bitch", "pussy", "cock"];
    if offensive.iter().any(|&bad| word == bad || word.contains(bad)) {
        return false;
    }
    
    true
}

fn is_common_enough(word: &str) -> bool {
    // Filter out words that are too obscure
    // This is a heuristic - we avoid words that are:
    // - Very technical/scientific
    // - Archaic
    // - Too long (over 12 chars usually means technical)
    
    if word.len() > 12 {
        return false;
    }
    
    // Skip words with these patterns that indicate obscurity
    let obscure_patterns = [
        "ology", "itis", "osis", "aceous", "esque", "iform",
        "zz", "qq", "xx", "aa", "ii", "uu" // unusual letter patterns
    ];
    
    for pattern in &obscure_patterns {
        if word.contains(pattern) {
            return false;
        }
    }
    
    // Skip if it starts with these prefixes (often technical)
    let technical_prefixes = ["xeno", "zygo", "phylo", "pseudo", "proto", "crypto"];
    for prefix in &technical_prefixes {
        if word.starts_with(prefix) {
            return false;
        }
    }
    
    true
}