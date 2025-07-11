#!/usr/bin/env rust
//! Create natural readable dictionary with common English words (3-10 chars)
//! Includes natural variants like: mountain/mountains, run/running, walk/walked/walking

use std::collections::HashSet;
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Creating natural readable dictionary (3-10 chars)...");
    
    // Load the current improved dictionary
    let input_file = "data/improved_word_list_65k.txt";
    let content = fs::read_to_string(input_file)?;
    let words: Vec<&str> = content.lines().collect();
    
    println!("Loaded {} words from {}", words.len(), input_file);
    
    // Create comprehensive word lists
    let common_english = create_comprehensive_english_words();
    let natural_variants = create_natural_word_variants();
    let combined_words: HashSet<String> = common_english.union(&natural_variants).cloned().collect();
    
    // Filter words with relaxed criteria
    let mut filtered_words = Vec::new();
    let mut filtered_count = 0;
    
    for word in words {
        let word = word.trim().to_lowercase();
        
        // Allow 3-10 characters as requested
        if word.len() < 3 || word.len() > 10 {
            filtered_count += 1;
            continue;
        }
        
        // Check if it's a good readable word
        if is_natural_readable(&word) && (combined_words.contains(&word) || is_obviously_readable(&word)) {
            filtered_words.push(word);
        } else {
            filtered_count += 1;
        }
    }
    
    // Add any missing common words that weren't in the original list
    for word in &combined_words {
        if word.len() >= 3 && word.len() <= 10 && !filtered_words.contains(word) && is_natural_readable(word) {
            filtered_words.push(word.clone());
        }
    }
    
    println!("Filtered out {} unreadable words", filtered_count);
    println!("Kept {} readable words", filtered_words.len());
    
    // Sort alphabetically for consistency
    filtered_words.sort();
    filtered_words.dedup(); // Remove duplicates
    
    // If we have fewer than 65536 words, add more quality variants
    if filtered_words.len() < 65536 {
        let needed = 65536 - filtered_words.len();
        println!("Need {} more words, generating natural variants...", needed);
        
        let variants = generate_natural_variants(&filtered_words, needed);
        filtered_words.extend(variants);
        filtered_words.sort();
        filtered_words.dedup(); // Remove duplicates
    }
    
    // Truncate to exactly 65536 if we have too many
    filtered_words.truncate(65536);
    
    // Write the natural readable dictionary
    let output_file = "data/natural_readable_word_list_65k.txt";
    let output = filtered_words.join("\n");
    fs::write(output_file, output)?;
    
    println!("Created natural readable dictionary with {} words in {}", filtered_words.len(), output_file);
    
    // Show first 30 words as examples
    println!("\nFirst 30 words:");
    for (i, word) in filtered_words.iter().take(30).enumerate() {
        println!("  {}: {}", i, word);
    }
    
    // Show some random samples from different parts
    println!("\nSample words from middle:");
    let middle_start = filtered_words.len() / 2;
    for (i, word) in filtered_words.iter().skip(middle_start).take(10).enumerate() {
        println!("  {}: {}", middle_start + i, word);
    }
    
    Ok(())
}

