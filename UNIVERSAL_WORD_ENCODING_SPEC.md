# Universal Word Encoding System - Implementation Specification

## Project Overview
Implement a Universal Word Encoding System that can encode ANY data from network addresses (multiaddresses) up to 32-byte hashes into human-memorable word sequences, with 100% accurate encode/decode capability.

## Core Requirements

### 1. Data Types to Support
- **Network Addresses**: IPv4, IPv6, multiaddresses (e.g., /ip4/127.0.0.1/tcp/8080)
- **Cryptocurrency Addresses**: Bitcoin (25 bytes typical), Ethereum (20 bytes)
- **Content Hashes**: SHA-256 (32 bytes), IPFS CIDs
- **Node IDs**: Ed25519 public keys (32 bytes), libp2p peer IDs
- **Arbitrary Data**: Any byte sequence up to 32 bytes

### 2. Encoding Modes
Implement three encoding strategies based on data size:
```rust
pub enum EncodingStrategy {
    Simple,      // <= 8 bytes: 4 words only
    Fractal,     // 9-20 bytes: 3 base words + zoom levels
    Holographic, // 21-32 bytes: Multiple story views
}
```

### 3. Dictionary Requirements
Create four specialized dictionaries with 4096 words each:
```rust
pub struct Dictionaries {
    actors: Vec<String>,    // Characters: falcon, wizard, knight, dragon
    actions: Vec<String>,   // Verbs: crosses, guards, seeks, finds
    objects: Vec<String>,   // Things: bridge, mountain, treasure, forest
    modifiers: Vec<String>, // Adjectives: ancient, silver, hidden, northern
}
```

**Dictionary Constraints:**
- Each word 4-8 characters
- No homophones (to/too/two)
- No offensive words
- Phonetically distinct
- Easy to pronounce across languages

## Implementation Structure

### Phase 1: Core Encoding Engine
```rust
pub struct UniversalEncoder {
    dictionaries: Dictionaries,
    simple_encoder: SimpleEncoder,
    fractal_encoder: FractalEncoder,
    holographic_encoder: HolographicEncoder,
}

impl UniversalEncoder {
    pub fn encode(&self, data: &[u8]) -> Result<UniversalEncoding, EncodingError> {
        // Determine strategy based on data length
        // Route to appropriate encoder
        // Return unified encoding format
    }
    
    pub fn decode(&self, encoding: &UniversalEncoding) -> Result<Vec<u8>, DecodingError> {
        // Parse encoding format
        // Route to appropriate decoder
        // Return original bytes
    }
}
```

### Phase 2: Simple Encoder (≤ 8 bytes)
For network addresses and small data:
```rust
impl SimpleEncoder {
    pub fn encode(&self, data: &[u8]) -> ThreeWords {
        // Convert bytes to 3 word indices
        // Each word from different dictionary
        // Deterministic mapping
    }
    
    pub fn decode(&self, words: &ThreeWords) -> Vec<u8> {
        // Reverse word indices to bytes
        // Perfect reconstruction
    }
}
```

### Phase 3: Fractal Encoder (9-20 bytes)
For cryptocurrency addresses and node IDs:
```rust
impl FractalEncoder {
    pub fn encode(&self, data: &[u8]) -> FractalEncoding {
        // Generate base 4 words (first 8 bytes)
        // Generate zoom levels for remaining bytes
        // Each zoom level adds ~5 bytes of precision
    }
    
    pub fn decode(&self, encoding: &FractalEncoding) -> Vec<u8> {
        // Decode base region
        // Apply zoom refinements
        // Reconstruct exact bytes
    }
}
```

### Phase 4: Holographic Encoder (21-32 bytes)
For full hashes:
```rust
impl HolographicEncoder {
    pub fn encode(&self, data: &[u8]) -> Vec<StoryView> {
        // Generate 3-4 different story views
        // Each view encodes hash from different "angle"
        // Together they specify unique hash
    }
    
    pub fn decode(&self, views: &[StoryView]) -> Result<Vec<u8>, DecodingError> {
        // Combine constraints from all views
        // Solve for unique hash
        // Error if insufficient views
    }
}
```

