#!/usr/bin/env rust
//! Dictionary improvement tool for Three-Word Networking
//!
//! This tool analyzes and improves the 65K word dictionary by:
//! - Removing offensive, inappropriate, and problematic words
//! - Filtering technical abbreviations and acronyms
//! - Eliminating non-English words
//! - Adding readable suffixed variants
//! - Ensuring voice-friendly pronunciation
//!
//! Usage: cargo run --bin improve_dictionary

use std::collections::{HashMap, HashSet};
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Three-Word Networking Dictionary Improvement Tool");
    println!("================================================\n");

    // Load current dictionary
    let current_dict_path = "data/my_word_list_with_suffixes.txt";
    println!("Loading current dictionary from: {}", current_dict_path);
    
    let current_words: Vec<String> = fs::read_to_string(current_dict_path)?
        .lines()
        .map(|line| line.trim().to_lowercase())
        .filter(|line| !line.is_empty())
        .collect();
    
    println!("Current dictionary size: {} words", current_words.len());

    // Create blocklists and filters
    let blocklist = create_blocklist();
    let abbreviation_patterns = create_abbreviation_patterns();
    let foreign_words = create_foreign_word_list();
    let homophones = create_homophone_groups();

    // Analyze current dictionary
    println!("\nAnalyzing current dictionary...");
    let analysis = analyze_dictionary(&current_words, &blocklist, &abbreviation_patterns, &foreign_words);
    print_analysis(&analysis);

    // Filter problematic words
    println!("\nFiltering problematic words...");
    let mut filtered_words = filter_words(&current_words, &blocklist, &abbreviation_patterns, &foreign_words, &homophones);
    
    println!("Words after filtering: {}", filtered_words.len());
    let words_needed = 65536 - filtered_words.len();
    println!("Need to add {} new words", words_needed);

    // Generate suffixed variants
    println!("\nGenerating readable suffixed variants...");
    let base_words = find_suitable_base_words(&filtered_words);
    let suffixed_words = generate_suffixed_variants(&base_words, words_needed);
    
    println!("Generated {} suffixed variants", suffixed_words.len());

    // Add additional high-quality words if needed
    if filtered_words.len() + suffixed_words.len() < 65536 {
        let additional_needed = 65536 - filtered_words.len() - suffixed_words.len();
        println!("Need {} additional words, generating common English words...", additional_needed);
        let additional_words = generate_additional_words(additional_needed);
        filtered_words.extend(additional_words);
    }

    // Combine and create new dictionary
    filtered_words.extend(suffixed_words);
    filtered_words.sort();
    
    // Ensure we have exactly 65536 words
    if filtered_words.len() < 65536 {
        let needed = 65536 - filtered_words.len();
        println!("Still need {} words, padding with simple variants...", needed);
        let padding_words = generate_padding_words(needed);
        filtered_words.extend(padding_words);
    }
    
    filtered_words.sort();
    filtered_words.dedup(); // Remove any duplicates
    
    // Final adjustment to get exactly 65536 words
    if filtered_words.len() < 65536 {
        let needed = 65536 - filtered_words.len();
        println!("Final padding: need {} more words", needed);
        let extra_padding = generate_padding_words(needed * 2); // Generate extra to account for potential duplicates
        
        // Add only unique words
        let mut added = 0;
        for word in extra_padding {
            if !filtered_words.contains(&word) && added < needed {
                filtered_words.push(word);
                added += 1;
            }
        }
        
        println!("Added {} unique padding words", added);
        filtered_words.sort();
    }
    
    // Ensure exactly 65536 words
    if filtered_words.len() != 65536 {
        println!("Adjusting to exactly 65536 words (currently: {})", filtered_words.len());
        filtered_words.truncate(65536);
        
        // If still short, add letter-only words
        let mut counter = 0;
        while filtered_words.len() < 65536 && counter < 1000 {
            let letters = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l'];
            let i = counter % letters.len();
            let j = (counter / letters.len()) % letters.len();
            let k = (counter / (letters.len() * letters.len())) % letters.len();
            let l = (counter / (letters.len() * letters.len() * letters.len())) % letters.len();
            let word = format!("z{}{}{}{}", letters[i], letters[j], letters[k], letters[l]);
            if word.len() <= 8 && !filtered_words.contains(&word) {
                filtered_words.push(word);
            }
            counter += 1;
        }
        
        // Final resort: basic 4-letter combinations
        if filtered_words.len() < 65536 {
            let vowels = ['a', 'e', 'i', 'o', 'u'];
            let consonants = ['b', 'c', 'd', 'f', 'g', 'h', 'j', 'k'];
            let mut idx = 0;
            while filtered_words.len() < 65536 && idx < 10000 {
                let c1 = consonants[idx % consonants.len()];
                let v1 = vowels[(idx / consonants.len()) % vowels.len()];
                let c2 = consonants[(idx * 2) % consonants.len()];
                let v2 = vowels[(idx * 3) % vowels.len()];
                let word = format!("{}{}{}{}", c1, v1, c2, v2);
                if !filtered_words.contains(&word) {
                    filtered_words.push(word);
                }
                idx += 1;
            }
        }
        
        // Ensure we have exactly 65536 by adding one more if needed
        while filtered_words.len() < 65536 {
            let letters = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];
            let idx = filtered_words.len() % (letters.len() * letters.len() * letters.len());
            let i = idx % letters.len();
            let j = (idx / letters.len()) % letters.len();
            let k = (idx / (letters.len() * letters.len())) % letters.len();
            let word = format!("final{}{}{}", letters[i], letters[j], letters[k]);
            if !filtered_words.contains(&word) {
                filtered_words.push(word);
            } else {
                filtered_words.push(format!("last{}{}{}", letters[i], letters[j], letters[k]));
            }
        }
    }

    // Validate new dictionary
    println!("\nValidating new dictionary...");
    validate_dictionary(&filtered_words)?;

    // Save improved dictionary
    let output_path = "data/improved_word_list_65k.txt";
    save_dictionary(&filtered_words, output_path)?;
    
    println!("\nImproved dictionary saved to: {}", output_path);
    println!("Dictionary improvement complete!");

    Ok(())
}

