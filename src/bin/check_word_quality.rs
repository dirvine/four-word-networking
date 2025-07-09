use four_word_networking::{dictionary16k::Dictionary16K, UltraCompactEncoder};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔍 CHECKING WORD QUALITY IN DICTIONARY");
    println!("=====================================");

    let encoder = UltraCompactEncoder::new()?;
    let dict = Dictionary16K::new()?;

    let test_cases = vec![
        "/ip4/127.0.0.1/tcp/4001",     // "zea yon bvt"
        "/ip4/192.168.1.1/tcp/80",     // "slightly wahl bal"
        "/ip6/::1/tcp/4001",           // "zoa enid afar"
        "/ip4/192.168.1.100/tcp/8080", // "fv oppressor jurist"
    ];

    for multiaddr in test_cases {
        match encoder.encode(multiaddr) {
            Ok(encoded) => {
                let words = encoded.to_words();
                println!("\n🔬 Analyzing: {}", multiaddr);
                println!("   Output: {}", words);

                // Check each word
                for word in words.split_whitespace() {
                    if let Ok(index) = dict.get_index(word) {
                        let is_proper_english = is_proper_english_word(word);
                        let quality = if is_proper_english {
                            "✅ Good English"
                        } else {
                            "❌ Questionable"
                        };
                        println!("   '{}' (index {}): {}", word, index, quality);

                        if !is_proper_english {
                            // Find some better words around this index
                            println!("     Better alternatives near index {}:", index);
                            for offset in [-10i32, -5, 5, 10] {
                                let alt_idx = (index as i32 + offset).max(0).min(16383) as u16;
                                if let Ok(alt_word) = dict.get_word(alt_idx) {
                                    if is_proper_english_word(alt_word) {
                                        println!("       '{}' (index {})", alt_word, alt_idx);
                                    }
                                }
                            }
                        }
                    }
                }
            }
            Err(e) => println!("Error: {}", e),
        }
    }

    // Check the first 100 words in dictionary for quality
    println!("\n📝 DICTIONARY QUALITY SAMPLE (first 100 words):");
    let mut good_words = 0;
    let mut questionable_words = 0;

    for i in 0..100 {
        if let Ok(word) = dict.get_word(i) {
            let is_good = is_proper_english_word(word);
            if is_good {
                good_words += 1;
            } else {
                questionable_words += 1;
                if questionable_words <= 10 {
                    // Show first 10 questionable words
                    println!("   Index {}: '{}' ❌", i, word);
                }
            }
        }
    }

    println!("\n📊 QUALITY SUMMARY (first 100 words):");
    println!(
        "   Good English words: {}/100 ({:.1}%)",
        good_words, good_words as f64
    );
    println!(
        "   Questionable words: {}/100 ({:.1}%)",
        questionable_words, questionable_words as f64
    );

    if questionable_words > 20 {
        println!("\n⚠️  Dictionary quality issue detected!");
        println!("   Many non-standard English words found.");
        println!("   Consider filtering dictionary for better user experience.");
    }

    Ok(())
}

