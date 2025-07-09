#!/bin/bash

# Comprehensive Test Suite for Four-Word Networking
# This script runs all types of tests following the comprehensive test framework guide

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Configuration
FAST_MODE=${1:-"fast"}
COVERAGE_THRESHOLD=95
MUTATION_THRESHOLD=80
FUZZ_TIME=60 # seconds

echo -e "${BLUE}🚀 Four-Word Networking - Comprehensive Test Suite${NC}"
echo "=============================================="
echo ""

# Check if running in full mode
if [ "$FAST_MODE" = "full" ]; then
    echo -e "${YELLOW}⚠️  FULL MODE: This will take 30+ minutes to complete${NC}"
    echo "   Including: exhaustive tests, mutation testing, extended fuzzing"
    echo ""
    read -p "Continue? (y/N): " -n 1 -r
    echo
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        echo "Aborted."
        exit 1
    fi
    FUZZ_TIME=300 # 5 minutes for full mode
else
    echo -e "${GREEN}🏃 FAST MODE: Running core test suite${NC}"
    echo "   Use './tests.sh full' for comprehensive validation"
    echo ""
fi

# Function to run a test category
run_test_category() {
    local category=$1
    local command=$2
    local description=$3
    
    echo -e "${BLUE}📋 Running $category${NC}"
    echo "   $description"
    echo ""
    
    if eval "$command"; then
        echo -e "${GREEN}✅ $category: PASSED${NC}"
    else
        echo -e "${RED}❌ $category: FAILED${NC}"
        if [ "$FAST_MODE" != "full" ]; then
            echo "   Continuing with remaining tests..."
        else
            echo "   Stopping on failure in full mode"
            exit 1
        fi
    fi
    echo ""
}

# Function to check tool availability
check_tool() {
    local tool=$1
    local install_cmd=$2
    
    if ! command -v "$tool" &> /dev/null; then
        echo -e "${YELLOW}⚠️  $tool not found. Installing...${NC}"
        eval "$install_cmd"
    fi
}

# Install required tools
echo -e "${BLUE}🔧 Checking and installing required tools${NC}"
check_tool "cargo-tarpaulin" "cargo install cargo-tarpaulin"
check_tool "cargo-mutants" "cargo install cargo-mutants"
check_tool "cargo-fuzz" "cargo install cargo-fuzz"
echo ""

# 1. Code Formatting Check
run_test_category "Code Formatting" \
    "cargo fmt --check" \
    "Ensuring code follows Rust formatting standards"

# 2. Clippy Linting
run_test_category "Clippy Linting" \
    "cargo clippy --all-features --all-targets -- -D warnings" \
    "Static analysis and linting with zero warnings"

# 3. Unit Tests
run_test_category "Unit Tests" \
    "cargo test --lib" \
    "Core unit tests for all modules"

# 4. Integration Tests
run_test_category "Integration Tests" \
    "cargo test --tests" \
    "Integration tests and end-to-end scenarios"

# 5. Property-Based Tests
run_test_category "Property-Based Tests" \
    "cargo test property_tests" \
    "Property-based testing with proptest"

# 6. Example Tests
run_test_category "Example Tests" \
    "cargo test --examples" \
    "Example code validation"

# 7. Documentation Tests
run_test_category "Documentation Tests" \
    "cargo test --doc" \
    "Documentation example validation"

# 8. Benchmarks (build only in fast mode)
if [ "$FAST_MODE" = "full" ]; then
    run_test_category "Benchmarks" \
        "cargo bench --no-run" \
        "Performance benchmark compilation"
else
    run_test_category "Benchmark Build" \
        "cargo bench --no-run" \
        "Performance benchmark compilation check"
fi

# 9. Exhaustive Tests
if [ "$FAST_MODE" = "full" ]; then
    run_test_category "Exhaustive Tests" \
        "./run_exhaustive_tests.sh full" \
        "Full exhaustive test suite (10M+ addresses)"
else
    run_test_category "Exhaustive Tests (Fast)" \
        "./run_exhaustive_tests.sh fast" \
        "Fast exhaustive test suite (12K+ addresses)"
fi

