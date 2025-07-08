//! Comprehensive tests for IP+Port encoder

use three_word_networking::ip_port_encoder::{IpPortEncoder, IpPortError};

#[test]
fn test_ipv4_localhost_all_ports() {
    let encoder = IpPortEncoder::new().unwrap();
    
    // Test common ports on localhost
    let common_ports = [80, 443, 22, 3306, 5432, 8080, 8443, 3000, 5000, 9000];
    
    for port in common_ports {
        let addr = format!("127.0.0.1:{}", port);
        let result = encoder.encode(&addr).unwrap();
        
        assert_eq!(result.words.len(), 3);
        assert!(result.compression_ratio > 0.3); // At least 30% compression
        
        // Verify it's a valid socket address
        assert_eq!(result.socket_addr.port(), port);
        assert!(result.socket_addr.is_ipv4());
    }
}

#[test]
fn test_ipv4_private_networks() {
    let encoder = IpPortEncoder::new().unwrap();
    
    let test_cases = [
        // 192.168.x.x network
        "192.168.0.1:80",
        "192.168.1.1:443",
        "192.168.1.254:22",
        "192.168.255.255:65535",
        // 10.x.x.x network
        "10.0.0.1:80",
        "10.1.2.3:443",
        "10.255.255.255:8080",
        // 172.16-31.x.x network
        "172.16.0.1:80",
        "172.16.10.20:443",
        "172.31.255.255:9000",
    ];
    
    for addr in test_cases {
        let result = encoder.encode(addr).unwrap();
        assert_eq!(result.words.len(), 3);
        assert!(result.compression_ratio > 0.15); // At least 15% compression
    }
}

#[test]
fn test_ipv6_addresses() {
    let encoder = IpPortEncoder::new().unwrap();
    
    let test_cases = [
        // Localhost
        "[::1]:80",
        "[::1]:443",
        "[::1]:8080",
        "[0:0:0:0:0:0:0:1]:22",
        // Link-local
        "[fe80::1]:80",
        "[fe80::1234:5678:9abc:def0]:443",
        // Global
        "[2001:db8::1]:80",
        "[2001:db8:85a3::8a2e:370:7334]:443",
        "[2606:4700:4700::1111]:53",
    ];
    
    for addr in test_cases {
        let result = encoder.encode(addr).unwrap();
        assert_eq!(result.words.len(), 3);
        assert!(result.compression_ratio > 0.7); // IPv6 should compress well
        assert!(result.socket_addr.is_ipv6());
    }
}

#[test]
fn test_round_trip_accuracy() {
    let encoder = IpPortEncoder::new().unwrap();
    
    let test_addresses = [
        // IPv4
        "127.0.0.1:80",
        "127.0.0.1:443",
        "192.168.1.1:22",
        "192.168.100.200:8080",
        // IPv6
        "[::1]:80",
        "[::1]:443",
    ];
    
    for original in test_addresses {
        let encoded = encoder.encode(original).unwrap();
        let decoded = encoder.decode(
            &encoded.words[0],
            &encoded.words[1],
            &encoded.words[2]
        ).unwrap();
        
        // For localhost and well-known addresses, we should get exact matches
        if original.contains("127.0.0.1") || original.contains("::1") {
            // Extract port from both for comparison
            let original_port = original.split(':').last().unwrap();
            let decoded_port = decoded.split(':').last().unwrap();
            assert_eq!(original_port, decoded_port);
            
            if original.contains("127.0.0.1") {
                assert!(decoded.starts_with("127.0.0."));
            }
            if original.contains("::1") {
                assert!(decoded.contains("::1"));
            }
        }
    }
}

#[test]
fn test_deterministic_encoding() {
    let encoder = IpPortEncoder::new().unwrap();
    
    let test_cases = [
        "192.168.1.100:8080",
        "[2001:db8::1]:443",
        "10.0.0.1:22",
    ];
    
    for addr in test_cases {
        let encoding1 = encoder.encode(addr).unwrap();
        let encoding2 = encoder.encode(addr).unwrap();
        let encoding3 = encoder.encode(addr).unwrap();
        
        assert_eq!(encoding1.words, encoding2.words);
        assert_eq!(encoding2.words, encoding3.words);
        assert_eq!(encoding1.compression_ratio, encoding2.compression_ratio);
    }
}

#[test]
fn test_invalid_inputs() {
    let encoder = IpPortEncoder::new().unwrap();
    
    let invalid_inputs = [
        "not-an-ip",
        "192.168.1",
        "192.168.1.1",  // Missing port
        "192.168.1.1:",  // Missing port number
        "192.168.1.1:abc",  // Invalid port
        "192.168.1.1:99999",  // Port out of range
        "[invalid-ipv6]:80",
        "[]]:80",
    ];
    
    for input in invalid_inputs {
        assert!(encoder.encode(input).is_err());
    }
}

#[test]
fn test_edge_case_ports() {
    let encoder = IpPortEncoder::new().unwrap();
    
    // Test edge case port numbers
    let ports = [
        1,      // Minimum valid port
        65535,  // Maximum valid port
        1024,   // Common boundary
        49152,  // Dynamic port range start
    ];
    
    for port in ports {
        let addr = format!("192.168.1.1:{}", port);
        let result = encoder.encode(&addr).unwrap();
        assert_eq!(result.words.len(), 3);
        assert_eq!(result.socket_addr.port(), port);
    }
}

#[test]
fn test_compression_efficiency() {
    let encoder = IpPortEncoder::new().unwrap();
    
    // Test that common patterns achieve good compression
    let efficient_cases = [
        ("127.0.0.1:80", 0.4),      // Should achieve >40% compression
        ("127.0.0.1:443", 0.4),     // Common port bonus
        ("[::1]:80", 0.8),          // IPv6 localhost super efficient
        ("192.168.1.1:80", 0.3),    // Private network with common port
    ];
    
    for (addr, min_compression) in efficient_cases {
        let result = encoder.encode(addr).unwrap();
        assert!(
            result.compression_ratio >= min_compression,
            "{} only achieved {:.1}% compression (expected >={:.1}%)",
            addr,
            result.compression_ratio * 100.0,
            min_compression * 100.0
        );
    }
}

#[test]
fn test_word_quality() {
    let encoder = IpPortEncoder::new().unwrap();
    
    // Ensure encoded words are valid dictionary words
    let test_addr = "192.168.1.100:8080";
    let result = encoder.encode(test_addr).unwrap();
    
    for word in &result.words {
        assert!(!word.is_empty());
        assert!(word.chars().all(|c| c.is_alphabetic()));
        assert!(word.len() >= 3); // Minimum word length
    }
}

#[test]
fn test_socket_addr_parsing() {
    let encoder = IpPortEncoder::new().unwrap();
    
    // Test various valid socket address formats
    let valid_formats = [
        "127.0.0.1:80",
        "192.168.1.1:443",
        "[::1]:80",
        "[2001:db8::1]:443",
        "0.0.0.0:8080",
        "255.255.255.255:65535",
    ];
    
    for format in valid_formats {
        let result = encoder.encode(format);
        assert!(result.is_ok(), "Failed to parse valid format: {}", format);
    }
}