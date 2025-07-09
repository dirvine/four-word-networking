use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🎯 CREATING COMMON ENGLISH WORDS DICTIONARY");
    println!("==========================================");

    // Read the current dictionary
    let wordlist_content = fs::read_to_string("data/wordlist_16384.txt")?;
    let all_words: Vec<&str> = wordlist_content.lines().collect();

    println!("Original dictionary size: {} words", all_words.len());

    // Categorize by commonality and length
    let mut very_common = Vec::new();
    let mut common = Vec::new();
    let mut uncommon_but_valid = Vec::new();
    let mut filtered_out = Vec::new();

    for word in &all_words {
        match assess_word_commonality(word) {
            WordCommonality::VeryCommon => very_common.push(*word),
            WordCommonality::Common => common.push(*word),
            WordCommonality::UncommonButValid => uncommon_but_valid.push(*word),
            WordCommonality::Filtered => filtered_out.push(*word),
        }
    }

    println!("Very common words: {}", very_common.len());
    println!("Common words: {}", common.len());
    println!("Uncommon but valid: {}", uncommon_but_valid.len());
    println!("Filtered out: {}", filtered_out.len());

    // Show samples
    println!("\nSample very common words:");
    for word in very_common.iter().take(20) {
        println!("  {}", word);
    }

    println!("\nSample filtered words:");
    for word in filtered_out.iter().take(20) {
        println!("  {}", word);
    }

    // Build dictionary: Very common first, then common, then fill with uncommon
    let mut final_words: Vec<&str> = Vec::new();
    final_words.extend(&very_common);
    final_words.extend(&common);

    // Add uncommon words to reach exactly 16384
    let needed = 16384 - final_words.len();
    if needed > 0 {
        if needed <= uncommon_but_valid.len() {
            final_words.extend(uncommon_but_valid.iter().take(needed));
        } else {
            final_words.extend(&uncommon_but_valid);
            // If still not enough, add back some filtered words (least problematic ones)
            let remaining_needed = 16384 - final_words.len();
            let safe_filtered: Vec<&str> = filtered_out
                .iter()
                .filter(|w| is_safe_to_recover(w))
                .take(remaining_needed)
                .cloned()
                .collect();
            final_words.extend(&safe_filtered);

            // If STILL not enough, add the remaining filtered words (except the worst ones)
            while final_words.len() < 16384 {
                let still_needed = 16384 - final_words.len();
                let really_bad = [
                    "rape", "kill", "murder", "nazi", "fuck", "shit", "bvt", "xj", "xr", "uvw",
                    "xyz",
                ];
                let final_recovery: Vec<&str> = filtered_out
                    .iter()
                    .filter(|w| !final_words.contains(w) && !really_bad.contains(w))
                    .take(still_needed)
                    .cloned()
                    .collect();

                if final_recovery.is_empty() {
                    // If no more safe words, add any remaining words except the really bad ones
                    let last_resort: Vec<&str> = all_words
                        .iter()
                        .filter(|w| !final_words.contains(w) && !really_bad.contains(w))
                        .take(still_needed)
                        .cloned()
                        .collect();
                    final_words.extend(&last_resort);
                    break;
                } else {
                    final_words.extend(&final_recovery);
                }
            }
        }
    }

    println!("\nFinal distribution:");
    println!("  Very common: {}", very_common.len());
    println!("  Common: {}", common.len());
    println!(
        "  Uncommon: {}",
        (final_words.len() - very_common.len() - common.len()).min(uncommon_but_valid.len())
    );
    println!("  Total: {}", final_words.len());

    // Write the common words dictionary
    let common_content = final_words.join("\n");
    fs::write("data/wordlist_16384_common.txt", common_content)?;

    println!("\n✅ Created common words dictionary: data/wordlist_16384_common.txt");
    println!(
        "   Very common words at indices 0-{}",
        very_common.len() - 1
    );
    println!(
        "   Common words at indices {}-{}",
        very_common.len(),
        very_common.len() + common.len() - 1
    );
    println!("   Focus on longer, familiar English words for better voice communication");

    Ok(())
}

