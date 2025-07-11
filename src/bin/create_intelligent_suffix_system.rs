#!/usr/bin/env rust
//! Create intelligent suffix system with real English word patterns

use std::collections::HashSet;
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Creating intelligent suffix system...");
    
    // Load current dictionary
    let content = fs::read_to_string("data/natural_readable_word_list_65k.txt")?;
    let words: Vec<String> = content.lines().map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).collect();
    
    println!("Loaded {} words", words.len());
    
    // Find high-value base words that need better suffix coverage
    let high_value_bases = find_high_value_base_words(&words);
    
    // Generate natural suffix variants
    let missing_variants = generate_natural_variants(&high_value_bases, &words);
    
    // Find low-quality words to replace
    let low_quality_words = find_low_quality_words(&words);
    
    // Create improvement plan
    let improvement_plan = create_improvement_plan(&missing_variants, &low_quality_words);
    
    // Report and save results
    report_improvement_plan(&improvement_plan);
    save_improvement_plan(&improvement_plan)?;
    
    Ok(())
}

#[derive(Debug, Clone)]
struct HighValueBase {
    _word: String,
    word_type: WordType,
    _existing_forms: Vec<String>,
    missing_forms: Vec<String>,
    quality_score: f64,
}

#[derive(Debug, Clone)]
enum WordType {
    Verb,
    Noun,
    Adjective,
}

#[derive(Debug)]
struct ImprovementPlan {
    words_to_add: Vec<String>,
    words_to_remove: Vec<String>,
    quality_improvements: Vec<(String, String)>, // (remove, add)
}

fn find_high_value_base_words(words: &[String]) -> Vec<HighValueBase> {
    let word_set: HashSet<_> = words.iter().collect();
    let mut bases = Vec::new();
    
    // Define high-value English base words we want complete coverage for
    let high_value_verbs = [
        "work", "play", "run", "walk", "talk", "think", "look", "make", "take", "give",
        "help", "start", "stop", "open", "close", "write", "read", "speak", "listen",
        "watch", "learn", "teach", "build", "create", "connect", "insert", "update",
        "process", "manage", "handle", "control", "change", "move", "turn", "push",
        "pull", "send", "receive", "load", "save", "print", "scan", "copy", "paste",
        "click", "type", "search", "find", "sort", "filter", "edit", "delete", "add",
        "remove", "install", "download", "upload", "sync", "backup", "restore", "fix",
        "debug", "test", "check", "verify", "validate", "configure", "setup", "login",
        "logout", "sign", "share", "post", "like", "follow", "join", "leave", "enter",
        "exit", "visit", "travel", "drive", "ride", "fly", "swim", "climb", "jump"
    ];
    
    let high_value_nouns = [
        "computer", "phone", "message", "email", "file", "folder", "document", "image",
        "video", "audio", "photo", "picture", "music", "song", "movie", "book", "page",
        "text", "word", "letter", "number", "data", "system", "program", "software",
        "hardware", "network", "internet", "website", "server", "database", "table",
        "record", "field", "value", "setting", "option", "feature", "function", "tool",
        "device", "machine", "screen", "keyboard", "mouse", "printer", "scanner",
        "camera", "speaker", "headphone", "battery", "cable", "wire", "button", "switch",
        "window", "door", "house", "room", "kitchen", "bedroom", "bathroom", "office",
        "school", "hospital", "store", "market", "restaurant", "hotel", "airport",
        "station", "street", "road", "bridge", "building", "tower", "city", "town"
    ];
    
    let high_value_adjectives = [
        "good", "bad", "big", "small", "large", "huge", "tiny", "long", "short", "tall",
        "wide", "narrow", "thick", "thin", "heavy", "light", "fast", "slow", "quick",
        "easy", "hard", "simple", "complex", "safe", "dangerous", "clean", "dirty",
        "new", "old", "fresh", "hot", "cold", "warm", "cool", "bright", "dark", "clear",
        "empty", "full", "open", "closed", "active", "passive", "busy", "free", "ready",
        "available", "popular", "famous", "important", "useful", "helpful", "powerful",
        "strong", "weak", "smart", "clever", "beautiful", "pretty", "nice", "ugly",
        "happy", "sad", "angry", "calm", "quiet", "loud", "soft", "hard", "smooth", "rough"
    ];
    
    // Check each high-value base word
    for &verb in &high_value_verbs {
        if word_set.contains(&verb.to_string()) {
            let base = analyze_base_word(verb, WordType::Verb, &word_set);
            if !base.missing_forms.is_empty() {
                bases.push(base);
            }
        }
    }
    
    for &noun in &high_value_nouns {
        if word_set.contains(&noun.to_string()) {
            let base = analyze_base_word(noun, WordType::Noun, &word_set);
            if !base.missing_forms.is_empty() {
                bases.push(base);
            }
        }
    }
    
    for &adj in &high_value_adjectives {
        if word_set.contains(&adj.to_string()) {
            let base = analyze_base_word(adj, WordType::Adjective, &word_set);
            if !base.missing_forms.is_empty() {
                bases.push(base);
            }
        }
    }
    
    // Sort by quality score (prioritize words with highest potential impact)
    bases.sort_by(|a, b| b.quality_score.partial_cmp(&a.quality_score).unwrap());
    
    bases
}

