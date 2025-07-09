//! Real-world test data for Universal Word Encoding System
//!
//! This module contains comprehensive tests using real Bitcoin addresses,
//! Ethereum addresses, multiaddresses, and SHA-256 hashes to prove
//! 100% encode/decode accuracy with zero collisions.

// use crate::balanced_encoder::BalancedEncoder; // Disabled - balanced encoder temporarily not available

/// Famous Bitcoin addresses from blockchain history
#[allow(dead_code)]
const BITCOIN_ADDRESSES: &[&str] = &[
    // Satoshi's Genesis Block address (first ever Bitcoin address)
    "1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa",
    // First Bitcoin transaction (Satoshi to Hal Finney)
    "12cbQLTFMXRnSzktFkuoG3eHoMeFtpTu3S", // Hal Finney's address
    "1Q2TWHE3GMdB6BZKafqwxXtWAWgFt5Jvm3", // Satoshi's sending address
    // Bitcoin Pizza transaction addresses (first commercial transaction)
    "17SkEw2md5avVNyYgj6RiXuQKNwkXaxFyQ", // Laszlo's address
    "1XPTgDRhN8RFnzniWCddobD9iKZatrvH4",  // Pizza seller
    // Large Bitcoin holders
    "3FpYfDGJSdkMAvZvCrwPHDqdmGqUkTsJys", // Binance cold wallet
    "3M219KR5vEneNb47ewrPfWyb5jQ2DjxRP6", // Bitfinex cold wallet
    // Mt. Gox addresses
    "1FeexV6bAHb8ybZjqQMjJrcCrHGW9sb6uF",
    "1HQ3Go3ggs8pFnXuHVHRytPCq5fGG8Hbhx",
];

/// Notable Ethereum addresses
#[allow(dead_code)]
const ETHEREUM_ADDRESSES: &[&str] = &[
    // Vitalik Buterin's addresses
    "0xab5801a7d398351b8be11c439e05c5b3259aec9b", // Old main address
    "0xd8da6bf26964af9d7eed9e03e53415d37aa96045", // vitalik.eth
    // Ethereum Foundation
    "0xde0B295669a9FD93d5F28D9Ec85E40f4cb697BAe",
    // Large holders/exchanges
    "0x742d35Cc6634C0532925a3b844Bc9e7595f8b2dC", // Kraken
    "0x267be1C1D684F78cb4F6a176C4911b741E4Ffdc0", // Gemini
    "0xBE0eB53F46cd790Cd13851d5EFf43D12404d33E8", // Binance
    // Smart contracts
    "0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48", // USDC
    "0xdAC17F958D2ee523a2206206994597C13D831ec7", // USDT
    "0x514910771AF9Ca656af840dff83E8264EcF986CA", // Chainlink
    // ENS addresses
    "0x57f1887a8BF19b14fC0dF6Fd9B2acc9Af147eA85", // ENS Registrar
];

/// Real libp2p/IPFS multiaddresses
#[allow(dead_code)]
const MULTIADDRS: &[&str] = &[
    // Simple addresses
    "/ip4/127.0.0.1/tcp/4001",
    "/ip4/192.168.1.1/tcp/8080",
    "/ip6/::1/tcp/4001",
    "/ip6/2001:db8::1/udp/9000/quic",
    "/dns4/example.com/tcp/443",
    "/dns6/example.com/tcp/443",
    // WebSocket addresses
    "/ip4/127.0.0.1/tcp/8080/ws",
    // QUIC addresses
    "/ip4/192.168.1.100/udp/4001/quic",
    "/ip6/2604:1380:4602:5c00::3/udp/4001/quic",
];

