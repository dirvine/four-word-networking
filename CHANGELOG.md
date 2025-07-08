# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [1.1.0] - 2025-01-08

### Major Architecture Update - Four-Word Networking

#### Added
- **Perfect IPv4 Reconstruction**: IPv4 addresses now use exactly 4 words with 100% perfect reconstruction
- **Four-word encoder system**: New `FourWordAdaptiveEncoder` for optimal encoding
- **4wn CLI tool**: Renamed from `4wn` with updated functionality
- **Visual IP distinction**: IPv4 uses dots (lowercase), IPv6 uses dashes (title case)
- **Enhanced IPv6 compression**: Adaptive 4-6 word encoding with category-based optimization

#### Changed
- **BREAKING**: Crate renamed from `four-word-networking` to `four-word-networking`
- **BREAKING**: CLI renamed from `4wn` to `4wn`
- **BREAKING**: IPv4 addresses now produce 4 words instead of 3 (with perfect reconstruction)
- **BREAKING**: Library crate name changed to `four_word_networking`
- IPv6 addresses now use 4-6 words with intelligent compression
- Updated all documentation and examples to reflect new architecture

#### Fixed
- IPv4 reconstruction is now 100% perfect (was previously lossy compression)
- IPv6 encoding now has guaranteed minimum 4 words for clear differentiation
- All mathematical precision issues resolved with expanded bit capacity

#### Migration Guide
- Update Cargo.toml: `four-word-networking` → `four-word-networking`
- Update imports: `use four_word_networking` → `use four_word_networking`
- Update CLI usage: `4wn` → `4wn`
- IPv4 addresses will now have 4 words instead of 3, but with perfect reconstruction
- IPv6 addresses will use dashes and title case for visual distinction

## [1.0.1] - 2025-01-08

### Fixed
- Added partial decode implementation for IPv4 addresses (approximate reconstruction)
- Fixed IPv6 loopback address encoding to include port information
- Added decode support for IPv6 loopback (::1) and unspecified (::) addresses

### Changed
- Updated README to document compression trade-offs and decoding limitations
- Clarified that IPv4 decoding is approximate due to lossy compression (48→42 bits)

### Known Issues
- IPv4 addresses cannot be perfectly reconstructed due to mathematical compression
- IPv6 decode only supports loopback and unspecified addresses
- Other IPv6 categories return an error on decode attempt

## [1.0.0] - 2025-01-08

### Added
- Initial production release
- IPv4 addresses always encode to exactly 4 words
- IPv6 addresses always encode to 4-6 words for clear differentiation
- Mathematical compression achieving 87.5% for IPv4
- Hierarchical compression for IPv6 addresses
- Clean API supporting String, &str, SocketAddr, and IpAddr inputs
- 4wn CLI tool with auto-detection and verbose mode
- Variable-length dictionary supporting 3-6 word combinations
- Comprehensive test coverage

### Technical Details
- Zero collisions with deterministic encoding
- Upgraded to u128 for handling up to 84-bit encodings
- Fixed overflow issues in link-local IPv6 addresses