fn analyze_base_word(word: &str, word_type: WordType, word_set: &HashSet<&String>) -> HighValueBase {
    let possible_forms = generate_correct_forms(word, &word_type);
    let mut existing_forms = Vec::new();
    let mut missing_forms = Vec::new();
    
    for form in possible_forms {
        if word_set.contains(&form) {
            existing_forms.push(form);
        } else {
            missing_forms.push(form);
        }
    }
    
    // Score based on how useful this word family would be
    let quality_score = calculate_quality_score(word, &word_type, &existing_forms, &missing_forms);
    
    HighValueBase {
        _word: word.to_string(),
        word_type,
        _existing_forms: existing_forms,
        missing_forms,
        quality_score,
    }
}

fn generate_correct_forms(word: &str, word_type: &WordType) -> Vec<String> {
    let mut forms = Vec::new();
    forms.push(word.to_string()); // Base form
    
    match word_type {
        WordType::Verb => {
            // Use correct English conjugations
            match word {
                // Irregular verbs
                "run" => {
                    forms.extend(["runs", "ran", "running", "runner"].iter().map(|s| s.to_string()));
                },
                "give" => {
                    forms.extend(["gives", "gave", "given", "giving", "giver"].iter().map(|s| s.to_string()));
                },
                "take" => {
                    forms.extend(["takes", "took", "taken", "taking", "taker"].iter().map(|s| s.to_string()));
                },
                "make" => {
                    forms.extend(["makes", "made", "making", "maker"].iter().map(|s| s.to_string()));
                },
                "think" => {
                    forms.extend(["thinks", "thought", "thinking", "thinker"].iter().map(|s| s.to_string()));
                },
                "speak" => {
                    forms.extend(["speaks", "spoke", "spoken", "speaking", "speaker"].iter().map(|s| s.to_string()));
                },
                "write" => {
                    forms.extend(["writes", "wrote", "written", "writing", "writer"].iter().map(|s| s.to_string()));
                },
                "read" => {
                    forms.extend(["reads", "reading", "reader"].iter().map(|s| s.to_string()));
                },
                "find" => {
                    forms.extend(["finds", "found", "finding", "finder"].iter().map(|s| s.to_string()));
                },
                "build" => {
                    forms.extend(["builds", "built", "building", "builder"].iter().map(|s| s.to_string()));
                },
                "send" => {
                    forms.extend(["sends", "sent", "sending", "sender"].iter().map(|s| s.to_string()));
                },
                "teach" => {
                    forms.extend(["teaches", "taught", "teaching", "teacher"].iter().map(|s| s.to_string()));
                },
                "learn" => {
                    forms.extend(["learns", "learned", "learning", "learner"].iter().map(|s| s.to_string()));
                },
                _ => {
                    // Regular verbs
                    forms.push(format!("{}s", word)); // third person
                    
                    if word.ends_with("e") {
                        forms.push(format!("{}d", word)); // past tense: like -> liked
                        forms.push(format!("{}r", word)); // agent: like -> liker
                        forms.push(format!("{}ing", &word[..word.len()-1])); // present: like -> liking
                    } else {
                        forms.push(format!("{}ed", word)); // past tense: walk -> walked
                        forms.push(format!("{}er", word)); // agent: walk -> walker
                        forms.push(format!("{}ing", word)); // present: walk -> walking
                    }
                }
            }
        },
        WordType::Noun => {
            // Generate plurals
            if word.ends_with("s") || word.ends_with("sh") || word.ends_with("ch") || word.ends_with("x") || word.ends_with("z") {
                forms.push(format!("{}es", word));
            } else if word.ends_with("y") && word.len() > 1 {
                let prev_char = word.chars().nth(word.len() - 2).unwrap_or('a');
                if !"aeiou".contains(prev_char) {
                    forms.push(format!("{}ies", &word[..word.len()-1])); // city -> cities
                } else {
                    forms.push(format!("{}s", word)); // boy -> boys
                }
            } else if word == "mouse" {
                forms.push("mice".to_string());
            } else if word == "child" {
                forms.push("children".to_string());
            } else if word == "person" {
                forms.push("people".to_string());
            } else {
                forms.push(format!("{}s", word)); // regular plural
            }
        },
        WordType::Adjective => {
            // Generate comparative forms for short adjectives
            if word.len() <= 6 {
                if word.ends_with("e") {
                    forms.push(format!("{}r", word)); // large -> larger
                    forms.push(format!("{}st", word)); // large -> largest
                } else if word.ends_with("y") && word.len() > 1 {
                    let stem = &word[..word.len()-1];
                    forms.push(format!("{}ier", stem)); // happy -> happier
                    forms.push(format!("{}iest", stem)); // happy -> happiest
                } else {
                    forms.push(format!("{}er", word)); // small -> smaller
                    forms.push(format!("{}est", word)); // small -> smallest
                }
            }
            
            // Generate adverb
            if word.ends_with("y") {
                let stem = &word[..word.len()-1];
                forms.push(format!("{}ily", stem)); // happy -> happily
            } else if word.ends_with("le") {
                let stem = &word[..word.len()-2];
                forms.push(format!("{}ly", stem)); // simple -> simply
            } else {
                forms.push(format!("{}ly", word)); // quick -> quickly
            }
        }
    }
    
    // Filter out forms that would be too long or invalid
    forms.into_iter()
        .filter(|f| f.len() >= 3 && f.len() <= 10)
        .filter(|f| is_pronounceable(f))
        .collect()
}

