/// Test configuration and utilities for comprehensive testing
use std::collections::HashMap;
use std::sync::Once;
use tempfile::TempDir;

static INIT: Once = Once::new();

/// Initialize test environment
pub fn init_test_env() {
    INIT.call_once(|| {
        env_logger::init();
    });
}

/// Test data generator for addresses
pub struct AddressGenerator {
    ipv4_addresses: Vec<String>,
    ipv6_addresses: Vec<String>,
    multiaddrs: Vec<String>,
}

impl AddressGenerator {
    pub fn new() -> Self {
        Self {
            ipv4_addresses: vec![
                "127.0.0.1".to_string(),
                "192.168.1.1".to_string(),
                "10.0.0.1".to_string(),
                "172.16.0.1".to_string(),
                "8.8.8.8".to_string(),
                "1.1.1.1".to_string(),
                "208.67.222.222".to_string(),
                "0.0.0.0".to_string(),
                "255.255.255.255".to_string(),
            ],
            ipv6_addresses: vec![
                "::1".to_string(),
                "::".to_string(),
                "2001:db8::1".to_string(),
                "fe80::1".to_string(),
                "ff02::1".to_string(),
                "2001:4860:4860::8888".to_string(),
                "2606:4700:4700::1111".to_string(),
            ],
            multiaddrs: vec![
                "/ip4/127.0.0.1/tcp/8080".to_string(),
                "/ip6/::1/tcp/443".to_string(),
                "/ip4/192.168.1.1/udp/53".to_string(),
                "/ip6/2001:db8::1/tcp/80".to_string(),
            ],
        }
    }

    pub fn ipv4_addresses(&self) -> &[String] {
        &self.ipv4_addresses
    }

    pub fn ipv6_addresses(&self) -> &[String] {
        &self.ipv6_addresses
    }

    pub fn multiaddrs(&self) -> &[String] {
        &self.multiaddrs
    }
}

/// Test coverage metrics
pub struct TestCoverage {
    pub lines_covered: usize,
    pub lines_total: usize,
    pub branches_covered: usize,
    pub branches_total: usize,
    pub functions_covered: usize,
    pub functions_total: usize,
}

impl TestCoverage {
    pub fn line_coverage(&self) -> f64 {
        if self.lines_total == 0 {
            0.0
        } else {
            (self.lines_covered as f64 / self.lines_total as f64) * 100.0
        }
    }

    pub fn branch_coverage(&self) -> f64 {
        if self.branches_total == 0 {
            0.0
        } else {
            (self.branches_covered as f64 / self.branches_total as f64) * 100.0
        }
    }

    pub fn function_coverage(&self) -> f64 {
        if self.functions_total == 0 {
            0.0
        } else {
            (self.functions_covered as f64 / self.functions_total as f64) * 100.0
        }
    }
}

/// Test performance metrics
pub struct TestPerformance {
    pub encoding_time_us: u64,
    pub decoding_time_us: u64,
    pub memory_usage_bytes: usize,
    pub throughput_ops_per_sec: f64,
}

/// Test fixture for temporary directories
pub struct TestFixture {
    pub temp_dir: TempDir,
}

impl TestFixture {
    pub fn new() -> Self {
        Self {
            temp_dir: TempDir::new().expect("Failed to create temp dir"),
        }
    }

    pub fn path(&self) -> &std::path::Path {
        self.temp_dir.path()
    }
}

/// Test result aggregator
pub struct TestResults {
    pub results: HashMap<String, TestResult>,
}

#[derive(Debug, Clone)]
pub struct TestResult {
    pub name: String,
    pub passed: bool,
    pub duration_ms: u64,
    pub error: Option<String>,
}

impl TestResults {
    pub fn new() -> Self {
        Self {
            results: HashMap::new(),
        }
    }

    pub fn add_result(&mut self, result: TestResult) {
        self.results.insert(result.name.clone(), result);
    }

    pub fn passed_count(&self) -> usize {
        self.results.values().filter(|r| r.passed).count()
    }

    pub fn failed_count(&self) -> usize {
        self.results.values().filter(|r| !r.passed).count()
    }

    pub fn total_duration_ms(&self) -> u64 {
        self.results.values().map(|r| r.duration_ms).sum()
    }
}

/// Test data for edge cases
pub fn edge_case_data() -> Vec<String> {
    vec![
        "0.0.0.0".to_string(),
        "255.255.255.255".to_string(),
        "::".to_string(),
        "ffff:ffff:ffff:ffff:ffff:ffff:ffff:ffff".to_string(),
        "127.0.0.1".to_string(),
        "::1".to_string(),
        "169.254.1.1".to_string(),  // Link-local IPv4
        "fe80::1".to_string(),      // Link-local IPv6
        "224.0.0.1".to_string(),    // Multicast IPv4
        "ff02::1".to_string(),      // Multicast IPv6
    ]
}

/// Test data for real-world scenarios
pub fn real_world_data() -> Vec<String> {
    vec![
        "8.8.8.8".to_string(),               // Google DNS
        "1.1.1.1".to_string(),               // Cloudflare DNS
        "208.67.222.222".to_string(),        // OpenDNS
        "2001:4860:4860::8888".to_string(),  // Google IPv6 DNS
        "2606:4700:4700::1111".to_string(),  // Cloudflare IPv6 DNS
        "192.168.1.1".to_string(),           // Common router IP
        "10.0.0.1".to_string(),              // Private network
        "172.16.0.1".to_string(),            // Private network
    ]
}

/// Assertion helpers
pub fn assert_encoding_roundtrip(original: &str, encoded: &str, decoded: &str) {
    assert_eq!(original, decoded, "Roundtrip failed: {} -> {} -> {}", original, encoded, decoded);
}

pub fn assert_performance_bounds(time_us: u64, max_time_us: u64) {
    assert!(time_us <= max_time_us, "Performance test failed: {}μs > {}μs", time_us, max_time_us);
}

pub fn assert_compression_ratio(original_size: usize, compressed_size: usize, min_ratio: f64) {
    let ratio = compressed_size as f64 / original_size as f64;
    assert!(ratio >= min_ratio, "Compression ratio too low: {} < {}", ratio, min_ratio);
}

/// Test macros
#[macro_export]
macro_rules! test_roundtrip {
    ($encoder:expr, $input:expr) => {
        let encoded = $encoder.encode($input).expect("Encoding failed");
        let decoded = $encoder.decode(&encoded).expect("Decoding failed");
        assert_eq!($input, decoded, "Roundtrip failed for: {}", $input);
    };
}

#[macro_export]
macro_rules! test_performance {
    ($name:expr, $operation:expr, $max_time_us:expr) => {
        let start = std::time::Instant::now();
        $operation;
        let duration = start.elapsed();
        let time_us = duration.as_micros() as u64;
        assert!(time_us <= $max_time_us, 
            "Performance test '{}' failed: {}μs > {}μs", 
            $name, time_us, $max_time_us);
    };
}

#[macro_export]
macro_rules! test_batch {
    ($test_fn:expr, $inputs:expr) => {
        for (i, input) in $inputs.iter().enumerate() {
            match std::panic::catch_unwind(|| $test_fn(input)) {
                Ok(_) => {},
                Err(e) => {
                    panic!("Test failed for input {} (index {}): {:?}", input, i, e);
                }
            }
        }
    };
}

impl Default for AddressGenerator {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for TestResults {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for TestFixture {
    fn default() -> Self {
        Self::new()
    }
}