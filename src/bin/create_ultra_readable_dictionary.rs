#!/usr/bin/env rust
//! Create ultra-readable dictionary with only common, easily pronounceable English words

use std::collections::HashSet;
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Creating ultra-readable dictionary...");
    
    // Load the current improved dictionary
    let input_file = "data/improved_word_list_65k.txt";
    let content = fs::read_to_string(input_file)?;
    let words: Vec<&str> = content.lines().collect();
    
    println!("Loaded {} words from {}", words.len(), input_file);
    
    // Create ultra-aggressive filters
    let common_words = create_common_english_words();
    let readable_patterns = create_readable_patterns();
    let pronunciation_friendly = create_pronunciation_friendly_words();
    
    // Filter words aggressively
    let mut filtered_words = Vec::new();
    let mut filtered_count = 0;
    
    for word in words {
        let word = word.trim().to_lowercase();
        
        // Skip empty or very short words
        if word.len() < 3 || word.len() > 8 {
            continue;
        }
        
        // Must be in common English words OR match readable patterns OR be pronunciation friendly
        if common_words.contains(&word) || 
           readable_patterns.iter().any(|pattern| word.contains(pattern)) ||
           pronunciation_friendly.contains(&word) {
            
            // Additional quality checks
            if is_ultra_readable(&word) {
                filtered_words.push(word);
            } else {
                filtered_count += 1;
            }
        } else {
            filtered_count += 1;
        }
    }
    
    println!("Filtered out {} unreadable words", filtered_count);
    println!("Kept {} readable words", filtered_words.len());
    
    // Sort alphabetically for consistency
    filtered_words.sort();
    
    // If we have fewer than 65536 words, add more readable variants
    if filtered_words.len() < 65536 {
        let needed = 65536 - filtered_words.len();
        println!("Need {} more words, generating readable variants...", needed);
        
        let variants = generate_readable_variants(&filtered_words, needed);
        filtered_words.extend(variants);
        filtered_words.sort();
        filtered_words.dedup(); // Remove duplicates
    }
    
    // Truncate to exactly 65536 if we have too many
    filtered_words.truncate(65536);
    
    // Write the ultra-readable dictionary
    let output_file = "data/ultra_readable_word_list_65k.txt";
    let output = filtered_words.join("\n");
    fs::write(output_file, output)?;
    
    println!("Created ultra-readable dictionary with {} words in {}", filtered_words.len(), output_file);
    
    // Show first 20 words as examples
    println!("\nFirst 20 words:");
    for (i, word) in filtered_words.iter().take(20).enumerate() {
        println!("  {}: {}", i, word);
    }
    
    Ok(())
}

fn create_common_english_words() -> HashSet<String> {
    // Most common, easily readable English words
    let common = [
        // Basic words everyone knows
        "the", "and", "you", "that", "was", "for", "are", "with", "his", "they",
        "all", "any", "can", "had", "her", "has", "one", "our", "out", "day",
        "get", "use", "man", "new", "now", "way", "may", "say", "each", "which",
        "she", "how", "its", "who", "oil", "sit", "but", "not", "what", "all",
        
        // Simple nouns
        "cat", "dog", "car", "house", "book", "tree", "water", "fire", "earth", "air",
        "sun", "moon", "star", "sky", "sea", "land", "rock", "hill", "river", "lake",
        "hand", "foot", "head", "eye", "ear", "arm", "leg", "back", "face", "hair",
        "door", "window", "table", "chair", "bed", "room", "wall", "floor", "roof",
        "bread", "milk", "meat", "fish", "apple", "orange", "banana", "grape",
        
        // Simple verbs
        "run", "walk", "jump", "sit", "stand", "look", "see", "hear", "feel", "think",
        "know", "give", "take", "make", "come", "go", "eat", "drink", "sleep", "wake",
        "love", "like", "want", "need", "help", "work", "play", "stop", "start", "end",
        "open", "close", "read", "write", "draw", "sing", "dance", "laugh", "cry",
        
        // Simple adjectives
        "big", "small", "good", "bad", "hot", "cold", "fast", "slow", "high", "low",
        "old", "new", "young", "long", "short", "wide", "narrow", "thick", "thin",
        "light", "dark", "red", "blue", "green", "yellow", "black", "white", "brown",
        "happy", "sad", "angry", "calm", "quiet", "loud", "soft", "hard", "easy", "hard",
        
        // Numbers and time
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "ten",
        "first", "last", "next", "time", "day", "week", "month", "year", "hour", "minute",
        "today", "tomorrow", "yesterday", "morning", "evening", "night", "spring", "summer",
        
        // Family and people
        "family", "mother", "father", "parent", "child", "baby", "boy", "girl", "man", "woman",
        "friend", "person", "people", "group", "team", "class", "student", "teacher",
        
        // Common actions
        "move", "turn", "push", "pull", "lift", "drop", "throw", "catch", "hold", "carry",
        "build", "break", "fix", "clean", "wash", "cook", "serve", "order", "buy", "sell",
        
        // Places
        "home", "school", "work", "store", "park", "city", "town", "country", "world",
        "place", "area", "space", "point", "line", "side", "top", "bottom", "front", "back",
        
        // Weather and nature
        "rain", "snow", "wind", "cloud", "storm", "sunny", "cloudy", "windy", "rainy",
        "flower", "grass", "leaf", "branch", "root", "seed", "plant", "garden",
        
        // Technology (simple)
        "phone", "computer", "screen", "button", "switch", "light", "power", "energy",
        "machine", "tool", "key", "lock", "box", "bag", "cup", "plate", "knife", "fork",
        
        // Transportation
        "bike", "train", "bus", "plane", "boat", "ship", "road", "path", "bridge", "tunnel",
        "drive", "ride", "fly", "sail", "travel", "trip", "journey", "visit", "arrive", "leave",
        
        // Emotions and states
        "feel", "emotion", "mood", "hope", "fear", "worry", "care", "trust", "believe",
        "remember", "forget", "learn", "teach", "understand", "explain", "answer", "question",
        
        // Common suffixed words
        "running", "walking", "talking", "working", "playing", "looking", "thinking", "feeling",
        "making", "taking", "giving", "coming", "going", "moving", "turning", "trying",
        "books", "cars", "dogs", "cats", "trees", "houses", "rooms", "doors", "windows",
        "hands", "feet", "eyes", "ears", "arms", "legs", "heads", "faces",
        "player", "worker", "teacher", "driver", "writer", "reader", "speaker", "helper",
        "bigger", "smaller", "better", "faster", "slower", "older", "newer", "longer", "shorter",
    ];
    
    common.iter().map(|&s| s.to_string()).collect()
}