#[derive(Debug, PartialEq)]
enum WordCommonality {
    VeryCommon,       // Words everyone knows - prioritize these
    Common,           // Good English words people recognize
    UncommonButValid, // Valid but less familiar words
    Filtered,         // Remove completely
}

fn assess_word_commonality(word: &str) -> WordCommonality {
    // Filter out completely
    if word.len() < 3 || word.len() > 15 {
        return WordCommonality::Filtered;
    }

    if !word.chars().all(|c| c.is_ascii_lowercase()) {
        return WordCommonality::Filtered;
    }

    // Remove offensive, gibberish, and problematic words
    let filtered_words = [
        "rape", "kill", "murder", "nazi", "fuck", "shit", "penis", "vagina", "cocaine", "heroin",
        "meth", "bvt", "zoa", "zea", "wahl", "xj", "xr", "apio", "cps", "uvw", "www", "abey",
        "alle", "sparge", "yds", "kepler", "augur", "nigh", "gerry", "bok", "duh", "gab", "gag",
        "keg", "nag", "oaf", "pox", "pry", "pug", "rut", "wad", "wok", "yam", "yen", "yin", "zap",
        "zen", "zit", "pep", "pod", "tux", "ahoy", "fax", "wow", "huh", "hmm", "err", "umm", "ugh",
        "meh", "xyz", "css", "php", "asp", "jsp", "api", "gui", "cpu", "ram", "rom", "usb", "dvd",
        "lcd", "led", "gps", "wifi", "lan", "wan", "vpn", "dns", "tcp", "udp", "http", "https",
        "smtp", "pop", "imap", "ssl", "tls", "ssh", "fyi", "sql", "xml",
    ];

    if filtered_words.contains(&word) {
        return WordCommonality::Filtered;
    }

    // Very common words - everyone knows these, prefer longer ones
    let very_common_words = [
        // Common everyday words (4+ letters preferred)
        "about",
        "after",
        "again",
        "against",
        "almost",
        "alone",
        "along",
        "already",
        "always",
        "among",
        "angry",
        "animal",
        "another",
        "answer",
        "anyone",
        "anything",
        "around",
        "asking",
        "beautiful",
        "because",
        "become",
        "before",
        "being",
        "believe",
        "better",
        "between",
        "building",
        "business",
        "cannot",
        "change",
        "children",
        "church",
        "closed",
        "coming",
        "company",
        "computer",
        "country",
        "course",
        "during",
        "early",
        "enough",
        "evening",
        "every",
        "everyone",
        "everything",
        "example",
        "family",
        "father",
        "feeling",
        "final",
        "first",
        "follow",
        "friend",
        "getting",
        "given",
        "going",
        "government",
        "great",
        "group",
        "hand",
        "happy",
        "having",
        "heard",
        "heart",
        "help",
        "here",
        "high",
        "home",
        "hope",
        "hour",
        "house",
        "human",
        "important",
        "information",
        "instead",
        "interest",
        "just",
        "keep",
        "kind",
        "know",
        "large",
        "last",
        "late",
        "later",
        "leave",
        "left",
        "life",
        "light",
        "line",
        "little",
        "live",
        "local",
        "long",
        "look",
        "love",
        "made",
        "make",
        "many",
        "maybe",
        "mean",
        "might",
        "mind",
        "money",
        "more",
        "most",
        "move",
        "much",
        "music",
        "must",
        "name",
        "national",
        "need",
        "never",
        "news",
        "next",
        "night",
        "nothing",
        "number",
        "office",
        "often",
        "only",
        "open",
        "order",
        "other",
        "over",
        "part",
        "people",
        "person",
        "place",
        "point",
        "possible",
        "power",
        "probably",
        "problem",
        "program",
        "public",
        "question",
        "quite",
        "really",
        "reason",
        "right",
        "said",
        "same",
        "school",
        "second",
        "seem",
        "several",
        "short",
        "should",
        "show",
        "since",
        "small",
        "social",
        "some",
        "something",
        "sometimes",
        "sound",
        "space",
        "speak",
        "special",
        "start",
        "state",
        "still",
        "story",
        "student",
        "study",
        "support",
        "sure",
        "system",
        "take",
        "talk",
        "teacher",
        "team",
        "tell",
        "than",
        "thank",
        "that",
        "their",
        "them",
        "then",
        "there",
        "these",
        "they",
        "thing",
        "think",
        "this",
        "those",
        "three",
        "through",
        "time",
        "today",
        "together",
        "tonight",
        "turn",
        "under",
        "understand",
        "university",
        "until",
        "used",
        "using",
        "very",
        "want",
        "water",
        "week",
        "well",
        "what",
        "when",
        "where",
        "which",
        "while",
        "white",
        "will",
        "with",
        "within",
        "without",
        "woman",
        "women",
        "word",
        "work",
        "world",
        "would",
        "write",
        "year",
        "years",
        "young",
        "your",
        // Common objects and concepts
        "table",
        "chair",
        "door",
        "window",
        "book",
        "paper",
        "phone",
        "computer",
        "car",
        "road",
        "tree",
        "flower",
        "garden",
        "park",
        "mountain",
        "river",
        "ocean",
        "beach",
        "city",
        "town",
        "street",
        "building",
        "hospital",
        "restaurant",
        "store",
        "market",
        "hotel",
        "airport",
        "train",
        "bus",
        "ship",
        "plane",
        "bicycle",
        "camera",
        "television",
        "radio",
        "music",
        "movie",
        "game",
        "sport",
        "football",
        "basketball",
        "baseball",
        "tennis",
        "swimming",
        "running",
        "walking",
        "cooking",
        "eating",
        "drinking",
        "sleeping",
        "reading",
        "writing",
        "learning",
        "teaching",
        "working",
        "playing",
        "traveling",
        "shopping",
        "driving",
        "flying",
        "singing",
        "dancing",
        "painting",
        "drawing",
        "listening",
        "watching",
        "talking",
        "laughing",
        "crying",
        "smiling",
        "helping",
        "caring",
        "loving",
        "sharing",
        "giving",
        "taking",
        "buying",
        "selling",
        "making",
        "creating",
        "building",
        "fixing",
        "cleaning",
        "washing",
        "cooking",
        "baking",
        "growing",
        "farming",
        "fishing",
        "hunting",
        "camping",
        "hiking",
        "climbing",
        "swimming",
        "surfing",
        "skiing",
        "skating",
        "riding",
        // Colors, numbers, time
        "black",
        "white",
        "brown",
        "green",
        "blue",
        "yellow",
        "orange",
        "purple",
        "pink",
        "gray",
        "silver",
        "golden",
        "bright",
        "dark",
        "light",
        "monday",
        "tuesday",
        "wednesday",
        "thursday",
        "friday",
        "saturday",
        "sunday",
        "january",
        "february",
        "march",
        "april",
        "june",
        "july",
        "august",
        "september",
        "october",
        "november",
        "december",
        "morning",
        "afternoon",
        "evening",
        "night",
        "today",
        "tomorrow",
        "yesterday",
        "weekend",
        "holiday",
        "birthday",
        "christmas",
        "summer",
        "winter",
        "spring",
        "autumn",
    ];

    if very_common_words.contains(&word) {
        return WordCommonality::VeryCommon;
    }

    // Common words - good English words people recognize
    // Prioritize longer words (5+ characters) as they're easier to understand in speech
    if word.len() >= 5 && is_good_english_word(word) {
        return WordCommonality::Common;
    }

    // 4-letter words that are clearly common English
    if word.len() == 4 && is_very_common_4_letter(word) {
        return WordCommonality::Common;
    }

    // Anything else that's valid English but not as common
    if is_valid_english_word(word) {
        return WordCommonality::UncommonButValid;
    }

    WordCommonality::Filtered
}

