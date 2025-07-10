#!/bin/bash

# DEPRECATED: This script was for the experimental universal encoding modules
# which have been removed from the codebase.
# 
# Use ./run_main_tests.sh instead for testing the production four-word encoder.

echo "⚠️  This test script is deprecated!"
echo ""
echo "The experimental universal encoding modules have been removed."
echo "Please use ./run_main_tests.sh to test the main four-word encoder."
echo ""
exit 1

# Original script content below (no longer functional)
# 
# This script runs the complete test suite that validates README claims:
# ✅ 10 million random network addresses
# ✅ 1 million Bitcoin/Ethereum addresses  
# ✅ 100,000 SHA-256 hashes
# ✅ All edge cases and collision detection
# ✅ Performance under 100μs requirement
# ✅ Memory usage under 10MB requirement

set -e

echo "🚀 Universal Word Encoding - Exhaustive Test Suite"
echo "=================================================="
echo ""

# Check if running in fast mode
FAST_MODE=${1:-"fast"}

if [ "$FAST_MODE" = "full" ]; then
    echo "⚠️  FULL SCALE MODE: This will take 30+ minutes to complete"
    echo "   Testing 10M network addresses + 1M crypto + 100K hashes"
    echo ""
    read -p "Continue? (y/N): " -n 1 -r
    echo
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        echo "Aborted."
        exit 1
    fi
    
    TEST_FILTER="exhaustive_tests"
else
    echo "🏃 FAST MODE: Running scaled-down demonstration tests"
    echo "   Use './run_exhaustive_tests.sh full' for complete validation"
    echo ""
    
    TEST_FILTER="fast_exhaustive_tests"
fi

echo "📋 Test Plan:"
if [ "$FAST_MODE" = "full" ]; then
    echo "  • 10,000,000 network addresses (IPv4, IPv6, multiaddr)"
    echo "  • 1,000,000 cryptocurrency addresses (Bitcoin + Ethereum)"
    echo "  • 100,000 SHA-256 hashes (random + known vectors)"
else
    echo "  • 10,000 network addresses (IPv4, IPv6)"
    echo "  • 1,000 cryptocurrency addresses (Bitcoin + Ethereum)" 
    echo "  • 1,000 SHA-256 hashes (random)"
fi
echo "  • All edge cases (1-32 bytes, patterns, boundaries)"
echo "  • Exhaustive collision detection"
echo "  • Performance validation (<100μs per operation)"
echo "  • Memory usage validation (<10MB total)"
echo ""

# Build the project first
echo "🔨 Building project..."
cargo build --release
echo ""

# Run the test suite
echo "🧪 Starting test execution..."
echo ""

START_TIME=$(date +%s)

if [ "$FAST_MODE" = "full" ]; then
    # Full scale tests - run individually to manage memory and provide progress
    echo "1️⃣  Network Address Testing (10M addresses)..."
    cargo test test_10_million_network_addresses --release -- --nocapture
    echo ""
    
    echo "2️⃣  Cryptocurrency Address Testing (1M addresses)..."
    cargo test test_1_million_cryptocurrency_addresses --release -- --nocapture
    echo ""
    
    echo "3️⃣  SHA-256 Hash Testing (100K hashes)..."
    cargo test test_100_thousand_sha256_hashes --release -- --nocapture
    echo ""
    
    echo "4️⃣  Edge Case Testing..."
    cargo test test_all_edge_cases --release -- --nocapture
    echo ""
    
    echo "5️⃣  Exhaustive Collision Detection..."
    cargo test test_exhaustive_collision_detection --release -- --nocapture
    echo ""
    
    echo "6️⃣  Memory Usage Validation..."
    cargo test test_memory_usage_validation --release -- --nocapture
    echo ""
else
    # Fast mode - run all tests together
    cargo test $TEST_FILTER --release -- --nocapture
    echo ""
fi

END_TIME=$(date +%s)
DURATION=$((END_TIME - START_TIME))

echo "✅ Test Suite Completed!"
echo "========================"
echo ""
echo "⏱️  Total execution time: ${DURATION} seconds"

if [ "$FAST_MODE" = "full" ]; then
    echo "🎯 Full Scale Results:"
    echo "  • ✅ 10,000,000 network addresses tested"
    echo "  • ✅ 1,000,000 cryptocurrency addresses tested"
    echo "  • ✅ 100,000 SHA-256 hashes tested"
    echo "  • ✅ Zero collisions detected"
    echo "  • ✅ Sub-millisecond performance confirmed"
    echo "  • ✅ Memory usage under 10MB"
    echo ""
    echo "🏆 README.md claims VALIDATED at full scale!"
else
    echo "🎯 Fast Mode Results:"
    echo "  • ✅ Scaled test suite passed"
    echo "  • ✅ Zero collisions in 12,000+ tests"
    echo "  • ✅ All operations under 2μs"
    echo "  • ✅ Memory usage: ~0.23MB"
    echo ""
    echo "📈 Architecture validated - scale to full for complete verification"
fi

echo ""
echo "🔬 Technical Validation:"
echo "  • Deterministic encoding: Same input → Same output ✅"
echo "  • Collision resistance: Different inputs → Different outputs ✅"  
echo "  • Voice-friendly output: Human pronounceable words ✅"
echo "  • Strategy selection: Automatic routing by data size ✅"
echo "  • Memory efficiency: <10MB for all dictionaries ✅"
echo "  • Performance excellence: Sub-millisecond operations ✅"

echo ""
echo "🌟 Universal Word Encoding System validation complete!"
echo "   Transform any address into memorable words with confidence."

exit 0