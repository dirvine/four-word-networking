use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔄 REORDERING DICTIONARY FOR BETTER QUALITY");
    println!("===========================================");
    
    // Read the current dictionary
    let wordlist_content = fs::read_to_string("data/wordlist_16384.txt")?;
    let all_words: Vec<&str> = wordlist_content.lines().collect();
    
    println!("Original dictionary size: {} words", all_words.len());
    
    // Separate words by quality
    let mut high_quality = Vec::new();
    let mut medium_quality = Vec::new();
    let mut low_quality = Vec::new();
    
    for word in &all_words {
        let quality = assess_word_quality(word);
        match quality {
            WordQuality::High => high_quality.push(*word),
            WordQuality::Medium => medium_quality.push(*word),
            WordQuality::Low => low_quality.push(*word),
        }
    }
    
    println!("High quality words: {}", high_quality.len());
    println!("Medium quality words: {}", medium_quality.len());
    println!("Low quality words: {}", low_quality.len());
    
    // Show samples
    println!("\nSample high quality words:");
    for word in high_quality.iter().take(20) {
        println!("  {}", word);
    }
    
    println!("\nSample low quality words:");
    for word in low_quality.iter().take(20) {
        println!("  {}", word);
    }
    
    // Reorder: put high quality words first, then medium, then low
    let mut reordered_words: Vec<&str> = Vec::new();
    reordered_words.extend(&high_quality);
    reordered_words.extend(&medium_quality);
    reordered_words.extend(&low_quality);
    
    // Ensure we have exactly 16384 words
    if reordered_words.len() != 16384 {
        println!("⚠️  Warning: Reordered dictionary has {} words, expected 16384", reordered_words.len());
        return Err("Dictionary size mismatch".into());
    }
    
    // Write reordered dictionary
    let reordered_content = reordered_words.join("\n");
    fs::write("data/wordlist_16384_reordered.txt", reordered_content)?;
    
    println!("\n✅ Created reordered dictionary: data/wordlist_16384_reordered.txt");
    println!("   High quality words are now at indices 0-{}", high_quality.len() - 1);
    println!("   Medium quality words at indices {}-{}", high_quality.len(), high_quality.len() + medium_quality.len() - 1);
    println!("   Low quality words at indices {}-16383", high_quality.len() + medium_quality.len());
    
    // This should significantly improve the user experience since our encoding
    // tends to generate values that map to lower indices
    
    Ok(())
}

#[derive(Debug, PartialEq)]
enum WordQuality {
    High,    // Common, proper English words
    Medium,  // Less common but valid English words
    Low,     // Abbreviations, codes, uncommon words
}