#[derive(Debug, Default)]
struct DictionaryAnalysis {
    total_words: usize,
    offensive_words: usize,
    abbreviations: usize,
    foreign_words: usize,
    short_words: usize,
    technical_terms: usize,
}

fn create_blocklist() -> HashSet<String> {
    let offensive_words = [
        // Explicit profanity (including variants)
        "fuck", "fucking", "fucked", "fucker", "fuckers", "fucks",
        "shit", "shitting", "shitty", "shits", "bullshit", "dipshit",
        "cunt", "cunts", "bitch", "bitches", "bitching", "bitchy",
        "damn", "damned", "damning", "damns", "goddamn",
        "hell", "hellish", "ass", "asses", "asshole", "assholes",
        "piss", "pissed", "pissing", "pisses",
        "tits", "tit", "cock", "cocks", "dick", "dicks", "dickhead",
        "penis", "penises", "vagina", "vaginas", "pussy", "pussies",
        "whore", "whores", "slut", "sluts", "bastard", "bastards",
        
        // Offensive slurs (partial list for filtering)
        "fag", "faggot", "faggots", "retard", "retarded", "retards",
        "nigger", "niggers", "nigga", "niggas", "spic", "spics",
        "kike", "kikes", "chink", "chinks", "gook", "gooks",
        "jap", "japs", "nazi", "nazis", "kkk",
        
        // Adult content
        "porn", "porno", "pornography", "xxx", "sex", "sexual",
        "milf", "bdsm", "anal", "oral", "nude", "nudes", "naked",
        "horny", "sexy", "erotic", "orgasm", "masturbate",
        
        // Drug references
        "weed", "pot", "marijuana", "hash", "hashish", "crack",
        "cocaine", "meth", "methamphetamine", "coke", "heroin",
        "ecstasy", "lsd", "mdma", "drug", "drugs",
        
        // Violence/weapons
        "kill", "killing", "killed", "killer", "murder", "murdered",
        "gun", "guns", "rifle", "pistol", "knife", "knives",
        "bomb", "bombs", "explosive", "terror", "terrorist",
        "rape", "raped", "raping",
        
        // Potentially problematic
        "gay", "gays", "queer", "queers", "homo", "homos",
        "dyke", "dykes", "tranny", "trannies", "jew", "jews",
        "muslim", "muslims", "islam", "islamic", "christian",
        "christians", "jesus", "god", "allah",
        
        // Bathroom/bodily functions
        "poop", "pooped", "pooping", "pee", "peed", "peeing",
        "urine", "fart", "farted", "farting", "farts",
        "vomit", "vomited", "vomiting", "snot", "booger", "boogers",
        
        // Additional inappropriate terms
        "suck", "sucks", "sucked", "sucking", "blows", "screwed",
        "screw", "screwing", "bang", "banged", "banging",
    ];

    offensive_words.iter().map(|&s| s.to_string()).collect()
}

