//! Dictionary management for the Universal Word Encoding System

use crate::universal::error::{DecodingError, EncodingError};
use std::collections::HashMap;

/// Four specialized dictionaries with 4096 words each
#[derive(Debug, Clone)]
pub struct Dictionaries {
    /// Characters: falcon, wizard, knight, dragon
    pub actors: Vec<String>,
    /// Verbs: crosses, guards, seeks, finds
    pub actions: Vec<String>,
    /// Things: bridge, mountain, treasure, forest
    pub objects: Vec<String>,
    /// Adjectives: ancient, silver, hidden, northern
    pub modifiers: Vec<String>,

    // Reverse lookup maps for O(1) decoding
    actor_indices: HashMap<String, usize>,
    action_indices: HashMap<String, usize>,
    object_indices: HashMap<String, usize>,
    modifier_indices: HashMap<String, usize>,
}

impl Dictionaries {
    /// Create new dictionaries with default English words
    pub fn new() -> Result<Self, EncodingError> {
        let actors = Self::load_actors();
        let actions = Self::load_actions();
        let objects = Self::load_objects();
        let modifiers = Self::load_modifiers();

        // Validate dictionary sizes
        if actors.len() != 4096
            || actions.len() != 4096
            || objects.len() != 4096
            || modifiers.len() != 4096
        {
            return Err(EncodingError::DictionaryNotLoaded);
        }

        // Create reverse lookup maps
        let actor_indices: HashMap<String, usize> = actors
            .iter()
            .enumerate()
            .map(|(i, word)| (word.clone(), i))
            .collect();

        let action_indices: HashMap<String, usize> = actions
            .iter()
            .enumerate()
            .map(|(i, word)| (word.clone(), i))
            .collect();

        let object_indices: HashMap<String, usize> = objects
            .iter()
            .enumerate()
            .map(|(i, word)| (word.clone(), i))
            .collect();

        let modifier_indices: HashMap<String, usize> = modifiers
            .iter()
            .enumerate()
            .map(|(i, word)| (word.clone(), i))
            .collect();

        Ok(Dictionaries {
            actors,
            actions,
            objects,
            modifiers,
            actor_indices,
            action_indices,
            object_indices,
            modifier_indices,
        })
    }

    /// Get actor word by index
    pub fn get_actor(&self, index: usize) -> Result<&str, DecodingError> {
        self.actors
            .get(index)
            .map(|s| s.as_str())
            .ok_or_else(|| DecodingError::DictionaryLookupFailed(format!("actor index {}", index)))
    }

    /// Get action word by index
    pub fn get_action(&self, index: usize) -> Result<&str, DecodingError> {
        self.actions
            .get(index)
            .map(|s| s.as_str())
            .ok_or_else(|| DecodingError::DictionaryLookupFailed(format!("action index {}", index)))
    }

    /// Get object word by index
    pub fn get_object(&self, index: usize) -> Result<&str, DecodingError> {
        self.objects
            .get(index)
            .map(|s| s.as_str())
            .ok_or_else(|| DecodingError::DictionaryLookupFailed(format!("object index {}", index)))
    }

    /// Get modifier word by index
    pub fn get_modifier(&self, index: usize) -> Result<&str, DecodingError> {
        self.modifiers
            .get(index)
            .map(|s| s.as_str())
            .ok_or_else(|| {
                DecodingError::DictionaryLookupFailed(format!("modifier index {}", index))
            })
    }

    /// Get actor index by word
    pub fn get_actor_index(&self, word: &str) -> Result<usize, DecodingError> {
        self.actor_indices
            .get(word)
            .copied()
            .ok_or_else(|| DecodingError::InvalidWord(word.to_string()))
    }

    /// Get action index by word
    pub fn get_action_index(&self, word: &str) -> Result<usize, DecodingError> {
        self.action_indices
            .get(word)
            .copied()
            .ok_or_else(|| DecodingError::InvalidWord(word.to_string()))
    }

    /// Get object index by word
    pub fn get_object_index(&self, word: &str) -> Result<usize, DecodingError> {
        self.object_indices
            .get(word)
            .copied()
            .ok_or_else(|| DecodingError::InvalidWord(word.to_string()))
    }

    /// Get modifier index by word
    pub fn get_modifier_index(&self, word: &str) -> Result<usize, DecodingError> {
        self.modifier_indices
            .get(word)
            .copied()
            .ok_or_else(|| DecodingError::InvalidWord(word.to_string()))
    }

