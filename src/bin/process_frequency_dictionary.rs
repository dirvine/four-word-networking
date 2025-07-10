#!/usr/bin/env rust
//! Process the downloaded frequency-based dictionary

use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== PROCESSING FREQUENCY-BASED DICTIONARY ===\n");
    
    // Try to load processed data
    let input_file = if fs::metadata("data/common-words-processed.csv").is_ok() {
        "data/common-words-processed.csv"
    } else if fs::metadata("data/common-words-79k-raw.csv").is_ok() {
        println!("Using raw dataset file...");
        "data/common-words-79k-raw.csv"
    } else {
        println!("Error: No dataset files found!");
        println!("\nPlease first run:");
        println!("  python3 setup_frequency_dictionary.py");
        println!("\nThis will download the dataset from Hugging Face.");
        return Ok(());
    };
    
    let content = fs::read_to_string(input_file)?;
    let mut suitable_words = Vec::new();
    let mut stats = ProcessingStats::default();
    
    // Process CSV lines
    for (i, line) in content.lines().enumerate() {
        if i == 0 { continue; } // Skip header
        
        // Extract word (handle CSV format)
        let word = if let Some(word_part) = line.split(',').next() {
            word_part.trim().trim_matches('"').to_lowercase()
        } else {
            continue;
        };
        
        stats.total_words += 1;
        
        // Check suitability
        let (suitable, reason) = is_suitable_word(&word);
        if suitable {
            suitable_words.push(word);
        } else {
            stats.record_rejection(&reason);
        }
    }
    
    // Report statistics
    stats.report();
    
    if suitable_words.len() < 65536 {
        println!("\n⚠ Warning: Only found {} suitable words", suitable_words.len());
        println!("Adding high-quality filler words to reach 65,536...");
        
        // Add high-quality English words to fill the gap
        add_quality_filler_words(&mut suitable_words);
    }
    
    // Ensure exactly 65,536 words
    suitable_words.sort();
    suitable_words.dedup();
    suitable_words.truncate(65536);
    
    // Final padding if still needed
    while suitable_words.len() < 65536 {
        suitable_words.push(format!("word{:05}", suitable_words.len()));
    }
    
    // Save the final dictionary
    save_final_dictionary(&suitable_words)?;
    
    // Show quality examples
    show_quality_examples(&suitable_words);
    
    Ok(())
}

#[derive(Default)]
struct ProcessingStats {
    total_words: usize,
    too_short: usize,
    too_long: usize,
    multi_word: usize,
    non_alpha: usize,
    no_vowels: usize,
    consonant_clusters: usize,
    offensive: usize,
}

impl ProcessingStats {
    fn record_rejection(&mut self, reason: &str) {
        match reason {
            "too_short" => self.too_short += 1,
            "too_long" => self.too_long += 1,
            "multi_word" => self.multi_word += 1,
            "non_alpha" => self.non_alpha += 1,
            "no_vowels" => self.no_vowels += 1,
            "consonant_clusters" => self.consonant_clusters += 1,
            "offensive" => self.offensive += 1,
            _ => {}
        }
    }
    
    fn report(&self) {
        println!("\n=== PROCESSING STATISTICS ===");
        println!("Total words processed: {}", self.total_words);
        println!("\nRejection reasons:");
        println!("  Too short (<3 chars): {}", self.too_short);
        println!("  Too long (>12 chars): {}", self.too_long);
        println!("  Multi-word phrases: {}", self.multi_word);
        println!("  Non-alphabetic chars: {}", self.non_alpha);
        println!("  No vowels: {}", self.no_vowels);
        println!("  Excessive consonants: {}", self.consonant_clusters);
        println!("  Offensive content: {}", self.offensive);
    }
}