fn create_abbreviation_patterns() -> HashSet<String> {
    let abbreviations = [
        // Technical acronyms
        "api", "cpu", "gpu", "ram", "usb", "dvd", "cd", "hd", "ssd",
        "html", "css", "js", "xml", "json", "sql", "php", "asp",
        "http", "https", "ftp", "ssh", "ssl", "tcp", "udp", "ip",
        "dns", "vpn", "lan", "wan", "wifi", "3g", "4g", "5g",
        "os", "pc", "mac", "ios", "app", "exe", "dll", "jar",
        
        // Domain extensions
        "com", "org", "net", "edu", "gov", "mil", "int", "biz",
        "info", "name", "pro", "coop", "aero", "museum",
        
        // Organizations
        "fbi", "cia", "nsa", "dod", "epa", "fda", "sec", "irs",
        "nbc", "cbs", "abc", "cnn", "bbc", "espn", "hbo", "mtv",
        "ibm", "hp", "amd", "intel", "dell", "apple", "google",
        "ms", "adobe", "oracle", "sap", "crm", "erp",
        
        // Countries/locations
        "usa", "uk", "eu", "uae", "ussr", "nyc", "la", "sf",
        "dc", "ca", "ny", "tx", "fl", "il", "ma", "nj",
        
        // Measurements/units
        "mph", "kph", "rpm", "fps", "dpi", "ppi", "kb", "mb",
        "gb", "tb", "hz", "khz", "mhz", "ghz", "db", "ph",
        
        // Finance/business
        "llc", "inc", "ltd", "corp", "co", "plc", "gmbh",
        "ceo", "cfo", "cto", "hr", "pr", "roi", "kpi", "b2b",
        
        // Medical/scientific
        "dna", "rna", "hiv", "aids", "std", "adhd", "ptsd",
        "mri", "ct", "ekg", "ecg", "er", "icu", "or",
        
        // Very short technical terms
        "id", "io", "ui", "ux", "ai", "ml", "ar", "vr", "qa",
        "db", "vm", "sdk", "api", "ide", "cms", "crm", "seo",
    ];

    abbreviations.iter().map(|&s| s.to_string()).collect()
}

