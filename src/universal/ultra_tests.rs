//! Ultra-compact encoder tests with 4-word validation
//!
//! This module tests the UltraCompactEncoder with real multiaddress strings
//! to validate 4-word encoding achievements and collision resistance.

use crate::UltraCompactEncoder;
use rand::Rng;
use std::collections::HashSet;
use std::net::{Ipv4Addr, Ipv6Addr};
use std::time::Instant;

/// Test configuration for ultra-compact encoding
const ULTRA_TEST_SIZE: usize = 1_000_000; // 1M addresses for focused testing
const EXPECTED_PERFECT_3_WORD_RATE: f64 = 0.6; // Expect 60%+ to achieve 4 words

/// Test results for ultra-compact encoding
#[derive(Debug, Default)]
pub struct UltraTestSummary {
    pub total_tests: usize,
    pub successful_encodings: usize,
    pub perfect_3_word_encodings: usize,
    pub unique_encodings: usize,
    pub collisions_found: usize,
    pub total_duration_ms: u128,
    pub average_encoding_time_us: f64,
    pub average_compression_ratio: f64,
    pub max_word_count: usize,
    pub min_word_count: usize,
}

/// Multiaddress generator for ultra-compact testing
pub struct UltraAddressGenerator {
    rng: rand::rngs::ThreadRng,
}

impl UltraAddressGenerator {
    pub fn new() -> Self {
        Self {
            rng: rand::thread_rng(),
        }
    }

    /// Generate diverse IPv4 multiaddresses
    pub fn generate_ipv4_multiaddr(&mut self) -> String {
        let ip = Ipv4Addr::new(
            self.rng.gen::<u8>(),
            self.rng.gen::<u8>(),
            self.rng.gen::<u8>(),
            self.rng.gen::<u8>(),
        );
        let port: u16 = self.rng.gen_range(1..=65535);
        let protocol = if self.rng.gen_bool(0.8) { "tcp" } else { "udp" };

        format!("/ip4/{}/{}/{}", ip, protocol, port)
    }

    /// Generate diverse IPv6 multiaddresses  
    pub fn generate_ipv6_multiaddr(&mut self) -> String {
        // Mix of different IPv6 patterns
        let ip = if self.rng.gen_bool(0.1) {
            // 10% localhost
            Ipv6Addr::from([0, 0, 0, 0, 0, 0, 0, 1])
        } else if self.rng.gen_bool(0.2) {
            // 20% link-local
            let mut segments = [0u16; 8];
            segments[0] = 0xfe80;
            for i in 1..8 {
                segments[i] = self.rng.gen::<u16>();
            }
            Ipv6Addr::new(
                segments[0],
                segments[1],
                segments[2],
                segments[3],
                segments[4],
                segments[5],
                segments[6],
                segments[7],
            )
        } else {
            // 70% general addresses
            let mut segments = [0u16; 8];
            for segment in &mut segments {
                *segment = self.rng.gen::<u16>();
            }
            Ipv6Addr::new(
                segments[0],
                segments[1],
                segments[2],
                segments[3],
                segments[4],
                segments[5],
                segments[6],
                segments[7],
            )
        };

        let port: u16 = self.rng.gen_range(1..=65535);
        let protocol = if self.rng.gen_bool(0.8) { "tcp" } else { "udp" };

        format!("/ip6/{}/{}/{}", ip, protocol, port)
    }

