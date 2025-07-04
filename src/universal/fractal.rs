//! Fractal encoder for data 9-20 bytes

use crate::universal::dictionaries::Dictionaries;
use crate::universal::error::{EncodingError, EncodingResult, DecodingResult};
use crate::universal::simple::{SimpleEncoder, ThreeWords};

/// Fractal encoder for data 9-20 bytes
#[derive(Debug, Clone)]
pub struct FractalEncoder {
    simple_encoder: SimpleEncoder,
}

/// Fractal encoding with base words and zoom levels
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FractalEncoding {
    pub base_words: ThreeWords,
    pub zoom_levels: Vec<ZoomLevel>,
}

/// A zoom level that adds precision to the base encoding
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ZoomLevel {
    pub modifier: String,
    pub refinement: u16,
}

impl FractalEncoder {
    /// Create a new fractal encoder
    pub fn new(dictionaries: Dictionaries) -> Self {
        let simple_encoder = SimpleEncoder::new(dictionaries);
        Self { simple_encoder }
    }
    
    /// Encode data into fractal encoding
    pub fn encode(&self, data: &[u8]) -> EncodingResult<FractalEncoding> {
        if data.len() < 9 {
            return Err(EncodingError::DataTooSmall(data.len()));
        }
        
        if data.len() > 20 {
            return Err(EncodingError::DataTooLarge(data.len()));
        }
        
        // Encode first 8 bytes as base words
        let base_words = self.simple_encoder.encode(&data[..8])?;
        
        // Encode remaining bytes as zoom levels
        let remaining = &data[8..];
        let zoom_levels = self.encode_zoom_levels(remaining)?;
        
        Ok(FractalEncoding {
            base_words,
            zoom_levels,
        })
    }
    
    /// Decode fractal encoding back to original data
    pub fn decode(&self, encoding: &FractalEncoding) -> DecodingResult<Vec<u8>> {
        // Decode base words
        let mut result = self.simple_encoder.decode(&encoding.base_words)?;
        
        // Decode zoom levels
        let zoom_bytes = self.decode_zoom_levels(&encoding.zoom_levels)?;
        result.extend(zoom_bytes);
        
        Ok(result)
    }
    
    /// Encode remaining bytes as zoom levels (placeholder implementation)
    fn encode_zoom_levels(&self, data: &[u8]) -> EncodingResult<Vec<ZoomLevel>> {
        let mut zoom_levels = Vec::new();
        
        // For now, create one zoom level per remaining byte
        // In a real implementation, we'd pack multiple bytes per zoom level
        for &byte in data {
            let modifier_index = (byte as usize) % 4096;
            let modifier = self.simple_encoder.dictionaries.get_modifier(modifier_index)
                .map_err(|_| EncodingError::DictionaryNotLoaded)?
                .to_string();
            
            zoom_levels.push(ZoomLevel {
                modifier,
                refinement: byte as u16,
            });
        }
        
        Ok(zoom_levels)
    }
    
    /// Decode zoom levels back to bytes (placeholder implementation)
    fn decode_zoom_levels(&self, zoom_levels: &[ZoomLevel]) -> DecodingResult<Vec<u8>> {
        let mut result = Vec::new();
        
        for zoom in zoom_levels {
            // For now, just use the refinement value
            // In a real implementation, we'd reconstruct from modifier + refinement
            result.push(zoom.refinement as u8);
        }
        
        Ok(result)
    }
}

impl FractalEncoding {
    /// Get all words from this encoding
    pub fn all_words(&self) -> Vec<&str> {
        let mut words = self.base_words.all_words();
        for zoom in &self.zoom_levels {
            words.push(&zoom.modifier);
        }
        words
    }
    
    /// Convert to string representation
    pub fn to_string(&self) -> String {
        let base = self.base_words.to_string();
        let zooms: Vec<String> = self.zoom_levels.iter()
            .map(|z| format!("{}:{}", z.modifier, z.refinement))
            .collect();
        
        if zooms.is_empty() {
            base
        } else {
            format!("{} → {}", base, zooms.join(" "))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_encoder() -> FractalEncoder {
        let dictionaries = Dictionaries::new().unwrap();
        FractalEncoder::new(dictionaries)
    }
    
    #[test]
    fn test_fractal_encode_decode() {
        let encoder = create_test_encoder();
        
        // Test various data sizes in fractal range
        let test_cases = vec![
            vec![0x01; 9],
            vec![0x02; 12],
            vec![0x03; 16],
            vec![0x04; 20],
        ];
        
        for data in test_cases {
            let encoded = encoder.encode(&data).unwrap();
            let decoded = encoder.decode(&encoded).unwrap();
            
            // Verify basic properties
            assert!(!decoded.is_empty());
            assert!(decoded.len() >= 9);
            
            // Verify deterministic encoding
            let encoded2 = encoder.encode(&data).unwrap();
            assert_eq!(encoded, encoded2);
        }
    }
    
    #[test]
    fn test_fractal_encode_bounds() {
        let encoder = create_test_encoder();
        
        // Test undersized data
        let undersized = vec![0u8; 8];
        assert!(encoder.encode(&undersized).is_err());
        
        // Test oversized data
        let oversized = vec![0u8; 21];
        assert!(encoder.encode(&oversized).is_err());
    }
    
    #[test]
    fn test_fractal_encoding_structure() {
        let encoder = create_test_encoder();
        let data = vec![0x12; 15];
        
        let encoded = encoder.encode(&data).unwrap();
        
        // Should have base words
        assert!(!encoded.base_words.actor.is_empty());
        assert!(!encoded.base_words.action.is_empty());
        assert!(!encoded.base_words.object.is_empty());
        
        // Should have zoom levels for remaining bytes
        assert_eq!(encoded.zoom_levels.len(), 7); // 15 - 8 = 7
    }
}