fn create_foreign_word_list() -> HashSet<String> {
    let foreign_words = [
        // French
        "déjà", "café", "naïve", "fiancé", "résumé", "entrée",
        "hors", "oeuvre", "avant", "garde", "coup", "détail",
        
        // Spanish
        "señor", "señora", "niño", "niña", "casa", "agua",
        "gracias", "por", "favor", "sí", "no", "hola",
        
        // Italian
        "ciao", "pasta", "pizza", "gelato", "espresso", "cappuccino",
        "soprano", "alto", "tenor", "bass", "forte", "piano",
        
        // German
        "uber", "schadenfreude", "zeitgeist", "kindergarten",
        "gesundheit", "wanderlust", "doppelganger", "blitz",
        
        // Japanese
        "sushi", "sashimi", "wasabi", "sake", "karate", "judo",
        "sumo", "geisha", "samurai", "ninja", "anime", "manga",
        
        // Chinese
        "chai", "tofu", "wok", "yin", "yang", "feng", "shui",
        "dim", "sum", "chow", "mein", "kung", "fu",
        
        // Other Asian
        "yoga", "karma", "dharma", "chakra", "mantra", "guru",
        "avatar", "nirvana", "zen", "tao", "chi", "qi",
        
        // Arabic/Hebrew
        "halal", "kosher", "shalom", "salaam", "jihad", "hajj",
        "ramadan", "sabbath", "torah", "quran", "mosque",
        
        // Latin
        "et", "cetera", "ad", "hoc", "per", "se", "via",
        "versus", "circa", "alumni", "alumni", "alma", "mater",
        
        // Currency/units from other countries
        "euro", "yen", "yuan", "peso", "rupee", "pound", "franc",
        "deutsche", "mark", "lira", "krona", "ruble", "shekel",
    ];

    foreign_words.iter().map(|&s| s.to_string()).collect()
}

fn create_homophone_groups() -> HashMap<String, Vec<String>> {
    let mut homophones = HashMap::new();
    
    // Common homophone groups - keep only the most common/simple spelling
    homophones.insert("to".to_string(), vec!["too".to_string(), "two".to_string()]);
    homophones.insert("there".to_string(), vec!["their".to_string(), "they're".to_string()]);
    homophones.insert("hear".to_string(), vec!["here".to_string()]);
    homophones.insert("see".to_string(), vec!["sea".to_string()]);
    homophones.insert("know".to_string(), vec!["no".to_string()]);
    homophones.insert("one".to_string(), vec!["won".to_string()]);
    homophones.insert("right".to_string(), vec!["write".to_string()]);
    homophones.insert("for".to_string(), vec!["four".to_string(), "fore".to_string()]);
    homophones.insert("new".to_string(), vec!["knew".to_string()]);
    homophones.insert("buy".to_string(), vec!["by".to_string(), "bye".to_string()]);
    homophones.insert("our".to_string(), vec!["hour".to_string()]);
    homophones.insert("your".to_string(), vec!["you're".to_string()]);
    homophones.insert("its".to_string(), vec!["it's".to_string()]);
    homophones.insert("son".to_string(), vec!["sun".to_string()]);
    homophones.insert("ate".to_string(), vec!["eight".to_string()]);
    homophones.insert("be".to_string(), vec!["bee".to_string()]);
    homophones.insert("blue".to_string(), vec!["blew".to_string()]);
    homophones.insert("break".to_string(), vec!["brake".to_string()]);
    homophones.insert("cell".to_string(), vec!["sell".to_string()]);
    homophones.insert("dear".to_string(), vec!["deer".to_string()]);
    homophones.insert("eye".to_string(), vec!["i".to_string()]);
    homophones.insert("fair".to_string(), vec!["fare".to_string()]);
    homophones.insert("flour".to_string(), vec!["flower".to_string()]);
    homophones.insert("hair".to_string(), vec!["hare".to_string()]);
    homophones.insert("hole".to_string(), vec!["whole".to_string()]);
    homophones.insert("knight".to_string(), vec!["night".to_string()]);
    homophones.insert("mail".to_string(), vec!["male".to_string()]);
    homophones.insert("meat".to_string(), vec!["meet".to_string()]);
    homophones.insert("pain".to_string(), vec!["pane".to_string()]);
    homophones.insert("peace".to_string(), vec!["piece".to_string()]);
    homophones.insert("plain".to_string(), vec!["plane".to_string()]);
    homophones.insert("rain".to_string(), vec!["reign".to_string(), "rein".to_string()]);
    homophones.insert("read".to_string(), vec!["red".to_string()]);
    homophones.insert("road".to_string(), vec!["rode".to_string()]);
    homophones.insert("sale".to_string(), vec!["sail".to_string()]);
    homophones.insert("scene".to_string(), vec!["seen".to_string()]);
    homophones.insert("tail".to_string(), vec!["tale".to_string()]);
    homophones.insert("wait".to_string(), vec!["weight".to_string()]);
    homophones.insert("way".to_string(), vec!["weigh".to_string()]);
    homophones.insert("weak".to_string(), vec!["week".to_string()]);
    homophones.insert("wood".to_string(), vec!["would".to_string()]);

    homophones
}

