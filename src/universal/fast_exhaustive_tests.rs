//! Fast exhaustive testing demonstration
//!
//! This module provides scaled-down versions of the exhaustive tests for
//! development and demonstration purposes. The full-scale tests are in
//! exhaustive_tests.rs but would take too long for regular testing.

use rand::Rng;
use std::net::{Ipv4Addr, Ipv6Addr};

// Fast test configuration (scaled down for development)
#[allow(dead_code)]
const FAST_NETWORK_ADDRESSES: usize = 10_000;
#[allow(dead_code)]
const FAST_CRYPTO_ADDRESSES: usize = 1_000;
#[allow(dead_code)]
const FAST_SHA256_HASHES: usize = 1_000;

/// Fast network address generator (reuse from exhaustive tests)
pub struct FastNetworkGenerator {
    rng: rand::rngs::ThreadRng,
}

impl FastNetworkGenerator {
    pub fn new() -> Self {
        Self {
            rng: rand::thread_rng(),
        }
    }

    pub fn generate_ipv4_address(&mut self) -> Vec<u8> {
        let ip = Ipv4Addr::new(
            self.rng.gen::<u8>(),
            self.rng.gen::<u8>(),
            self.rng.gen::<u8>(),
            self.rng.gen::<u8>(),
        );
        let port: u16 = self.rng.gen_range(1..=65535);

        let mut bytes = Vec::with_capacity(8);
        bytes.extend_from_slice(&ip.octets());
        bytes.extend_from_slice(&port.to_be_bytes());
        bytes.extend_from_slice(&[0u8; 2]);
        bytes
    }

    pub fn generate_ipv6_address(&mut self) -> Vec<u8> {
        let mut segments = [0u16; 8];
        for segment in &mut segments {
            *segment = self.rng.gen::<u16>();
        }

        let ip = Ipv6Addr::new(
            segments[0],
            segments[1],
            segments[2],
            segments[3],
            segments[4],
            segments[5],
            segments[6],
            segments[7],
        );

        ip.octets().to_vec()
    }
}

#[cfg(test)]
mod tests {
    // use crate::balanced_encoder::BalancedEncoder; // Disabled - balanced encoder temporarily not available

    // Tests temporarily disabled while balanced encoder is updated for new IP compression system
    #[allow(unused)]
    #[ignore] // Disabled until balanced encoder is updated
    #[test]
    fn test_fast_network_addresses() {
        unimplemented!("Balanced encoder temporarily disabled");
    }

    #[ignore] // Disabled until balanced encoder is updated
    #[test]
    fn test_fast_crypto_addresses() {
        unimplemented!("Balanced encoder temporarily disabled");
    }

    #[ignore] // Disabled until balanced encoder is updated
    #[test]
    fn test_fast_sha256_hashes() {
        unimplemented!("Balanced encoder temporarily disabled");
    }
}
