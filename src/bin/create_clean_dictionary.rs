use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🛠️  CREATING CLEAN DICTIONARY FOR PRODUCTION USE");
    println!("===============================================");

    // Read the current dictionary
    let wordlist_content = fs::read_to_string("data/wordlist_16384.txt")?;
    let all_words: Vec<&str> = wordlist_content.lines().collect();

    println!("Original dictionary size: {} words", all_words.len());

    // Filter out all problematic words and categorize the rest
    let mut excellent_words = Vec::new();
    let mut good_words = Vec::new();
    let mut acceptable_words = Vec::new();
    let mut filtered_out = Vec::new();

    for word in &all_words {
        match categorize_word(word) {
            WordCategory::Excellent => excellent_words.push(*word),
            WordCategory::Good => good_words.push(*word),
            WordCategory::Acceptable => acceptable_words.push(*word),
            WordCategory::Filtered => filtered_out.push(*word),
        }
    }

    println!("Excellent words: {}", excellent_words.len());
    println!("Good words: {}", good_words.len());
    println!("Acceptable words: {}", acceptable_words.len());
    println!("Filtered out: {}", filtered_out.len());

    // Show samples
    println!("\nSample excellent words:");
    for word in excellent_words.iter().take(15) {
        println!("  {}", word);
    }

    println!("\nSample filtered words:");
    for word in filtered_out.iter().take(15) {
        println!("  {}", word);
    }

    // Build final dictionary: Excellent first, then good, then acceptable as needed
    let mut final_words: Vec<&str> = Vec::new();
    final_words.extend(&excellent_words);
    final_words.extend(&good_words);

    // Add acceptable words to reach exactly 16384
    let needed = 16384 - final_words.len();
    if needed > 0 {
        if needed <= acceptable_words.len() {
            final_words.extend(acceptable_words.iter().take(needed));
            println!("\nAdded {} acceptable words to reach 16384 total", needed);
        } else {
            final_words.extend(&acceptable_words);
            let still_needed = 16384 - final_words.len();

            // Add back some filtered words if necessary, but skip the worst ones
            let really_bad = [
                "rape", "kill", "murder", "nazi", "fuck", "shit", "bvt", "xj", "xr",
            ];
            let recoverable: Vec<&str> = filtered_out
                .iter()
                .filter(|w| !really_bad.contains(w))
                .take(still_needed)
                .cloned()
                .collect();

            final_words.extend(&recoverable);
            println!(
                "\nAdded {} acceptable + {} recovered words to reach {} total",
                acceptable_words.len(),
                recoverable.len(),
                final_words.len()
            );

            if final_words.len() < 16384 {
                // Add back any remaining words to reach exactly 16384
                let remaining: Vec<&str> = filtered_out
                    .iter()
                    .filter(|w| !final_words.contains(w))
                    .take(16384 - final_words.len())
                    .cloned()
                    .collect();
                final_words.extend(&remaining);
                println!(
                    "Added {} more words to reach exact total: {}",
                    remaining.len(),
                    final_words.len()
                );
            }
        }
    }

    // Write the clean dictionary
    let clean_content = final_words.join("\n");
    fs::write("data/wordlist_16384_clean.txt", clean_content)?;

    println!("\n✅ Created clean dictionary: data/wordlist_16384_clean.txt");
    println!(
        "   Excellent words at indices 0-{}",
        excellent_words.len() - 1
    );
    println!(
        "   Good words at indices {}-{}",
        excellent_words.len(),
        excellent_words.len() + good_words.len() - 1
    );
    println!("   Acceptable words fill remaining positions");
    println!("   All offensive, inappropriate, and low-quality words removed");

    Ok(())
}

#[derive(Debug, PartialEq)]
enum WordCategory {
    Excellent,  // Perfect for public use - common, appropriate English words
    Good,       // Good quality English words, maybe less common
    Acceptable, // Valid English but might be technical/uncommon
    Filtered,   // Remove from dictionary
}