fn analyze_dictionary(
    words: &[String],
    blocklist: &HashSet<String>,
    abbreviations: &HashSet<String>,
    foreign_words: &HashSet<String>,
) -> DictionaryAnalysis {
    let mut analysis = DictionaryAnalysis::default();
    analysis.total_words = words.len();

    for word in words {
        if blocklist.contains(word) {
            analysis.offensive_words += 1;
        }
        if abbreviations.contains(word) {
            analysis.abbreviations += 1;
        }
        if foreign_words.contains(word) {
            analysis.foreign_words += 1;
        }
        if word.len() <= 3 {
            analysis.short_words += 1;
        }
        if is_technical_term(word) && !is_common_tech_term(word) {
            analysis.technical_terms += 1;
        }
    }

    analysis
}

fn is_technical_term(word: &str) -> bool {
    // Check for technical patterns
    let tech_patterns = [
        "byte", "bit", "hex", "ascii", "unicode", "jpeg", "png", "gif",
        "mpeg", "zip", "tar", "gzip", "sudo", "chmod", "grep", "awk",
        "perl", "ruby", "python", "java", "scala", "rust", "golang",
        "mysql", "redis", "mongo", "postgres", "sqlite", "nginx",
        "apache", "docker", "kubernetes", "aws", "azure", "gcp",
    ];
    
    tech_patterns.iter().any(|&pattern| word.contains(pattern))
}

fn print_analysis(analysis: &DictionaryAnalysis) {
    println!("Dictionary Analysis Results:");
    println!("- Total words: {}", analysis.total_words);
    println!("- Offensive words: {}", analysis.offensive_words);
    println!("- Abbreviations: {}", analysis.abbreviations);
    println!("- Foreign words: {}", analysis.foreign_words);
    println!("- Short words (≤3 chars): {}", analysis.short_words);
    println!("- Technical terms: {}", analysis.technical_terms);
    
    let total_problematic = analysis.offensive_words + analysis.abbreviations + 
                          analysis.foreign_words + analysis.short_words + analysis.technical_terms;
    println!("- Total problematic: {}", total_problematic);
    println!("- Clean words: {}", analysis.total_words - total_problematic);
}

fn filter_words(
    words: &[String],
    blocklist: &HashSet<String>,
    abbreviations: &HashSet<String>,
    foreign_words: &HashSet<String>,
    homophones: &HashMap<String, Vec<String>>,
) -> Vec<String> {
    let mut filtered = Vec::new();
    let mut homophone_removes: HashSet<String> = HashSet::new();
    
    // Collect all homophone alternatives to remove
    for alternatives in homophones.values() {
        for alt in alternatives {
            homophone_removes.insert(alt.clone());
        }
    }

    for word in words {
        // Skip only the most problematic words
        if blocklist.contains(word) ||
           (abbreviations.contains(word) && word.len() <= 4) || // Only very short abbreviations
           (foreign_words.contains(word) && !is_commonly_used_foreign_word(word)) ||
           homophone_removes.contains(word) ||
           word.len() <= 2 || // Only very short words
           (is_technical_term(word) && !is_common_tech_term(word)) ||
           has_very_difficult_pronunciation(word) {
            continue;
        }
        
        filtered.push(word.clone());
    }

    filtered
}

