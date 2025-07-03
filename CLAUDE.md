# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Three-Word Networking is a Rust library and CLI that converts complex network multiaddresses into memorable three-word combinations for human-friendly networking. The system maps multiaddrs like `/ip6/2001:db8::1/udp/9000/quic` to addresses like `ocean.thunder.falcon`.

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

## Architecture

### Core Components

- **`src/lib.rs`**: Main library interface and public API
- **`src/words.rs`**: Core three-word address system with bidirectional encoding/decoding logic
- **`src/multiaddr_parser.rs`**: Multiaddr parsing and component extraction module
- **`src/error.rs`**: Error types using `thiserror` for structured error handling
- **`src/main.rs`**: CLI application using `clap` for command-line interface

### Key Data Structures

- **`ThreeWordAddress`**: Represents a three-word address with optional numeric suffix
- **`WordEncoder`**: Main interface for encoding/decoding between multiaddrs and three-word addresses
- **`WordDictionary`**: Contains curated word lists for each position (context, quality, identity)
- **`ParsedMultiaddr`**: Structured representation of multiaddr components (IP type, protocol, address, port)

### Address Space Design

The system provides massive scale through:
- **Base Format**: `context.quality.identity` (68.7 billion combinations)
- **Extended Format**: `context.quality.identity.1847` (4.5 quadrillion combinations)
- **Dictionary Structure**: 4,096 words per position for maximum diversity

### Word Dictionary Structure

1. **Position 1 (Context)**: Geographic, network, and scale contexts
2. **Position 2 (Quality)**: Performance, purpose, and status descriptors
3. **Position 3 (Identity)**: Nature, objects, and abstract concepts

## Development Patterns

### Error Handling
- Uses `thiserror` for structured error types
- All public functions return `Result<T, ThreeWordError>`
- Never use `unwrap()` or `expect()` in production code
- Use `?` operator for error propagation

### Testing Strategy
- Unit tests for individual components in each module
- Integration tests for complete workflows
- Deterministic encoding tests to ensure consistency
- Address space validation tests
- Edge case testing for invalid inputs

### Code Organization
- Each module has comprehensive `#[cfg(test)]` sections
- Tests follow Arrange-Act-Assert pattern
- Public APIs are documented with rustdoc comments including examples
- Error messages are descriptive and user-friendly

## Key Implementation Details

### Encoding Process
1. Parse multiaddr into components (IP type, protocol, address, port)
2. Map IP type and additional protocols to context word index
3. Map primary protocol to quality word index  
4. Combine address hash and port to generate identity word index
5. Look up actual words from dictionary positions

### Decoding Process
1. Extract word indices from three-word address
2. Decode IP type from context index
3. Decode protocol from quality index
4. Reconstruct address and port from identity index (simplified in demo)
5. Generate valid multiaddr string

### Deterministic Behavior
- Same multiaddr always produces same three-word address
- Uses component-based deterministic mapping
- No external dependencies required for conversion
- Dictionary order must remain stable across versions

### Current Implementation
- **Bidirectional conversion**: Full encode/decode without registry
- **Collision resistant**: Advanced encoding reduces conflicts
- **Simplified address recovery**: Demo uses placeholder values for addresses
- **English-only dictionary**: Multi-language support planned

## Future Development Areas

### High Priority
- Enhanced address compression for perfect reconstruction
- Multi-language dictionary support
- Performance optimization for large-scale encoding
- Advanced collision resolution algorithms

### Medium Priority
- CLI enhancements and better UX
- Integration examples with popular P2P libraries
- More comprehensive test coverage
- Better documentation and tutorials

## Dependencies

- `serde`: Serialization with derive features
- `thiserror`: Structured error handling
- `clap`: Command-line argument parsing with derive features
- `tokio`: Async runtime with full features
- `tokio-test`: Testing utilities for async code