## Critical Test Suite

### Test 1: Exhaustive Simple Encoding
```rust
#[test]
fn test_all_network_addresses() {
    let encoder = UniversalEncoder::new();
    
    // Test all possible IPv4 addresses (sample)
    for _ in 0..10000 {
        let addr = random_ipv4();
        let encoded = encoder.encode(&addr.octets()).unwrap();
        let decoded = encoder.decode(&encoded).unwrap();
        assert_eq!(addr.octets().to_vec(), decoded);
    }
    
    // Test multiaddresses
    let multiaddrs = vec![
        "/ip4/127.0.0.1/tcp/8080",
        "/ip6/::1/tcp/8080",
        "/dns4/example.com/tcp/443",
        "/ip4/192.168.1.1/udp/4001/quic",
    ];
    
    for ma in multiaddrs {
        let bytes = parse_multiaddr(ma);
        let encoded = encoder.encode(&bytes).unwrap();
        let decoded = encoder.decode(&encoded).unwrap();
        assert_eq!(bytes, decoded);
    }
}
```

### Test 2: Cryptocurrency Address Coverage
```rust
#[test]
fn test_crypto_addresses() {
    let encoder = UniversalEncoder::new();
    
    // Test 1000 random Bitcoin addresses
    for _ in 0..1000 {
        let privkey = random_bytes(32);
        let pubkey = derive_pubkey(&privkey);
        let addr = bitcoin_address(&pubkey);
        
        let encoded = encoder.encode(&addr).unwrap();
        let decoded = encoder.decode(&encoded).unwrap();
        assert_eq!(addr.to_vec(), decoded);
    }
    
    // Test Ethereum addresses
    for _ in 0..1000 {
        let addr = random_eth_address();
        let encoded = encoder.encode(&addr).unwrap();
        let decoded = encoder.decode(&encoded).unwrap();
        assert_eq!(addr.to_vec(), decoded);
    }
}
```

### Test 3: Full 32-byte Hash Coverage
```rust
#[test]
fn test_32_byte_hashes() {
    let encoder = UniversalEncoder::new();
    
    // Test 10,000 random 32-byte hashes
    for _ in 0..10000 {
        let hash = random_bytes(32);
        let encoded = encoder.encode(&hash).unwrap();
        let decoded = encoder.decode(&encoded).unwrap();
        assert_eq!(hash, decoded);
    }
    
    // Test known SHA-256 hashes
    let test_vectors = vec![
        sha256(b""),
        sha256(b"hello world"),
        sha256(b"The quick brown fox jumps over the lazy dog"),
    ];
    
    for hash in test_vectors {
        let encoded = encoder.encode(&hash).unwrap();
        let decoded = encoder.decode(&encoded).unwrap();
        assert_eq!(hash.to_vec(), decoded);
    }
}
```

### Test 4: Edge Cases
```rust
#[test]
fn test_edge_cases() {
    let encoder = UniversalEncoder::new();
    
    // Empty data
    let empty = vec![];
    test_round_trip(&encoder, &empty);
    
    // Single byte
    for byte in 0u8..=255 {
        test_round_trip(&encoder, &[byte]);
    }
    
    // All zeros
    let zeros = vec![0u8; 32];
    test_round_trip(&encoder, &zeros);
    
    // All ones
    let ones = vec![0xFFu8; 32];
    test_round_trip(&encoder, &ones);
    
    // Sequential patterns
    let sequential: Vec<u8> = (0..32).collect();
    test_round_trip(&encoder, &sequential);
}
```

### Test 5: Collision Resistance
```rust
#[test]
fn test_no_collisions() {
    let encoder = UniversalEncoder::new();
    let mut encodings = HashMap::new();
    
    // Generate 100,000 random inputs
    for i in 0..100_000 {
        let size = (i % 32) + 1; // 1-32 bytes
        let data = random_bytes(size);
        let encoded = encoder.encode(&data).unwrap();
        let encoded_str = encoded.to_string();
        
        // Check for collisions
        if let Some(existing) = encodings.get(&encoded_str) {
            if existing != &data {
                panic!("Collision found! {} and {:?} both encode to {}", 
                       hex::encode(existing), hex::encode(&data), encoded_str);
            }
        }
        
        encodings.insert(encoded_str, data);
    }
}
```

