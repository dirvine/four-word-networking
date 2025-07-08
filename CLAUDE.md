# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Three-Word Networking is a Rust library and CLI that converts complex network multiaddresses into memorable three-word combinations for human-friendly networking. The system has evolved through multiple encoding strategies, from the original 4,096-word system to advanced 16K word dictionaries and ultra-compact encoding that achieves 75-87% compression while maintaining perfect three-word outputs.

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
cargo run -- examples --count 5
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
# Convert multiaddr to three words
cargo run -- encode "/ip6/2001:db8::1/udp/9000/quic"

# Convert with base format only (no suffix)
cargo run -- encode "/ip6/2001:db8::1/udp/9000/quic" --base

# Validate a three-word address
cargo run -- validate "ocean.thunder.falcon"

# Show address space information
cargo run -- info

# Generate examples
cargo run -- examples --count 10
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
- **`src/words.rs`**: Original three-word address system (4,096 words, 12 bits/word)
- **`src/multiaddr_parser.rs`**: Multiaddr parsing and component extraction
- **`src/error.rs`**: Comprehensive error types using `thiserror`
- **`src/main.rs`**: CLI application using `clap`

### Advanced Encoding Systems

- **`src/dictionary16k.rs`**: 16,384-word dictionary with quality filtering
- **`src/encoder16k.rs`**: Enhanced encoder using 14-bit word indices
- **`src/compression.rs`**: Multiaddress compression achieving 40-60% reduction
- **`src/balanced_encoder.rs`**: Natural word grouping with compression
- **`src/ultra_compression.rs`**: Aggressive compression for ≤5 byte output
- **`src/ultra_compact_encoder.rs`**: Perfect 3-word encoding with 75-87% compression

### Universal Encoding Module (`src/universal/`)

Experimental system for encoding arbitrary 32-byte data:
- **`simple.rs`**: ≤8 byte encoding (3 words only)
- **`fractal.rs`**: 9-20 byte encoding (base + zoom levels)
- **`holographic.rs`**: 21-32 byte encoding (multiple story views)
- **`dictionaries.rs`**: Four specialized 4,096-word dictionaries

### Key Data Structures

- **`ThreeWordAddress`**: Three-word address with optional numeric suffix
- **`WordEncoder`**: Main interface for encoding/decoding
- **`WordDictionary`**: Position-specific word lists (context, quality, identity)
- **`ParsedMultiaddr`**: Structured multiaddr components
- **`MultiAddrBytes`**: Compressed multiaddr representation
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