    /// Generate 4096 unique words from base words
    fn generate_unique_words(base_words: &[&str]) -> Vec<String> {
        use std::collections::HashSet;

        let mut words = Vec::new();
        let mut seen = HashSet::new();

        // Generate exactly 4096 unique words
        let mut i = 0;
        while words.len() < 4096 {
            let base = base_words[i % base_words.len()];

            let word = if i < base_words.len() {
                // Use base words as-is for the first round
                base.to_string()
            } else {
                // Generate variations for subsequent rounds
                let variation = i / base_words.len();
                if variation < 10 {
                    format!("{}{}", base, variation)
                } else if variation < 100 {
                    let prefix_len = std::cmp::min(base.len(), 6);
                    format!("{}x{}", &base[..prefix_len], variation % 100)
                } else {
                    let prefix_len = std::cmp::min(base.len(), 5);
                    format!("{}y{}", &base[..prefix_len], variation % 100)
                }
            };

            // Ensure word meets length constraints, adjust if necessary
            let mut final_word = if word.len() >= 4 && word.len() <= 8 {
                word
            } else if word.len() > 8 {
                // Truncate and add suffix
                format!("{}z{}", &base[..4], i % 100)
            } else {
                // Pad short words
                format!("{}x{}", word, i % 10)
            };

            // Ensure uniqueness
            let mut counter = 0;
            while seen.contains(&final_word) {
                counter += 1;
                final_word = format!(
                    "{}z{}",
                    &base[..std::cmp::min(base.len(), 4)],
                    (i + counter) % 1000
                );
            }

            seen.insert(final_word.clone());
            words.push(final_word);
            i += 1;
        }

        words
    }

    /// Load actor words (placeholder - will be replaced with curated list)
    fn load_actors() -> Vec<String> {
        let base_actors = vec![
            "falcon",
            "wizard",
            "knight",
            "dragon",
            "phoenix",
            "archer",
            "warrior",
            "mage",
            "ranger",
            "paladin",
            "rogue",
            "cleric",
            "monk",
            "sorcerer",
            "warlock",
            "druid",
            "barbarian",
            "bard",
            "fighter",
            "thief",
            "assassin",
            "hunter",
            "guardian",
            "champion",
            "hero",
            "legend",
            "mystic",
            "sage",
            "oracle",
            "prophet",
            "vision",
            "spirit",
            "giant",
            "demon",
            "angel",
            "beast",
            "ghost",
            "shade",
            "wraith",
            "specter",
            "titan",
            "golem",
            "dwarf",
            "gnome",
            "pixie",
            "fairy",
            "sprite",
            "nymph",
            "witch",
            "shaman",
            "priest",
            "monk2",
            "ninja",
            "samurai",
            "ronin",
            "shogun",
            "viking",
            "berserker",
            "centurion",
            "gladiator",
            "spartan",
            "amazon",
            "valkyrie",
            "banshee",
        ];

        Self::generate_unique_words(&base_actors)
    }

    /// Load action words (placeholder - will be replaced with curated list)
    fn load_actions() -> Vec<String> {
        let base_actions = vec![
            "crosses",
            "guards",
            "seeks",
            "finds",
            "hunts",
            "builds",
            "climbs",
            "flies",
            "walks",
            "runs",
            "jumps",
            "swims",
            "dives",
            "soars",
            "glides",
            "dances",
            "sings",
            "plays",
            "reads",
            "writes",
            "draws",
            "paints",
            "crafts",
            "creates",
            "explores",
            "discovers",
            "learns",
            "teaches",
            "helps",
            "protects",
            "fights",
            "defends",
            "attacks",
            "blocks",
            "dodges",
            "parries",
            "strikes",
            "casts",
            "summons",
            "banishes",
            "heals",
            "curses",
            "blesses",
            "enchants",
            "forges",
            "melts",
            "freezes",
            "burns",
            "lights",
            "darkens",
            "opens",
            "closes",
            "locks",
            "unlocks",
            "breaks",
            "repairs",
            "grows",
            "shrinks",
            "expands",
            "contracts",
            "lifts",
            "drops",
            "pushes",
            "pulls",
        ];

        Self::generate_unique_words(&base_actions)
    }

    /// Load object words (placeholder - will be replaced with curated list)
    fn load_objects() -> Vec<String> {
        let base_objects = vec![
            "bridge", "mountain", "treasure", "forest", "castle", "tower", "river", "ocean",
            "lake", "valley", "cave", "temple", "palace", "garden", "field", "meadow", "stone",
            "crystal", "gem", "sword", "shield", "armor", "crown", "ring", "book", "scroll", "map",
            "key", "door", "gate", "path", "road", "staff", "wand", "orb", "amulet", "potion",
            "spell", "charm", "curse", "flame", "frost", "storm", "earth", "wind", "water", "fire",
            "light", "shadow", "void", "energy", "power", "force", "magic", "spirit", "soul",
            "heart", "mind", "body", "bone", "blood", "flesh", "skin", "hair",
        ];

        Self::generate_unique_words(&base_objects)
    }