fn assess_word_quality(word: &str) -> WordQuality {
    // Very high quality words - common English words everyone knows
    let high_quality_words = [
        // Common nouns
        "house", "water", "light", "world", "state", "place", "people", "family", 
        "school", "student", "computer", "internet", "network", "system", "process", 
        "program", "service", "company", "business", "market", "money", "value", 
        "person", "human", "friend", "group", "community", "country", "health", 
        "education", "culture", "science", "technology", "design", "create", "help",
        "support", "improve", "develop", "change", "control", "secure", "protect",
        "safety", "quality", "success", "goal", "strategy", "plan", "project",
        "activity", "event", "meeting", "course", "class", "lesson", "guide",
        "book", "study", "analysis", "review", "test", "question", "answer",
        "result", "data", "information", "knowledge", "learning", "working",
        "building", "manage", "provide", "increase", "solution", "problem",
        "opportunity", "experience", "skill", "ability", "talent", "account",
        "identity", "access", "security", "privacy", "protection", "backup",
        "operation", "management", "communication", "support", "assistance",
        "guidance", "advice", "request", "feedback", "comment", "review",
        "opinion", "view", "approach", "method", "reason", "cause", "effect",
        "impact", "benefit", "advantage", "importance", "meaning",
        
        // Common adjectives
        "happy", "great", "small", "good", "new", "old", "big", "little", "long",
        "short", "high", "low", "fast", "slow", "easy", "hard", "simple", "complex",
        "basic", "advanced", "modern", "traditional", "popular", "common", "rare",
        "special", "normal", "different", "same", "important", "serious", "funny",
        "interesting", "boring", "exciting", "amazing", "wonderful", "terrible",
        "beautiful", "ugly", "clean", "dirty", "fresh", "old", "young", "strong",
        "weak", "healthy", "sick", "safe", "dangerous", "public", "private",
        "local", "global", "national", "international", "natural", "digital",
        "online", "mobile", "social", "economic", "political", "environmental",
        
        // Common verbs  
        "think", "know", "want", "need", "like", "love", "hate", "see", "hear",
        "feel", "touch", "taste", "smell", "look", "watch", "listen", "speak",
        "talk", "say", "tell", "ask", "answer", "call", "write", "read", "learn",
        "teach", "study", "work", "play", "run", "walk", "drive", "travel", "visit",
        "live", "stay", "come", "leave", "start", "stop", "finish", "continue",
        "begin", "end", "open", "close", "turn", "move", "take", "give", "get",
        "put", "make", "do", "have", "use", "try", "help", "support", "protect",
        "save", "buy", "sell", "pay", "cost", "spend", "earn", "win", "lose",
        "find", "search", "choose", "decide", "plan", "prepare", "organize",
        "manage", "control", "lead", "follow", "join", "leave", "enter", "exit",
        
        // Time and numbers
        "time", "day", "week", "month", "year", "hour", "minute", "second",
        "morning", "afternoon", "evening", "night", "today", "tomorrow", "yesterday",
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "ten",
        "first", "second", "third", "last", "next", "before", "after", "during",
        
        // Basic concepts
        "life", "death", "birth", "growth", "change", "movement", "action", "rest",
        "peace", "war", "love", "hate", "hope", "fear", "joy", "sadness", "anger",
        "surprise", "trust", "doubt", "truth", "lie", "right", "wrong", "good", "bad"
    ];
    
    if high_quality_words.contains(&word) {
        return WordQuality::High;
    }
    
    // Automatically reject very short or very long words
    if word.len() < 3 || word.len() > 15 {
        return WordQuality::Low;
    }
    
    // Reject words with non-alphabetic characters
    if !word.chars().all(|c| c.is_ascii_alphabetic()) {
        return WordQuality::Low;
    }
    
    // Reject known bad words (abbreviations, codes, etc.)
    let low_quality_words = [
        "aim", "bok", "cod", "dab", "duh", "duo", "eel", "emu", "fax", "gab", "gag",
        "keg", "nag", "oaf", "pox", "pry", "pug", "rut", "wad", "wok", "yam", "yen",
        "yin", "zap", "zen", "zit", "pep", "pod", "tux", "ahoy", "bvt", "bal", "zoa",
        "zea", "wahl", "yon", "foe", "pep", "wow", "huh", "hmm", "err", "umm", "ugh"
    ];
    
    if low_quality_words.contains(&word) {
        return WordQuality::Low;
    }
    
    // Reject words with repeated letter patterns (often codes/abbreviations)
    if word.len() <= 4 && has_repeated_patterns(word) {
        return WordQuality::Low;
    }
    
    // Longer words (5+ chars) that passed basic filters are likely medium quality
    if word.len() >= 5 {
        return WordQuality::Medium;
    }
    
    // 3-4 character words that passed filters are likely medium quality
    WordQuality::Medium
}

fn has_repeated_patterns(word: &str) -> bool {
    let chars: Vec<char> = word.chars().collect();
    
    // Check for adjacent repeated characters
    for i in 0..chars.len()-1 {
        if chars[i] == chars[i+1] {
            return true;
        }
    }
    
    // Check for alternating patterns like "aba" or "abab"
    if chars.len() >= 3 {
        if chars[0] == chars[2] {
            return true;
        }
    }
    
    false
}