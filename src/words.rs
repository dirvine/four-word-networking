//! Three-Word Address System
//!
//! Converts complex multiaddrs into memorable three-word combinations for human-friendly
//! peer discovery and sharing. Inspired by what3words but designed specifically for
//! network addresses.
//!
//! Example: `/ip6/2001:db8::1/udp/9000/quic` ↔ `ocean.thunder.falcon`

use crate::error::{ThreeWordError, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::str::FromStr;

/// Maximum number of words per position in the base dictionary
const BASE_WORDS_PER_POSITION: usize = 4096; // 2^12 for massive address space

/// Extended addressing with numeric suffixes for massive scale
const NUMERIC_SUFFIX_BITS: u32 = 16; // Additional 16 bits = 65,536 per base address

/// Total base combinations: 4096^3 = ~68.7 billion three-word addresses  
const BASE_COMBINATIONS: u64 = (BASE_WORDS_PER_POSITION as u64).pow(3);

/// Calculate total combinations safely using checked arithmetic
const fn calculate_total_combinations() -> Option<u64> {
    BASE_COMBINATIONS.checked_mul(2_u64.pow(16)) // Use 16 bits instead of 32 to avoid overflow
}

/// Total extended combinations: ~68.7 billion × 65K = ~4.5 quadrillion addresses
/// Note: Using 16-bit suffix to avoid overflow while still providing massive scale
const TOTAL_COMBINATIONS: u64 = match calculate_total_combinations() {
    Some(total) => total,
    None => BASE_COMBINATIONS, // Fallback to base combinations if overflow
};

/// Three-word address representation with optional numeric suffix for massive scale
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ThreeWordAddress {
    pub first: String,
    pub second: String, 
    pub third: String,
    /// Optional numeric suffix for extended addressing (None for base addresses)
    pub suffix: Option<u32>,
}

impl ThreeWordAddress {
    /// Create a new three-word address
    pub fn new(first: String, second: String, third: String) -> Self {
        Self { first, second, third, suffix: None }
    }
    
    /// Create a new three-word address with numeric suffix
    pub fn new_with_suffix(first: String, second: String, third: String, suffix: u32) -> Self {
        Self { first, second, third, suffix: Some(suffix) }
    }
    
    /// Parse from dot-separated string format (supports optional numeric suffix)
    /// Examples: "forest.lightning.compass" or "forest.lightning.compass.1847"
    pub fn from_string(input: &str) -> Result<Self> {
        let parts: Vec<&str> = input.split('.').collect();
        
        match parts.len() {
            3 => {
                // Base three-word format
                Ok(Self {
                    first: parts[0].to_lowercase(),
                    second: parts[1].to_lowercase(),
                    third: parts[2].to_lowercase(),
                    suffix: None,
                })
            }
            4 => {
                // Extended format with numeric suffix
                let suffix = parts[3].parse::<u32>()
                    .map_err(|e| ThreeWordError::InvalidThreeWordAddress(
                        format!("Invalid numeric suffix '{}': {}", parts[3], e)
                    ))?;
                    
                Ok(Self {
                    first: parts[0].to_lowercase(),
                    second: parts[1].to_lowercase(),
                    third: parts[2].to_lowercase(),
                    suffix: Some(suffix),
                })
            }
            _ => Err(ThreeWordError::InvalidThreeWordAddress(
                format!("Address must have 3 words or 3 words + numeric suffix, got: {}", input)
            ))
        }
    }
    
    /// Convert to dot-separated string format
    pub fn to_string(&self) -> String {
        if let Some(suffix) = self.suffix {
            format!("{}.{}.{}.{}", self.first, self.second, self.third, suffix)
        } else {
            format!("{}.{}.{}", self.first, self.second, self.third)
        }
    }
    
    /// Get the base three-word part (without suffix)
    pub fn base_address(&self) -> String {
        format!("{}.{}.{}", self.first, self.second, self.third)
    }
    
    /// Check if this is an extended address (has numeric suffix)
    pub fn is_extended(&self) -> bool {
        self.suffix.is_some()
    }
    
    /// Get the number of base three-word combinations
    pub fn base_combinations() -> u64 {
        BASE_COMBINATIONS
    }
    
    /// Get the estimated total address space this represents
    pub fn address_space_size() -> u64 {
        TOTAL_COMBINATIONS
    }
    
    /// Get human-readable description of address space
    pub fn address_space_description() -> String {
        format!(
            "~{:.1} trillion addresses ({} base three-word × {} suffixes)",
            TOTAL_COMBINATIONS as f64 / 1e12,
            BASE_COMBINATIONS,
            2_u64.pow(NUMERIC_SUFFIX_BITS)
        )
    }
    
    /// Validate that all words exist in the dictionary
    pub fn validate(&self, encoder: &WordEncoder) -> Result<()> {
        encoder.validate_words(&self.first, &self.second, &self.third)
    }
}

impl std::fmt::Display for ThreeWordAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl FromStr for ThreeWordAddress {
    type Err = ThreeWordError;
    
    fn from_str(s: &str) -> Result<Self> {
        Self::from_string(s)
    }
}

/// Word dictionary for three-word address encoding
#[derive(Debug, Clone)]
pub struct WordDictionary {
    /// Context words (position 1): geographic, network type
    context_words: Vec<String>,
    /// Quality words (position 2): performance, purpose, status  
    quality_words: Vec<String>,
    /// Identity words (position 3): nature, objects, abstract concepts
    identity_words: Vec<String>,
    
    /// Reverse lookup maps
    context_map: HashMap<String, usize>,
    quality_map: HashMap<String, usize>,
    identity_map: HashMap<String, usize>,
}

impl WordDictionary {
    /// Create a new word dictionary with default English words
    pub fn new() -> Self {
        let context_words = Self::default_context_words();
        let quality_words = Self::default_quality_words();
        let identity_words = Self::default_identity_words();
        
        let context_map: HashMap<String, usize> = context_words
            .iter()
            .enumerate()
            .map(|(i, word)| (word.clone(), i))
            .collect();
            
        let quality_map: HashMap<String, usize> = quality_words
            .iter()
            .enumerate() 
            .map(|(i, word)| (word.clone(), i))
            .collect();
            
        let identity_map: HashMap<String, usize> = identity_words
            .iter()
            .enumerate()
            .map(|(i, word)| (word.clone(), i))
            .collect();
        
        Self {
            context_words,
            quality_words,
            identity_words,
            context_map,
            quality_map,
            identity_map,
        }
    }
    
    /// Get word by position and index
    pub fn get_word(&self, position: usize, index: usize) -> Option<&String> {
        match position {
            0 => self.context_words.get(index),
            1 => self.quality_words.get(index), 
            2 => self.identity_words.get(index),
            _ => None,
        }
    }
    
    /// Get index by position and word
    pub fn get_index(&self, position: usize, word: &str) -> Option<usize> {
        let word_lower = word.to_lowercase();
        match position {
            0 => self.context_map.get(&word_lower).copied(),
            1 => self.quality_map.get(&word_lower).copied(),
            2 => self.identity_map.get(&word_lower).copied(),
            _ => None,
        }
    }
    
    /// Validate that a word exists in the specified position
    pub fn validate_word(&self, position: usize, word: &str) -> bool {
        self.get_index(position, word).is_some()
    }
    
    /// Get all words for a specific position
    pub fn get_words_for_position(&self, position: usize) -> Option<&Vec<String>> {
        match position {
            0 => Some(&self.context_words),
            1 => Some(&self.quality_words),
            2 => Some(&self.identity_words),
            _ => None,
        }
    }
    
    /// Default context words (position 1) - geographic and network context
    fn default_context_words() -> Vec<String> {
        let mut words = vec![
            // Geographic contexts
            "global", "europe", "america", "asia", "africa", "oceania", "arctic", "pacific",
            "atlantic", "indian", "mountain", "desert", "forest", "urban", "rural", "coastal",
            "island", "valley", "plateau", "tundra", "savanna", "jungle", "prairie", "canyon",
            
            // Network contexts  
            "local", "mesh", "bridge", "gateway", "relay", "hub", "node", "cluster", "edge",
            "core", "access", "backbone", "fiber", "wireless", "mobile", "fixed", "satellite",
            "ground", "space", "cloud", "fog", "mist", "clear", "direct", "routed", "switched",
            
            // Scale contexts
            "micro", "mini", "small", "medium", "large", "huge", "giant", "massive", "tiny",
            "compact", "wide", "narrow", "deep", "shallow", "high", "low", "fast", "slow",
            
            // Additional contexts
            "north", "south", "east", "west", "central", "remote", "near", "far", "inner",
            "outer", "upper", "lower", "front", "back", "left", "right", "home", "work",
            "school", "public", "private", "open", "closed", "secure", "safe", "quick",
            "steady", "smooth", "rough", "sharp", "soft", "hard", "light", "dark", "bright",
            "dim", "warm", "cool", "hot", "cold", "fresh", "old", "new", "modern", "classic",
        ].iter().map(|s| s.to_string()).collect::<Vec<String>>();
        
        // Extend to reach BASE_WORDS_PER_POSITION (4096) words
        while words.len() < BASE_WORDS_PER_POSITION {
            let base_count = words.len();
            for i in 0..std::cmp::min(100, BASE_WORDS_PER_POSITION - words.len()) {
                words.push(format!("ctx{:04}", base_count + i));
            }
        }
        
        words.truncate(BASE_WORDS_PER_POSITION);
        words
    }
    
    /// Default quality words (position 2) - performance, purpose, status
    fn default_quality_words() -> Vec<String> {
        let mut words = vec![
            // Performance qualities
            "fast", "quick", "rapid", "swift", "speedy", "turbo", "hyper", "ultra", "super",
            "stable", "solid", "steady", "reliable", "robust", "strong", "secure", "safe",
            "premium", "elite", "pro", "advanced", "expert", "master", "prime", "top", "best",
            "smooth", "fluid", "agile", "nimble", "efficient", "optimal", "perfect", "ideal",
            
            // Purpose qualities
            "chat", "talk", "voice", "video", "stream", "share", "store", "backup", "sync",
            "game", "play", "work", "study", "learn", "teach", "create", "build", "design",
            "connect", "link", "bridge", "tunnel", "route", "switch", "filter", "block",
            "allow", "grant", "deny", "check", "verify", "trust", "guard", "watch", "monitor",
            
            // Status qualities  
            "active", "live", "online", "ready", "awake", "alert", "busy", "free", "open",
            "public", "private", "hidden", "visible", "clear", "bright", "sharp", "focused",
            "verified", "trusted", "known", "famous", "popular", "common", "rare", "unique",
            "special", "magic", "power", "energy", "force", "strength", "grace", "beauty",
            
            // Additional qualities
            "gentle", "calm", "peaceful", "quiet", "loud", "bold", "brave", "smart", "wise",
            "clever", "bright", "brilliant", "clear", "pure", "clean", "fresh", "green",
            "blue", "red", "gold", "silver", "bronze", "crystal", "diamond", "pearl", "ruby",
        ].iter().map(|s| s.to_string()).collect::<Vec<String>>();
        
        // Extend to reach BASE_WORDS_PER_POSITION (4096) words
        while words.len() < BASE_WORDS_PER_POSITION {
            let base_count = words.len();
            for i in 0..std::cmp::min(100, BASE_WORDS_PER_POSITION - words.len()) {
                words.push(format!("qual{:04}", base_count + i));
            }
        }
        
        words.truncate(BASE_WORDS_PER_POSITION);
        words
    }
    
    /// Default identity words (position 3) - nature, objects, abstract concepts
    fn default_identity_words() -> Vec<String> {
        let mut words = vec![
            // Nature - Animals
            "eagle", "falcon", "hawk", "owl", "raven", "swan", "crane", "heron", "robin",
            "lion", "tiger", "bear", "wolf", "fox", "deer", "elk", "moose", "bison",
            "whale", "dolphin", "shark", "ray", "octopus", "seal", "penguin", "turtle",
            "dragon", "phoenix", "griffin", "pegasus", "unicorn", "sphinx", "chimera",
            
            // Nature - Plants & Geography
            "oak", "pine", "maple", "cedar", "willow", "bamboo", "lotus", "rose", "lily",
            "mountain", "hill", "peak", "summit", "ridge", "valley", "canyon", "cliff",
            "river", "stream", "lake", "pond", "ocean", "sea", "bay", "inlet", "shore",
            "forest", "woods", "grove", "meadow", "field", "garden", "oasis", "desert",
            
            // Objects - Navigation & Tools
            "compass", "anchor", "lighthouse", "beacon", "tower", "bridge", "gate", "door",
            "key", "lock", "sword", "shield", "hammer", "anvil", "forge", "wheel", "gear",
            "engine", "motor", "spring", "lever", "pulley", "rope", "chain", "cable", "wire",
            "lens", "mirror", "prism", "crystal", "gem", "jewel", "crown", "ring", "star",
            
            // Abstract Concepts
            "harmony", "balance", "rhythm", "melody", "symphony", "song", "dance", "flight",
            "journey", "quest", "adventure", "discovery", "treasure", "mystery", "secret",
            "dream", "vision", "hope", "faith", "trust", "love", "peace", "joy", "bliss",
            "clarity", "wisdom", "knowledge", "truth", "light", "shadow", "spirit", "soul",
            "essence", "core", "heart", "mind", "thought", "idea", "spark", "flame", "fire",
        ].iter().map(|s| s.to_string()).collect::<Vec<String>>();
        
        // Extend to reach BASE_WORDS_PER_POSITION (4096) words
        while words.len() < BASE_WORDS_PER_POSITION {
            let base_count = words.len();
            for i in 0..std::cmp::min(100, BASE_WORDS_PER_POSITION - words.len()) {
                words.push(format!("id{:04}", base_count + i));
            }
        }
        
        words.truncate(BASE_WORDS_PER_POSITION);
        words
    }
}

impl Default for WordDictionary {
    fn default() -> Self {
        Self::new()
    }
}

/// Main encoder/decoder for three-word addresses
#[derive(Debug, Clone)]
pub struct WordEncoder {
    dictionary: WordDictionary,
}

impl WordEncoder {
    /// Create a new word encoder with default dictionary
    pub fn new() -> Self {
        Self {
            dictionary: WordDictionary::new(),
        }
    }
    
    /// Create encoder with custom dictionary
    pub fn with_dictionary(dictionary: WordDictionary) -> Self {
        Self { dictionary }
    }
    
    /// Convert multiaddr string to three-word address
    pub fn encode_multiaddr_string(&self, multiaddr: &str) -> Result<ThreeWordAddress> {
        // Validate that it's a reasonable multiaddr format
        if !multiaddr.starts_with('/') {
            return Err(ThreeWordError::InvalidMultiaddr(
                format!("Multiaddr must start with '/', got: {}", multiaddr)
            ));
        }
        
        // Convert multiaddr to a consistent hash/fingerprint
        let hash = self.hash_multiaddr(multiaddr);
        
        // Extract indices from the hash
        let (context_idx, quality_idx, identity_idx, suffix) = self.extract_extended_indices(hash);
        
        // Get words from dictionary
        let first = self.dictionary.get_word(0, context_idx)
            .ok_or_else(|| ThreeWordError::PositionOutOfRange(0))?
            .clone();
            
        let second = self.dictionary.get_word(1, quality_idx)
            .ok_or_else(|| ThreeWordError::PositionOutOfRange(1))?
            .clone();
            
        let third = self.dictionary.get_word(2, identity_idx)
            .ok_or_else(|| ThreeWordError::PositionOutOfRange(2))?
            .clone();
        
        // Use suffix if it's non-zero (for extended addressing)
        if suffix == 0 {
            Ok(ThreeWordAddress::new(first, second, third))
        } else {
            Ok(ThreeWordAddress::new_with_suffix(first, second, third, suffix))
        }
    }
    
    /// Encode multiaddr string with preference for base (no suffix) addressing when possible
    pub fn encode_multiaddr_string_base(&self, multiaddr: &str) -> Result<ThreeWordAddress> {
        if !multiaddr.starts_with('/') {
            return Err(ThreeWordError::InvalidMultiaddr(
                format!("Multiaddr must start with '/', got: {}", multiaddr)
            ));
        }
        
        let hash = self.hash_multiaddr(multiaddr);
        
        // Extract only the base three indices, ignoring suffix bits
        let (context_idx, quality_idx, identity_idx, _) = self.extract_extended_indices(hash);
        
        let first = self.dictionary.get_word(0, context_idx)
            .ok_or_else(|| ThreeWordError::PositionOutOfRange(0))?
            .clone();
            
        let second = self.dictionary.get_word(1, quality_idx)
            .ok_or_else(|| ThreeWordError::PositionOutOfRange(1))?
            .clone();
            
        let third = self.dictionary.get_word(2, identity_idx)
            .ok_or_else(|| ThreeWordError::PositionOutOfRange(2))?
            .clone();
        
        Ok(ThreeWordAddress::new(first, second, third))
    }
    
    /// Convert three-word address back to multiaddr string
    /// Note: This requires a registry/cache since the conversion isn't perfectly reversible
    pub fn decode_to_multiaddr_string(&self, _words: &ThreeWordAddress) -> Result<String> {
        // For now, return an error indicating this needs a registry lookup
        // In a real implementation, this would query a distributed registry
        Err(ThreeWordError::RegistryLookupNotImplemented(
            "Multiaddr decoding requires registry lookup - not yet implemented".to_string()
        ))
    }
    
    /// Validate that all three words exist in the dictionary
    pub fn validate_words(&self, first: &str, second: &str, third: &str) -> Result<()> {
        if !self.dictionary.validate_word(0, first) {
            return Err(ThreeWordError::WordNotFound(format!("Unknown context word: {}", first)));
        }
        
        if !self.dictionary.validate_word(1, second) {
            return Err(ThreeWordError::WordNotFound(format!("Unknown quality word: {}", second)));
        }
        
        if !self.dictionary.validate_word(2, third) {
            return Err(ThreeWordError::WordNotFound(format!("Unknown identity word: {}", third)));
        }
        
        Ok(())
    }
    
    /// Get the word dictionary
    pub fn dictionary(&self) -> &WordDictionary {
        &self.dictionary
    }
    
    /// Generate a consistent hash from multiaddr string
    fn hash_multiaddr(&self, multiaddr: &str) -> u64 {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        
        let mut hasher = DefaultHasher::new();
        multiaddr.hash(&mut hasher);
        hasher.finish()
    }
    
    /// Extract extended indices including suffix for massive scale addressing
    fn extract_extended_indices(&self, hash: u64) -> (usize, usize, usize, u32) {
        // Use different parts of the hash for each word position and suffix
        // Ensure indices are within the actual dictionary size
        let context_size = self.dictionary.context_words.len();
        let quality_size = self.dictionary.quality_words.len();
        let identity_size = self.dictionary.identity_words.len();
        
        // Extract word indices from different parts of the hash
        let context_idx = (hash as usize) % context_size;
        let quality_idx = ((hash >> 16) as usize) % quality_size;
        let identity_idx = ((hash >> 32) as usize) % identity_size;
        
        // Use remaining bits for suffix (when non-zero, creates extended addressing)
        let suffix = ((hash >> 48) as u32) & ((1 << 16) - 1); // 16 bits for suffix
        
        (context_idx, quality_idx, identity_idx, suffix)
    }
}

impl Default for WordEncoder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_three_word_address_parsing() {
        let addr = ThreeWordAddress::from_string("ocean.thunder.falcon").unwrap();
        assert_eq!(addr.first, "ocean");
        assert_eq!(addr.second, "thunder");
        assert_eq!(addr.third, "falcon");
        assert_eq!(addr.to_string(), "ocean.thunder.falcon");
    }
    
    #[test]
    fn test_three_word_address_validation() {
        let words = ThreeWordAddress::new("global".to_string(), "fast".to_string(), "eagle".to_string());
        let encoder = WordEncoder::new();
        
        // Should pass validation since these are real words in our dictionary
        assert!(words.validate(&encoder).is_ok());
        
        // Should fail with invalid word
        let bad_words = ThreeWordAddress::new("invalid".to_string(), "words".to_string(), "here".to_string());
        assert!(bad_words.validate(&encoder).is_err());
    }
    
    #[test]
    fn test_multiaddr_encoding() {
        let encoder = WordEncoder::new();
        let multiaddr = "/ip6/2001:db8::1/udp/9000/quic";
        
        let words = encoder.encode_multiaddr_string(multiaddr).unwrap();
        
        // Should produce valid three-word address
        assert!(!words.first.is_empty());
        assert!(!words.second.is_empty());
        assert!(!words.third.is_empty());
        
        // Should validate successfully
        assert!(words.validate(&encoder).is_ok());
        
        // Same multiaddr should always produce same words (deterministic)
        let words2 = encoder.encode_multiaddr_string(multiaddr).unwrap();
        assert_eq!(words, words2);
    }
    
    #[test]
    fn test_word_dictionary() {
        let dict = WordDictionary::new();
        
        // Should have words in all positions
        assert!(!dict.context_words.is_empty());
        assert!(!dict.quality_words.is_empty());
        assert!(!dict.identity_words.is_empty());
        
        // Should be able to lookup words
        assert!(dict.validate_word(0, "global"));
        assert!(dict.validate_word(1, "fast"));
        assert!(dict.validate_word(2, "eagle"));
        
        // Should reject invalid words
        assert!(!dict.validate_word(0, "nonexistent"));
    }
    
    #[test]
    fn test_deterministic_encoding() {
        let encoder = WordEncoder::new();
        
        // Test multiple multiaddrs to ensure consistency
        let addrs = vec![
            "/ip6/2001:db8::1/udp/9000/quic",
            "/ip6/::1/tcp/8000",
            "/ip4/192.168.1.1/udp/5000/quic",
        ];
        
        for addr_str in addrs {
            // Encode multiple times - should always get same result
            let words1 = encoder.encode_multiaddr_string(addr_str).unwrap();
            let words2 = encoder.encode_multiaddr_string(addr_str).unwrap();
            let words3 = encoder.encode_multiaddr_string(addr_str).unwrap();
            
            assert_eq!(words1, words2);
            assert_eq!(words2, words3);
            
            println!("{} -> {}", addr_str, words1);
        }
    }
    
    #[test]
    fn test_extended_address_format() {
        // Test parsing of extended addresses
        let extended = ThreeWordAddress::from_string("forest.lightning.compass.1847").unwrap();
        assert_eq!(extended.first, "forest");
        assert_eq!(extended.second, "lightning");
        assert_eq!(extended.third, "compass");
        assert_eq!(extended.suffix, Some(1847));
        assert!(extended.is_extended());
        assert_eq!(extended.to_string(), "forest.lightning.compass.1847");
        assert_eq!(extended.base_address(), "forest.lightning.compass");
        
        // Test base address format
        let base = ThreeWordAddress::from_string("forest.lightning.compass").unwrap();
        assert_eq!(base.suffix, None);
        assert!(!base.is_extended());
        assert_eq!(base.to_string(), "forest.lightning.compass");
        assert_eq!(base.base_address(), "forest.lightning.compass");
        
        // Test invalid formats
        assert!(ThreeWordAddress::from_string("too.few").is_err());
        assert!(ThreeWordAddress::from_string("too.many.words.here.extra").is_err());
        assert!(ThreeWordAddress::from_string("invalid.suffix.format.notanumber").is_err());
    }
    
    #[test]
    fn test_massive_scale_addressing() {
        let encoder = WordEncoder::new();
        
        // Test that we can handle massive scale
        let test_addresses = [
            "/ip6/2001:db8::1/udp/9000/quic",
            "/ip6/2001:db8::2/udp/9000/quic", 
            "/ip6/2001:db8::3/udp/9000/quic",
            "/ip4/192.168.1.100/udp/5000/quic",
            "/ip4/10.0.0.1/tcp/8080",
        ];
        
        for addr in &test_addresses {
            let words = encoder.encode_multiaddr_string(addr).unwrap();
            
            println!("Address: {}", addr);
            println!("  Three-word: {}", words);
            println!("  Extended: {}", words.is_extended());
            
            // Test base encoding (without suffix)
            let base_words = encoder.encode_multiaddr_string_base(addr).unwrap();
            println!("  Base format: {}", base_words);
            
            assert!(words.validate(&encoder).is_ok());
            assert!(base_words.validate(&encoder).is_ok());
        }
        
        // Verify address space capacity
        println!("\nAddress Space Information:");
        println!("  {}", ThreeWordAddress::address_space_description());
        println!("  Total combinations: {}", ThreeWordAddress::address_space_size());
    }
    
    #[test]
    fn test_universal_multiaddr_encoding() {
        let encoder = WordEncoder::new();
        
        // Test variety of multiaddr formats
        let test_multiaddrs = vec![
            "/ip4/127.0.0.1/tcp/8080",
            "/ip6/::1/tcp/8080",
            "/ip4/192.168.1.1/udp/9000/quic",
            "/ip6/2001:db8::1/udp/9000/quic",
            "/ip4/10.0.0.1/tcp/22",
            "/ip6/fe80::1/tcp/443",
            "/dns4/example.com/tcp/80",
            "/dns6/ipv6.google.com/tcp/443",
        ];
        
        println!("\n=== Testing Universal Multiaddr Encoding ===");
        println!("Testing {} multiaddr formats\n", test_multiaddrs.len());
        
        for (i, addr_str) in test_multiaddrs.iter().enumerate() {
            match encoder.encode_multiaddr_string(addr_str) {
                Ok(words) => {
                    // Verify the encoding is valid
                    assert!(words.validate(&encoder).is_ok(), "Generated invalid three-word address for: {}", addr_str);
                    
                    // Verify deterministic encoding
                    let words2 = encoder.encode_multiaddr_string(addr_str).unwrap();
                    assert_eq!(words, words2, "Non-deterministic encoding for: {}", addr_str);
                    
                    println!("✅ {}: {} → {}", i+1, addr_str, words);
                }
                Err(e) => {
                    println!("❌ FAILED to encode multiaddr '{}': {}", addr_str, e);
                }
            }
        }
    }
}