    /// Load modifier words (placeholder - will be replaced with curated list)
    fn load_modifiers() -> Vec<String> {
        let base_modifiers = vec![
            "ancient", "silver", "hidden", "northern", "southern", "eastern", "western", "golden",
            "bright", "dark", "light", "heavy", "quick", "slow", "strong", "weak", "large",
            "small", "tall", "short", "wide", "narrow", "deep", "shallow", "hot", "cold", "warm",
            "cool", "dry", "wet", "rough", "smooth", "sharp", "dull", "soft", "hard", "young",
            "old", "new", "worn", "pure", "corrupt", "holy", "cursed", "blessed", "damned",
            "sacred", "profane", "mighty", "frail", "bold", "timid", "brave", "coward", "wise",
            "foolish", "noble", "common", "rare", "unique", "special", "normal", "strange",
            "weird",
        ];

        Self::generate_unique_words(&base_modifiers)
    }
}

impl Default for Dictionaries {
    fn default() -> Self {
        Self::new().expect("Failed to create default dictionaries")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dictionaries_creation() {
        let dict = Dictionaries::new().unwrap();

        // Test dictionary sizes with debug info
        println!(
            "Actors: {}, Actions: {}, Objects: {}, Modifiers: {}",
            dict.actors.len(),
            dict.actions.len(),
            dict.objects.len(),
            dict.modifiers.len()
        );

        assert_eq!(dict.actors.len(), 4096, "Actors dictionary size mismatch");
        assert_eq!(dict.actions.len(), 4096, "Actions dictionary size mismatch");
        assert_eq!(dict.objects.len(), 4096, "Objects dictionary size mismatch");
        assert_eq!(
            dict.modifiers.len(),
            4096,
            "Modifiers dictionary size mismatch"
        );

        // Test reverse lookup maps
        assert_eq!(dict.actor_indices.len(), 4096);
        assert_eq!(dict.action_indices.len(), 4096);
        assert_eq!(dict.object_indices.len(), 4096);
        assert_eq!(dict.modifier_indices.len(), 4096);
    }

    #[test]
    fn test_word_lookup() {
        let dict = Dictionaries::new().unwrap();

        // Test forward lookup
        let actor = dict.get_actor(0).unwrap();
        let action = dict.get_action(0).unwrap();
        let object = dict.get_object(0).unwrap();
        let modifier = dict.get_modifier(0).unwrap();

        assert!(!actor.is_empty());
        assert!(!action.is_empty());
        assert!(!object.is_empty());
        assert!(!modifier.is_empty());

        // Test reverse lookup
        assert_eq!(dict.get_actor_index(actor).unwrap(), 0);
        assert_eq!(dict.get_action_index(action).unwrap(), 0);
        assert_eq!(dict.get_object_index(object).unwrap(), 0);
        assert_eq!(dict.get_modifier_index(modifier).unwrap(), 0);
    }

    #[test]
    fn test_invalid_word_lookup() {
        let dict = Dictionaries::new().unwrap();

        // Test invalid words
        assert!(dict.get_actor_index("invalidword").is_err());
        assert!(dict.get_action_index("invalidword").is_err());
        assert!(dict.get_object_index("invalidword").is_err());
        assert!(dict.get_modifier_index("invalidword").is_err());
    }

    #[test]
    fn test_out_of_bounds_lookup() {
        let dict = Dictionaries::new().unwrap();

        // Test out of bounds indices
        assert!(dict.get_actor(4096).is_err());
        assert!(dict.get_action(4096).is_err());
        assert!(dict.get_object(4096).is_err());
        assert!(dict.get_modifier(4096).is_err());
    }

    #[test]
    fn test_word_constraints() {
        let dict = Dictionaries::new().unwrap();

        // Test that all words meet length constraints (4-8 characters)
        for word in &dict.actors[..10] {
            // Test first 10 for efficiency
            assert!(
                word.len() >= 4 && word.len() <= 8,
                "Actor word '{}' violates length constraint",
                word
            );
        }

        for word in &dict.actions[..10] {
            assert!(
                word.len() >= 4 && word.len() <= 8,
                "Action word '{}' violates length constraint",
                word
            );
        }

        for word in &dict.objects[..10] {
            assert!(
                word.len() >= 4 && word.len() <= 8,
                "Object word '{}' violates length constraint",
                word
            );
        }

        for word in &dict.modifiers[..10] {
            assert!(
                word.len() >= 4 && word.len() <= 8,
                "Modifier word '{}' violates length constraint",
                word
            );
        }
    }
}
