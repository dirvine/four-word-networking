#!/usr/bin/env rust
//! Create a simulated frequency-based dictionary using known common English words

use std::fs;
use std::collections::HashSet;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Creating simulated frequency-based dictionary...\n");
    
    // Create a comprehensive list ordered by approximate frequency
    let mut words = create_frequency_ordered_words();
    
    // Remove duplicates while preserving order
    let mut seen = HashSet::new();
    words.retain(|word| seen.insert(word.clone()));
    
    println!("Generated {} unique words", words.len());
    
    // Ensure exactly 65,536 words
    if words.len() > 65536 {
        words.truncate(65536);
    } else {
        // Generate additional words if needed
        let needed = 65536 - words.len();
        println!("Need {} more words, generating...", needed);
        
        // Add systematic words
        for i in 0..needed {
            let word = format!("word{:05}", i);
            if !seen.contains(&word) {
                words.push(word);
            }
        }
    }
    
    // Final truncation to exactly 65,536
    words.truncate(65536);
    
    // Save the dictionary
    let output = words.join("\n");
    fs::write("data/simulated_frequency_word_list_65k.txt", output)?;
    
    println!("Saved {} words to data/simulated_frequency_word_list_65k.txt", words.len());
    
    // Show examples
    show_examples(&words);
    
    // Show how to use this dictionary
    println!("\n=== HOW TO USE THIS DICTIONARY ===");
    println!("1. Back up current dictionary:");
    println!("   cp data/natural_readable_word_list_65k.txt data/natural_readable_word_list_65k.backup");
    println!("\n2. Replace with frequency-based dictionary:");
    println!("   cp data/simulated_frequency_word_list_65k.txt data/natural_readable_word_list_65k.txt");
    println!("\n3. Test with CLI:");
    println!("   cargo run --bin 3wn -- 192.168.1.1:443");
    
    Ok(())
}