fn is_proper_english_word(word: &str) -> bool {
    // Basic checks for proper English words
    if word.len() < 2 {
        return false;
    }

    // Check if word contains only letters
    if !word.chars().all(|c| c.is_ascii_alphabetic()) {
        return false;
    }

    // Known good words (basic subset)
    let good_words = [
        "aim", "art", "cod", "cut", "dad", "dig", "dry", "elf", "elk", "elm", "fit", "fox", "gap",
        "gas", "gem", "guy", "had", "hug", "hut", "ice", "icy", "irk", "ivy", "job", "jot", "lid",
        "lip", "map", "mom", "mop", "mud", "mug", "net", "oil", "pen", "pet", "pig", "pot", "red",
        "run", "sea", "sky", "sun", "tea", "top", "toy", "war", "way", "web", "win", "zoo", "able",
        "acid", "acts", "aged", "also", "army", "baby", "back", "band", "bank", "base", "bath",
        "bear", "beat", "been", "beer", "bell", "best", "bike", "bird", "blue", "boat", "body",
        "bone", "book", "born", "both", "boys", "busy", "call", "came", "camp", "card", "care",
        "cars", "case", "cash", "cats", "city", "club", "coal", "coat", "code", "cold", "come",
        "cook", "cool", "copy", "cost", "crew", "dark", "data", "date", "days", "dead", "deal",
        "dear", "deep", "desk", "diet", "dogs", "done", "door", "down", "draw", "drew", "drop",
        "drug", "each", "easy", "east", "edge", "eggs", "else", "even", "ever", "face", "fact",
        "fair", "fall", "farm", "fast", "fear", "feel", "feet", "fell", "felt", "file", "fill",
        "film", "find", "fine", "fire", "firm", "fish", "five", "flat", "flow", "food", "foot",
        "form", "fort", "four", "free", "from", "full", "fund", "game", "gave", "gift", "girl",
        "give", "glad", "goes", "gold", "gone", "good", "gray", "grew", "grow", "half", "hall",
        "hand", "hang", "hard", "harm", "hate", "have", "head", "hear", "heat", "held", "help",
        "here", "hide", "high", "hill", "hire", "hold", "hole", "home", "hope", "host", "hour",
        "huge", "hung", "hunt", "hurt", "idea", "inch", "into", "iron", "item", "jobs", "join",
        "jump", "just", "keep", "kept", "kids", "kill", "kind", "king", "knew", "know", "lack",
        "lady", "laid", "lake", "land", "last", "late", "lead", "left", "less", "life", "lift",
        "like", "line", "list", "live", "loan", "lock", "long", "look", "loop", "lord", "lose",
        "loss", "lost", "lots", "loud", "love", "luck", "made", "mail", "main", "make", "male",
        "many", "mark", "mass", "mate", "math", "meal", "mean", "meat", "meet", "men's", "menu",
        "mind", "mine", "miss", "mode", "moon", "more", "most", "move", "much", "must", "name",
        "near", "neck", "need", "news", "next", "nice", "nine", "node", "none", "noon", "nose",
        "note", "noun", "odds", "once", "only", "onto", "open", "oral", "over", "owns", "pace",
        "pack", "page", "paid", "pain", "pair", "pale", "park", "part", "pass", "past", "path",
        "peak", "pick", "pile", "pink", "plan", "play", "plot", "plus", "poem", "poet", "poll",
        "pool", "poor", "port", "post", "pull", "pure", "push", "race", "rain", "rank", "rate",
        "read", "real", "rear", "rely", "rent", "rest", "rich", "ride", "ring", "rise", "risk",
        "road", "rock", "role", "roll", "room", "root", "rope", "rose", "rule", "runs", "safe",
        "said", "sail", "sake", "sale", "salt", "same", "sand", "save", "seat", "seed", "seek",
        "seem", "seen", "self", "sell", "send", "sent", "ship", "shop", "shot", "show", "shut",
        "sick", "side", "sign", "site", "size", "skin", "slip", "slow", "snow", "soft", "soil",
        "sold", "sole", "some", "song", "soon", "sort", "soul", "soup", "spin", "spot", "star",
        "stay", "step", "stop", "such", "suit", "sure", "take", "tale", "talk", "tall", "tank",
        "tape", "task", "team", "tell", "term", "test", "text", "than", "that", "them", "then",
        "they", "thin", "this", "thus", "tide", "tied", "ties", "time", "tiny", "told", "tone",
        "took", "tool", "tops", "tour", "town", "tree", "trip", "true", "tune", "turn", "twin",
        "type", "unit", "upon", "used", "user", "uses", "vary", "vast", "very", "view", "vote",
        "wait", "wake", "walk", "wall", "want", "ward", "warm", "warn", "wash", "wave", "ways",
        "weak", "wear", "week", "well", "went", "were", "west", "what", "when", "wide", "wife",
        "wild", "will", "wind", "wine", "wing", "wins", "wire", "wise", "wish", "with", "woke",
        "wood", "word", "wore", "work", "yard", "year", "your", "zero", "zone",
    ];

    // Known problematic abbreviations/codes that should be filtered
    let bad_words = [
        "bvt", "bal", "zoa", "zea", "fv", "wahl", "bok", "duh", "duo", "eel", "emu", "fax", "gab",
        "gag", "keg", "nag", "dab", "jab", "yon", "afar",
    ];

    if bad_words.contains(&word) {
        return false;
    }

    good_words.contains(&word) || word.len() >= 4 // Longer words are generally more likely to be proper English
}