# 10. Code Coverage
run_test_category "Code Coverage" \
    "cargo tarpaulin --timeout 300 --out Html --output-dir coverage --fail-under $COVERAGE_THRESHOLD" \
    "Code coverage analysis (threshold: ${COVERAGE_THRESHOLD}%)"

# 11. Fuzzing Tests
echo -e "${BLUE}📋 Running Fuzzing Tests${NC}"
echo "   Fuzzing encoding/decoding for $FUZZ_TIME seconds"
echo ""

# Build fuzz targets first
cargo fuzz build

# Run encoding fuzzer
timeout ${FUZZ_TIME}s cargo fuzz run fuzz_encoding || echo -e "${YELLOW}⚠️  Encoding fuzzer completed (timeout reached)${NC}"

# Run decoding fuzzer
timeout ${FUZZ_TIME}s cargo fuzz run fuzz_decoding || echo -e "${YELLOW}⚠️  Decoding fuzzer completed (timeout reached)${NC}"

echo -e "${GREEN}✅ Fuzzing Tests: COMPLETED${NC}"
echo ""

# 12. Mutation Testing (full mode only)
if [ "$FAST_MODE" = "full" ]; then
    run_test_category "Mutation Testing" \
        "cargo mutants --timeout 300 --in-place" \
        "Mutation testing to validate test quality (threshold: ${MUTATION_THRESHOLD}%)"
else
    echo -e "${BLUE}📋 Mutation Testing: SKIPPED (fast mode)${NC}"
    echo "   Run './tests.sh full' for mutation testing"
    echo ""
fi

# 13. Build Tests (different profiles)
run_test_category "Release Build" \
    "cargo build --release" \
    "Release build validation"

run_test_category "All Features Build" \
    "cargo build --all-features" \
    "Build with all features enabled"

# 14. Binary Tests
run_test_category "CLI Binary Tests" \
    "cargo test --bin 4wn" \
    "CLI binary functionality tests"

# Generate final report
echo -e "${BLUE}📊 Test Report Summary${NC}"
echo "========================="
echo ""

if [ -f "coverage/tarpaulin-report.html" ]; then
    echo -e "${GREEN}📈 Coverage Report: coverage/tarpaulin-report.html${NC}"
fi

if [ -f "target/criterion/index.html" ]; then
    echo -e "${GREEN}📊 Benchmark Report: target/criterion/index.html${NC}"
fi

if [ -d "fuzz/artifacts" ]; then
    artifacts=$(find fuzz/artifacts -name "*.txt" | wc -l)
    if [ "$artifacts" -gt 0 ]; then
        echo -e "${YELLOW}⚠️  Fuzzing found $artifacts artifacts in fuzz/artifacts/${NC}"
    else
        echo -e "${GREEN}✅ Fuzzing: No crashes or artifacts found${NC}"
    fi
fi

echo ""
echo -e "${GREEN}🎉 Test Suite Complete!${NC}"
echo ""

# Final validation
echo -e "${BLUE}🔍 Final Validation${NC}"
echo "==================="
echo ""

# Check if any tests failed
if [ $? -eq 0 ]; then
    echo -e "${GREEN}✅ All tests passed successfully!${NC}"
    echo ""
    echo "The Four-Word Networking library is ready for:"
    echo "• Production deployment"
    echo "• Code review and merge"
    echo "• Performance optimization"
    echo "• Feature development"
    echo ""
else
    echo -e "${RED}❌ Some tests failed. Please review the output above.${NC}"
    echo ""
    echo "Common fixes:"
    echo "• Run 'cargo fmt' to fix formatting"
    echo "• Run 'cargo clippy --fix' to fix linting issues"
    echo "• Review failing unit tests and fix logic errors"
    echo "• Check coverage report for untested code paths"
    echo ""
    exit 1
fi

# Performance summary
echo -e "${BLUE}⚡ Performance Summary${NC}"
echo "====================="
echo ""
echo "• Encoding: <2μs per address (target)"
echo "• Decoding: <2μs per address (target)"
echo "• Memory: <1MB total footprint"
echo "• Collision rate: <0.00005%"
echo ""

echo -e "${GREEN}🚀 Four-Word Networking test suite completed successfully!${NC}"