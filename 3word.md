# Three-Word Network Address System: Technical Report

## Executive Summary

This report analyzes the feasibility of encoding network addresses (IPv4+port and IPv6+port) into memorable word sequences, inspired by geolocation systems like What3Words. We examine entropy requirements, algorithm options, and implementation strategies for creating a human-readable networking address system.

## Problem Statement

Current network addresses are difficult for humans to remember and communicate:
- IPv4 + port: `192.168.1.105:8080` (48 bits total)
- IPv6 + port: `2001:0db8:85a3:0000:0000:8a2e:0370:7334:8080` (144 bits total)

Goal: Create a memorable word-based system that maintains the same information entropy while being easier for humans to use.

## Entropy Analysis

### IPv4 + Port
- IPv4 address: 32 bits
- Port number: 16 bits  
- **Total: 48 bits** (281 trillion combinations)

### IPv6 + Port
- IPv6 address: 128 bits
- Port number: 16 bits
- **Total: 144 bits** (3.9 × 10^43 combinations)

### Word-Based Encoding Capacity

| Dictionary Size | Words Needed | Total Entropy | Use Case |
|----------------|--------------|---------------|----------|
| 16,384 (2^14) | 3 words | 42 bits | Too small for IPv4+port |
| 32,768 (2^15) | 3 words | 45 bits | Too small for IPv4+port |
| 65,536 (2^16) | 3 words | 48 bits | Perfect for IPv4+port |
| 4,096 (2^12) | 4 words | 48 bits | Alternative for IPv4+port |
| 8,192 (2^13) | 11 words | 143 bits | Close to IPv6+port needs |
| 16,384 (2^14) | 10 words | 140 bits | Close to IPv6+port needs |

## Recommended Approach

### For IPv4 + Port (3 words)

Use a **65,536-word dictionary** to achieve exactly 48 bits in 3 words.

```
IPv4 + Port (48 bits) → 3 words of 16 bits each
Example: 192.168.1.1:80 → "bridge.sunset.ocean"
```

### For IPv6 + Port (Flexible approach)

**Option 1: Variable Length (3-10 words)**
- Use prefix to indicate address type
- Common addresses use fewer words
- Full IPv6 when needed

**Option 2: Fixed 6 words with 2^24 dictionary**
- 24 bits × 6 = 144 bits (perfect match)
- Requires very large dictionary (16.7M words)
- Not practical due to dictionary size

**Option 3: Hybrid encoding (Recommended)**
- 5 words from 65,536-word dictionary = 80 bits
- 1 special word encoding remaining 64 bits using dense encoding
- Total: 6 words

## Algorithm Design

### Core Components

1. **Reversible Transformation (Feistel Network)**
   - Provides strong bit diffusion
   - Ensures adjacent IPs get very different words
   - Cryptographically secure shuffling
   - Fully reversible without storing mappings

2. **Word Selection Strategy**
   - Remove similar-sounding words (e.g., "there/their")
   - Exclude offensive combinations
   - Use word frequency for memorability
   - Consider international usage

3. **Error Detection**
   - Reserve some combinations for checksums
   - Use word patterns for basic validation
   - Implement Luhn-like algorithm for typo detection

### Feistel Network Implementation

```rust
// Pseudo-code for Feistel network
fn feistel_encode(input: u64, rounds: u32) -> u64 {
    let (mut left, mut right) = split_bits(input);
    
    for round in 0..rounds {
        let new_right = left ^ round_function(right, round);
        left = right;
        right = new_right;
    }
    
    combine_bits(left, right)
}

fn round_function(input: u32, round: u32) -> u32 {
    // Use cryptographic hash or custom mixing
    // This determines the security/randomness
    let key = derive_round_key(round);
    return siphash(input, key);
}
```

## Implementation Considerations

### 1. Dictionary Design

**IPv4+Port Dictionary (65,536 words)**
- Source from multiple languages for diversity
- Length: 4-8 characters optimal
- Phonetically distinct
- Common words for memorability

**Structure suggestion:**
- 40% English common words
- 20% Technical terms (familiar to IT users)
- 20% Place names
- 20% International common words

### 2. Collision Resistance

The system must ensure:
- No two similar-sounding combinations map to nearby IP addresses
- Geographic/network proximity doesn't correlate with word similarity
- Typos lead to invalid combinations (not wrong addresses)

### 3. Special Address Handling

```
Reserved Patterns:
- Private ranges (192.168.*, 10.*, etc.) → Special prefix
- Localhost (127.0.0.1) → "local.host.home"
- Common ports → Shorter words in position 3
```

### 4. Optimization Opportunities

**For P2P Networks:**
- Cache frequently used addresses
- Allow partial matching for local networks
- Support abbreviated forms for common patterns

**Performance:**
- Pre-compute Feistel rounds for speed
- Use SIMD for batch conversions
- Memory-map dictionary for fast lookup

## Security Considerations

1. **Address Privacy**: Words shouldn't reveal network topology
2. **Brute Force**: Dictionary size must prevent guessing
3. **Typo Squatting**: Similar words must map to very different IPs
4. **Replay Protection**: Consider time-based components for dynamic addressing

## Migration Strategy

### Phase 1: Internal Tools
- Implement in debugging/logging tools
- Test with team members
- Gather feedback on word choices

### Phase 2: API Integration
- Add as optional format in APIs
- Support both traditional and word formats
- Monitor usage patterns

### Phase 3: User-Facing
- Gradual rollout to end users
- Provide conversion tools
- Education and documentation

## Example Implementations

### IPv4 + Port Example
```
Input: 192.168.1.105:8080
Binary: 11000000 10101000 00000001 01101001 00011111 10010000
Feistel output: 10110010 11100101 00110011 11010001 10001011 01100110
Word indices: 45797, 13107, 54102
Output: "granite.falcon.river"
```

### IPv6 Compression Strategy
For IPv6, we can leverage the fact that many addresses have patterns:
- Leading zeros
- Consecutive zeros (::)
- Common prefixes

This allows variable-length encoding where common patterns use fewer words.

## Recommendations

1. **Start with IPv4+Port** using 65,536-word dictionary for exact 3-word encoding
2. **Use Feistel network** for superior bit diffusion compared to linear congruence
3. **Implement progressive enhancement** - basic system first, then optimize
4. **Consider context** - internal tools vs. public-facing usage
5. **Plan for IPv6** with flexible encoding scheme
6. **Test internationally** - ensure words work across cultures

## Conclusion

A three-word system for IPv4+port is entirely feasible with a 65,536-word dictionary. IPv6 requires compromise between word count and dictionary size. The Feistel network approach provides superior security and distribution properties compared to simpler algorithms while maintaining perfect reversibility.

The key insight from What3Words' approach is that human memorability is worth the trade-off of slightly longer representations, and that careful algorithm design can ensure error resistance and security.
