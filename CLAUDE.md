# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Three-Word Networking is a Rust library and CLI that converts IP addresses into memorable word combinations for human-friendly networking. The system provides perfect reconstruction for IPv4 addresses using exactly 3 words, and adaptive compression for IPv6 addresses using 6 or 9 words (groups of 3) with intelligent category-based optimization.

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

# Run main test suite
./run_main_tests.sh

# Run the CLI
cargo run --bin 3wn -- 192.168.1.1:443
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
# Convert IPv4 to three words (perfect reconstruction)
cargo run --bin 3wn -- 192.168.1.1:443

# Convert IPv6 to 6 or 9 words (groups of 3)
cargo run --bin 3wn -- "[::1]:443"

# Decode words back to IP addresses (dots or spaces)
cargo run --bin 3wn -- lehr.delfs.enrages
cargo run --bin 3wn -- lehr delfs enrages

# Decode IPv6 from words
cargo run --bin 3wn -- Kaufhof-Dingley-Inno-Roupe-Stimuli-Bugger

# Verbose output
cargo run --bin 3wn -- -v 192.168.1.1:443
```

### Binary Tools
```bash
# Build and validate the dictionary systems
cargo run --bin validate_16k_system

# Check word quality in dictionary
cargo run --bin check_word_quality

# Create clean dictionary (removes homophones, offensive words)
cargo run --bin create_clean_dictionary

# Test three-word system
cargo run --bin test_three_word

# Benchmark three vs four word systems
cargo run --bin benchmark_three_vs_four
```

## Architecture

### Core Components

- **`src/lib.rs`**: Main library interface and public API
- **`src/three_word_adaptive_encoder.rs`**: Three-word adaptive encoder system for perfect IPv4 and adaptive IPv6
- **`src/ipv6_compression.rs`**: IPv6 category-based compression algorithms
- **`src/error.rs`**: Comprehensive error types using `thiserror`
- **`src/main.rs`**: CLI application using `clap`
- **`src/bin/3wn.rs`**: Command-line interface for three-word networking

### Three-Word Encoding Systems

- **`src/dictionary65k.rs`**: 65,536-word dictionary for IPv4 3-word encoding (2^16 words)
- **`src/three_word_encoder.rs`**: Perfect 3-word encoding for IPv4 with Feistel network
- **`src/three_word_ipv6_encoder.rs`**: Groups of 3 encoding for IPv6 (6 or 9 words)
- **`src/dictionary16k.rs`**: 16,384-word dictionary used in legacy four-word system

### Advanced Encoding Systems

- **`src/compression.rs`**: IP address compression achieving 40-60% reduction
- **`src/ipv6_compression.rs`**: Category-based IPv6 compression
- **`src/encoder16k.rs`**: Legacy four-word encoder using 14-bit word indices
- **`src/balanced_encoder.rs`**: Natural word grouping with compression
- **`src/ultra_compression.rs`**: Aggressive compression for ≤5 byte output

### Key Data Structures

- **`ThreeWordEncoding`**: Three-word IPv4 address structure
- **`ThreeWordAdaptiveEncoder`**: Main interface for encoding/decoding IP addresses
- **`Dictionary65K`**: 65,536-word dictionary for 16-bit per word encoding
- **`Ipv6ThreeWordGroupEncoding`**: IPv6 encoding in groups of 3 words
- **`ThreeWordGroup`**: Container for 3-word groups
- **`CompressedIpv6`**: IPv6 compression with category-based optimization
- **`Ipv6Category`**: IPv6 address types (Loopback, LinkLocal, GlobalUnicast, etc.)

## Encoding Strategies

### Three-Word IPv4 Encoding
- **Perfect Reconstruction**: 48 bits (IPv4 + port) encoded in 3 × 16-bit words
- **Feistel Network**: 8 rounds of cryptographic bit diffusion
- **Dictionary**: 65,536 words (2^16) for perfect 16-bit encoding
- **Format**: Lowercase words separated by dots or spaces

### IPv6 Group Encoding
- **Groups of 3**: Always 6 or 9 words (2 or 3 groups)
- **Category Detection**: Optimizes based on IPv6 type
- **Compression**: 6 words for common patterns, 9 for complex addresses
- **Format**: Title case words separated by dashes

### Compression Techniques
```rust
// Example: IPv6 category-based compression
match category {
    Ipv6Category::Loopback => compress_loopback(),
    Ipv6Category::LinkLocal => compress_link_local(),
    Ipv6Category::Documentation => compress_documentation(),
    Ipv6Category::GlobalUnicast => compress_global(),
}
```

## Dictionary Management

### 65K Dictionary (IPv4)
- **Size**: 65,536 words (2^16)
- **Sources**: EFF, BIP39, Diceware, custom English words
- **Quality**: Voice-friendly, no homophones, 3-7 characters

### Word Quality Criteria
- Length: 3-7 characters optimal
- Voice-friendly: Easy to pronounce
- No homophones or offensive terms
- Phonetically distinct
- Common English usage preferred

## Development Patterns

### Error Handling
```rust
// Always use Result types
pub fn encode(addr: &str) -> Result<String, FourWordError> {
    // Implementation
}

// Use ? operator for propagation
let parsed = parse_address(addr)?;
```

### Testing Strategy
- Unit tests in `#[cfg(test)]` modules
- Integration tests for workflows
- Real-world address testing
- Performance benchmarks (<2μs requirement)
- CLI integration tests

### Code Organization
- Feature-focused module structure
- Clear separation of concerns
- Comprehensive rustdoc documentation
- Examples in all public APIs

## Performance Targets

- **Encoding**: <1μs for IPv4, <2μs for IPv6
- **Decoding**: <1μs for IPv4, <2μs for IPv6
- **Memory**: ~1MB total dictionary size
- **Throughput**: ~1,000,000 addresses/second
- **Zero Collisions**: Deterministic encoding

## Current Implementation Status

### Production Ready
- Three-word IPv4 encoding with perfect reconstruction
- IPv6 encoding in groups of 3 (6 or 9 words)
- 65K word dictionary system
- CLI with full feature set (`3wn`)
- Space-separated word support
- Comprehensive test coverage

### Features
- IPv4: Always exactly 3 words
- IPv6: 6 words for common patterns, 9 for complex
- Visual distinction: dots vs dashes, case differences
- Voice-optimized dictionary
- Sub-microsecond performance

### Known Limitations
- English-only dictionaries currently
- Some IPv6 patterns may require 9 words

## Future Development Areas

### High Priority
- Multi-language dictionary support
- WebAssembly bindings
- Python/JavaScript bindings
- Integration with networking libraries

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
- `once_cell`: Global dictionary singleton

### Testing
- `tokio-test`: Async test utilities
- `rand`: Random generation
- `criterion`: Benchmarking

## Useful Resources

- **Test Script**: `./run_main_tests.sh` for test suite
- **Binary Tools**: Multiple utilities in `src/bin/` for development
- **Word Lists**: Raw dictionaries in `wordlists/` directory
- **65K Dictionary**: Pre-built in `data/dictionary_65536.txt`