#[allow(unused_imports)]
use std::collections::HashSet;
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔧 FILTERING DICTIONARY FOR BETTER QUALITY");
    println!("==========================================");

    // Read the current dictionary
    let wordlist_content = fs::read_to_string("data/wordlist_16384.txt")?;
    let all_words: Vec<&str> = wordlist_content.lines().collect();

    println!("Original dictionary size: {} words", all_words.len());

    // Filter for high-quality English words
    let mut quality_words = Vec::new();
    let mut rejected_words = Vec::new();

    for word in &all_words {
        if is_quality_english_word(word) {
            quality_words.push(*word);
        } else {
            rejected_words.push(*word);
        }
    }

    println!("Quality words found: {}", quality_words.len());
    println!("Rejected words: {}", rejected_words.len());

    // Show some rejected words
    println!("\nSample rejected words:");
    for word in rejected_words.iter().take(20) {
        println!("  '{}'", word);
    }

    // If we don't have enough quality words, we need to be more lenient
    if quality_words.len() < 8192 {
        println!(
            "\n⚠️  Not enough quality words for half-dictionary (need 8192, got {})",
            quality_words.len()
        );
        println!("Need to either:");
        println!("1. Be more lenient in filtering");
        println!("2. Find additional quality word sources");
        println!("3. Create a smaller but higher-quality dictionary");

        // Try more lenient filtering
        println!("\nTrying more lenient filtering...");
        quality_words.clear();

        for word in &all_words {
            if is_lenient_quality_word(word) {
                quality_words.push(*word);
            }
        }

        println!("Lenient quality words: {}", quality_words.len());
    }

    // If we have enough, create a filtered dictionary
    if quality_words.len() >= 8192 {
        // Take the first 8192 quality words for a smaller but better dictionary
        quality_words.truncate(8192);

        // Write to a new file
        let filtered_content = quality_words.join("\n");
        fs::write("data/wordlist_8192_quality.txt", filtered_content)?;

        println!("\n✅ Created filtered dictionary: data/wordlist_8192_quality.txt");
        println!("   Size: {} words (13-bit encoding)", quality_words.len());
        println!(
            "   This will require updating the encoder to use 13-bit indices instead of 14-bit"
        );
    } else {
        println!("\n❌ Cannot create quality filtered dictionary - insufficient words");

        // Show statistics about what we have
        let mut length_stats = std::collections::HashMap::new();
        for word in &quality_words {
            *length_stats.entry(word.len()).or_insert(0) += 1;
        }

        println!("\nWord length distribution in quality subset:");
        for (len, count) in length_stats {
            println!("  {}-letter words: {}", len, count);
        }
    }

    Ok(())
}

fn is_quality_english_word(word: &str) -> bool {
    // Must be at least 3 characters
    if word.len() < 3 {
        return false;
    }

    // Must be at most 12 characters (for voice-friendliness)
    if word.len() > 12 {
        return false;
    }

    // Must contain only lowercase letters
    if !word.chars().all(|c| c.is_ascii_lowercase()) {
        return false;
    }

    // Reject obvious abbreviations, codes, and made-up words
    let rejected_patterns = [
        "aaa", "bbb", "ccc", "ddd", "eee", "fff", "ggg", "hhh", "iii", "jjj", "kkk", "lll", "mmm",
        "nnn", "ooo", "ppp", "qqq", "rrr", "sss", "ttt", "uuu", "vvv", "www", "xxx", "yyy", "zzz",
    ];

    for pattern in &rejected_patterns {
        if word.contains(pattern) {
            return false;
        }
    }

    // Reject specific bad words we identified
    let bad_words = [
        "bvt", "bok", "duh", "duo", "emu", "fax", "gab", "gag", "keg", "nag", "dab", "jab", "yon",
        "zea", "zoa", "wahl", "bal", "oaf", "pox", "pry", "pug", "rut", "wad", "wok", "yam", "yen",
        "yin", "zap", "zen", "zit", "pep", "pod", "tux", "sly", "pep", "ahoy",
    ];

    if bad_words.contains(&word) {
        return false;
    }

    // Prefer longer words (4+ characters) as they tend to be more standard English
    if word.len() >= 4 {
        return true;
    }

    // For 3-letter words, only allow common ones
    let good_three_letter = [
        "the", "and", "for", "are", "but", "not", "you", "all", "can", "had", "her", "was", "one",
        "our", "out", "day", "get", "has", "him", "his", "how", "its", "may", "new", "now", "old",
        "see", "two", "who", "boy", "did", "car", "eat", "eye", "far", "fun", "got", "hot", "let",
        "man", "mom", "run", "sit", "top", "try", "way", "win", "yes", "big", "red", "bad", "cat",
        "dog", "end", "few", "got", "job", "lot", "put", "say", "too", "use", "war", "win", "yet",
        "zoo", "act", "add", "age", "ago", "air", "ask", "bag", "bar", "bed", "bee", "bit", "box",
        "buy", "cry", "cup", "cut", "die", "egg", "fit", "fly", "hat", "hit", "ice", "key", "lay",
        "leg", "lie", "map", "mix", "pay", "pen", "pot", "row", "sea", "set", "six", "sun", "tea",
        "ten", "tie", "toy", "van", "wet", "why",
    ];

    good_three_letter.contains(&word)
}

fn is_lenient_quality_word(word: &str) -> bool {
    // More lenient version - allows more words but still filters obvious junk

    if word.len() < 3 || word.len() > 12 {
        return false;
    }

    if !word.chars().all(|c| c.is_ascii_lowercase()) {
        return false;
    }

    // Only reject the most obviously bad words
    let very_bad_words = [
        "bvt", "zoa", "wahl", "duh", "gab", "gag", "nag", "pep", "pox", "tux", "wok", "yin", "zit",
        "ahoy",
    ];

    if very_bad_words.contains(&word) {
        return false;
    }

    // Reject repetitive patterns
    if word.len() == 3
        && word
            .chars()
            .collect::<Vec<_>>()
            .windows(2)
            .any(|w| w[0] == w[1])
    {
        let repeated = word.chars().nth(0) == word.chars().nth(1)
            || word.chars().nth(1) == word.chars().nth(2);
        if repeated {
            return false;
        }
    }

    true
}
