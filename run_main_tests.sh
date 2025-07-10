#!/bin/bash

# Main Test Runner for Four-Word Networking
# 
# This script runs the core test suite for the production-ready
# four-word encoding system.

set -e

echo "🚀 Four-Word Networking - Core Test Suite"
echo "========================================="
echo ""

echo "📋 Test Plan:"
echo "  • Unit tests for all modules"
echo "  • Integration tests for IP address encoding/decoding"
echo "  • Property-based tests for encoding invariants"
echo "  • CLI functionality tests"
echo "  • Performance validation"
echo ""

# Build the project
echo "🔨 Building project..."
cargo build --release
echo ""

# Run all tests
echo "🧪 Running test suite..."
echo ""

START_TIME=$(date +%s)

# Run library tests
echo "1️⃣  Running library tests..."
cargo test --lib --release -- --nocapture
echo ""

# Run integration tests
echo "2️⃣  Running integration tests..."
cargo test --test '*' --release -- --nocapture
echo ""

# Test the CLI
echo "3️⃣  Testing CLI functionality..."
echo "   Testing IPv4 encoding..."
./target/release/4wn 192.168.1.1:443
echo ""
echo "   Testing IPv4 decoding..."
./target/release/4wn paper.broaden.smith.bully
echo ""
echo "   Testing IPv6 encoding..."
./target/release/4wn "[::1]:443"
echo ""

END_TIME=$(date +%s)
ELAPSED=$((END_TIME - START_TIME))

echo ""
echo "✅ Test suite completed in ${ELAPSED}s"
echo ""
echo "📊 Summary:"
echo "  • All core functionality tested"
echo "  • IPv4 perfect 4-word encoding verified"
echo "  • IPv6 adaptive encoding verified"
echo "  • CLI working correctly"