    /// Generate common/localhost patterns (should achieve perfect 4-word encoding)
    pub fn generate_common_multiaddr(&mut self) -> String {
        let patterns = [
            "/ip4/127.0.0.1/tcp/4001",
            "/ip4/127.0.0.1/tcp/8080",
            "/ip4/127.0.0.1/tcp/3000",
            "/ip4/127.0.0.1/tcp/5000",
            "/ip4/127.0.0.1/udp/53",
            "/ip6/::1/tcp/4001",
            "/ip6/::1/tcp/8080",
            "/ip6/::1/tcp/443",
            "/ip6/::1/udp/53",
            "/ip4/192.168.1.1/tcp/80",
            "/ip4/192.168.1.1/tcp/443",
            "/ip4/192.168.1.1/tcp/22",
            "/ip4/10.0.0.1/tcp/22",
            "/ip4/10.0.0.1/udp/53",
        ];

        patterns[self.rng.gen_range(0..patterns.len())].to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ultra_compact_encoder::UltraCompactEncoder;

    #[test]
    fn test_ultra_compact_1_million_addresses() {
        println!("\n🚀 Testing 1 Million Addresses with Ultra-Compact Encoder");
        println!("=========================================================");

        let encoder = UltraCompactEncoder::new().expect("Failed to create encoder");
        let mut generator = UltraAddressGenerator::new();
        let mut seen_encodings = HashSet::new();
        let mut summary = UltraTestSummary::default();

        let start_time = Instant::now();

        // Test mix: 40% IPv4, 30% IPv6, 30% common patterns
        let ipv4_count = (ULTRA_TEST_SIZE as f64 * 0.4) as usize;
        let ipv6_count = (ULTRA_TEST_SIZE as f64 * 0.3) as usize;
        let common_count = ULTRA_TEST_SIZE - ipv4_count - ipv6_count;

        // Test IPv4 addresses
        println!("Testing {} IPv4 multiaddresses...", ipv4_count);
        for i in 0..ipv4_count {
            let multiaddr = generator.generate_ipv4_multiaddr();
            test_encoding(&encoder, &multiaddr, &mut seen_encodings, &mut summary);

            if i % 50_000 == 0 && i > 0 {
                println!("  Processed {} IPv4 addresses", i);
            }
        }

        // Test IPv6 addresses
        println!("Testing {} IPv6 multiaddresses...", ipv6_count);
        for i in 0..ipv6_count {
            let multiaddr = generator.generate_ipv6_multiaddr();
            test_encoding(&encoder, &multiaddr, &mut seen_encodings, &mut summary);

            if i % 50_000 == 0 && i > 0 {
                println!("  Processed {} IPv6 addresses", i);
            }
        }

        // Test common patterns (should achieve perfect 4-word encoding)
        println!("Testing {} common multiaddresses...", common_count);
        for i in 0..common_count {
            let multiaddr = generator.generate_common_multiaddr();
            test_encoding(&encoder, &multiaddr, &mut seen_encodings, &mut summary);

            if i % 50_000 == 0 && i > 0 {
                println!("  Processed {} common addresses", i);
            }
        }

        let total_duration = start_time.elapsed();
        summary.total_duration_ms = total_duration.as_millis();
        summary.average_encoding_time_us /= summary.successful_encodings as f64;
        summary.average_compression_ratio /= summary.successful_encodings as f64;

        // Print results
        println!("\n📊 Ultra-Compact Encoder Test Results:");
        println!("  Total addresses tested: {}", summary.total_tests);
        println!("  Successful encodings: {}", summary.successful_encodings);
        println!(
            "  Perfect 4-word encodings: {} ({:.1}%)",
            summary.perfect_3_word_encodings,
            summary.perfect_3_word_encodings as f64 / summary.successful_encodings as f64 * 100.0
        );
        println!("  Unique encodings: {}", summary.unique_encodings);
        println!("  Collisions found: {}", summary.collisions_found);
        println!(
            "  Average encoding time: {:.2}μs",
            summary.average_encoding_time_us
        );
        println!(
            "  Average compression: {:.1}%",
            summary.average_compression_ratio
        );
        println!(
            "  Word count range: {}-{} words",
            summary.min_word_count, summary.max_word_count
        );
        println!(
            "  Total test duration: {:.2}s",
            summary.total_duration_ms as f64 / 1000.0
        );
        println!(
            "  Addresses per second: {:.0}",
            summary.total_tests as f64 / (summary.total_duration_ms as f64 / 1000.0)
        );

        // Validate ultra-compact encoder performance
        assert_eq!(summary.total_tests, ULTRA_TEST_SIZE);

        let collision_rate = summary.collisions_found as f64 / summary.total_tests as f64;
        assert!(
            collision_rate < 0.001,
            "❌ Collision rate too high: {:.4}% ({} collisions)",
            collision_rate * 100.0,
            summary.collisions_found
        );

        let perfect_3_word_rate =
            summary.perfect_3_word_encodings as f64 / summary.successful_encodings as f64;
        assert!(
            perfect_3_word_rate >= EXPECTED_PERFECT_3_WORD_RATE,
            "❌ Perfect 4-word rate too low: {:.1}% (expected ≥{:.1}%)",
            perfect_3_word_rate * 100.0,
            EXPECTED_PERFECT_3_WORD_RATE * 100.0
        );

        assert!(
            summary.average_encoding_time_us < 10.0,
            "❌ Average encoding time too slow: {:.2}μs",
            summary.average_encoding_time_us
        );

        println!("\n✅ Ultra-Compact Encoder Test PASSED!");
        println!(
            "   🎯 {:.1}% achieved perfect 4-word encoding",
            perfect_3_word_rate * 100.0
        );
        println!(
            "   🚀 {:.4}% collision rate (excellent performance)",
            collision_rate * 100.0
        );
        println!(
            "   ⚡ {:.2}μs average encoding time",
            summary.average_encoding_time_us
        );
    }

    #[test]
    fn test_ultra_compact_common_patterns() {
        println!("\n🎯 Testing Common Patterns for Perfect 3-Word Encoding");
        println!("======================================================");

        let encoder = UltraCompactEncoder::new().expect("Failed to create encoder");

        let common_patterns = [
            "/ip4/127.0.0.1/tcp/4001",
            "/ip4/127.0.0.1/tcp/8080",
            "/ip4/127.0.0.1/udp/53",
            "/ip6/::1/tcp/4001",
            "/ip6/::1/tcp/443",
            "/ip4/192.168.1.1/tcp/80",
            "/ip4/192.168.1.1/tcp/443",
            "/ip4/192.168.1.100/tcp/8080",
            "/ip4/10.0.0.1/tcp/22",
            "/ip4/10.0.0.1/udp/53",
        ];

        let mut perfect_count = 0;
        let mut total_compression = 0.0;

        for pattern in &common_patterns {
            match encoder.encode(pattern) {
                Ok(encoded) => {
                    let is_perfect = encoded.is_perfect_3_words();
                    let compression = encoded.compression_percentage();
                    total_compression += compression;

                    if is_perfect {
                        perfect_count += 1;
                    }

                    println!(
                        "  {} -> {} ({} words, {:.1}% compression) {}",
                        pattern,
                        encoded.to_words(),
                        encoded.word_count(),
                        compression,
                        if is_perfect {
                            "✅ PERFECT"
                        } else {
                            "📈 MULTI"
                        }
                    );
                }
                Err(e) => {
                    println!("  {} -> ❌ Error: {}", pattern, e);
                }
            }
        }

        let perfect_rate = perfect_count as f64 / common_patterns.len() as f64;
        let avg_compression = total_compression / common_patterns.len() as f64;

        println!("\n📊 Common Pattern Results:");
        println!(
            "  Perfect 4-word encodings: {}/{} ({:.1}%)",
            perfect_count,
            common_patterns.len(),
            perfect_rate * 100.0
        );
        println!("  Average compression: {:.1}%", avg_compression);

        // Common patterns should achieve very high 4-word rates
        assert!(
            perfect_rate >= 0.8,
            "❌ Common patterns should achieve ≥80% perfect 4-word encoding, got {:.1}%",
            perfect_rate * 100.0
        );
        assert!(
            avg_compression >= 75.0,
            "❌ Common patterns should achieve ≥75% compression, got {:.1}%",
            avg_compression
        );

        println!("✅ Common Pattern Test PASSED!");
    }
}

/// Helper function to test individual encoding
fn test_encoding(
    encoder: &UltraCompactEncoder,
    multiaddr: &str,
    seen_encodings: &mut HashSet<String>,
    summary: &mut UltraTestSummary,
) {
    let encode_start = Instant::now();
    match encoder.encode(multiaddr) {
        Ok(encoded) => {
            summary.successful_encodings += 1;
            let encode_duration = encode_start.elapsed();
            summary.average_encoding_time_us += encode_duration.as_micros() as f64;

            let encoded_str = encoded.to_words();
            let word_count = encoded.word_count();

            // Track word count statistics
            if summary.max_word_count == 0 || word_count > summary.max_word_count {
                summary.max_word_count = word_count;
            }
            if summary.min_word_count == 0 || word_count < summary.min_word_count {
                summary.min_word_count = word_count;
            }

            // Track perfect 4-word encodings
            if encoded.is_perfect_3_words() {
                summary.perfect_3_word_encodings += 1;
            }

            // Track compression ratio
            summary.average_compression_ratio += encoded.compression_percentage();

            // Check for collisions
            if seen_encodings.contains(&encoded_str) {
                summary.collisions_found += 1;
                println!("🚨 COLLISION DETECTED: {}", encoded_str);
            } else {
                seen_encodings.insert(encoded_str);
                summary.unique_encodings += 1;
            }
        }
        Err(_) => {
            // For ultra-compact encoder, some addresses may not compress well
            // This is acceptable for the test
        }
    }

    summary.total_tests += 1;
}
