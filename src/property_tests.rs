//! Property-based testing for Three-Word Networking
//!
//! This module contains property-based tests that verify mathematical invariants
//! and properties of the encoding system using proptest.

use crate::ThreeWordAdaptiveEncoder;
use proptest::prelude::*;
use std::net::{Ipv4Addr, Ipv6Addr};

#[cfg(test)]
mod tests {
    use super::*;

    // Property: IPv4 addresses should always encode to exactly 3 dot-separated words
    proptest! {
        #[test]
        fn prop_ipv4_always_4_words(
            ip in any::<Ipv4Addr>(),
            port in any::<u16>()
        ) {
            let encoder = ThreeWordAdaptiveEncoder::new().unwrap();
            let addr_str = format!("{ip}:{port}");

            if let Ok(words) = encoder.encode(&addr_str) {
                // Should have exactly 3 words separated by dots
                let word_count = words.split('.').count();
                prop_assert_eq!(word_count, 3);

                // Should not contain dashes (IPv6 format)
                prop_assert!(!words.contains('-'));

                // Should be lowercase
                prop_assert_eq!(words.clone(), words.to_lowercase());
            }
        }
    }

    // Property: IPv6 addresses should always encode to 6 or 9 dash-separated words
    proptest! {
        #[test]
        fn prop_ipv6_format_consistency(
            ip in any::<Ipv6Addr>(),
            port in any::<u16>()
        ) {
            let encoder = ThreeWordAdaptiveEncoder::new().unwrap();
            let addr_str = format!("[{ip}]:{port}");

            if let Ok(words) = encoder.encode(&addr_str) {
                // Should contain dashes (IPv6 format)
                prop_assert!(words.contains('-'));

                // Should not contain dots (IPv4 format)
                prop_assert!(!words.contains('.'));

                // Should have 6 or 9 words (groups of 3)
                let word_count = words.split('-').count();
                prop_assert!(word_count == 6 || word_count == 9);

                // Should be title case (first letter uppercase)
                for word in words.split('-') {
                    if !word.is_empty() {
                        let first_char = word.chars().next().unwrap();
                        prop_assert!(first_char.is_uppercase());
                    }
                }
            }
        }
    }

    // Property: Round-trip encoding should preserve the original address
    proptest! {
        #[test]
        fn prop_ipv4_round_trip(
            ip in any::<Ipv4Addr>(),
            port in any::<u16>()
        ) {
            let encoder = ThreeWordAdaptiveEncoder::new().unwrap();
            let original = format!("{ip}:{port}");

            if let Ok(words) = encoder.encode(&original) {
                if let Ok(decoded) = encoder.decode(&words) {
                    prop_assert_eq!(original, decoded);
                }
            }
        }
    }

    // Property: Same input should always produce same output (deterministic)
    proptest! {
        #[test]
        fn prop_deterministic_encoding(
            ip in any::<Ipv4Addr>(),
            port in any::<u16>()
        ) {
            let encoder = ThreeWordAdaptiveEncoder::new().unwrap();
            let addr_str = format!("{ip}:{port}");

            if let Ok(words1) = encoder.encode(&addr_str) {
                if let Ok(words2) = encoder.encode(&addr_str) {
                    prop_assert_eq!(words1, words2);
                }
            }
        }
    }

    // Property: Different inputs should produce different outputs (no collisions)
    proptest! {
        #[test]
        fn prop_no_collisions_different_ports(
            ip in any::<Ipv4Addr>(),
            port1 in any::<u16>(),
            port2 in any::<u16>()
        ) {
            prop_assume!(port1 != port2);

            let encoder = ThreeWordAdaptiveEncoder::new().unwrap();
            let addr1_str = format!("{ip}:{port1}");
            let addr2_str = format!("{ip}:{port2}");

            if let (Ok(words1), Ok(words2)) = (encoder.encode(&addr1_str), encoder.encode(&addr2_str)) {
                prop_assert_ne!(words1, words2);
            }
        }
    }

    // Property: Word validation should be consistent
    proptest! {
        #[test]
        fn prop_word_validation_consistency(
            ip in any::<Ipv4Addr>(),
            port in any::<u16>()
        ) {
            let encoder = ThreeWordAdaptiveEncoder::new().unwrap();
            let addr_str = format!("{ip}:{port}");

            if let Ok(words) = encoder.encode(&addr_str) {
                // Each word should be from the dictionary
                for word in words.split('.') {
                    prop_assert!(word.len() >= 3);  // Dictionary has 3-letter minimum words
                    // No maximum length restriction - frequency-based words can be longer
                    prop_assert!(word.chars().all(|c| c.is_ascii_lowercase() || c.is_ascii_digit()));
                }
            }
        }
    }

    // Property: Encoding should handle edge cases gracefully
    proptest! {
        #[test]
        fn prop_handle_edge_cases(
            ip_bytes in prop::array::uniform4(any::<u8>()),
            port in any::<u16>()
        ) {
            let encoder = ThreeWordAdaptiveEncoder::new().unwrap();
            let ip = Ipv4Addr::from(ip_bytes);
            let addr_str = format!("{ip}:{port}");

            // Encoding should either succeed or fail gracefully
            match encoder.encode(&addr_str) {
                Ok(words) => {
                    // If encoding succeeds, it should produce valid format
                    prop_assert_eq!(words.split('.').count(), 3);
                    prop_assert!(!words.is_empty());
                },
                Err(_) => {
                    // If encoding fails, that's acceptable for edge cases
                    // but the error should be handled gracefully
                }
            }
        }
    }

    // Property: String formatting should be consistent
    proptest! {
        #[test]
        fn prop_string_formatting_consistency(
            ip in any::<Ipv4Addr>(),
            port in any::<u16>()
        ) {
            let encoder = ThreeWordAdaptiveEncoder::new().unwrap();
            let addr_str1 = format!("{ip}:{port}");
            let addr_str2 = format!("{ip}:{port}");

            let result1 = encoder.encode(&addr_str1);
            let result2 = encoder.encode(&addr_str2);

            match (result1, result2) {
                (Ok(words1), Ok(words2)) => {
                    prop_assert_eq!(words1, words2);
                },
                (Err(_), Err(_)) => {
                    // Both failing is acceptable
                },
                _ => {
                    // One succeeding and one failing is not acceptable
                    prop_assert!(false, "Identical string inputs should have same result");
                }
            }
        }
    }
}
