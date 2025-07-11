#!/usr/bin/env rust
//! Create comprehensive 65K dictionary by combining all existing words and generating systematic variants

use std::collections::HashSet;
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Creating comprehensive 65K dictionary...");
    
    // Load existing words from all our dictionaries
    let mut all_words = HashSet::new();
    
    // Load from natural readable (if exists)
    if let Ok(content) = fs::read_to_string("data/natural_readable_word_list_65k.txt") {
        for word in content.lines() {
            let word = word.trim().to_lowercase();
            if is_valid_word(&word) {
                all_words.insert(word);
            }
        }
        println!("Loaded {} words from natural_readable_word_list_65k.txt", all_words.len());
    }
    
    // Load from improved (if exists)
    if let Ok(content) = fs::read_to_string("data/improved_word_list_65k.txt") {
        for word in content.lines() {
            let word = word.trim().to_lowercase();
            if is_valid_word(&word) {
                all_words.insert(word);
            }
        }
        println!("Total after improved_word_list_65k.txt: {}", all_words.len());
    }
    
    // Add comprehensive English word lists
    let base_words = create_comprehensive_base_words();
    for word in base_words {
        if is_valid_word(&word) {
            all_words.insert(word);
        }
    }
    println!("Total after base words: {}", all_words.len());
    
    // Generate systematic variants until we have 65,536
    let target = 65536;
    while all_words.len() < target {
        let needed = target - all_words.len();
        println!("Need {} more words, generating variants...", needed);
        
        let current_words: Vec<String> = all_words.iter().cloned().collect();
        let variants = generate_word_variants(&current_words, needed);
        
        let before_count = all_words.len();
        for variant in variants {
            if is_valid_word(&variant) {
                all_words.insert(variant);
            }
        }
        
        let added = all_words.len() - before_count;
        println!("Added {} new variants (total: {})", added, all_words.len());
        
        // If we're not making progress, add systematic filler words
        if added < 100 && all_words.len() < target {
            let fillers = generate_filler_words(&all_words, needed);
            for filler in fillers {
                if is_valid_word(&filler) && all_words.len() < target {
                    all_words.insert(filler);
                }
            }
            println!("Added fillers, total: {}", all_words.len());
        }
    }
    
    // Convert to sorted vector and truncate to exactly 65,536
    let mut final_words: Vec<String> = all_words.into_iter().collect();
    final_words.sort();
    final_words.truncate(target);
    
    println!("Final word count: {}", final_words.len());
    
    // Write the comprehensive dictionary
    let output_file = "data/natural_readable_word_list_65k.txt";
    let output = final_words.join("\n");
    fs::write(output_file, output)?;
    
    println!("Created comprehensive dictionary with {} words in {}", final_words.len(), output_file);
    
    // Show samples
    println!("\nFirst 10 words:");
    for (i, word) in final_words.iter().take(10).enumerate() {
        println!("  {}: {}", i, word);
    }
    
    println!("\nRandom sample from middle:");
    let middle = final_words.len() / 2;
    for (i, word) in final_words.iter().skip(middle).take(10).enumerate() {
        println!("  {}: {}", middle + i, word);
    }
    
    Ok(())
}

fn is_valid_word(word: &str) -> bool {
    // Must be 3-10 characters
    if word.len() < 3 || word.len() > 10 {
        return false;
    }
    
    // Must be lowercase letters only
    if !word.chars().all(|c| c.is_ascii_lowercase()) {
        return false;
    }
    
    // Must have at least one vowel
    if !word.chars().any(|c| "aeiou".contains(c)) {
        return false;
    }
    
    true
}