### Test 6: Determinism
```rust
#[test]
fn test_deterministic_encoding() {
    let encoder1 = UniversalEncoder::new();
    let encoder2 = UniversalEncoder::new();
    
    for _ in 0..1000 {
        let data = random_bytes(rand::random::<u8>() % 32 + 1);
        
        let enc1 = encoder1.encode(&data).unwrap();
        let enc2 = encoder2.encode(&data).unwrap();
        
        assert_eq!(enc1, enc2, "Encoding must be deterministic");
    }
}
```

### Test 7: Human Usability
```rust
#[test]
fn test_human_friendly() {
    let encoder = UniversalEncoder::new();
    
    // Test pronunciability
    for _ in 0..100 {
        let data = random_bytes(32);
        let encoded = encoder.encode(&data).unwrap();
        
        for word in encoded.all_words() {
            assert!(word.len() >= 4 && word.len() <= 8);
            assert!(is_pronounceable(&word));
            assert!(!contains_numbers(&word));
        }
    }
    
    // Test memorability (story structure)
    match encoded {
        UniversalEncoding::Holographic(views) => {
            for view in views {
                assert!(forms_valid_story(&view));
            }
        }
        _ => {}
    }
}
```

## Performance Requirements
- **Encoding**: < 1ms for any input up to 32 bytes
- **Decoding**: < 1ms for any valid encoding
- **Memory**: < 10MB for all dictionaries
- **Zero allocations in hot path**

## Error Handling
```rust
pub enum EncodingError {
    DataTooLarge(usize),
    InvalidMultiaddr(String),
    DictionaryNotLoaded,
}

pub enum DecodingError {
    InvalidWord(String),
    InsufficientViews,
    ChecksumMismatch,
    InvalidFormat,
}
```

## Deliverables

1. **Core Library** (universal-word-encoder)
   - Encoding/decoding engine
   - Dictionary management
   - Error handling

2. **Test Suite** (tests/)
   - Unit tests for each encoder
   - Integration tests for round-trip
   - Property-based tests
   - Benchmark suite

3. **Examples** (examples/)
   - Network address encoding
   - Cryptocurrency address encoding
   - Content hash encoding
   - P2P node discovery

4. **Documentation**
   - API documentation
   - Usage guide
   - Dictionary format specification

## Success Criteria

- **100% Round-trip Accuracy**: Every possible input up to 32 bytes must encode and decode perfectly
- **No Collisions**: Different inputs must produce different encodings
- **Deterministic**: Same input always produces same output
- **Human-Friendly**: All words pronounceable and memorable
- **Fast**: Sub-millisecond performance
- **Tested**: >95% code coverage with property-based tests

## Implementation Notes

- Use Rust for performance and safety
- Consider using proptest for property-based testing
- Implement custom serialization to avoid overhead
- Use const arrays where possible
- Pre-compute lookup tables for decoding
- Consider SIMD optimizations for batch operations

## Example Usage
```rust
use universal_word_encoder::UniversalEncoder;

fn main() {
    let encoder = UniversalEncoder::new();
    
    // Network address
    let ip = "192.168.1.100:8080";
    let encoded = encoder.encode_multiaddr(ip).unwrap();
    println!("{} → {}", ip, encoded); // "falcon.crosses.bridge"
    
    // Bitcoin address
    let btc = "1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa";
    let encoded = encoder.encode_bitcoin(btc).unwrap();
    println!("{} → {}", btc, encoded); // "ocean.treasure.chest → ancient.northern"
    
    // SHA-256 hash
    let hash = sha256(b"Hello, world!");
    let encoded = encoder.encode(&hash).unwrap();
    println!("SHA256 → {}", encoded); // Multiple story views
}
```

This system will revolutionize how we share network addresses and cryptocurrency addresses, making them truly human-friendly while maintaining perfect accuracy.

---

## TODO List