fn create_comprehensive_english_words() -> HashSet<String> {
    let words = [
        // Basic common words (3-10 chars)
        "the", "and", "you", "that", "was", "for", "are", "with", "his", "they",
        "all", "any", "can", "had", "her", "has", "one", "our", "out", "day",
        "get", "use", "man", "new", "now", "way", "may", "say", "each", "which",
        "she", "how", "its", "who", "oil", "sit", "but", "not", "what", "when",
        
        // Animals
        "cat", "cats", "dog", "dogs", "bird", "birds", "fish", "fishes", "mouse", "mice",
        "horse", "horses", "cow", "cows", "pig", "pigs", "sheep", "chicken", "chickens",
        "duck", "ducks", "goose", "geese", "rabbit", "rabbits", "deer", "bear", "bears",
        "lion", "lions", "tiger", "tigers", "elephant", "elephants", "monkey", "monkeys",
        
        // Nature
        "tree", "trees", "flower", "flowers", "grass", "mountain", "mountains", "river", "rivers",
        "lake", "lakes", "ocean", "oceans", "sea", "seas", "rock", "rocks", "stone", "stones",
        "hill", "hills", "valley", "valleys", "forest", "forests", "beach", "beaches",
        "cloud", "clouds", "rain", "snow", "wind", "storm", "storms", "sunny", "cloudy",
        "leaf", "leaves", "branch", "branches", "root", "roots", "seed", "seeds",
        
        // Body parts
        "head", "heads", "face", "faces", "eye", "eyes", "ear", "ears", "nose", "noses",
        "mouth", "mouths", "hand", "hands", "finger", "fingers", "arm", "arms", "leg", "legs",
        "foot", "feet", "knee", "knees", "shoulder", "shoulders", "back", "chest", "heart", "hearts",
        
        // Actions (present)
        "run", "walk", "jump", "sit", "stand", "look", "see", "hear", "feel", "think",
        "know", "give", "take", "make", "come", "bring", "carry", "hold", "throw", "catch",
        "eat", "drink", "sleep", "wake", "work", "play", "sing", "dance", "laugh", "cry",
        "read", "write", "draw", "paint", "cook", "clean", "wash", "drive", "ride", "fly",
        "swim", "climb", "build", "break", "open", "close", "push", "pull", "lift", "drop",
        
        // Actions (past)
        "ran", "walked", "jumped", "sat", "stood", "looked", "saw", "heard", "felt", "thought",
        "knew", "gave", "took", "made", "came", "brought", "carried", "held", "threw", "caught",
        "ate", "drank", "slept", "woke", "worked", "played", "sang", "danced", "laughed", "cried",
        "read", "wrote", "drew", "painted", "cooked", "cleaned", "washed", "drove", "rode", "flew",
        "swam", "climbed", "built", "broke", "opened", "closed", "pushed", "pulled", "lifted", "dropped",
        
        // Actions (ing)
        "running", "walking", "jumping", "sitting", "standing", "looking", "seeing", "hearing",
        "feeling", "thinking", "knowing", "giving", "taking", "making", "coming", "bringing",
        "carrying", "holding", "throwing", "catching", "eating", "drinking", "sleeping", "waking",
        "working", "playing", "singing", "dancing", "laughing", "crying", "reading", "writing",
        "drawing", "painting", "cooking", "cleaning", "washing", "driving", "riding", "flying",
        "swimming", "climbing", "building", "breaking", "opening", "closing", "pushing", "pulling",
        
        // Household items
        "house", "houses", "home", "homes", "room", "rooms", "door", "doors", "window", "windows",
        "table", "tables", "chair", "chairs", "bed", "beds", "sofa", "sofas", "lamp", "lamps",
        "book", "books", "pen", "pens", "pencil", "pencils", "paper", "papers", "bag", "bags",
        "box", "boxes", "cup", "cups", "plate", "plates", "bowl", "bowls", "spoon", "spoons",
        "fork", "forks", "knife", "knives", "bottle", "bottles", "glass", "glasses",
        "phone", "phones", "computer", "computers", "screen", "screens", "keyboard", "keyboards",
        
        // Food
        "bread", "milk", "water", "juice", "coffee", "tea", "sugar", "salt", "pepper",
        "apple", "apples", "orange", "oranges", "banana", "bananas", "grape", "grapes",
        "carrot", "carrots", "potato", "potatoes", "tomato", "tomatoes", "onion", "onions",
        "cheese", "butter", "egg", "eggs", "meat", "chicken", "beef", "pork", "rice", "pasta",
        
        // Colors
        "red", "blue", "green", "yellow", "orange", "purple", "pink", "brown", "black", "white",
        "gray", "grey", "silver", "gold", "dark", "light", "bright", "pale",
        
        // Numbers
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "ten",
        "eleven", "twelve", "thirteen", "fourteen", "fifteen", "sixteen", "seventeen", "eighteen", "nineteen", "twenty",
        "thirty", "forty", "fifty", "sixty", "seventy", "eighty", "ninety", "hundred", "thousand",
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth", "tenth",
        
        // Time
        "time", "times", "day", "days", "week", "weeks", "month", "months", "year", "years",
        "hour", "hours", "minute", "minutes", "second", "seconds", "morning", "mornings",
        "afternoon", "evening", "evenings", "night", "nights", "today", "tomorrow", "yesterday",
        "monday", "tuesday", "wednesday", "thursday", "friday", "saturday", "sunday",
        "january", "february", "march", "april", "june", "july", "august", "september", "october", "november", "december",
        
        // Weather
        "weather", "sun", "moon", "star", "stars", "sky", "skies", "air", "fire", "earth",
        "hot", "cold", "warm", "cool", "wet", "dry", "windy", "rainy", "snowy", "foggy",
        
        // Adjectives
        "big", "small", "large", "little", "huge", "tiny", "long", "short", "tall", "wide",
        "narrow", "thick", "thin", "heavy", "light", "strong", "weak", "fast", "slow", "quick",
        "old", "new", "young", "fresh", "clean", "dirty", "smooth", "rough", "soft", "hard",
        "easy", "difficult", "simple", "complex", "safe", "dangerous", "quiet", "loud", "calm",
        "happy", "sad", "angry", "excited", "tired", "hungry", "thirsty", "sick", "healthy",
        "beautiful", "ugly", "pretty", "nice", "good", "bad", "great", "terrible", "amazing",
        "wonderful", "perfect", "awful", "excellent", "poor", "rich", "cheap", "expensive",
        
        // People
        "person", "people", "man", "men", "woman", "women", "child", "children", "baby", "babies",
        "boy", "boys", "girl", "girls", "family", "families", "parent", "parents", "mother", "mothers",
        "father", "fathers", "brother", "brothers", "sister", "sisters", "friend", "friends",
        "teacher", "teachers", "student", "students", "doctor", "doctors", "nurse", "nurses",
        "driver", "drivers", "worker", "workers", "player", "players", "singer", "singers",
        
        // Places
        "place", "places", "city", "cities", "town", "towns", "country", "countries", "world",
        "school", "schools", "store", "stores", "shop", "shops", "market", "markets", "park", "parks",
        "church", "churches", "hospital", "hospitals", "library", "libraries", "restaurant", "restaurants",
        "hotel", "hotels", "office", "offices", "bank", "banks", "station", "stations",
        "airport", "airports", "road", "roads", "street", "streets", "bridge", "bridges",
        
        // Transportation
        "car", "cars", "truck", "trucks", "bus", "buses", "train", "trains", "plane", "planes",
        "boat", "boats", "ship", "ships", "bike", "bikes", "bicycle", "bicycles", "motorcycle", "motorcycles",
        "wheel", "wheels", "engine", "engines", "fuel", "gas", "oil", "battery", "batteries",
        
        // Technology
        "machine", "machines", "tool", "tools", "device", "devices", "system", "systems",
        "program", "programs", "software", "hardware", "internet", "website", "websites",
        "email", "emails", "message", "messages", "video", "videos", "photo", "photos",
        "camera", "cameras", "radio", "radios", "television", "televisions", "monitor", "monitors",
        
        // Materials
        "wood", "metal", "plastic", "glass", "paper", "cloth", "leather", "rubber", "concrete",
        "steel", "iron", "copper", "aluminum", "silver", "gold", "diamond", "crystal",
        
        // Abstract concepts
        "idea", "ideas", "thought", "thoughts", "dream", "dreams", "hope", "hopes", "fear", "fears",
        "love", "hate", "anger", "joy", "peace", "war", "truth", "lie", "lies", "fact", "facts",
        "story", "stories", "news", "information", "knowledge", "wisdom", "skill", "skills",
        "power", "energy", "force", "strength", "speed", "sound", "sounds", "music", "song", "songs",
        
        // Common verbs in different forms
        "help", "helps", "helped", "helping", "want", "wants", "wanted", "wanting",
        "need", "needs", "needed", "needing", "like", "likes", "liked", "liking",
        "love", "loves", "loved", "loving", "hate", "hates", "hated", "hating",
        "try", "tries", "tried", "trying", "start", "starts", "started", "starting",
        "stop", "stops", "stopped", "stopping", "finish", "finishes", "finished", "finishing",
        "begin", "begins", "began", "beginning", "end", "ends", "ended", "ending",
        "change", "changes", "changed", "changing", "move", "moves", "moved", "moving",
        "turn", "turns", "turned", "turning", "grow", "grows", "grew", "growing",
        "learn", "learns", "learned", "learning", "teach", "teaches", "taught", "teaching",
        "show", "shows", "showed", "showing", "tell", "tells", "told", "telling",
        "ask", "asks", "asked", "asking", "answer", "answers", "answered", "answering",
        "call", "calls", "called", "calling", "talk", "talks", "talked", "talking",
        "speak", "speaks", "spoke", "speaking", "listen", "listens", "listened", "listening",
        "watch", "watches", "watched", "watching", "wait", "waits", "waited", "waiting",
        "follow", "follows", "followed", "following", "lead", "leads", "led", "leading",
        "join", "joins", "joined", "joining", "leave", "leaves", "left", "leaving",
        "stay", "stays", "stayed", "staying", "arrive", "arrives", "arrived", "arriving",
        "return", "returns", "returned", "returning", "visit", "visits", "visited", "visiting",
        "travel", "travels", "traveled", "traveling", "trip", "trips", "journey", "journeys",
    ];
    
    words.iter().map(|&s| s.to_string()).collect()
}

