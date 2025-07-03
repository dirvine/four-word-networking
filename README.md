# Three-Word Networking

Convert complex network multiaddresses into memorable three-word combinations for human-friendly networking.

## 🌟 What is Three-Word Networking?

Three-Word Networking transforms complex network addresses like `/ip6/2001:db8::1/udp/9000/quic` into memorable combinations like `ocean.thunder.falcon`. It's inspired by what3words but designed specifically for networking and peer-to-peer applications.

### Why Three Words?

**Before:** "Connect to `/ip6/2001:0db8:85a3:0000:0000:8a2e:0370:7334/udp/9000/quic`"  
**After:** "Connect to `ocean thunder falcon`"

## ✨ Features

- **🗣️ Voice-Friendly**: Easy to share over phone calls or voice chat
- **🧠 Memorable**: Three carefully chosen words are easier to remember than long addresses
- **🔄 Deterministic**: Same multiaddr always produces the same three-word address
- **🌍 Universal**: Works with any valid multiaddr format (IPv4, IPv6, DNS, etc.)
- **📈 Massive Scale**: 68.7 billion base combinations, extensible to 4.5 quadrillion
- **❌ Error-Resistant**: Much less prone to typos than long technical addresses

## 🚀 Quick Start

### Installation

```bash
cargo install three-word-networking
```

### CLI Usage

```bash
# Convert multiaddr to three words
three-word-networking encode "/ip6/2001:db8::1/udp/9000/quic"
# Output: ocean.thunder.falcon

# Validate a three-word address
three-word-networking validate "ocean.thunder.falcon"

# Show address space information
three-word-networking info

# Generate examples
three-word-networking examples --count 10
```

### Library Usage

```rust
use three_word_networking::{WordEncoder, ThreeWordAddress};

let encoder = WordEncoder::new();

// Convert multiaddr to three words
let multiaddr = "/ip6/2001:db8::1/udp/9000/quic";
let words = encoder.encode_multiaddr_string(multiaddr)?;
println!("Connect to: {}", words); // ocean.thunder.falcon

// Parse and validate three-word addresses
let addr = ThreeWordAddress::from_string("ocean.thunder.falcon")?;
assert!(addr.validate(&encoder).is_ok());

// Check address space
println!("Total combinations: {}", ThreeWordAddress::address_space_size());
```

## 🏗️ How It Works

### Dictionary Structure

Three-Word Networking uses a carefully curated dictionary with three positions:

1. **Context Words** (Position 1): Geographic, network, and scale contexts
   - Examples: `global`, `local`, `mesh`, `cloud`, `europe`, `mobile`

2. **Quality Words** (Position 2): Performance, purpose, and status descriptors  
   - Examples: `fast`, `secure`, `reliable`, `premium`, `active`, `smart`

3. **Identity Words** (Position 3): Nature, objects, and abstract concepts
   - Examples: `eagle`, `compass`, `crystal`, `harmony`, `mountain`, `flame`

### Address Space

- **Base Format**: `context.quality.identity` (68.7 billion combinations)
- **Extended Format**: `context.quality.identity.1847` (4.5 quadrillion combinations)
- **4,096 words** per position for maximum diversity

### Encoding Process

1. Hash the multiaddr string using a deterministic algorithm
2. Extract three indices from different parts of the hash
3. Map indices to words in each dictionary position
4. Optionally add numeric suffix for extended addressing

## 📋 Examples

| Multiaddr | Three-Word Address | Use Case |
|-----------|-------------------|----------|
| `/ip4/192.168.1.1/tcp/8080` | `local.smart.compass` | Local development server |
| `/ip6/::1/tcp/22` | `global.secure.anchor` | SSH connection |
| `/ip4/10.0.0.1/udp/5000/quic` | `mesh.fast.eagle` | P2P gaming |
| `/dns4/example.com/tcp/443` | `cloud.premium.crystal` | HTTPS website |

## 🎯 Use Cases

### 🎮 Gaming & P2P Applications
```bash
# Traditional way
"Join my server at /ip4/203.0.113.42/udp/7777/quic"

# Three-word way  
"Join my server at global.turbo.dragon"
```

### 📞 Voice Communication
```bash
# Phone call
"Connect to ocean thunder falcon"
# Much easier than reading out a 50-character multiaddr!
```

### 📱 QR Codes with Backup
```
[QR CODE]
Backup: forest.lightning.compass
```

### 🏢 Enterprise Configuration
```yaml
# config.yaml
bootstrap_nodes:
  - "global.secure.anchor"    # Primary datacenter
  - "europe.fast.beacon"      # European region  
  - "asia.premium.crystal"    # Asian region
```

## 🔧 API Reference

### `WordEncoder`

Main interface for encoding/decoding operations.

```rust
impl WordEncoder {
    // Create new encoder with default dictionary
    pub fn new() -> Self
    
    // Encode multiaddr string to three words
    pub fn encode_multiaddr_string(&self, multiaddr: &str) -> Result<ThreeWordAddress>
    
    // Encode to base format only (no suffix)
    pub fn encode_multiaddr_string_base(&self, multiaddr: &str) -> Result<ThreeWordAddress>
    
    // Validate three words exist in dictionary  
    pub fn validate_words(&self, first: &str, second: &str, third: &str) -> Result<()>
}
```

### `ThreeWordAddress`

Represents a three-word address with optional numeric suffix.

```rust
impl ThreeWordAddress {
    // Parse from string format
    pub fn from_string(input: &str) -> Result<Self>
    
    // Convert to string format
    pub fn to_string(&self) -> String
    
    // Check if extended format (has suffix)
    pub fn is_extended(&self) -> bool
    
    // Get base address without suffix
    pub fn base_address(&self) -> String
    
    // Validate against encoder dictionary
    pub fn validate(&self, encoder: &WordEncoder) -> Result<()>
}
```

## 🔒 Limitations & Future Work

### Current Limitations

1. **Registry Requirement**: Converting back from three-words to multiaddr requires a distributed registry (not yet implemented)
2. **Hash Collisions**: Different multiaddrs can theoretically produce the same three-word address
3. **Dictionary Language**: Currently English-only dictionary

### Planned Features

- **🌐 Distributed Registry**: Decentralized lookup system for reverse conversion
- **🌍 Multi-language Support**: Dictionaries in multiple languages  
- **🔍 Collision Resolution**: Enhanced algorithms to minimize conflicts
- **📱 Mobile SDKs**: Native libraries for iOS and Android
- **🔗 Integration Examples**: Ready-to-use integrations for popular P2P libraries

## 🤝 Contributing

We welcome contributions! Please see our [Contributing Guide](CONTRIBUTING.md) for details.

### Development Setup

```bash
git clone https://github.com/YOUR_USERNAME/three-word-networking.git
cd three-word-networking
cargo build
cargo test
cargo run -- examples --count 5
```

## 📜 License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT License ([LICENSE-MIT](LICENSE-MIT))

at your option.

## 🙏 Acknowledgments

- Inspired by [what3words](https://what3words.com/) for geographic locations
- Built on the [multiaddr](https://multiformats.io/multiaddr/) specification
- Part of the broader effort to make networking more human-friendly

---

**Made with ❤️ for the P2P and networking community**

*"Making networking as easy as saying three words"*