fn is_pronounceable(word: &str) -> bool {
    // Basic pronunciation check
    let vowel_count = word.chars().filter(|c| "aeiou".contains(*c)).count();
    
    // Must have at least one vowel
    if vowel_count == 0 {
        return false;
    }
    
    // Check for reasonable consonant clusters
    let mut consonant_streak = 0;
    for ch in word.chars() {
        if "bcdfghjklmnpqrstvwxyz".contains(ch) {
            consonant_streak += 1;
            if consonant_streak > 3 {
                return false; // Too many consonants in a row
            }
        } else {
            consonant_streak = 0;
        }
    }
    
    true
}

fn calculate_quality_score(word: &str, word_type: &WordType, existing: &[String], missing: &[String]) -> f64 {
    let mut score = 0.0;
    
    // Base score for word familiarity
    let familiarity_score = match word {
        "work" | "play" | "run" | "walk" | "talk" | "think" | "make" | "take" => 10.0,
        "computer" | "phone" | "message" | "file" | "system" | "program" => 8.0,
        "good" | "bad" | "big" | "small" | "new" | "old" | "fast" | "slow" => 7.0,
        _ => 5.0,
    };
    score += familiarity_score;
    
    // Bonus for missing high-value forms
    for missing_form in missing {
        if missing_form.ends_with("ing") {
            score += 3.0; // -ing forms are very natural
        } else if missing_form.ends_with("er") && matches!(word_type, WordType::Verb) {
            score += 2.5; // agent nouns are useful
        } else if missing_form.ends_with("s") && missing_form.len() == word.len() + 1 {
            score += 2.0; // simple plurals
        } else if missing_form.ends_with("ly") {
            score += 1.5; // adverbs
        }
    }
    
    // Bonus for having some existing forms (shows it's already partially covered)
    score += existing.len() as f64 * 0.5;
    
    // Penalty for very long words
    if word.len() > 7 {
        score -= 1.0;
    }
    
    score
}

fn generate_natural_variants(bases: &[HighValueBase], words: &[String]) -> Vec<String> {
    let word_set: HashSet<_> = words.iter().collect();
    let mut variants = Vec::new();
    
    for base in bases.iter().take(100) { // Focus on top 100 high-value bases
        for missing_form in &base.missing_forms {
            if !word_set.contains(missing_form) && is_high_quality_form(missing_form, &base.word_type) {
                variants.push(missing_form.clone());
            }
        }
    }
    
    variants
}

fn is_high_quality_form(form: &str, word_type: &WordType) -> bool {
    // Check if this is a high-quality word form worth adding
    
    // Prioritize -ing forms
    if form.ends_with("ing") && form.len() > 4 {
        return true;
    }
    
    // Prioritize agent nouns from verbs
    if matches!(word_type, WordType::Verb) && form.ends_with("er") && form.len() <= 8 {
        return true;
    }
    
    // Prioritize simple plurals
    if matches!(word_type, WordType::Noun) && form.ends_with("s") && !form.ends_with("ss") {
        return true;
    }
    
    // Prioritize adverbs
    if matches!(word_type, WordType::Adjective) && form.ends_with("ly") && form.len() <= 9 {
        return true;
    }
    
    // Prioritize comparative adjectives for short words
    if matches!(word_type, WordType::Adjective) && (form.ends_with("er") || form.ends_with("est")) && form.len() <= 8 {
        return true;
    }
    
    false
}