### Phase 1: Project Setup and Core Architecture
- [ ] Create new Rust project structure for universal-word-encoder
- [ ] Design core data structures (UniversalEncoder, EncodingStrategy, Dictionaries)
- [ ] Implement basic error handling types (EncodingError, DecodingError)
- [ ] Create unified encoding format enum (UniversalEncoding)

### Phase 2: Dictionary Development
- [ ] Research and curate 4096 actor words (falcon, wizard, knight, dragon)
- [ ] Research and curate 4096 action words (crosses, guards, seeks, finds)
- [ ] Research and curate 4096 object words (bridge, mountain, treasure, forest)
- [ ] Research and curate 4096 modifier words (ancient, silver, hidden, northern)
- [ ] Validate dictionary constraints (4-8 chars, no homophones, pronounceable)
- [ ] Implement dictionary loading and management system

### Phase 3: Simple Encoder Implementation (≤ 8 bytes)
- [ ] Implement SimpleEncoder struct and basic encode/decode methods
- [ ] Create deterministic byte-to-word-index mapping algorithm
- [ ] Implement ThreeWords data structure
- [ ] Add round-trip testing for simple encoder
- [ ] Optimize for performance (sub-millisecond requirement)

### Phase 4: Fractal Encoder Implementation (9-20 bytes)
- [ ] Design FractalEncoding data structure
- [ ] Implement base 3-word generation (first 8 bytes)
- [ ] Implement zoom level system for remaining bytes
- [ ] Create fractal encode/decode algorithms
- [ ] Add comprehensive testing for fractal encoder

### Phase 5: Holographic Encoder Implementation (21-32 bytes)
- [ ] Design StoryView data structure
- [ ] Implement multiple story view generation algorithm
- [ ] Create holographic encoding that generates 3-4 story views
- [ ] Implement constraint solving for holographic decoding
- [ ] Add extensive testing for holographic encoder

### Phase 6: Integration and Testing
- [ ] Integrate all encoders into UniversalEncoder
- [ ] Implement routing logic based on data size
- [ ] Create comprehensive test suite (Tests 1-7 from spec)
- [ ] Add property-based testing with proptest
- [ ] Implement collision resistance testing
- [ ] Add performance benchmarks

### Phase 7: Specialized Input Handlers
- [ ] Add multiaddr parsing and encoding support
- [ ] Implement cryptocurrency address encoding (Bitcoin, Ethereum)
- [ ] Add content hash encoding (SHA-256, IPFS CIDs)
- [ ] Implement node ID encoding (Ed25519, libp2p peer IDs)
- [ ] Create specialized API methods for each data type

### Phase 8: Performance Optimization
- [ ] Optimize dictionary lookup tables
- [ ] Implement zero-allocation encoding paths
- [ ] Add SIMD optimizations where applicable
- [ ] Profile and optimize memory usage (<10MB requirement)
- [ ] Ensure sub-millisecond performance for all operations

### Phase 9: Documentation and Examples
- [ ] Write comprehensive API documentation
- [ ] Create usage guide and tutorials
- [ ] Implement network address encoding examples
- [ ] Add cryptocurrency address encoding examples
- [ ] Create P2P node discovery examples
- [ ] Document dictionary format specification

### Phase 10: Final Testing and Validation
- [ ] Achieve >95% code coverage
- [ ] Validate 100% round-trip accuracy
- [ ] Confirm zero collisions in extensive testing
- [ ] Verify deterministic behavior across all inputs
- [ ] Test human-friendliness and pronunciability
- [ ] Performance validation (all requirements met)

### Phase 11: CLI and Integration
- [ ] Create command-line interface for the universal encoder
- [ ] Integrate with existing four-word-networking CLI
- [ ] Add migration path from current system
- [ ] Create compatibility layer if needed

### Phase 12: Documentation and Release
- [ ] Finalize all documentation
- [ ] Create release notes and changelog
- [ ] Prepare for crates.io publishing
- [ ] Set up CI/CD pipeline for testing
- [ ] Create examples and usage demonstrations

---

*This specification serves as the comprehensive implementation guide for the Universal Word Encoding System. Each TODO item represents a concrete step toward building a revolutionary system for human-friendly data encoding.*