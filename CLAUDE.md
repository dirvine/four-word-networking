# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Four-Word Networking is a Rust library and CLI that converts IP addresses into memorable word combinations for human-friendly networking. The system provides perfect reconstruction for IPv4 addresses using exactly 4 words, and adaptive compression for IPv6 addresses using 4-6 words with intelligent category-based optimization.

## Common Development Commands

### Building and Testing
```bash
# Build the project
cargo build

# Build with release optimizations
cargo build --release

# Run all tests
cargo test

# Run tests with verbose output
cargo test -- --nocapture

# Run specific test
cargo test test_three_word_address_parsing

# Run exhaustive test suite (fast mode - 12,000+ tests)
./run_exhaustive_tests.sh

# Run full exhaustive test suite (11.1M tests - takes 30+ minutes)
./run_exhaustive_tests.sh full

# Run the CLI
cargo run --bin 4wn -- 192.168.1.1:443
```

### Code Quality
```bash
# Check code formatting
cargo fmt --check

# Format code
cargo fmt

# Run clippy for linting
cargo clippy -- -D warnings

# Check for unused dependencies
cargo machete
```

### CLI Usage Examples
```bash
# Convert IPv4 to four words (perfect reconstruction)
cargo run --bin 4wn -- 192.168.1.1:443

# Convert IPv6 to 4-6 words (adaptive compression)
cargo run --bin 4wn -- "[::1]:443"

# Decode words back to IP addresses
cargo run --bin 4wn -- paper.broaden.smith.bully

# Decode IPv6 from words
cargo run --bin 4wn -- City-Tub-Book-April-Book

# Verbose output
cargo run --bin 4wn -- -v 192.168.1.1:443
```

### Binary Tools
```bash
# Build and validate the 16K dictionary system
cargo run --bin validate_16k_system

# Check word quality in dictionary
cargo run --bin check_word_quality

# Create clean dictionary (removes homophones, offensive words)
cargo run --bin create_clean_dictionary

# Debug specific encoding issues
cargo run --bin debug_aim_issue
```

## Architecture

### Core Components

- **`src/lib.rs`**: Main library interface and public API
- **`src/four_word_encoder.rs`**: Four-word encoder system for perfect IPv4 and adaptive IPv6
- **`src/ipv6_compression.rs`**: IPv6 category-based compression algorithms
- **`src/error.rs`**: Comprehensive error types using `thiserror`
- **`src/main.rs`**: CLI application using `clap`
- **`src/bin/4wn.rs`**: Command-line interface for four-word networking

### Advanced Encoding Systems

- **`src/dictionary16k.rs`**: 16,384-word dictionary with quality filtering
- **`src/encoder16k.rs`**: Enhanced encoder using 14-bit word indices
- **`src/compression.rs`**: Multiaddress compression achieving 40-60% reduction
- **`src/balanced_encoder.rs`**: Natural word grouping with compression
- **`src/ultra_compression.rs`**: Aggressive compression for ≤5 byte output
- **`src/ultra_compact_encoder.rs`**: Perfect 3-word encoding with 75-87% compression

### Universal Encoding Module (`src/universal/`)

Experimental system for encoding arbitrary 32-byte data:
- **`simple.rs`**: ≤8 byte encoding (4 words only)
- **`fractal.rs`**: 9-20 byte encoding (base + zoom levels)
- **`holographic.rs`**: 21-32 byte encoding (multiple story views)
- **`dictionaries.rs`**: Four specialized 4,096-word dictionaries

### Key Data Structures

- **`FourWordEncoding`**: Four-word address structure with IP version detection
- **`FourWordAdaptiveEncoder`**: Main interface for encoding/decoding IP addresses
- **`FourWordDictionary`**: 16,384-word dictionary for encoding
- **`CompressedIpv6`**: IPv6 compression with category-based optimization
- **`Ipv6Category`**: IPv6 address types (Loopback, LinkLocal, GlobalUnicast, etc.)
- **`UltraCompactData`**: 5-byte compressed format

## Encoding Strategies

### Ultra-Compact Encoding (Production Ready)
- **Localhost**: 3-byte encoding for 127.0.0.1 patterns
- **Private Networks**: 4-5 byte encoding for RFC1918 addresses
- **Common Ports**: Single-byte codes for well-known ports
- **Protocol Packing**: Bit-packed protocol headers
- **Performance**: 0.37-1.79μs encoding, <0.00005% collision rate

### Compression Techniques
```rust
// Example: Localhost compression
// /ip4/127.0.0.1/tcp/8080 → 3 bytes total
// Header: 0b10000000 (localhost marker)
// Port: 2 bytes for 8080
```

## Dictionary Management

### Word Sources
- **EFF Large**: 7,776 secure passphrase words
- **BIP39**: 2,048 cryptocurrency mnemonic words
- **Diceware 8K**: 8,192 security-focused words
- **Custom English**: Additional curated words

### Quality Criteria
- Length: 4-8 characters
- Voice-friendly: Easy to pronounce
- No homophones or offensive terms
- Phonetically distinct
- Common English usage preferred

## Development Patterns

### Error Handling
```rust
// Always use Result types
pub fn encode(addr: &str) -> Result<String, ThreeWordError> {
    // Implementation
}

// Use ? operator for propagation
let parsed = parse_multiaddr(addr)?;
```

### Testing Strategy
- Unit tests in `#[cfg(test)]` modules
- Integration tests for workflows
- Real-world address testing (Bitcoin, Ethereum)
- Exhaustive validation (millions of addresses)
- Performance benchmarks (<2μs requirement)

### Code Organization
- Feature-focused module structure
- Clear separation of concerns
- Comprehensive rustdoc documentation
- Examples in all public APIs

## Performance Targets

- **Encoding**: <2μs per address
- **Decoding**: <2μs per address
- **Memory**: <1MB total dictionary size
- **Throughput**: ~100,000 addresses/second
- **Collision Rate**: <0.00005%

## Current Implementation Status

### Production Ready
- Ultra-compact multiaddr encoding
- 16K word dictionary system
- CLI with full feature set
- Comprehensive test coverage

### Experimental
- Universal 32-byte encoding
- Multi-language support structure
- Advanced collision resolution

### Known Limitations
- English-only dictionaries currently
- Simplified address recovery in demo mode
- Some edge cases in exotic multiaddr formats

## Future Development Areas

### High Priority
- Perfect address reconstruction
- Multi-language dictionary support
- WebAssembly bindings
- Integration with libp2p

### Medium Priority
- GUI applications
- Browser extensions
- Mobile SDKs
- Network visualization tools

## Dependencies

### Core
- `serde`: Serialization (with derive)
- `thiserror`: Error handling
- `clap`: CLI parsing (with derive)
- `tokio`: Async runtime (full features)

### Encoding
- `hex`: Hexadecimal encoding
- `bs58`: Base58 encoding
- `bitvec`: Bit manipulation

### Testing
- `tokio-test`: Async test utilities
- `rand`: Random generation
- `sha2`: Hashing for tests

## Useful Resources

- **Test Script**: `./run_exhaustive_tests.sh` for comprehensive validation
- **Binary Tools**: 16 utilities in `src/bin/` for development tasks
- **Word Lists**: Raw dictionaries in `wordlists/` directory
- **Processed Dictionaries**: Quality-filtered versions in `data/`