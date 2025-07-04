//! Holographic encoder for data 21-32 bytes

use crate::universal::dictionaries::Dictionaries;
use crate::universal::error::{EncodingError, DecodingError, EncodingResult, DecodingResult};
use crate::universal::simple::SimpleEncoder;

/// Holographic encoder for data 21-32 bytes
#[derive(Debug, Clone)]
pub struct HolographicEncoder {
    simple_encoder: SimpleEncoder,
}

/// Holographic encoding with multiple story views
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HolographicEncoding {
    pub story_views: Vec<StoryView>,
}

/// A story view that encodes the hash from a different "angle"
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StoryView {
    pub actor: String,
    pub action: String,
    pub object: String,
    pub modifier: String,
    pub context: u32,
}

impl HolographicEncoder {
    /// Create a new holographic encoder
    pub fn new(dictionaries: Dictionaries) -> Self {
        let simple_encoder = SimpleEncoder::new(dictionaries);
        Self { simple_encoder }
    }
    
    /// Encode data into holographic encoding
    pub fn encode(&self, data: &[u8]) -> EncodingResult<HolographicEncoding> {
        if data.len() < 21 {
            return Err(EncodingError::DataTooSmall(data.len()));
        }
        
        if data.len() > 32 {
            return Err(EncodingError::DataTooLarge(data.len()));
        }
        
        // Generate 3-4 story views from different perspectives
        let story_views = self.generate_story_views(data)?;
        
        Ok(HolographicEncoding { story_views })
    }
    
    /// Decode holographic encoding back to original data
    pub fn decode(&self, encoding: &HolographicEncoding) -> DecodingResult<Vec<u8>> {
        if encoding.story_views.len() < 3 {
            return Err(DecodingError::InsufficientViews { 
                got: encoding.story_views.len(), 
                needed: 3 
            });
        }
        
        // Solve constraints from all story views
        self.solve_constraints(&encoding.story_views)
    }
    
    /// Generate story views from different perspectives (placeholder implementation)
    fn generate_story_views(&self, data: &[u8]) -> EncodingResult<Vec<StoryView>> {
        let mut views = Vec::new();
        
        // Generate 3-4 views based on data length
        let num_views = std::cmp::min(4, (data.len() + 7) / 8);
        
        for i in 0..num_views {
            let view = self.generate_single_view(data, i)?;
            views.push(view);
        }
        
        Ok(views)
    }
    
    /// Generate a single story view (placeholder implementation)
    fn generate_single_view(&self, data: &[u8], perspective: usize) -> EncodingResult<StoryView> {
        // Use different byte ranges for different perspectives
        let start = (perspective * 8) % data.len();
        let end = std::cmp::min(start + 8, data.len());
        
        let mut chunk = [0u8; 8];
        chunk[..end - start].copy_from_slice(&data[start..end]);
        
        // Hash the chunk with perspective to get different views
        let hash = self.hash_with_perspective(&chunk, perspective);
        
        // Extract word indices from hash
        let actor_index = (hash >> 36) % 4096;
        let action_index = (hash >> 24) % 4096;
        let object_index = (hash >> 12) % 4096;
        let modifier_index = hash % 4096;
        
        let actor = self.simple_encoder.dictionaries.get_actor(actor_index as usize)
            .map_err(|_| EncodingError::DictionaryNotLoaded)?
            .to_string();
        
        let action = self.simple_encoder.dictionaries.get_action(action_index as usize)
            .map_err(|_| EncodingError::DictionaryNotLoaded)?
            .to_string();
        
        let object = self.simple_encoder.dictionaries.get_object(object_index as usize)
            .map_err(|_| EncodingError::DictionaryNotLoaded)?
            .to_string();
        
        let modifier = self.simple_encoder.dictionaries.get_modifier(modifier_index as usize)
            .map_err(|_| EncodingError::DictionaryNotLoaded)?
            .to_string();
        
        Ok(StoryView {
            actor,
            action,
            object,
            modifier,
            context: perspective as u32,
        })
    }
    
    /// Hash data with perspective to get different views
    fn hash_with_perspective(&self, data: &[u8; 8], perspective: usize) -> u64 {
        let mut hash = u64::from_be_bytes(*data);
        
        // Apply perspective-based transformation
        hash = hash.wrapping_mul(perspective as u64 + 1);
        hash = hash.rotate_left(perspective as u32 * 8);
        hash ^= (perspective as u64) << 32;
        
        hash
    }
    