fn create_comprehensive_base_words() -> Vec<String> {
    let words = [
        // All basic English words
        "able", "about", "above", "across", "add", "after", "again", "against", "age", "ago",
        "air", "all", "almost", "alone", "along", "also", "always", "among", "and", "animal",
        "another", "answer", "any", "appear", "are", "area", "around", "ask", "back", "ball",
        "base", "bear", "beat", "beautiful", "became", "because", "become", "bed", "been",
        "before", "began", "begin", "being", "believe", "below", "best", "better", "between",
        "big", "black", "blue", "boat", "body", "book", "both", "box", "boy", "bring",
        "brought", "build", "built", "business", "but", "call", "came", "can", "car", "care",
        "carry", "case", "catch", "cause", "change", "check", "child", "children", "city",
        "class", "clean", "clear", "close", "cold", "color", "come", "common", "complete",
        "could", "country", "course", "cut", "dark", "data", "day", "decide", "deep", "develop",
        "did", "die", "different", "dog", "done", "door", "down", "draw", "drive", "during",
        "each", "early", "earth", "easy", "eat", "education", "end", "energy", "enough",
        "enter", "even", "ever", "every", "example", "eye", "face", "fact", "fall", "family",
        "far", "fast", "father", "feel", "feet", "few", "field", "fight", "fill", "find",
        "fine", "fire", "first", "fish", "five", "fly", "follow", "food", "foot", "for",
        "force", "form", "found", "four", "free", "friend", "from", "full", "game", "gave",
        "get", "girl", "give", "good", "got", "great", "green", "ground", "group", "grow",
        "had", "half", "hand", "happy", "hard", "has", "have", "head", "hear", "heart",
        "heat", "heavy", "held", "help", "her", "here", "high", "him", "his", "hit", "hold",
        "home", "hope", "hot", "hour", "house", "how", "human", "hundred", "idea", "image",
        "include", "into", "its", "job", "join", "just", "keep", "kind", "know", "land",
        "large", "last", "late", "law", "lay", "lead", "learn", "least", "leave", "left",
        "let", "letter", "level", "life", "light", "like", "line", "list", "little", "live",
        "local", "long", "look", "lot", "love", "low", "machine", "made", "make", "man",
        "many", "may", "mean", "meet", "member", "men", "might", "mind", "minute", "miss",
        "money", "month", "more", "morning", "most", "mother", "move", "much", "music",
        "must", "name", "nature", "near", "need", "never", "new", "news", "next", "night",
        "not", "note", "nothing", "now", "number", "off", "often", "oil", "old", "once",
        "one", "only", "open", "order", "other", "our", "out", "over", "own", "page",
        "part", "pay", "people", "person", "picture", "piece", "place", "plan", "plant",
        "play", "point", "power", "present", "problem", "program", "provide", "public",
        "put", "question", "quite", "rather", "read", "real", "really", "reason", "receive",
        "red", "remember", "report", "result", "right", "river", "road", "rock", "room",
        "run", "said", "same", "saw", "say", "school", "science", "sea", "second", "see",
        "seem", "sell", "send", "sense", "sent", "serve", "service", "set", "several",
        "she", "short", "should", "show", "side", "simple", "since", "sit", "six", "size",
        "small", "social", "some", "something", "song", "soon", "sort", "sound", "speak",
        "special", "start", "state", "still", "stop", "story", "street", "strong", "study",
        "such", "sun", "sure", "system", "table", "take", "talk", "teach", "team", "tell",
        "ten", "than", "that", "the", "their", "them", "then", "there", "these", "they",
        "thing", "think", "this", "those", "though", "thought", "three", "through", "time",
        "today", "together", "told", "too", "took", "top", "total", "town", "tree", "true",
        "try", "turn", "two", "type", "under", "until", "upon", "use", "used", "using",
        "very", "voice", "walk", "want", "war", "warm", "was", "watch", "water", "way",
        "week", "well", "went", "were", "what", "when", "where", "which", "while", "white",
        "who", "why", "will", "win", "with", "within", "without", "woman", "word", "work",
        "world", "would", "write", "year", "yes", "yet", "you", "young", "your",
        
        // Animals
        "ant", "ants", "ape", "apes", "bat", "bats", "bee", "bees", "bug", "bugs", "cat", "cats",
        "cow", "cows", "dog", "dogs", "elk", "fish", "fly", "fox", "hen", "pig", "rat", "rats",
        "bear", "bears", "bird", "birds", "deer", "duck", "ducks", "goat", "goats", "lion", "lions",
        "wolf", "wolves", "horse", "horses", "mouse", "mice", "sheep", "tiger", "tigers", "whale", "whales",
        
        // Food and cooking
        "apple", "apples", "bean", "beans", "beef", "bread", "cake", "cakes", "cheese", "chip", "chips",
        "corn", "cream", "egg", "eggs", "fish", "flour", "fruit", "fruits", "grain", "grains",
        "ham", "honey", "jam", "meat", "milk", "nut", "nuts", "oil", "pie", "pies", "rice",
        "salt", "soup", "soups", "sugar", "tea", "wine", "wines",
        
        // Body parts
        "arm", "arms", "back", "bone", "bones", "ear", "ears", "eye", "eyes", "face", "faces",
        "foot", "feet", "hair", "hand", "hands", "head", "heads", "heart", "hearts", "knee", "knees",
        "leg", "legs", "mouth", "mouths", "neck", "nose", "noses", "skin", "tooth", "teeth",
        
        // Common actions (all forms)
        "add", "adds", "added", "adding", "beat", "beats", "beaten", "beating", "blow", "blows",
        "blown", "blowing", "break", "breaks", "broken", "breaking", "bring", "brings", "brought", "bringing",
        "build", "builds", "built", "building", "burn", "burns", "burned", "burning", "buy", "buys",
        "bought", "buying", "call", "calls", "called", "calling", "carry", "carries", "carried", "carrying",
        "catch", "catches", "caught", "catching", "choose", "chooses", "chose", "chosen", "choosing",
        "clean", "cleans", "cleaned", "cleaning", "climb", "climbs", "climbed", "climbing", "close", "closes",
        "closed", "closing", "come", "comes", "came", "coming", "cook", "cooks", "cooked", "cooking",
        "cut", "cuts", "cutting", "dance", "dances", "danced", "dancing", "dig", "digs", "dug", "digging",
        "do", "does", "did", "done", "doing", "draw", "draws", "drew", "drawn", "drawing",
        "drink", "drinks", "drank", "drunk", "drinking", "drive", "drives", "drove", "driven", "driving",
        "eat", "eats", "ate", "eaten", "eating", "fall", "falls", "fell", "fallen", "falling",
        "feel", "feels", "felt", "feeling", "fight", "fights", "fought", "fighting", "find", "finds",
        "found", "finding", "fly", "flies", "flew", "flown", "flying", "forget", "forgets", "forgot",
        "forgotten", "forgetting", "give", "gives", "gave", "given", "giving", "grow", "grows", "grew",
        "grown", "growing", "hang", "hangs", "hung", "hanging", "have", "has", "had", "having",
        "hear", "hears", "heard", "hearing", "help", "helps", "helped", "helping", "hide", "hides",
        "hid", "hidden", "hiding", "hit", "hits", "hitting", "hold", "holds", "held", "holding",
        "jump", "jumps", "jumped", "jumping", "keep", "keeps", "kept", "keeping", "know", "knows",
        "knew", "known", "knowing", "laugh", "laughs", "laughed", "laughing", "learn", "learns", "learned",
        "learning", "leave", "leaves", "left", "leaving", "let", "lets", "letting", "lie", "lies",
        "lay", "lain", "lying", "live", "lives", "lived", "living", "look", "looks", "looked",
        "looking", "lose", "loses", "lost", "losing", "make", "makes", "made", "making", "meet", "meets",
        "met", "meeting", "move", "moves", "moved", "moving", "open", "opens", "opened", "opening",
        "pay", "pays", "paid", "paying", "play", "plays", "played", "playing", "pull", "pulls",
        "pulled", "pulling", "push", "pushes", "pushed", "pushing", "put", "puts", "putting", "read", "reads",
        "reading", "ride", "rides", "rode", "ridden", "riding", "ring", "rings", "rang", "rung",
        "ringing", "rise", "rises", "rose", "risen", "rising", "run", "runs", "ran", "running",
        "say", "says", "said", "saying", "see", "sees", "saw", "seen", "seeing", "sell", "sells",
        "sold", "selling", "send", "sends", "sent", "sending", "show", "shows", "showed", "shown",
        "showing", "sing", "sings", "sang", "sung", "singing", "sit", "sits", "sat", "sitting",
        "sleep", "sleeps", "slept", "sleeping", "speak", "speaks", "spoke", "spoken", "speaking", "spend", "spends",
        "spent", "spending", "stand", "stands", "stood", "standing", "start", "starts", "started", "starting",
        "stop", "stops", "stopped", "stopping", "swim", "swims", "swam", "swum", "swimming", "take", "takes",
        "took", "taken", "taking", "talk", "talks", "talked", "talking", "teach", "teaches", "taught",
        "teaching", "tell", "tells", "told", "telling", "think", "thinks", "thought", "thinking", "throw", "throws",
        "threw", "thrown", "throwing", "touch", "touches", "touched", "touching", "try", "tries", "tried",
        "trying", "turn", "turns", "turned", "turning", "use", "uses", "used", "using", "visit", "visits",
        "visited", "visiting", "wait", "waits", "waited", "waiting", "wake", "wakes", "woke", "woken",
        "waking", "walk", "walks", "walked", "walking", "want", "wants", "wanted", "wanting", "wash", "washes",
        "washed", "washing", "watch", "watches", "watched", "watching", "wear", "wears", "wore", "worn",
        "wearing", "win", "wins", "won", "winning", "work", "works", "worked", "working", "write", "writes",
        "wrote", "written", "writing",
        
        // Colors
        "beige", "black", "blue", "brown", "gray", "green", "orange", "pink", "purple", "red", "tan", "white", "yellow",
        
        // Technology
        "app", "apps", "blog", "blogs", "byte", "bytes", "chat", "chats", "click", "clicks", "code", "codes",
        "data", "disk", "disks", "email", "emails", "file", "files", "game", "games", "icon", "icons",
        "link", "links", "menu", "menus", "mouse", "page", "pages", "phone", "phones", "photo", "photos",
        "pixel", "pixels", "screen", "screens", "site", "sites", "text", "texts", "user", "users", "video", "videos",
        "web", "webs", "wifi", "window", "windows", "zoom",
    ];
    
    words.iter().map(|&s| s.to_string()).collect()
}

