# Contributing to Four-Word Networking

Thank you for your interest in contributing to Four-Word Networking! This document provides guidelines and information for contributors.

## 🎯 Project Goals

Four-Word Networking aims to make network addressing human-friendly by:

- Converting complex IP addresses into memorable four-word combinations
- Providing deterministic, collision-resistant encoding
- Supporting massive scale addressing (quadrillions of addresses)  
- Maintaining universal compatibility with IP address formats
- Being voice-friendly and error-resistant

## 🚀 Getting Started

### Prerequisites

- Rust 1.70 or later
- Git
- Basic understanding of networking concepts and IP addresses

### Development Setup

```bash
# Clone the repository
git clone https://github.com/YOUR_USERNAME/four-word-networking.git
cd four-word-networking

# Build the project
cargo build

# Run tests
cargo test

# Run the CLI
cargo run -- examples --count 5

# Run with release optimizations
cargo build --release
cargo test --release
```

### Project Structure

```
four-word-networking/
├── src/
│   ├── lib.rs           # Main library interface
│   ├── words.rs         # Core four-word address implementation
│   ├── error.rs         # Error types and handling
│   └── main.rs          # CLI application
├── tests/               # Integration tests
├── examples/            # Usage examples
├── docs/                # Additional documentation
└── README.md            # Project overview
```

## 🛠️ Development Workflow

### 1. Fork and Branch

```bash
# Fork the repository on GitHub, then:
git clone https://github.com/YOUR_USERNAME/four-word-networking.git
cd four-word-networking

# Create a feature branch
git checkout -b feature/your-feature-name
```

### 2. Make Changes

- Write clear, documented code following Rust conventions
- Add tests for new functionality  
- Update documentation as needed
- Follow the existing code style

### 3. Test Your Changes

```bash
# Run all tests
cargo test

# Run tests with verbose output
cargo test -- --nocapture

# Run specific tests
cargo test test_four_word_address_parsing

# Check code formatting
cargo fmt --check

# Run clippy for additional linting
cargo clippy -- -D warnings
```

### 4. Submit Pull Request

```bash
# Commit your changes
git add .
git commit -m "feat: add your feature description"

# Push to your fork
git push origin feature/your-feature-name

# Create pull request on GitHub
```

## 📝 Code Guidelines

### Code Style

- Follow standard Rust formatting (`cargo fmt`)
- Use descriptive variable and function names
- Add documentation comments for public APIs
- Keep functions focused and small
- Use `Result<T>` for error handling

### Testing Requirements

- Add unit tests for new functions
- Include integration tests for major features
- Test edge cases and error conditions
- Ensure deterministic behavior in tests
- Aim for high test coverage

### Example Test Structure

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_feature_name_should_describe_expected_behavior() {
        // Arrange
        let encoder = WordEncoder::new();
        let input = "/ip6/2001:db8::1/udp/9000/quic";
        
        // Act
        let result = encoder.encode_multiaddr_string(input).unwrap();
        
        // Assert
        assert!(!result.first.is_empty());
        assert!(result.validate(&encoder).is_ok());
    }
}
```

### Documentation

- Document all public APIs with rustdoc comments
- Include examples in documentation
- Update README.md for new features
- Add inline comments for complex logic

```rust
/// Converts a multiaddr string to a four-word address.
/// 
/// # Arguments
/// 
/// * `multiaddr` - A valid multiaddr string (e.g., "/ip6/::1/tcp/8080")
/// 
/// # Returns
/// 
/// A `FourWordAddress` representing the encoded address.
/// 
/// # Examples
/// 
/// ```
/// use four_word_networking::WordEncoder;
/// 
/// let encoder = WordEncoder::new();
/// let words = encoder.encode_multiaddr_string("/ip6/::1/tcp/8080")?;
/// println!("Four-word address: {}", words);
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub fn encode_multiaddr_string(&self, multiaddr: &str) -> Result<FourWordAddress> {
    // Implementation...
}
```

## 🐛 Bug Reports

When reporting bugs, please include:

- **Clear description** of the issue
- **Steps to reproduce** the behavior
- **Expected behavior** vs actual behavior
- **Environment details** (OS, Rust version, etc.)
- **Code sample** demonstrating the issue

### Bug Report Template

```markdown
## Bug Description
Clear description of what the bug is.

## To Reproduce
Steps to reproduce the behavior:
1. Run command '...'
2. Input '...'
3. See error

## Expected Behavior
What you expected to happen.

## Actual Behavior  
What actually happened.

## Environment
- OS: [e.g., macOS 13.0]
- Rust version: [e.g., 1.70.0]
- Library version: [e.g., 0.1.0]

## Additional Context
Any other context about the problem.
```

## 💡 Feature Requests

We welcome feature requests! Please:

- **Check existing issues** to avoid duplicates
- **Describe the use case** clearly
- **Explain the benefits** to users
- **Consider implementation complexity**
- **Provide examples** if possible

### Feature Request Template

```markdown
## Feature Description
Clear description of the proposed feature.

## Use Case
Describe the problem this feature would solve.

## Proposed Solution
Describe your preferred solution.

## Alternatives Considered
Any alternative solutions you've considered.

## Additional Context
Any other context or screenshots.
```

## 🎯 Areas for Contribution

We especially welcome contributions in these areas:

### High Priority
- **Registry Implementation**: Distributed lookup system for reverse conversion
- **Performance Optimization**: Faster encoding/decoding algorithms
- **Multi-language Dictionaries**: Support for non-English languages
- **Error Handling**: Better error messages and recovery

### Medium Priority  
- **CLI Enhancements**: More commands and better UX
- **Integration Examples**: Examples with popular P2P libraries
- **Documentation**: Better examples and tutorials
- **Testing**: More comprehensive test coverage

### Future Considerations
- **Mobile SDKs**: Native libraries for iOS/Android
- **Web Assembly**: Browser-compatible version
- **Collision Detection**: Advanced conflict resolution
- **Custom Dictionaries**: User-defined word sets

## 🔍 Code Review Process

All contributions go through code review:

1. **Automated Checks**: CI runs tests, formatting, and linting
2. **Manual Review**: Maintainers review code for:
   - Correctness and functionality
   - Code quality and style
   - Test coverage
   - Documentation completeness
   - Performance implications

3. **Feedback**: Reviewers provide constructive feedback
4. **Iteration**: Address feedback and update PR
5. **Approval**: Once approved, changes are merged

## 📜 License

By contributing, you agree that your contributions will be licensed under the same terms as the project (MIT OR Apache-2.0).

## 🤝 Community

- **Be Respectful**: Treat all community members with respect
- **Be Helpful**: Help others learn and contribute
- **Be Patient**: Remember that maintainers are often volunteers
- **Be Constructive**: Provide helpful feedback and suggestions

## 📞 Getting Help

If you need help:

- **GitHub Issues**: For bugs and feature requests
- **GitHub Discussions**: For questions and general discussion
- **Documentation**: Check the README and inline docs first

## 🏆 Recognition

Contributors will be recognized in:

- **CONTRIBUTORS.md**: List of all contributors
- **Release Notes**: Major contributions mentioned in releases
- **GitHub**: Contributor statistics and graphs

Thank you for contributing to Four-Word Networking! 🎉