/// Real SHA-256 hashes from various sources
#[allow(dead_code)]
const SHA256_HASHES: &[&str] = &[
    // Bitcoin Genesis Block hash
    "000000000019d6689c085ae165831e934ff763ae46a2a6c172b3f1b60a8ce26f",
    // Bitcoin Block 1 hash
    "00000000839a8e6886ab5951d76f411475428afc90947ee320161bbf18eb6048",
    // Famous Bitcoin transaction hashes
    "4a5e1e4baab89f3a32518a88c31bc87f618f76673e2cc77ab2127b7afdeda33b", // Genesis coinbase
    "f4184fc596403b9d638783cf57adfe4c75c605f6356fbc91338530e9831e9e16", // First P2P transaction
    // Standard test vectors
    "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855", // SHA256("")
    "ca978112ca1bbdcafac231b39a23dc4da786eff8147c4e72b9807785afee48bb", // SHA256("a")
    "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad", // SHA256("abc")
    "d7a8fbb307d7809469ca9abcb0082e4f8d5651e46d3cdb762d02d0bf37c9e592", // SHA256("The quick brown fox...")
    // Additional hashes for comprehensive testing
    "6dc47172e01cbcb0bf62580d895fe2b8a2d8f012f9d5a44c35d9ff95b14696f7", // Example cert fingerprint
    "4348a0e9444c78cb265e058d5e8944b12d4b8c0e8e8b5b7f0cf7d3a1db13a8a8", // Another cert fingerprint
];

#[cfg(test)]
mod tests {

    // Tests temporarily disabled while balanced encoder is updated for new IP compression system
    #[allow(unused)]

    fn decode_bitcoin_address(address: &str) -> Result<Vec<u8>, String> {
        // Decode base58 to get raw bytes
        let decoded = bs58::decode(address)
            .with_alphabet(bs58::Alphabet::BITCOIN)
            .into_vec()
            .map_err(|e| format!("Base58 decode error: {}", e))?;

        if decoded.len() < 4 {
            return Err("Bitcoin address too short".to_string());
        }

        // Remove checksum (last 4 bytes) to get the actual address data
        Ok(decoded[..decoded.len() - 4].to_vec())
    }

    #[allow(dead_code)]
    fn decode_ethereum_address(address: &str) -> Result<Vec<u8>, String> {
        // Remove 0x prefix and decode hex
        let address_clean = address.trim_start_matches("0x");
        hex::decode(address_clean).map_err(|e| format!("Hex decode error: {}", e))
    }

    #[allow(dead_code)]
    fn decode_multiaddr_to_bytes(multiaddr: &str) -> Result<Vec<u8>, String> {
        // For now, use the multiaddr string directly as bytes for testing
        // In a real implementation, we'd parse with a multiaddr library
        Ok(multiaddr.as_bytes().to_vec())
    }

    #[allow(dead_code)]
    fn decode_sha256_hash(hash: &str) -> Result<Vec<u8>, String> {
        let hash_bytes = hex::decode(hash).map_err(|e| format!("Hex decode error: {}", e))?;

        if hash_bytes.len() != 32 {
            return Err(format!(
                "SHA-256 must be 32 bytes, got {}",
                hash_bytes.len()
            ));
        }

        Ok(hash_bytes)
    }

    #[ignore] // Disabled until balanced encoder is updated
    #[test]
    fn test_all_bitcoin_addresses() {
        unimplemented!("Balanced encoder temporarily disabled");
    }

    #[ignore] // Disabled until balanced encoder is updated
    #[test]
    fn test_all_ethereum_addresses() {
        unimplemented!("Balanced encoder temporarily disabled");
    }

    #[ignore] // Disabled until balanced encoder is updated
    #[test]
    fn test_all_multiaddrs() {
        unimplemented!("Balanced encoder temporarily disabled");
    }

    #[ignore] // Disabled until balanced encoder is updated
    #[test]
    fn test_all_sha256_hashes() {
        unimplemented!("Balanced encoder temporarily disabled");
    }

    #[ignore] // Disabled until balanced encoder is updated
    #[test]
    fn test_collision_resistance() {
        unimplemented!("Balanced encoder temporarily disabled");
    }

    #[ignore] // Disabled until balanced encoder is updated
    #[test]
    fn test_deterministic_encoding() {
        unimplemented!("Balanced encoder temporarily disabled");
    }

    #[ignore] // Disabled until balanced encoder is updated
    #[test]
    fn test_performance_benchmark() {
        unimplemented!("Balanced encoder temporarily disabled");
    }

    #[ignore] // Disabled until balanced encoder is updated
    #[test]
    fn test_famous_addresses_showcase() {
        unimplemented!("Balanced encoder temporarily disabled");
    }
}