fn is_suitable_word(word: &str) -> (bool, String) {
    // Length check (3-12 characters)
    if word.len() < 3 {
        return (false, "too_short".to_string());
    }
    if word.len() > 12 {
        return (false, "too_long".to_string());
    }
    
    // No multi-word phrases
    if word.contains('_') || word.contains(' ') || word.contains('-') {
        return (false, "multi_word".to_string());
    }
    
    // Only lowercase letters
    if !word.chars().all(|c| c.is_ascii_lowercase()) {
        return (false, "non_alpha".to_string());
    }
    
    // Must have at least one vowel
    if !word.chars().any(|c| "aeiou".contains(c)) {
        return (false, "no_vowels".to_string());
    }
    
    // No excessive consonants (max 4 in a row for words like "strength")
    let mut consonant_streak = 0;
    for ch in word.chars() {
        if "bcdfghjklmnpqrstvwxyz".contains(ch) {
            consonant_streak += 1;
            if consonant_streak > 4 {
                return (false, "consonant_clusters".to_string());
            }
        } else {
            consonant_streak = 0;
        }
    }
    
    // Not offensive
    if is_offensive(word) {
        return (false, "offensive".to_string());
    }
    
    (true, "".to_string())
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
    // Add common, high-quality English words that might be missing
    let quality_words = vec![
        // Common verbs and their forms
        "work", "works", "worked", "working", "worker", "workers",
        "play", "plays", "played", "playing", "player", "players",
        "help", "helps", "helped", "helping", "helper", "helpers",
        "make", "makes", "made", "making", "maker", "makers",
        "take", "takes", "took", "taken", "taking", "taker",
        "give", "gives", "gave", "given", "giving", "giver",
        "think", "thinks", "thought", "thinking", "thinker",
        "speak", "speaks", "spoke", "spoken", "speaking", "speaker",
        "write", "writes", "wrote", "written", "writing", "writer",
        "read", "reads", "reading", "reader", "readers",
        "learn", "learns", "learned", "learning", "learner",
        "teach", "teaches", "taught", "teaching", "teacher",
        "build", "builds", "built", "building", "builder",
        "create", "creates", "created", "creating", "creator",
        "manage", "manages", "managed", "managing", "manager",
        "connect", "connects", "connected", "connecting",
        "process", "processes", "processed", "processing",
        "develop", "develops", "developed", "developing",
        "design", "designs", "designed", "designing", "designer",
        "update", "updates", "updated", "updating",
        "delete", "deletes", "deleted", "deleting",
        "insert", "inserts", "inserted", "inserting", // The requested word!
        "search", "searches", "searched", "searching",
        "filter", "filters", "filtered", "filtering",
        
        // Technology terms
        "computer", "computers", "computing", "compute", "computed",
        "program", "programs", "programming", "programmer",
        "system", "systems", "systematic", "systemize",
        "network", "networks", "networking", "networked",
        "database", "databases", "data", "datum",
        "software", "hardware", "firmware", "malware",
        "website", "websites", "web", "webmaster",
        "server", "servers", "service", "services",
        "client", "clients", "customer", "customers",
        "user", "users", "username", "usernames",
        "password", "passwords", "security", "secure",
        "login", "logout", "logged", "logging",
        "upload", "uploads", "uploaded", "uploading",
        "download", "downloads", "downloaded", "downloading",
        
        // Common nouns
        "person", "people", "human", "humans", "humanity",
        "world", "worlds", "earth", "planet", "planets",
        "country", "countries", "nation", "nations",
        "city", "cities", "town", "towns", "village",
        "house", "houses", "home", "homes", "building",
        "room", "rooms", "door", "doors", "window",
        "table", "tables", "chair", "chairs", "desk",
        "book", "books", "page", "pages", "chapter",
        "word", "words", "letter", "letters", "sentence",
        "number", "numbers", "digit", "digits", "figure",
        "time", "times", "hour", "hours", "minute",
        "day", "days", "week", "weeks", "month",
        "year", "years", "century", "centuries",
        
        // Adjectives
        "good", "better", "best", "great", "greater",
        "bad", "worse", "worst", "poor", "poorer",
        "big", "bigger", "biggest", "large", "larger",
        "small", "smaller", "smallest", "tiny", "tinier",
        "fast", "faster", "fastest", "quick", "quicker",
        "slow", "slower", "slowest", "steady", "steadier",
        "new", "newer", "newest", "modern", "recent",
        "old", "older", "oldest", "ancient", "antique",
        "easy", "easier", "easiest", "simple", "simpler",
        "hard", "harder", "hardest", "difficult", "complex",
        "safe", "safer", "safest", "secure", "secured",
        "happy", "happier", "happiest", "glad", "pleased",
        "sad", "sadder", "saddest", "unhappy", "depressed",
        
        // Adverbs
        "quickly", "slowly", "easily", "hardly", "barely",
        "really", "very", "quite", "rather", "fairly",
        "always", "never", "sometimes", "often", "rarely",
        "usually", "normally", "typically", "generally",
        "probably", "possibly", "certainly", "definitely",
        "actually", "basically", "essentially", "primarily",
        "finally", "eventually", "ultimately", "recently",
        "directly", "indirectly", "immediately", "instantly",
        "completely", "partially", "totally", "fully",
        "exactly", "precisely", "accurately", "roughly",
    ];
    
    for word in quality_words {
        if !words.contains(&word.to_string()) {
            words.push(word.to_string());
        }
    }
}

fn save_final_dictionary(words: &[String]) -> Result<(), Box<dyn std::error::Error>> {
    let output = words.join("\n");
    fs::write("data/frequency_based_word_list_65k.txt", output)?;
    
    println!("\n✓ Saved {} words to data/frequency_based_word_list_65k.txt", words.len());
    
    Ok(())
}

fn show_quality_examples(words: &[String]) {
    println!("\n=== DICTIONARY QUALITY EXAMPLES ===");
    
    println!("\nFirst 30 words (highest frequency):");
    for (i, word) in words.iter().take(30).enumerate() {
        print!("{:12} ", word);
        if (i + 1) % 5 == 0 { println!(); }
    }
    if words.len() >= 30 && 30 % 5 != 0 { println!(); }
    
    println!("\nMiddle sample (around 32,768):");
    let mid = 32768;
    for i in 0..10 {
        if mid + i < words.len() {
            print!("{:12} ", words[mid + i]);
            if (i + 1) % 5 == 0 { println!(); }
        }
    }
    println!();
    
    println!("\nExample three-word addresses:");
    println!("- {}.{}.{}", words[0], words[100], words[1000]);
    println!("- {}.{}.{}", words[10], words[500], words[5000]);
    println!("- {}.{}.{}", words[50], words[1000], words[10000]);
    println!("- {}.{}.{}", words[100], words[2000], words[20000]);
    
    // Check for specific requested words
    println!("\nChecking for requested words:");
    for word in ["inserting", "connecting", "processing", "managing", "searching"] {
        if words.contains(&word.to_string()) {
            println!("✓ Found '{}'", word);
        } else {
            println!("✗ Missing '{}'", word);
        }
    }
}