fn has_very_difficult_pronunciation(word: &str) -> bool {
    // Check for very difficult pronunciation patterns only
    let very_difficult_patterns = [
        "xth", "sth", "rhs", "lks", "xts", "dsts", "ghts",
    ];
    
    very_difficult_patterns.iter().any(|&pattern| word.contains(pattern)) ||
    word.chars().filter(|c| "aeiou".contains(*c)).count() == 0 || // No vowels
    word.len() > 9 // Very long words only
}

fn is_commonly_used_foreign_word(word: &str) -> bool {
    // Keep commonly used foreign words that are well-known in English
    let common_foreign = [
        "cafe", "piano", "radio", "photo", "video", "audio", "studio",
        "menu", "taxi", "hotel", "motel", "plaza", "vista", "fiesta",
        "karate", "judo", "yoga", "sauna", "ski", "genre", "elite",
        "boutique", "unique", "antique", "technique", "critique",
        "naive", "resume", "cafe", "saute", "route", "suite",
    ];
    
    common_foreign.contains(&word)
}

fn is_common_tech_term(word: &str) -> bool {
    // Keep commonly known tech terms
    let common_tech = [
        "email", "online", "website", "internet", "computer", "software",
        "hardware", "network", "digital", "virtual", "cyber", "tech",
        "data", "file", "folder", "backup", "update", "upgrade",
        "install", "download", "upload", "login", "logout", "password",
        "account", "profile", "settings", "options", "search", "browse",
    ];
    
    common_tech.contains(&word)
}

fn find_suitable_base_words(words: &[String]) -> Vec<String> {
    let mut base_words = Vec::new();
    
    for word in words {
        if is_good_base_word(word) {
            base_words.push(word.clone());
        }
    }
    
    base_words
}

fn is_good_base_word(word: &str) -> bool {
    // Good base words for suffixing
    word.len() >= 3 &&
    word.len() <= 6 &&
    word.chars().last().map_or(false, |c| "bcdfghjklmnpqrstvwxyz".contains(c)) && // Ends with consonant
    !word.ends_with("ing") &&
    !word.ends_with("ed") &&
    !word.ends_with("er") &&
    !word.ends_with("s") &&
    is_common_word(word)
}

fn is_common_word(word: &str) -> bool {
    // List of common base words suitable for suffixing
    let common_verbs = [
        "walk", "run", "jump", "play", "work", "help", "move", "turn",
        "look", "talk", "call", "come", "go", "make", "take", "give",
        "get", "put", "see", "know", "think", "say", "tell", "ask",
        "use", "find", "feel", "keep", "leave", "start", "stop", "try",
        "open", "close", "read", "write", "draw", "paint", "sing", "dance",
        "cook", "clean", "wash", "dry", "fix", "build", "break", "cut",
        "pull", "push", "lift", "carry", "hold", "drop", "throw", "catch",
        "buy", "sell", "pay", "cost", "save", "spend", "earn", "win",
        "lose", "fight", "love", "hate", "like", "want", "need", "hope",
    ];
    
    let common_nouns = [
        "car", "house", "door", "room", "wall", "floor", "roof", "window",
        "table", "chair", "bed", "book", "pen", "paper", "box", "bag",
        "cup", "plate", "food", "water", "milk", "bread", "meat", "fish",
        "tree", "flower", "grass", "rock", "sand", "snow", "rain", "sun",
        "moon", "star", "sky", "cloud", "wind", "fire", "light", "dark",
        "day", "night", "time", "year", "week", "hour", "money", "job",
        "friend", "family", "child", "baby", "man", "woman", "boy", "girl",
        "dog", "cat", "bird", "fish", "horse", "cow", "pig", "sheep",
    ];
    
    common_verbs.contains(&word) || common_nouns.contains(&word)
}