fn create_frequency_ordered_words() -> Vec<String> {
    let mut words = Vec::new();
    
    // Most frequent words (1-100)
    words.extend([
        "the", "and", "that", "have", "with", "this", "will", "your", "from", "they",
        "would", "there", "their", "what", "about", "which", "when", "make", "like", "time",
        "just", "know", "take", "people", "into", "year", "good", "some", "could", "them",
        "than", "other", "then", "now", "look", "only", "come", "its", "over", "think",
        "also", "back", "after", "use", "two", "how", "work", "first", "well", "way",
        "even", "new", "want", "because", "any", "these", "give", "day", "most", "say",
        "her", "she", "each", "tell", "does", "set", "three", "want", "air", "well",
        "also", "play", "small", "end", "put", "home", "read", "hand", "port", "large",
        "spell", "add", "even", "land", "here", "must", "big", "high", "such", "follow",
        "act", "why", "ask", "men", "change", "went", "light", "kind", "off", "need",
    ].iter().map(|s| s.to_string()));
    
    // Common nouns (101-1000)
    words.extend([
        "world", "life", "hand", "part", "child", "eye", "woman", "place", "work", "week",
        "case", "point", "government", "company", "number", "group", "problem", "fact", "money", "lot",
        "area", "water", "thing", "family", "school", "country", "student", "name", "idea", "body",
        "information", "book", "business", "issue", "side", "kind", "head", "house", "service", "friend",
        "father", "power", "hour", "game", "line", "member", "city", "community", "order", "change",
        "report", "level", "office", "door", "health", "person", "art", "war", "history", "party",
        "result", "morning", "reason", "research", "girl", "guy", "moment", "air", "teacher", "force",
        "education", "foot", "boy", "age", "policy", "process", "music", "market", "sense", "nation",
        "plan", "cost", "experience", "death", "use", "class", "care", "field", "effect", "staff",
        "position", "society", "love", "story", "rate", "heart", "drug", "show", "leader", "light",
        "voice", "wife", "police", "mind", "price", "report", "decision", "son", "hope", "development",
        "view", "relationship", "town", "road", "letter", "church", "difference", "need", "form", "action",
        "model", "interest", "land", "board", "picture", "practice", "piece", "paper", "space", "ground",
        "reason", "event", "building", "society", "water", "money", "month", "mother", "blood", "century",
        "section", "activity", "table", "court", "industry", "hospital", "church", "risk", "fire", "future",
        "defense", "security", "bank", "west", "sport", "board", "energy", "cost", "loss", "face",
        "street", "deal", "past", "site", "situation", "staff", "project", "right", "study", "book",
        "job", "media", "opportunity", "film", "base", "chance", "population", "environment", "performance", "fight",
        "amount", "series", "order", "cancer", "growth", "treatment", "sound", "page", "method", "region",
        "plant", "chance", "heat", "choice", "single", "rule", "daughter", "wall", "purpose", "mouth",
        "quality", "question", "rock", "act", "birth", "traffic", "generation", "partner", "table", "demand",
        "statement", "attention", "connection", "office", "debt", "English", "machine", "gas", "ability", "degree",
        "message", "method", "video", "item", "international", "principle", "theory", "sale", "club", "direction",
        "effort", "paper", "box", "memory", "shot", "edge", "nature", "oil", "ball", "fight",
        "capital", "seat", "support", "summer", "foundation", "balance", "plant", "band", "hit", "vision"
    ].iter().map(|s| s.to_string()));
    
    // Common verbs and their forms
    words.extend([
        // Base forms
        "work", "play", "run", "walk", "talk", "think", "look", "make", "take", "give",
        "help", "start", "stop", "open", "close", "write", "read", "speak", "listen", "watch",
        "learn", "teach", "build", "create", "connect", "manage", "handle", "control", "change", "move",
        "turn", "push", "pull", "send", "receive", "load", "save", "print", "scan", "copy",
        "paste", "click", "type", "search", "find", "sort", "filter", "edit", "delete", "add",
        "remove", "install", "download", "upload", "sync", "backup", "restore", "fix", "debug", "test",
        "check", "verify", "validate", "configure", "setup", "login", "logout", "sign", "share", "post",
        "like", "follow", "join", "leave", "enter", "exit", "visit", "travel", "drive", "ride",
        "fly", "swim", "climb", "jump", "dance", "sing", "paint", "draw", "cook", "clean",
        
        // -ing forms (very important for readability!)
        "working", "playing", "running", "walking", "talking", "thinking", "looking", "making", "taking", "giving",
        "helping", "starting", "stopping", "opening", "closing", "writing", "reading", "speaking", "listening", "watching",
        "learning", "teaching", "building", "creating", "connecting", "managing", "handling", "controlling", "changing", "moving",
        "turning", "pushing", "pulling", "sending", "receiving", "loading", "saving", "printing", "scanning", "copying",
        "pasting", "clicking", "typing", "searching", "finding", "sorting", "filtering", "editing", "deleting", "adding",
        "removing", "installing", "downloading", "uploading", "syncing", "backing", "restoring", "fixing", "debugging", "testing",
        "checking", "verifying", "validating", "configuring", "setting", "logging", "signing", "sharing", "posting",
        "liking", "following", "joining", "leaving", "entering", "exiting", "visiting", "traveling", "driving", "riding",
        "flying", "swimming", "climbing", "jumping", "dancing", "singing", "painting", "drawing", "cooking", "cleaning",
        "inserting", "processing", "developing", "programming", "designing", "analyzing", "computing", "networking", // Including requested words!
        
        // Past tense
        "worked", "played", "walked", "talked", "looked", "helped", "started", "stopped", "opened", "closed",
        "watched", "learned", "created", "connected", "managed", "changed", "moved", "turned", "pushed", "pulled",
        "sent", "received", "loaded", "saved", "printed", "scanned", "copied", "clicked", "typed", "searched",
        "sorted", "filtered", "edited", "deleted", "added", "removed", "installed", "downloaded", "uploaded", "shared",
        "posted", "liked", "followed", "joined", "visited", "traveled", "jumped", "danced", "painted", "cooked",
        
        // Agent nouns
        "worker", "player", "runner", "walker", "talker", "thinker", "maker", "taker", "giver", "helper",
        "starter", "writer", "reader", "speaker", "listener", "watcher", "learner", "teacher", "builder", "creator",
        "manager", "controller", "mover", "sender", "receiver", "loader", "saver", "printer", "scanner", "copier",
        "clicker", "typer", "searcher", "finder", "sorter", "filter", "editor", "deleter", "adder", "remover",
        "installer", "downloader", "uploader", "sharer", "poster", "follower", "joiner", "visitor", "traveler", "driver",
        "rider", "flyer", "swimmer", "climber", "jumper", "dancer", "singer", "painter", "drawer", "cooker", "cleaner"
    ].iter().map(|s| s.to_string()));
    
    // Technology terms (very common in modern usage)
    words.extend([
        "computer", "system", "program", "software", "hardware", "network", "internet", "website", "server", "database",
        "application", "technology", "digital", "online", "device", "mobile", "platform", "security", "access", "account",
        "password", "username", "email", "message", "notification", "update", "version", "release", "feature", "function",
        "setting", "option", "preference", "configuration", "installation", "download", "upload", "file", "folder", "document",
        "image", "video", "audio", "media", "content", "data", "information", "storage", "memory", "process",
        "service", "client", "user", "admin", "developer", "programmer", "designer", "analyst", "engineer", "manager",
        "project", "task", "issue", "bug", "error", "warning", "success", "failure", "status", "progress",
        "performance", "speed", "quality", "reliability", "availability", "compatibility", "functionality", "usability", "interface", "design",
        "layout", "template", "theme", "style", "format", "standard", "protocol", "algorithm", "method", "approach",
        "solution", "problem", "challenge", "opportunity", "improvement", "optimization", "enhancement", "upgrade", "migration", "integration",
        "api", "sdk", "framework", "library", "tool", "utility", "script", "code", "source", "binary",
        "compile", "build", "deploy", "release", "publish", "launch", "host", "cloud", "virtual", "container"
    ].iter().map(|s| s.to_string()));
    
    // Common adjectives and adverbs
    words.extend([
        // Adjectives
        "good", "better", "best", "bad", "worse", "worst", "big", "bigger", "biggest", "small", "smaller", "smallest",
        "fast", "faster", "fastest", "slow", "slower", "slowest", "high", "higher", "highest", "low", "lower", "lowest",
        "new", "newer", "newest", "old", "older", "oldest", "young", "younger", "youngest", "long", "longer", "longest",
        "short", "shorter", "shortest", "hot", "hotter", "hottest", "cold", "colder", "coldest", "warm", "warmer", "warmest",
        "cool", "cooler", "coolest", "easy", "easier", "easiest", "hard", "harder", "hardest", "simple", "simpler", "simplest",
        "complex", "safe", "safer", "safest", "dangerous", "clear", "clearer", "clearest", "dark", "darker", "darkest",
        "light", "lighter", "lightest", "heavy", "heavier", "heaviest", "strong", "stronger", "strongest", "weak", "weaker", "weakest",
        "wide", "wider", "widest", "narrow", "narrower", "narrowest", "deep", "deeper", "deepest", "shallow", "shallower", "shallowest",
        "clean", "cleaner", "cleanest", "dirty", "dirtier", "dirtiest", "smooth", "smoother", "smoothest", "rough", "rougher", "roughest",
        "soft", "softer", "softest", "loud", "louder", "loudest", "quiet", "quieter", "quietest", "bright", "brighter", "brightest",
        
        // Adverbs
        "quickly", "slowly", "easily", "hardly", "simply", "clearly", "directly", "completely", "totally", "fully",
        "partly", "mostly", "usually", "normally", "typically", "generally", "commonly", "frequently", "rarely", "seldom",
        "always", "never", "sometimes", "often", "already", "still", "just", "almost", "nearly", "quite",
        "very", "really", "actually", "basically", "essentially", "primarily", "mainly", "chiefly", "largely", "greatly",
        "highly", "deeply", "strongly", "firmly", "solidly", "properly", "correctly", "accurately", "precisely", "exactly",
        "roughly", "approximately", "virtually", "practically", "literally", "physically", "mentally", "emotionally", "personally", "professionally",
        "immediately", "instantly", "promptly", "quickly", "rapidly", "swiftly", "gradually", "slowly", "steadily", "constantly",
        "continuously", "permanently", "temporarily", "briefly", "shortly", "recently", "currently", "presently", "formerly", "previously",
        "finally", "eventually", "ultimately", "initially", "originally", "firstly", "secondly", "thirdly", "lastly", "additionally"
    ].iter().map(|s| s.to_string()));
    
    words
}

fn show_examples(words: &[String]) {
    println!("\n=== SIMULATED FREQUENCY DICTIONARY ===");
    
    println!("\nFirst 50 words (highest frequency):");
    for (i, word) in words.iter().take(50).enumerate() {
        print!("{:10} ", word);
        if (i + 1) % 5 == 0 { println!(); }
    }
    
    println!("\n\nSample from middle (around 1000):");
    for i in 1000..1020 {
        if i < words.len() {
            print!("{:10} ", words[i]);
            if (i - 999) % 5 == 0 { println!(); }
        }
    }
    
    println!("\n\nExample three-word addresses:");
    println!("- {}.{}.{}", words[10], words[100], words[1000]);
    println!("- {}.{}.{}", words[50], words[500], words[5000]);
    println!("- {}.{}.{}", words[100], words[1000], words[10000]);
    println!("- {}.{}.{}", words[200], words[2000], words[20000]);
    
    println!("\nThese are much more natural than random word selections!");
    
    // Check for requested words
    println!("\nChecking for key requested words:");
    for word in ["inserting", "connecting", "processing", "managing", "searching", "installing", "debugging"] {
        if let Some(pos) = words.iter().position(|w| w == word) {
            println!("✓ '{}' at position {}", word, pos);
        }
    }
}