fn create_natural_word_variants() -> HashSet<String> {
    // Additional natural word forms and common English words
    let variants = [
        // Comparative and superlative forms
        "bigger", "biggest", "smaller", "smallest", "larger", "largest", "longer", "longest",
        "shorter", "shortest", "taller", "tallest", "faster", "fastest", "slower", "slowest",
        "older", "oldest", "newer", "newest", "better", "best", "worse", "worst",
        "higher", "highest", "lower", "lowest", "stronger", "strongest", "weaker", "weakest",
        
        // Agent nouns (doer forms)
        "runner", "runners", "walker", "walkers", "swimmer", "swimmers", "climber", "climbers",
        "builder", "builders", "maker", "makers", "writer", "writers", "reader", "readers",
        "speaker", "speakers", "listener", "listeners", "helper", "helpers", "teacher", "teachers",
        
        // Past participles and adjectives
        "broken", "written", "spoken", "taken", "given", "driven", "hidden", "chosen",
        "frozen", "stolen", "forgotten", "beaten", "eaten", "drunk", "sung", "rung",
        
        // Adverbs
        "quickly", "slowly", "carefully", "easily", "hardly", "nearly", "really", "actually",
        "probably", "possibly", "certainly", "definitely", "absolutely", "completely", "totally",
        "partly", "mostly", "usually", "normally", "regularly", "frequently", "rarely", "never",
        
        // Common compound words
        "something", "anything", "nothing", "everything", "someone", "anyone", "everyone",
        "somewhere", "anywhere", "everywhere", "sometimes", "anytime", "everyday", "somebody",
        "anybody", "everybody", "nobody", "playground", "classroom", "bedroom", "bathroom",
        "kitchen", "living", "dining", "backyard", "driveway", "sidewalk", "highway",
        "highway", "railway", "airport", "seaport", "downtown", "uptown", "outdoor", "indoor",
        
        // Common technology words
        "email", "website", "internet", "computer", "keyboard", "monitor", "printer", "scanner",
        "software", "hardware", "password", "username", "download", "upload", "online", "offline",
        
        // Common expressions and phrases (single words)
        "hello", "goodbye", "please", "thanks", "welcome", "excuse", "sorry", "pardon",
        "yes", "yeah", "okay", "alright", "sure", "maybe", "perhaps", "probably",
        
        // Weather and seasons
        "spring", "summer", "autumn", "winter", "season", "seasons", "sunshine", "moonlight",
        "rainbow", "rainbows", "thunder", "lightning", "breeze", "hurricane", "tornado",
        
        // Education and work
        "homework", "classroom", "textbook", "notebook", "pencil", "eraser", "ruler",
        "calculator", "laptop", "tablet", "smartphone", "application", "program", "file",
        
        // Sports and activities
        "football", "basketball", "baseball", "tennis", "golf", "swimming", "running",
        "cycling", "hiking", "camping", "fishing", "hunting", "dancing", "singing",
        "painting", "drawing", "reading", "writing", "cooking", "baking", "gardening",
        
        // Emotions and feelings
        "happiness", "sadness", "anger", "fear", "surprise", "disgust", "excitement",
        "nervousness", "anxiety", "worry", "stress", "relief", "comfort", "pleasure",
        
        // Common objects
        "television", "refrigerator", "microwave", "telephone", "calculator", "calendar",
        "dictionary", "magazine", "newspaper", "envelope", "package", "suitcase", "backpack",
        "umbrella", "sunglasses", "headphones", "earphones", "charger", "battery",
    ];
    
    variants.iter().map(|&s| s.to_string()).collect()
}