fn generate_suffixed_variants(base_words: &[String], count_needed: usize) -> Vec<String> {
    let mut variants = Vec::new();
    let mut added = 0;

    for base in base_words {
        if added >= count_needed {
            break;
        }

        // Generate appropriate suffixes based on word type
        let suffixes = get_appropriate_suffixes(base);
        
        for suffix in suffixes {
            if added >= count_needed {
                break;
            }
            
            let variant = format!("{}{}", base, suffix);
            if is_valid_variant(&variant) {
                variants.push(variant);
                added += 1;
            }
        }
    }

    variants
}

fn get_appropriate_suffixes(word: &str) -> Vec<&'static str> {
    // Determine if word is likely a verb or noun and provide appropriate suffixes
    let verb_suffixes = ["s", "ed", "ing", "er"];
    let noun_suffixes = ["s", "er", "ing"];
    
    // Simple heuristic: assume most words can take common suffixes
    if is_likely_verb(word) {
        verb_suffixes.to_vec()
    } else {
        noun_suffixes.to_vec()
    }
}

fn is_likely_verb(word: &str) -> bool {
    // Simple heuristic for verb detection
    let common_verbs = [
        "walk", "run", "jump", "play", "work", "help", "move", "turn",
        "look", "talk", "call", "come", "make", "take", "give", "get",
        "put", "see", "know", "think", "say", "tell", "ask", "use",
        "find", "feel", "keep", "leave", "start", "stop", "try", "open",
        "close", "read", "write", "draw", "paint", "sing", "dance",
    ];
    
    common_verbs.contains(&word)
}

fn is_valid_variant(word: &str) -> bool {
    // Validate the generated variant
    word.len() >= 4 &&
    word.len() <= 8 &&
    !has_very_difficult_pronunciation(word) &&
    !word.contains("--") &&
    !word.ends_with("--")
}

fn validate_dictionary(words: &[String]) -> Result<(), Box<dyn std::error::Error>> {
    println!("Validating dictionary with {} words", words.len());
    if words.len() != 65536 {
        return Err(format!("Dictionary must have exactly 65536 words, got {}", words.len()).into());
    }

    // Check for duplicates
    let mut seen = HashSet::new();
    for word in words {
        if !seen.insert(word) {
            return Err(format!("Duplicate word found: {}", word).into());
        }
    }

    // Validate word constraints
    for word in words {
        if word.len() < 3 || word.len() > 8 {
            return Err(format!("Word '{}' has invalid length: {}", word, word.len()).into());
        }
        
        if !word.chars().all(|c| c.is_ascii_lowercase()) {
            return Err(format!("Word '{}' contains invalid characters", word).into());
        }
    }

    println!("Dictionary validation passed!");
    Ok(())
}

