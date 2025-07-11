#!/usr/bin/env rust
//! Create a dictionary based on everyday English word frequency

use std::fs;
use std::collections::HashSet;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Creating everyday frequency-based dictionary...\n");
    
    let mut words = Vec::new();
    
    // Start with the most common everyday English words (not Wikipedia-specific)
    // Based on multiple corpus studies of spoken and written English
    words.extend([
        // Top 100 most common words in everyday English
        "the", "be", "to", "of", "and", "in", "he", "have", "it", "that",
        "for", "they", "with", "as", "not", "on", "she", "at", "by", "this",
        "we", "you", "do", "but", "from", "or", "which", "one", "would", "all",
        "will", "there", "say", "who", "make", "when", "can", "more", "if", "no",
        "man", "out", "other", "so", "what", "time", "up", "go", "about", "than",
        "into", "could", "state", "only", "new", "year", "some", "take", "come", "these",
        "know", "see", "use", "her", "get", "like", "then", "first", "any", "work",
        "now", "may", "such", "give", "over", "think", "most", "even", "find", "day",
        "also", "after", "way", "many", "must", "look", "before", "great", "back", "through",
        "long", "where", "much", "should", "well", "people", "down", "own", "just", "because",
        
        // Common everyday nouns
        "person", "man", "woman", "child", "hand", "eye", "head", "face", "arm", "leg",
        "heart", "mind", "body", "life", "death", "name", "word", "thing", "place", "home",
        "house", "room", "door", "window", "wall", "floor", "roof", "street", "road", "car",
        "bus", "train", "plane", "boat", "bike", "walk", "run", "food", "water", "bread",
        "milk", "meat", "fish", "fruit", "apple", "orange", "tree", "flower", "grass", "sky",
        "sun", "moon", "star", "cloud", "rain", "snow", "wind", "fire", "ice", "hot",
        "cold", "warm", "cool", "light", "dark", "night", "morning", "afternoon", "evening", "week",
        "month", "year", "hour", "minute", "second", "today", "tomorrow", "yesterday", "now", "then",
        "here", "there", "everywhere", "nowhere", "somewhere", "always", "never", "sometimes", "often", "seldom",
        
        // Common verbs - base forms
        "go", "come", "run", "walk", "sit", "stand", "lie", "eat", "drink", "sleep",
        "wake", "read", "write", "speak", "listen", "hear", "see", "look", "watch", "feel",
        "touch", "smell", "taste", "think", "know", "understand", "remember", "forget", "learn", "teach",
        "work", "play", "rest", "start", "stop", "begin", "end", "open", "close", "turn",
        "move", "stay", "wait", "follow", "lead", "help", "save", "kill", "die", "live",
        "love", "hate", "like", "want", "need", "have", "take", "give", "get", "make",
        "do", "say", "tell", "ask", "answer", "call", "cry", "laugh", "smile", "sing",
        
        // Include 2-letter words for better coverage
        "am", "is", "are", "was", "were", "be", "been", "being", "do", "did",
        "an", "as", "at", "by", "he", "if", "in", "is", "it", "me",
        "my", "no", "of", "on", "or", "so", "to", "up", "us", "we",
        
        // Common adjectives
        "good", "bad", "big", "small", "long", "short", "high", "low", "fast", "slow",
        "hot", "cold", "new", "old", "young", "first", "last", "great", "little", "own",
        "other", "same", "different", "early", "late", "hard", "easy", "near", "far", "right",
        "wrong", "left", "top", "bottom", "whole", "half", "all", "some", "many", "few",
        "more", "less", "most", "least", "very", "quite", "just", "only", "even", "still",
        "such", "real", "best", "better", "worse", "worst", "fine", "nice", "pretty", "beautiful",
        
        // Natural word forms with -ing (as requested)
        "being", "having", "doing", "saying", "going", "getting", "making", "taking", "coming", "looking",
        "using", "finding", "giving", "telling", "working", "calling", "trying", "asking", "feeling", "becoming",
        "leaving", "putting", "bringing", "beginning", "keeping", "thinking", "helping", "talking", "turning", "starting",
        "showing", "hearing", "playing", "running", "moving", "living", "believing", "holding", "bringing", "happening",
        "writing", "sitting", "losing", "paying", "meeting", "including", "continuing", "setting", "learning", "changing",
        "leading", "understanding", "watching", "following", "stopping", "speaking", "growing", "opening", "walking", "winning",
        "teaching", "offering", "remembering", "considering", "appearing", "buying", "serving", "dying", "sending", "expecting",
        "building", "staying", "falling", "cutting", "reaching", "killing", "remaining", "suggesting", "raising", "passing",
        "selling", "requiring", "reporting", "deciding", "pulling", "returning", "carrying", "breaking", "hoping", "developing",
        "driving", "dealing", "reading", "saving", "standing", "providing", "adding", "agreeing", "supporting", "hitting",
        "producing", "eating", "covering", "catching", "drawing", "choosing", "creating", "wanting", "planning", "wondering",
        "pulling", "offering", "dropping", "receiving", "joining", "lying", "representing", "accepting", "containing", "bearing",
        
        // Specifically requested: inserting, connecting, processing, managing, searching
        "inserting", "connecting", "processing", "managing", "searching", "installing", "debugging", "programming", "computing", "networking",
        
        // Past tense forms
        "was", "were", "had", "did", "got", "made", "went", "took", "came", "saw",
        "gave", "found", "thought", "told", "became", "left", "felt", "brought", "said", "used",
        "moved", "started", "turned", "wanted", "looked", "asked", "worked", "needed", "tried", "called",
        "walked", "talked", "played", "lived", "died", "sat", "stood", "lost", "paid", "met",
        "ran", "brought", "wrote", "won", "showed", "heard", "let", "meant", "kept", "began",
        "seemed", "helped", "talked", "turned", "started", "showed", "heard", "played", "ran", "moved",
        
        // Plural forms
        "people", "things", "years", "days", "times", "ways", "hands", "eyes", "words", "places",
        "homes", "houses", "rooms", "doors", "windows", "walls", "streets", "roads", "cars", "buses",
        "trains", "planes", "boats", "bikes", "foods", "waters", "trees", "flowers", "clouds", "stars",
        "hours", "minutes", "seconds", "weeks", "months", "children", "men", "women", "lives", "names",
        
        // Numbers and time
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "ten",
        "eleven", "twelve", "thirteen", "fourteen", "fifteen", "sixteen", "seventeen", "eighteen", "nineteen", "twenty",
        "thirty", "forty", "fifty", "sixty", "seventy", "eighty", "ninety", "hundred", "thousand", "million",
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth", "tenth",
        "once", "twice", "single", "double", "triple", "half", "quarter", "whole", "all", "none",
        
        // Technology words that normal people actually use
        "phone", "computer", "internet", "email", "website", "password", "click", "type", "save", "delete",
        "copy", "paste", "file", "folder", "screen", "keyboard", "mouse", "printer", "wifi", "online",
        "download", "upload", "search", "google", "facebook", "twitter", "instagram", "youtube", "amazon", "netflix",
        "app", "game", "video", "photo", "music", "movie", "show", "text", "call", "message",
        "send", "receive", "share", "post", "comment", "like", "follow", "friend", "block", "mute",
        
        // Common expressions and everyday words
        "hello", "hi", "bye", "goodbye", "yes", "no", "ok", "okay", "maybe", "please",
        "thanks", "sorry", "excuse", "pardon", "welcome", "sure", "right", "wrong", "true", "false",
        "real", "fake", "same", "different", "other", "another", "each", "every", "any", "some",
        "all", "none", "both", "either", "neither", "many", "few", "several", "various", "certain",
        
        // Family and relationships
        "family", "mother", "father", "parent", "child", "son", "daughter", "brother", "sister", "baby",
        "husband", "wife", "friend", "people", "person", "human", "man", "woman", "boy", "girl",
        "aunt", "uncle", "cousin", "grandma", "grandpa", "mom", "dad", "kid", "teen", "adult",
        
        // Actions people do every day
        "wake", "sleep", "eat", "drink", "cook", "clean", "wash", "shower", "dress", "drive",
        "ride", "shop", "buy", "pay", "cost", "spend", "save", "earn", "owe", "lend",
        "borrow", "return", "keep", "lose", "find", "search", "hide", "show", "teach", "learn",
        "study", "read", "write", "draw", "paint", "sing", "dance", "play", "watch", "listen",
        
        // Emotions and feelings
        "happy", "sad", "angry", "scared", "worried", "excited", "bored", "tired", "sick", "healthy",
        "hungry", "thirsty", "full", "empty", "strong", "weak", "brave", "afraid", "proud", "shy",
        "love", "hate", "like", "dislike", "want", "need", "hope", "wish", "dream", "fear",
        
        // Colors
        "red", "blue", "green", "yellow", "orange", "purple", "pink", "black", "white", "gray",
        "brown", "gold", "silver", "dark", "light", "bright", "pale", "deep", "soft", "hard",
        
        // Sizes and measurements
        "big", "small", "large", "tiny", "huge", "little", "tall", "short", "long", "wide",
        "narrow", "thick", "thin", "heavy", "light", "full", "empty", "deep", "shallow", "high",
        
        // Weather and nature
        "sun", "moon", "star", "sky", "cloud", "rain", "snow", "wind", "storm", "weather",
        "hot", "cold", "warm", "cool", "wet", "dry", "sunny", "cloudy", "rainy", "snowy",
        "tree", "flower", "grass", "leaf", "root", "seed", "plant", "garden", "forest", "field",
        "mountain", "hill", "valley", "river", "lake", "sea", "ocean", "beach", "island", "desert",
        
        // Animals
        "dog", "cat", "bird", "fish", "horse", "cow", "pig", "sheep", "chicken", "duck",
        "rabbit", "mouse", "rat", "bear", "lion", "tiger", "elephant", "monkey", "snake", "frog",
        
        // Body parts
        "head", "face", "eye", "ear", "nose", "mouth", "tooth", "tongue", "neck", "shoulder",
        "arm", "elbow", "hand", "finger", "thumb", "chest", "stomach", "back", "leg", "knee",
        "foot", "toe", "skin", "hair", "blood", "bone", "muscle", "brain", "heart", "lung",
        
        // Clothes
        "shirt", "pants", "dress", "skirt", "shoe", "sock", "hat", "coat", "jacket", "glove",
        "belt", "tie", "scarf", "bag", "pocket", "button", "zip", "wear", "fit", "match",
        
        // Money and shopping
        "money", "dollar", "cent", "price", "cost", "cheap", "expensive", "free", "sale", "buy",
        "sell", "pay", "cash", "card", "credit", "debit", "bill", "tip", "tax", "discount",
        "store", "shop", "mall", "market", "customer", "clerk", "manager", "owner", "business", "company"
    ].iter().map(|s| s.to_string()));
    
    // Now add suitable words from the Hugging Face dataset to fill up to 65,536
    if let Ok(csv_content) = fs::read_to_string("data/common-words-processed.csv") {
        let mut hugging_face_words = Vec::new();
        
        for (i, line) in csv_content.lines().enumerate() {
            if i == 0 { continue; }
            
            let parts: Vec<&str> = line.split(',').collect();
            if parts.len() >= 2 {
                let word = parts[0].trim().to_lowercase();
                let frequency = parts[1].trim().parse::<u64>().unwrap_or(0);
                
                if is_suitable_word(&word, true) { // Allow 2-char words
                    hugging_face_words.push((word, frequency));
                }
            }
        }
        
        // Sort by frequency
        hugging_face_words.sort_by(|a, b| b.1.cmp(&a.1));
        
        // Add them to our list
        for (word, _) in hugging_face_words {
            if !words.contains(&word) {
                words.push(word);
            }
        }
    }
    
    // Ensure exactly 65,536 unique words
    let mut seen = HashSet::new();
    words.retain(|word| seen.insert(word.clone()));
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
    fs::write("data/everyday_frequency_word_list_65k.txt", &output)?;
    
    println!("✓ Created everyday frequency dictionary with {} words", words.len());
    
    // Show examples
    println!("\nFirst 50 words (most common everyday English):");
    for (i, word) in words.iter().take(50).enumerate() {
        print!("{:8} ", word);
        if (i + 1) % 10 == 0 { println!(); }
    }
    
    println!("\n\nExample three-word addresses:");
    println!("- {}.{}.{}", words[0], words[50], words[100]);
    println!("- {}.{}.{}", words[5], words[25], words[75]);
    println!("- {}.{}.{}", words[10], words[100], words[500]);
    
    Ok(())
}

fn is_suitable_word(word: &str, allow_two_char: bool) -> bool {
    // Length check
    let min_len = if allow_two_char { 2 } else { 3 };
    if word.len() < min_len {
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
    
    // Skip offensive words
    let offensive = ["fuck", "shit", "damn", "hell", "ass", "dick", "cunt", "bitch"];
    if offensive.iter().any(|&bad| word.contains(bad)) {
        return false;
    }
    
    true
}