fn create_readable_patterns() -> Vec<String> {
    // Word patterns that are generally readable
    vec![
        "walk".to_string(), "talk".to_string(), "work".to_string(), "play".to_string(),
        "look".to_string(), "book".to_string(), "cook".to_string(), "took".to_string(),
        "make".to_string(), "take".to_string(), "wake".to_string(), "lake".to_string(),
        "like".to_string(), "bike".to_string(), "hike".to_string(), "mike".to_string(),
        "home".to_string(), "some".to_string(), "come".to_string(), "dome".to_string(),
        "time".to_string(), "lime".to_string(), "dime".to_string(), "mime".to_string(),
    ]
}

fn create_pronunciation_friendly_words() -> HashSet<String> {
    // Words that are easy to pronounce and common in English
    let friendly = [
        "able", "about", "above", "across", "after", "again", "against", "age", "ago",
        "air", "all", "almost", "alone", "along", "also", "always", "among", "and",
        "animal", "another", "answer", "any", "appear", "area", "around", "ask",
        "back", "ball", "base", "be", "bear", "beat", "beautiful", "became", "because",
        "become", "bed", "been", "before", "began", "begin", "being", "believe", "below",
        "best", "better", "between", "big", "black", "blue", "boat", "body", "book",
        "both", "box", "boy", "bring", "brought", "build", "built", "business", "but",
        "call", "came", "can", "car", "care", "carry", "case", "catch", "cause",
        "change", "check", "child", "children", "city", "class", "clean", "clear",
        "close", "cold", "color", "come", "common", "complete", "could", "country",
        "course", "cut", "dark", "data", "day", "decide", "deep", "develop", "did",
        "die", "different", "do", "does", "dog", "done", "door", "down", "draw",
        "drive", "during", "each", "early", "earth", "easy", "eat", "education",
        "end", "energy", "enough", "enter", "even", "ever", "every", "example",
        "eye", "face", "fact", "fall", "family", "far", "fast", "father", "feel",
        "few", "field", "fight", "fill", "find", "fine", "fire", "first", "fish",
        "five", "fly", "follow", "food", "foot", "for", "force", "form", "found",
        "four", "free", "friend", "from", "full", "game", "gave", "get", "girl",
        "give", "go", "goal", "god", "good", "got", "great", "green", "ground",
        "group", "grow", "had", "half", "hand", "happy", "hard", "has", "have",
        "he", "head", "hear", "heart", "heat", "heavy", "held", "help", "her",
        "here", "high", "him", "his", "hit", "hold", "home", "hope", "hot",
        "hour", "house", "how", "human", "hundred", "idea", "if", "image", "in",
        "include", "into", "is", "it", "its", "job", "join", "just", "keep",
        "kind", "know", "land", "large", "last", "late", "law", "lay", "lead",
        "learn", "least", "leave", "left", "let", "letter", "level", "life",
        "light", "like", "line", "list", "little", "live", "local", "long",
        "look", "lot", "love", "low", "machine", "made", "make", "man", "many",
        "may", "me", "mean", "meet", "member", "men", "might", "mind", "minute",
        "miss", "money", "month", "more", "morning", "most", "mother", "move",
        "much", "music", "must", "my", "name", "nature", "near", "need", "never",
        "new", "news", "next", "night", "no", "not", "note", "nothing", "now",
        "number", "of", "off", "often", "oil", "old", "on", "once", "one",
        "only", "open", "or", "order", "other", "our", "out", "over", "own",
        "page", "part", "pay", "people", "person", "picture", "piece", "place",
        "plan", "plant", "play", "point", "power", "present", "problem", "program",
        "provide", "public", "put", "question", "quite", "rather", "read", "real",
        "really", "reason", "receive", "red", "remember", "report", "result", "right",
        "river", "road", "rock", "room", "run", "said", "same", "saw", "say",
        "school", "science", "sea", "second", "see", "seem", "sell", "send",
        "sense", "sent", "serve", "service", "set", "several", "she", "short",
        "should", "show", "side", "simple", "since", "sing", "sit", "six",
        "size", "small", "so", "social", "some", "something", "song", "soon",
        "sort", "sound", "speak", "special", "start", "state", "still", "stop",
        "story", "street", "strong", "study", "such", "sun", "sure", "system",
        "table", "take", "talk", "teach", "team", "tell", "ten", "than",
        "that", "the", "their", "them", "then", "there", "these", "they",
        "thing", "think", "this", "those", "though", "thought", "three", "through",
        "time", "to", "today", "together", "told", "too", "took", "top",
        "total", "town", "tree", "true", "try", "turn", "two", "type",
        "under", "until", "up", "upon", "us", "use", "used", "using",
        "very", "voice", "walk", "want", "war", "warm", "was", "watch",
        "water", "way", "we", "week", "well", "went", "were", "what",
        "when", "where", "which", "while", "white", "who", "why", "will",
        "win", "with", "within", "without", "woman", "word", "work", "world",
        "would", "write", "year", "yes", "yet", "you", "young", "your",
    ];
    
    friendly.iter().map(|&s| s.to_string()).collect()
}