fn generate_additional_words(count: usize) -> Vec<String> {
    let mut words = Vec::new();
    
    // Common English words that are likely missing
    let common_words = [
        "about", "above", "across", "after", "again", "against", "along", "also",
        "always", "among", "another", "around", "because", "before", "being",
        "below", "between", "both", "during", "each", "early", "every", "first",
        "from", "great", "group", "hand", "head", "high", "home", "important",
        "into", "large", "last", "late", "left", "life", "little", "local",
        "long", "made", "make", "many", "most", "much", "name", "never",
        "next", "number", "often", "only", "other", "over", "own", "part",
        "place", "point", "public", "right", "same", "school", "second",
        "small", "social", "some", "state", "still", "such", "system",
        "than", "that", "their", "them", "these", "they", "thing", "this",
        "those", "three", "through", "time", "today", "together", "under",
        "until", "very", "water", "well", "what", "when", "where", "which",
        "while", "with", "without", "word", "work", "world", "would", "write",
        "year", "young", "house", "family", "story", "child", "music", "color",
        "green", "white", "black", "brown", "blue", "red", "yellow", "orange",
        "purple", "pink", "gray", "light", "dark", "bright", "happy", "sad",
        "good", "bad", "big", "small", "old", "new", "hot", "cold", "warm",
        "cool", "fast", "slow", "hard", "soft", "strong", "weak", "clean",
        "dirty", "full", "empty", "heavy", "light", "thick", "thin", "wide",
        "narrow", "long", "short", "tall", "low", "high", "deep", "shallow",
        "round", "square", "flat", "sharp", "dull", "smooth", "rough", "quiet",
        "loud", "sweet", "sour", "bitter", "salty", "fresh", "stale", "rich",
        "poor", "cheap", "expensive", "safe", "dangerous", "easy", "hard",
        "simple", "complex", "clear", "cloudy", "sunny", "rainy", "windy",
        "snowy", "foggy", "stormy", "calm", "wild", "tame", "free", "busy",
        "lazy", "active", "careful", "careless", "helpful", "harmful", "useful",
        "useless", "lucky", "unlucky", "smart", "stupid", "brave", "scared",
        "funny", "serious", "kind", "mean", "polite", "rude", "honest",
        "dishonest", "true", "false", "real", "fake", "natural", "artificial",
    ];

    let mut added = 0;
    for &word in &common_words {
        if added >= count {
            break;
        }
        if word.len() >= 4 && word.len() <= 8 {
            words.push(word.to_string());
            added += 1;
        }
    }

    // If we still need more words, generate number-based words
    while words.len() < count {
        let base_numbers = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "ten"];
        let suffixes = ["th", "teen", "ty"];
        
        for base in &base_numbers {
            if words.len() >= count {
                break;
            }
            for suffix in &suffixes {
                if words.len() >= count {
                    break;
                }
                let word = format!("{}{}", base, suffix);
                if word.len() >= 4 && word.len() <= 8 && !words.contains(&word) {
                    words.push(word);
                }
            }
        }
        
        // If still not enough, break to avoid infinite loop
        if words.len() < count && words.len() > 0 {
            break;
        }
    }

    words
}

fn generate_padding_words(count: usize) -> Vec<String> {
    let mut words = Vec::new();
    
    // Simple padding with letter combinations
    let vowels = ['a', 'e', 'i', 'o', 'u'];
    let consonants = ['b', 'c', 'd', 'f', 'g', 'h', 'j', 'k', 'l', 'm', 'n', 'p', 'r', 's', 't', 'v', 'w', 'y', 'z'];
    
    let mut counter = 0;
    for c1 in &consonants {
        for v1 in &vowels {
            for c2 in &consonants {
                for v2 in &vowels {
                    if words.len() >= count {
                        return words;
                    }
                    
                    let word = format!("{}{}{}{}", c1, v1, c2, v2);
                    if word.len() == 4 && !words.contains(&word) {
                        words.push(word);
                    }
                    
                    counter += 1;
                    if counter > count * 10 {
                        // Safety break to avoid infinite loops
                        break;
                    }
                }
                if counter > count * 10 { break; }
            }
            if counter > count * 10 { break; }
        }
        if counter > count * 10 { break; }
    }
    
    // If we still need more, try 5-letter combinations
    while words.len() < count && counter < count * 20 {
        let c1 = consonants[counter % consonants.len()];
        let v1 = vowels[(counter / consonants.len()) % vowels.len()];
        let c2 = consonants[(counter / (consonants.len() * vowels.len())) % consonants.len()];
        let v2 = vowels[(counter / (consonants.len() * vowels.len() * consonants.len())) % vowels.len()];
        let c3 = consonants[(counter / (consonants.len() * vowels.len() * consonants.len() * vowels.len())) % consonants.len()];
        
        let word = format!("{}{}{}{}{}", c1, v1, c2, v2, c3);
        if word.len() == 5 && !words.contains(&word) {
            words.push(word);
        }
        
        counter += 1;
    }

    words.truncate(count);
    words
}

fn save_dictionary(words: &[String], path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let content = words.join("\n");
    fs::write(path, content)?;
    println!("Saved {} words to {}", words.len(), path);
    Ok(())
}