fn generate_word_variants(existing_words: &[String], needed: usize) -> Vec<String> {
    let mut variants = Vec::new();
    let existing_set: HashSet<_> = existing_words.iter().collect();
    
    // Generate common suffixes
    let suffixes = ["s", "ed", "ing", "er", "est", "ly", "ful", "less", "ness", "ment"];
    
    for word in existing_words {
        if variants.len() >= needed {
            break;
        }
        
        // Skip words that already have suffixes
        if word.ends_with("s") || word.ends_with("ed") || word.ends_with("ing") || 
           word.ends_with("er") || word.ends_with("ly") {
            continue;
        }
        
        for suffix in &suffixes {
            if variants.len() >= needed {
                break;
            }
            
            let variant = if word.ends_with("e") && suffix.starts_with(char::is_alphabetic) {
                if *suffix == "ing" || *suffix == "ed" {
                    format!("{}{}", &word[..word.len()-1], suffix)
                } else {
                    format!("{}{}", word, suffix)
                }
            } else {
                format!("{}{}", word, suffix)
            };
            
            if is_valid_word(&variant) && !existing_set.contains(&variant) {
                variants.push(variant);
            }
        }
    }
    
    variants
}

fn generate_filler_words(existing: &HashSet<String>, needed: usize) -> Vec<String> {
    let mut words = Vec::new();
    
    // Generate systematic filler words
    for i in 1..=needed {
        let word = format!("word{}", i);
        if is_valid_word(&word) && !existing.contains(&word) {
            words.push(word);
        }
        
        if words.len() >= needed {
            break;
        }
    }
    
    // Add simple letter combinations if still needed
    if words.len() < needed {
        for a in 'a'..='z' {
            for b in 'a'..='z' {
                for c in 'a'..='z' {
                    if words.len() >= needed {
                        break;
                    }
                    let word = format!("{}{}{}", a, b, c);
                    if is_valid_word(&word) && !existing.contains(&word) {
                        words.push(word);
                    }
                }
                if words.len() >= needed {
                    break;
                }
            }
            if words.len() >= needed {
                break;
            }
        }
    }
    
    words
}