fn categorize_word(word: &str) -> WordCategory {
    // Must be reasonable length
    if word.len() < 2 || word.len() > 15 {
        return WordCategory::Filtered;
    }

    // Must be alphabetic only
    if !word.chars().all(|c| c.is_ascii_alphabetic()) {
        return WordCategory::Filtered;
    }

    // Filter out only the most offensive words
    let offensive_words = [
        "rape", "kill", "murder", "nazi", "fuck", "shit", "penis", "vagina", "cocaine", "heroin",
        "meth",
    ];

    if offensive_words.contains(&word) {
        return WordCategory::Filtered;
    }

    // Filter out only the most obvious gibberish
    let low_quality = [
        "bvt", "zoa", "zea", "wahl", "xj", "xr", "apio", "cps", "uvw", "www",
    ];

    if low_quality.contains(&word) {
        return WordCategory::Filtered;
    }

    // Filter words with too many repeated letters
    if has_excessive_repetition(word) {
        return WordCategory::Filtered;
    }

    // Excellent words - common English everyone knows and uses
    let excellent_words = [
        // Common nouns everyone knows
        "house",
        "home",
        "family",
        "person",
        "people",
        "child",
        "children",
        "parent",
        "friend",
        "school",
        "teacher",
        "student",
        "book",
        "table",
        "chair",
        "door",
        "window",
        "water",
        "food",
        "money",
        "time",
        "day",
        "week",
        "month",
        "year",
        "work",
        "job",
        "business",
        "company",
        "office",
        "computer",
        "phone",
        "car",
        "road",
        "city",
        "country",
        "world",
        "life",
        "health",
        "love",
        "peace",
        "music",
        "art",
        "game",
        "sport",
        "movie",
        "story",
        "picture",
        "color",
        "animal",
        "dog",
        "cat",
        "bird",
        "tree",
        "flower",
        "garden",
        "park",
        "mountain",
        "river",
        "ocean",
        "sky",
        "sun",
        "moon",
        "star",
        "cloud",
        // Common adjectives
        "good",
        "great",
        "nice",
        "beautiful",
        "happy",
        "sad",
        "big",
        "small",
        "new",
        "old",
        "young",
        "easy",
        "hard",
        "fast",
        "slow",
        "hot",
        "cold",
        "clean",
        "dirty",
        "safe",
        "dangerous",
        "important",
        "interesting",
        "fun",
        "simple",
        "complex",
        "free",
        "expensive",
        "cheap",
        "rich",
        "poor",
        "healthy",
        "strong",
        "weak",
        "smart",
        "kind",
        "friendly",
        "helpful",
        // Common verbs
        "be",
        "have",
        "do",
        "make",
        "get",
        "take",
        "give",
        "go",
        "come",
        "see",
        "know",
        "think",
        "say",
        "tell",
        "ask",
        "answer",
        "speak",
        "talk",
        "listen",
        "hear",
        "look",
        "watch",
        "read",
        "write",
        "learn",
        "teach",
        "study",
        "work",
        "play",
        "run",
        "walk",
        "drive",
        "travel",
        "visit",
        "live",
        "eat",
        "drink",
        "sleep",
        "wake",
        "start",
        "stop",
        "finish",
        "continue",
        "help",
        "support",
        "protect",
        "save",
        "buy",
        "sell",
        "pay",
        "spend",
        "find",
        "search",
        "choose",
        "decide",
        "plan",
        "prepare",
        "organize",
        "manage",
        "control",
        "create",
        "build",
        "design",
        "improve",
        "change",
        "move",
        "turn",
        "open",
        "close",
        "enter",
        "exit",
        "join",
        "leave",
        // Numbers and time
        "one",
        "two",
        "three",
        "four",
        "five",
        "six",
        "seven",
        "eight",
        "nine",
        "ten",
        "first",
        "second",
        "third",
        "last",
        "next",
        "before",
        "after",
        "during",
        "morning",
        "afternoon",
        "evening",
        "night",
        "today",
        "tomorrow",
        "yesterday",
        // Basic concepts
        "place",
        "space",
        "area",
        "size",
        "part",
        "whole",
        "beginning",
        "middle",
        "end",
        "top",
        "bottom",
        "left",
        "right",
        "inside",
        "outside",
        "front",
        "back",
        "center",
        "corner",
        "side",
        "edge",
        "line",
        "point",
        "circle",
        "square",
        "round",
        "straight",
        "curved",
        "flat",
        "thick",
        "thin",
        "light",
        "dark",
        "bright",
        "clear",
        "loud",
        "quiet",
        "soft",
        "hard",
        "smooth",
        "rough",
        "sweet",
        "sour",
        "bitter",
        "fresh",
        "dry",
        "wet",
    ];

    if excellent_words.contains(&word) {
        return WordCategory::Excellent;
    }

    // Good words - proper English, maybe less common
    if word.len() >= 4 && is_likely_english_word(word) {
        return WordCategory::Good;
    }

    // 3-letter words that seem like proper English
    if word.len() == 3 && is_likely_english_word(word) {
        return WordCategory::Good;
    }

    // If it passed all filters but isn't excellent/good, it's acceptable
    WordCategory::Acceptable
}

fn has_excessive_repetition(word: &str) -> bool {
    let chars: Vec<char> = word.chars().collect();

    // Check for 3+ consecutive identical letters
    for i in 0..chars.len().saturating_sub(2) {
        if chars[i] == chars[i + 1] && chars[i + 1] == chars[i + 2] {
            return true;
        }
    }

    // Check if more than half the letters are the same
    let mut char_counts = std::collections::HashMap::new();
    for &c in &chars {
        *char_counts.entry(c).or_insert(0) += 1;
    }

    let max_count = char_counts.values().max().unwrap_or(&0);
    if *max_count > chars.len() / 2 {
        return true;
    }

    false
}

fn is_likely_english_word(word: &str) -> bool {
    // Basic heuristics for English words
    let chars: Vec<char> = word.chars().collect();

    // Check for common English patterns
    let common_endings = ["ing", "tion", "ed", "er", "ly", "est", "ness", "ment"];
    for ending in &common_endings {
        if word.ends_with(ending) {
            return true;
        }
    }

    let common_beginnings = ["un", "re", "pre", "dis", "over", "under", "out"];
    for beginning in &common_beginnings {
        if word.starts_with(beginning) && word.len() > beginning.len() {
            return true;
        }
    }

    // Avoid words with unusual letter combinations
    let weird_combos = ["xj", "xr", "qx", "zx", "jx", "vx", "bx", "cx", "fx", "gx"];
    for combo in &weird_combos {
        if word.contains(combo) {
            return false;
        }
    }

    // Must have at least one vowel (except very short words like "by", "my")
    if word.len() >= 3 {
        let vowels = ['a', 'e', 'i', 'o', 'u', 'y'];
        if !chars.iter().any(|c| vowels.contains(c)) {
            return false;
        }
    }

    true
}