fn find_low_quality_words(words: &[String]) -> Vec<String> {
    let mut low_quality = Vec::new();
    
    for word in words {
        let quality_score = score_word_quality(word);
        if quality_score < 2.0 { // Threshold for low quality
            low_quality.push(word.clone());
        }
    }
    
    // Sort by quality (worst first)
    low_quality.sort_by(|a, b| score_word_quality(a).partial_cmp(&score_word_quality(b)).unwrap());
    
    low_quality
}

fn score_word_quality(word: &str) -> f64 {
    let mut score = 5.0; // Start with neutral score
    
    // Penalty for very short words that are hard to distinguish
    if word.len() <= 3 {
        score -= 2.0;
    }
    
    // Penalty for words with poor vowel distribution
    let vowel_count = word.chars().filter(|c| "aeiou".contains(*c)).count();
    if vowel_count == 0 {
        score -= 3.0;
    } else if vowel_count == 1 && word.len() > 5 {
        score -= 1.0;
    }
    
    // Penalty for difficult consonant clusters
    if word.contains("ckl") || word.contains("schl") || word.contains("tch") {
        score -= 1.0;
    }
    
    // Penalty for obvious proper names or technical terms
    if word.chars().any(|c| c.is_uppercase()) {
        score -= 2.0;
    }
    
    // Penalty for words ending in uncommon patterns
    if word.ends_with("lich") || word.ends_with("weg") || word.ends_with("heim") {
        score -= 1.5; // Likely German
    }
    
    // Bonus for clearly English patterns
    if word.ends_with("ing") || word.ends_with("tion") || word.ends_with("ly") {
        score += 2.0;
    }
    
    // Bonus for simple, pronounceable words
    if word.len() >= 4 && word.len() <= 7 && vowel_count >= 2 {
        score += 1.0;
    }
    
    score
}

fn create_improvement_plan(missing_variants: &[String], low_quality_words: &[String]) -> ImprovementPlan {
    let mut words_to_add = Vec::new();
    let mut words_to_remove = Vec::new();
    let mut quality_improvements = Vec::new();
    
    // Add high-quality missing variants
    for variant in missing_variants.iter().take(1000) { // Limit to top 1000
        words_to_add.push(variant.clone());
    }
    
    // Remove low-quality words (same number as we're adding)
    for low_quality in low_quality_words.iter().take(words_to_add.len()) {
        words_to_remove.push(low_quality.clone());
    }
    
    // Create paired improvements
    for (add, remove) in words_to_add.iter().zip(words_to_remove.iter()) {
        quality_improvements.push((remove.clone(), add.clone()));
    }
    
    ImprovementPlan {
        words_to_add,
        words_to_remove,
        quality_improvements,
    }
}

fn report_improvement_plan(plan: &ImprovementPlan) {
    println!("\n=== DICTIONARY IMPROVEMENT PLAN ===");
    println!("Words to add: {}", plan.words_to_add.len());
    println!("Words to remove: {}", plan.words_to_remove.len());
    println!("Quality improvements: {}", plan.quality_improvements.len());
    
    println!("\n=== TOP 20 QUALITY IMPROVEMENTS ===");
    for (i, (remove, add)) in plan.quality_improvements.iter().take(20).enumerate() {
        println!("{}. Remove '{}' → Add '{}'", i+1, remove, add);
    }
    
    println!("\n=== HIGH-VALUE ADDITIONS ===");
    let high_value_additions: Vec<_> = plan.words_to_add.iter()
        .filter(|w| w.ends_with("ing") || w.ends_with("er") || w.ends_with("ly"))
        .take(30)
        .collect();
    
    for (i, word) in high_value_additions.iter().enumerate() {
        println!("{}. {}", i+1, word);
    }
}

fn save_improvement_plan(plan: &ImprovementPlan) -> Result<(), Box<dyn std::error::Error>> {
    // Save words to add
    let add_content = plan.words_to_add.join("\n");
    fs::write("data/words_to_add.txt", add_content)?;
    
    // Save words to remove
    let remove_content = plan.words_to_remove.join("\n");
    fs::write("data/words_to_remove.txt", remove_content)?;
    
    // Save improvement pairs
    let improvements: Vec<String> = plan.quality_improvements.iter()
        .map(|(remove, add)| format!("{} -> {}", remove, add))
        .collect();
    let improvements_content = improvements.join("\n");
    fs::write("data/quality_improvements.txt", improvements_content)?;
    
    println!("\nSaved improvement plan to data/ directory");
    println!("- words_to_add.txt: {} words", plan.words_to_add.len());
    println!("- words_to_remove.txt: {} words", plan.words_to_remove.len());
    println!("- quality_improvements.txt: {} pairs", plan.quality_improvements.len());
    
    Ok(())
}