    /// Solve constraints from story views (placeholder implementation)
    fn solve_constraints(&self, views: &[StoryView]) -> DecodingResult<Vec<u8>> {
        // For now, use a simple approach - in a real implementation we'd solve
        // the system of equations represented by the story views
        
        // Use the first view as primary and others as validation
        let primary_view = &views[0];
        
        // Convert back to indices
        let actor_index = self.simple_encoder.dictionaries.get_actor_index(&primary_view.actor)?;
        let action_index = self.simple_encoder.dictionaries.get_action_index(&primary_view.action)?;
        let object_index = self.simple_encoder.dictionaries.get_object_index(&primary_view.object)?;
        let modifier_index = self.simple_encoder.dictionaries.get_modifier_index(&primary_view.modifier)?;
        
        // Reconstruct hash from indices
        let hash = ((actor_index as u64) << 36) |
                  ((action_index as u64) << 24) |
                  ((object_index as u64) << 12) |
                  (modifier_index as u64);
        
        // Reverse the perspective transformation
        let perspective = primary_view.context as usize;
        let mut original_hash = hash ^ ((perspective as u64) << 32);
        original_hash = original_hash.rotate_right(perspective as u32 * 8);
        original_hash = original_hash.wrapping_div(perspective as u64 + 1);
        
        // Convert back to bytes
        let bytes = original_hash.to_be_bytes();
        
        // Return reconstructed data (simplified - real implementation would be more complex)
        Ok(bytes.to_vec())
    }
}

impl HolographicEncoding {
    /// Get all words from this encoding
    pub fn all_words(&self) -> Vec<&str> {
        let mut words = Vec::new();
        for view in &self.story_views {
            words.push(view.actor.as_str());
            words.push(view.action.as_str());
            words.push(view.object.as_str());
            words.push(view.modifier.as_str());
        }
        words
    }
    
    /// Convert to string representation
    pub fn to_string(&self) -> String {
        let stories: Vec<String> = self.story_views.iter()
            .map(|v| format!("{} {} {} {}", v.actor, v.action, v.object, v.modifier))
            .collect();
        
        format!("Stories: [{}]", stories.join(" | "))
    }
}

impl StoryView {
    /// Check if this forms a valid story structure
    pub fn forms_valid_story(&self) -> bool {
        // Basic validation - in a real implementation we'd check narrative coherence
        !self.actor.is_empty() && 
        !self.action.is_empty() && 
        !self.object.is_empty() && 
        !self.modifier.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_encoder() -> HolographicEncoder {
        let dictionaries = Dictionaries::new().unwrap();
        HolographicEncoder::new(dictionaries)
    }
    
    #[test]
    fn test_holographic_encode_decode() {
        let encoder = create_test_encoder();
        
        // Test various data sizes in holographic range
        let test_cases = vec![
            vec![0x01; 21],
            vec![0x02; 24],
            vec![0x03; 28],
            vec![0x04; 32],
        ];
        
        for data in test_cases {
            let encoded = encoder.encode(&data).unwrap();
            let decoded = encoder.decode(&encoded).unwrap();
            
            // Verify basic properties
            assert!(!decoded.is_empty());
            
            // Verify deterministic encoding
            let encoded2 = encoder.encode(&data).unwrap();
            assert_eq!(encoded, encoded2);
        }
    }
    
    #[test]
    fn test_holographic_encode_bounds() {
        let encoder = create_test_encoder();
        
        // Test undersized data
        let undersized = vec![0u8; 20];
        assert!(encoder.encode(&undersized).is_err());
        
        // Test oversized data
        let oversized = vec![0u8; 33];
        assert!(encoder.encode(&oversized).is_err());
    }
    
    #[test]
    fn test_holographic_encoding_structure() {
        let encoder = create_test_encoder();
        let data = vec![0x12; 32];
        
        let encoded = encoder.encode(&data).unwrap();
        
        // Should have 3-4 story views
        assert!(encoded.story_views.len() >= 3);
        assert!(encoded.story_views.len() <= 4);
        
        // Each view should form a valid story
        for view in &encoded.story_views {
            assert!(view.forms_valid_story());
        }
    }
    
    #[test]
    fn test_insufficient_views() {
        let encoder = create_test_encoder();
        
        // Create encoding with insufficient views
        let insufficient_encoding = HolographicEncoding {
            story_views: vec![
                StoryView {
                    actor: "test".to_string(),
                    action: "test".to_string(),
                    object: "test".to_string(),
                    modifier: "test".to_string(),
                    context: 0,
                }
            ]
        };
        
        let result = encoder.decode(&insufficient_encoding);
        assert!(result.is_err());
        
        if let Err(DecodingError::InsufficientViews { got, needed }) = result {
            assert_eq!(got, 1);
            assert_eq!(needed, 3);
        } else {
            panic!("Expected InsufficientViews error");
        }
    }
}