fn is_ultra_readable(word: &str) -> bool {
    // Ultra-strict readability checks
    
    // Must be 3-8 characters
    if word.len() < 3 || word.len() > 8 {
        return false;
    }
    
    // Must contain only lowercase letters
    if !word.chars().all(|c| c.is_ascii_lowercase()) {
        return false;
    }
    
    // Must have at least one vowel
    if !word.chars().any(|c| "aeiou".contains(c)) {
        return false;
    }
    
    // Cannot have too many consonants in a row
    let mut consonant_streak = 0;
    for ch in word.chars() {
        if "bcdfghjklmnpqrstvwxyz".contains(ch) {
            consonant_streak += 1;
            if consonant_streak > 3 {
                return false;
            }
        } else {
            consonant_streak = 0;
        }
    }
    
    // Cannot have repeating patterns that are hard to say
    if word.contains("ck") && word.len() < 5 {
        return false; // "ck" words should be longer
    }
    
    // Filter out abbreviations and acronyms (no uppercase, no periods)
    if word.contains('.') || word.chars().any(|c| c.is_uppercase()) {
        return false;
    }
    
    // Filter out words with difficult letter combinations
    let difficult_patterns = ["xz", "qx", "xq", "zx", "qz", "zq", "zzz", "xxx"];
    for pattern in &difficult_patterns {
        if word.contains(pattern) {
            return false;
        }
    }
    
    // Must not be obviously foreign (basic check)
    let foreign_endings = ["ski", "ich", "ung", "sch", "tch"];
    for ending in &foreign_endings {
        if word.ends_with(ending) && word.len() < 6 {
            return false; // Short foreign words are harder
        }
    }
    
    // Filter out single-letter + common endings that sound awkward
    if word.len() == 3 && (word.ends_with("ly") || word.ends_with("er") || word.ends_with("ed")) {
        return false;
    }
    
    true
}

fn generate_readable_variants(base_words: &[String], needed: usize) -> Vec<String> {
    let mut variants = Vec::new();
    let mut added = 0;
    
    // Common suffixes that create readable variants
    let suffixes = ["s", "ed", "ing", "er", "ly", "est"];
    
    for word in base_words {
        if added >= needed {
            break;
        }
        
        for suffix in &suffixes {
            if added >= needed {
                break;
            }
            
            let variant = format!("{}{}", word, suffix);
            
            // Check if variant is readable and not too long
            if variant.len() <= 8 && is_ultra_readable(&variant) && !base_words.contains(&variant) {
                variants.push(variant);
                added += 1;
            }
        }
    }
    
    // If we still need more, add simple number variants
    if added < needed {
        let base_nums = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "ten"];
        for num in &base_nums {
            if added >= needed {
                break;
            }
            if !base_words.contains(&num.to_string()) {
                variants.push(num.to_string());
                added += 1;
            }
        }
    }
    
    variants
}