fn is_good_english_word(word: &str) -> bool {
    // Check for common English word patterns
    let good_endings = [
        "ing", "tion", "sion", "ment", "ness", "able", "ible", "ful", "less", "ward", "wise",
    ];
    for ending in &good_endings {
        if word.ends_with(ending) && word.len() > ending.len() + 2 {
            return true;
        }
    }

    let good_beginnings = [
        "under", "over", "inter", "trans", "super", "anti", "auto", "multi",
    ];
    for beginning in &good_beginnings {
        if word.starts_with(beginning) && word.len() > beginning.len() + 2 {
            return true;
        }
    }

    // Must have vowels
    let vowels = ['a', 'e', 'i', 'o', 'u'];
    if !word.chars().any(|c| vowels.contains(&c)) {
        return false;
    }

    // No weird letter combinations
    let weird_combos = [
        "xj", "xr", "qx", "zx", "jx", "vx", "bx", "cx", "fx", "gx", "kx", "px", "wx",
    ];
    for combo in &weird_combos {
        if word.contains(combo) {
            return false;
        }
    }

    true
}

fn is_very_common_4_letter(word: &str) -> bool {
    let common_4_letter = [
        "able", "back", "ball", "base", "been", "best", "book", "call", "came", "can't", "care",
        "case", "come", "cool", "data", "days", "dead", "deal", "does", "done", "door", "down",
        "draw", "each", "easy", "else", "even", "ever", "eyes", "face", "fact", "fair", "fall",
        "fast", "fear", "feel", "feet", "file", "fill", "find", "fine", "fire", "firm", "fish",
        "five", "food", "foot", "form", "four", "free", "from", "full", "game", "gave", "gets",
        "girl", "give", "goes", "gold", "gone", "good", "hand", "hard", "have", "head", "hear",
        "help", "here", "high", "hold", "home", "hope", "hour", "huge", "idea", "into", "item",
        "join", "jump", "just", "keep", "kept", "kids", "kind", "knew", "know", "land", "last",
        "late", "lead", "left", "less", "life", "like", "line", "list", "live", "long", "look",
        "lose", "love", "made", "main", "make", "many", "mean", "mind", "miss", "more", "most",
        "move", "much", "must", "name", "near", "need", "news", "next", "nice", "nine", "none",
        "note", "open", "over", "page", "paid", "park", "part", "pass", "past", "pick", "plan",
        "play", "poor", "post", "pull", "push", "puts", "real", "rest", "rich", "ride", "ring",
        "road", "room", "rule", "runs", "safe", "said", "same", "save", "seem", "seen", "self",
        "sell", "send", "ship", "shop", "show", "shut", "side", "sign", "site", "size", "slow",
        "snow", "soft", "some", "song", "soon", "sort", "stay", "step", "stop", "such", "sure",
        "take", "talk", "tell", "term", "test", "text", "than", "that", "them", "then", "they",
        "this", "time", "told", "took", "tree", "true", "turn", "type", "upon", "used", "user",
        "uses", "very", "walk", "want", "warm", "ways", "week", "well", "went", "were", "what",
        "when", "will", "wind", "with", "word", "work", "year", "your",
    ];

    common_4_letter.contains(&word)
}