fn is_natural_readable(word: &str) -> bool {
    // More relaxed readability checks for 3-10 character words
    
    // Must be 3-10 characters as requested
    if word.len() < 3 || word.len() > 10 {
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
    
    // Cannot have too many consonants in a row (relaxed to 4)
    let mut consonant_streak = 0;
    for ch in word.chars() {
        if "bcdfghjklmnpqrstvwxyz".contains(ch) {
            consonant_streak += 1;
            if consonant_streak > 4 {
                return false;
            }
        } else {
            consonant_streak = 0;
        }
    }
    
    // Filter out obviously foreign patterns (basic check)
    let foreign_patterns = ["xz", "qx", "xq", "zx", "zzzz", "xxxx"];
    for pattern in &foreign_patterns {
        if word.contains(pattern) {
            return false;
        }
    }
    
    true
}

fn is_obviously_readable(word: &str) -> bool {
    // Words that are obviously readable even if not in our main lists
    
    // Common English word patterns
    if word.ends_with("ing") && word.len() > 4 {
        return true;
    }
    if word.ends_with("ed") && word.len() > 3 {
        return true;
    }
    if word.ends_with("er") && word.len() > 3 {
        return true;
    }
    if word.ends_with("ly") && word.len() > 3 {
        return true;
    }
    if word.ends_with("s") && word.len() > 2 {
        return true; // Likely plural
    }
    
    // Common prefixes
    if word.starts_with("un") && word.len() > 3 {
        return true;
    }
    if word.starts_with("re") && word.len() > 3 {
        return true;
    }
    
    false
}

fn generate_natural_variants(base_words: &[String], needed: usize) -> Vec<String> {
    let mut variants = Vec::new();
    let mut added = 0;
    
    // Generate natural variants from base words
    for word in base_words {
        if added >= needed {
            break;
        }
        
        // Skip words that already have variants
        if word.ends_with("s") || word.ends_with("ed") || word.ends_with("ing") || word.ends_with("er") {
            continue;
        }
        
        // Generate plural
        let plural = if word.ends_with("y") && word.len() > 3 {
            format!("{}ies", &word[..word.len()-1])
        } else if word.ends_with("s") || word.ends_with("sh") || word.ends_with("ch") || word.ends_with("x") || word.ends_with("z") {
            format!("{}es", word)
        } else {
            format!("{}s", word)
        };
        
        if plural.len() <= 10 && is_natural_readable(&plural) && !base_words.contains(&plural) {
            variants.push(plural);
            added += 1;
            if added >= needed { break; }
        }
        
        // Generate -ing form for verbs
        let ing_form = if word.ends_with("e") && word.len() > 3 {
            format!("{}ing", &word[..word.len()-1])
        } else {
            format!("{}ing", word)
        };
        
        if ing_form.len() <= 10 && is_natural_readable(&ing_form) && !base_words.contains(&ing_form) {
            variants.push(ing_form);
            added += 1;
            if added >= needed { break; }
        }
        
        // Generate -ed form for verbs
        let ed_form = if word.ends_with("e") {
            format!("{}d", word)
        } else {
            format!("{}ed", word)
        };
        
        if ed_form.len() <= 10 && is_natural_readable(&ed_form) && !base_words.contains(&ed_form) {
            variants.push(ed_form);
            added += 1;
            if added >= needed { break; }
        }
        
        // Generate -er form
        let er_form = if word.ends_with("e") {
            format!("{}r", word)
        } else {
            format!("{}er", word)
        };
        
        if er_form.len() <= 10 && is_natural_readable(&er_form) && !base_words.contains(&er_form) {
            variants.push(er_form);
            added += 1;
            if added >= needed { break; }
        }
    }
    
    variants
}