fn is_valid_english_word(word: &str) -> bool {
    // Basic validation for English-like words
    if word.len() < 3 {
        return false;
    }

    // Must have at least one vowel
    let vowels = ['a', 'e', 'i', 'o', 'u', 'y'];
    if !word.chars().any(|c| vowels.contains(&c)) {
        return false;
    }

    // No excessive repetition
    if has_excessive_repetition(word) {
        return false;
    }

    true
}

fn has_excessive_repetition(word: &str) -> bool {
    let chars: Vec<char> = word.chars().collect();

    // No more than 2 consecutive identical letters (except some valid English patterns)
    let mut consecutive_count = 1;
    for i in 1..chars.len() {
        if chars[i] == chars[i - 1] {
            consecutive_count += 1;
            if consecutive_count > 2 {
                // Allow some common English patterns like "ness", "tress", "press", "glass"
                let valid_patterns = ["ss", "ll", "tt", "nn", "mm", "rr", "ff"];
                let current_pattern = format!("{}{}", chars[i - 1], chars[i]);
                if !valid_patterns.contains(&current_pattern.as_str()) {
                    return true;
                }
            }
        } else {
            consecutive_count = 1;
        }
    }

    false
}

fn is_safe_to_recover(word: &str) -> bool {
    // Only recover words that are at least somewhat reasonable
    let definitely_bad = [
        "rape", "kill", "murder", "nazi", "fuck", "shit", "bvt", "xj", "xr", "uvw", "xyz", "www",
        "abey", "yds", "cps", "apio",
    ];

    !definitely_bad.contains(&word) && word.len() >= 3 && is_valid_english_word(word)
}
