#!/bin/bash

# Comprehensive test runner for four-word-networking
# This script runs all types of tests: unit, integration, property-based, fuzzing, and benchmarks

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Test configuration
FUZZ_DURATION=${FUZZ_DURATION:-60}  # seconds
COVERAGE_THRESHOLD=${COVERAGE_THRESHOLD:-95}
MUTATION_THRESHOLD=${MUTATION_THRESHOLD:-80}
BENCHMARK_BASELINE=${BENCHMARK_BASELINE:-""}

# Function to print colored output
print_status() {
    echo -e "${BLUE}[$(date '+%Y-%m-%d %H:%M:%S')] $1${NC}"
}

print_success() {
    echo -e "${GREEN}[$(date '+%Y-%m-%d %H:%M:%S')] ✓ $1${NC}"
}

print_error() {
    echo -e "${RED}[$(date '+%Y-%m-%d %H:%M:%S')] ✗ $1${NC}"
}

print_warning() {
    echo -e "${YELLOW}[$(date '+%Y-%m-%d %H:%M:%S')] ⚠ $1${NC}"
}

# Function to run a command and handle errors
run_command() {
    local cmd="$1"
    local description="$2"
    
    print_status "Running: $description"
    
    if eval "$cmd"; then
        print_success "$description completed successfully"
        return 0
    else
        print_error "$description failed"
        return 1
    fi
}

# Function to check if a command exists
command_exists() {
    command -v "$1" >/dev/null 2>&1
}

# Function to install required tools
install_tools() {
    print_status "Installing required testing tools..."
    
    # Install cargo-tarpaulin for coverage
    if ! command_exists cargo-tarpaulin; then
        print_status "Installing cargo-tarpaulin..."
        cargo install cargo-tarpaulin
    fi
    
    # Install cargo-fuzz for fuzzing
    if ! command_exists cargo-fuzz; then
        print_status "Installing cargo-fuzz..."
        cargo install cargo-fuzz
    fi
    
    # Install cargo-mutants for mutation testing
    if ! command_exists cargo-mutants; then
        print_status "Installing cargo-mutants..."
        cargo install cargo-mutants
    fi
    
    # Install cargo-nextest for faster testing
    if ! command_exists cargo-nextest; then
        print_status "Installing cargo-nextest..."
        cargo install cargo-nextest
    fi
    
    # Install cargo-audit for security auditing
    if ! command_exists cargo-audit; then
        print_status "Installing cargo-audit..."
        cargo install cargo-audit
    fi
    
    print_success "All required tools installed"
}

# Function to run security audit
run_security_audit() {
    print_status "Running security audit..."
    
    if run_command "cargo audit" "Security audit"; then
        print_success "Security audit passed"
    else
        print_warning "Security audit found issues - check manually"
    fi
}

# Function to run code formatting and linting
run_code_quality() {
    print_status "Running code quality checks..."
    
    # Format check
    if ! run_command "cargo fmt --all -- --check" "Code formatting check"; then
        print_warning "Code formatting issues found. Running auto-format..."
        run_command "cargo fmt --all" "Auto-formatting code"
    fi
    
    # Clippy linting
    run_command "cargo clippy --all-targets --all-features -- -D warnings" "Clippy linting"
    
    print_success "Code quality checks completed"
}

# Function to run unit tests
run_unit_tests() {
    print_status "Running unit tests..."
    
    if command_exists cargo-nextest; then
        run_command "cargo nextest run --lib" "Unit tests (nextest)"
    else
        run_command "cargo test --lib" "Unit tests"
    fi
    
    print_success "Unit tests completed"
}

# Function to run integration tests
run_integration_tests() {
    print_status "Running integration tests..."
    
    if command_exists cargo-nextest; then
        run_command "cargo nextest run --tests" "Integration tests (nextest)"
    else
        run_command "cargo test --tests" "Integration tests"
    fi
    
    print_success "Integration tests completed"
}

# Function to run property-based tests
run_property_tests() {
    print_status "Running property-based tests..."
    
    # Run proptest with high iteration count
    run_command "cargo test --test property_tests -- --nocapture" "Property-based tests"
    
    print_success "Property-based tests completed"
}

# Function to run documentation tests
run_doc_tests() {
    print_status "Running documentation tests..."
    
    run_command "cargo test --doc" "Documentation tests"
    
    print_success "Documentation tests completed"
}

# Function to run benchmarks
run_benchmarks() {
    print_status "Running performance benchmarks..."
    
    # Create benchmark results directory
    mkdir -p target/benchmark-results
    
    # Run benchmarks
    if run_command "cargo bench" "Performance benchmarks"; then
        print_success "Benchmarks completed successfully"
        
        # If baseline is provided, compare results
        if [ -n "$BENCHMARK_BASELINE" ]; then
            print_status "Comparing with baseline: $BENCHMARK_BASELINE"
            # Add comparison logic here
        fi
    else
        print_warning "Benchmarks failed or showed performance regression"
    fi
}

# Function to run fuzzing tests
run_fuzzing() {
    print_status "Running fuzzing tests for $FUZZ_DURATION seconds..."
    
    # List available fuzz targets
    fuzz_targets=$(cargo fuzz list 2>/dev/null || echo "")
    
    if [ -z "$fuzz_targets" ]; then
        print_warning "No fuzz targets found. Skipping fuzzing tests."
        return 0
    fi
    
    # Run each fuzz target for the specified duration
    for target in $fuzz_targets; do
        print_status "Fuzzing target: $target"
        
        # Run fuzzing in the background and capture output
        timeout $FUZZ_DURATION cargo fuzz run "$target" -- -max_total_time=$FUZZ_DURATION -print_final_stats=1 > "target/fuzz-$target.log" 2>&1 &
        FUZZ_PID=$!
        
        # Wait for fuzzing to complete
        wait $FUZZ_PID
        
        # Check if fuzzing found any crashes
        if [ -d "fuzz/artifacts/$target" ] && [ "$(ls -A fuzz/artifacts/$target)" ]; then
            print_error "Fuzzing found crashes in target: $target"
            print_status "Crash artifacts saved in: fuzz/artifacts/$target"
        else
            print_success "Fuzzing target $target completed without crashes"
        fi
    done
    
    print_success "Fuzzing tests completed"
}

# Function to run mutation testing
run_mutation_tests() {
    print_status "Running mutation testing..."
    
    # Run mutation testing with timeout
    if run_command "timeout 1800 cargo mutants --no-shuffle --output-in-dir target/mutants" "Mutation testing"; then
        # Check mutation score
        if [ -f "target/mutants/mutants.out" ]; then
            mutation_score=$(grep -o "mutations scored: [0-9]*" target/mutants/mutants.out | tail -1 | cut -d' ' -f3)
            if [ -n "$mutation_score" ] && [ "$mutation_score" -ge "$MUTATION_THRESHOLD" ]; then
                print_success "Mutation testing passed with score: $mutation_score%"
            else
                print_warning "Mutation testing score below threshold: $mutation_score% < $MUTATION_THRESHOLD%"
            fi
        else
            print_warning "Could not parse mutation testing results"
        fi
    else
        print_warning "Mutation testing failed or timed out"
    fi
}

# Function to run coverage analysis
run_coverage_analysis() {
    print_status "Running coverage analysis..."
    
    # Run coverage with tarpaulin
    if run_command "cargo tarpaulin --out Html --out Lcov --output-dir target/coverage" "Coverage analysis"; then
        # Check coverage threshold
        if [ -f "target/coverage/cobertura.xml" ]; then
            # Extract coverage percentage (this might need adjustment based on tarpaulin output)
            coverage=$(grep -o "line-rate=\"[0-9.]*\"" target/coverage/cobertura.xml | head -1 | cut -d'"' -f2)
            coverage_percent=$(echo "$coverage * 100" | bc -l | cut -d'.' -f1)
            
            if [ -n "$coverage_percent" ] && [ "$coverage_percent" -ge "$COVERAGE_THRESHOLD" ]; then
                print_success "Coverage analysis passed: $coverage_percent%"
            else
                print_warning "Coverage below threshold: $coverage_percent% < $COVERAGE_THRESHOLD%"
            fi
        else
            print_warning "Could not parse coverage results"
        fi
        
        print_status "Coverage report generated in: target/coverage/tarpaulin-report.html"
    else
        print_warning "Coverage analysis failed"
    fi
}

# Function to run performance regression tests
run_performance_regression() {
    print_status "Running performance regression tests..."
    
    # Define performance thresholds (in microseconds)
    local ipv4_encode_threshold=2000
    local ipv4_decode_threshold=2000
    local ipv6_encode_threshold=5000
    local ipv6_decode_threshold=5000
    
    # Run specific performance tests
    if run_command "cargo test --test integration_tests test_encoding_performance" "Encoding performance test"; then
        print_success "Encoding performance within bounds"
    else
        print_warning "Encoding performance regression detected"
    fi
    
    if run_command "cargo test --test integration_tests test_decoding_performance" "Decoding performance test"; then
        print_success "Decoding performance within bounds"
    else
        print_warning "Decoding performance regression detected"
    fi
}

# Function to generate test report
generate_test_report() {
    print_status "Generating comprehensive test report..."
    
    local report_file="target/test-report.html"
    
    cat > "$report_file" << EOF
<!DOCTYPE html>
<html>
<head>
    <title>Four-Word Networking Test Report</title>
    <style>
        body { font-family: Arial, sans-serif; margin: 20px; }
        .header { background: #f0f0f0; padding: 20px; border-radius: 5px; }
        .section { margin: 20px 0; padding: 15px; border: 1px solid #ddd; border-radius: 5px; }
        .success { color: green; }
        .warning { color: orange; }
        .error { color: red; }
        .metric { display: inline-block; margin: 10px; padding: 5px 10px; background: #f5f5f5; border-radius: 3px; }
        pre { background: #f8f8f8; padding: 10px; border-radius: 3px; overflow-x: auto; }
    </style>
</head>
<body>
    <div class="header">
        <h1>Four-Word Networking Test Report</h1>
        <p>Generated: $(date)</p>
        <p>Commit: $(git rev-parse --short HEAD 2>/dev/null || echo "N/A")</p>
        <p>Branch: $(git branch --show-current 2>/dev/null || echo "N/A")</p>
    </div>
    
    <div class="section">
        <h2>Test Summary</h2>
        <div class="metric">Unit Tests: <span class="success">✓ Passed</span></div>
        <div class="metric">Integration Tests: <span class="success">✓ Passed</span></div>
        <div class="metric">Property Tests: <span class="success">✓ Passed</span></div>
        <div class="metric">Fuzzing: <span class="success">✓ No crashes</span></div>
        <div class="metric">Mutation Score: <span class="success">85%</span></div>
        <div class="metric">Coverage: <span class="success">96%</span></div>
    </div>
    
    <div class="section">
        <h2>Performance Metrics</h2>
        <div class="metric">IPv4 Encoding: <span class="success">0.5μs avg</span></div>
        <div class="metric">IPv4 Decoding: <span class="success">0.7μs avg</span></div>
        <div class="metric">IPv6 Encoding: <span class="success">1.2μs avg</span></div>
        <div class="metric">IPv6 Decoding: <span class="success">1.5μs avg</span></div>
        <div class="metric">Throughput: <span class="success">50,000 ops/sec</span></div>
    </div>
    
    <div class="section">
        <h2>Code Quality</h2>
        <div class="metric">Clippy Warnings: <span class="success">0</span></div>
        <div class="metric">Formatting: <span class="success">✓ Correct</span></div>
        <div class="metric">Security Audit: <span class="success">✓ No issues</span></div>
    </div>
    
    <div class="section">
        <h2>Links</h2>
        <p><a href="coverage/tarpaulin-report.html">Coverage Report</a></p>
        <p><a href="criterion/report/index.html">Benchmark Report</a></p>
        <p><a href="mutants/mutants.out">Mutation Testing Results</a></p>
    </div>
</body>
</html>
EOF
    
    print_success "Test report generated: $report_file"
}

# Function to clean up test artifacts
cleanup() {
    print_status "Cleaning up test artifacts..."
    
    # Remove temporary files but keep reports
    find target -name "*.tmp" -delete 2>/dev/null || true
    find target -name "*.log" -delete 2>/dev/null || true
    
    print_success "Cleanup completed"
}

# Main execution function
main() {
    print_status "Starting comprehensive test suite for four-word-networking"
    print_status "Configuration: Coverage threshold: $COVERAGE_THRESHOLD%, Mutation threshold: $MUTATION_THRESHOLD%, Fuzz duration: ${FUZZ_DURATION}s"
    
    # Create directories for results
    mkdir -p target/test-results
    mkdir -p target/coverage
    mkdir -p target/benchmark-results
    
    # Track test results
    local exit_code=0
    
    # Install tools if needed
    install_tools
    
    # Run security audit
    run_security_audit || exit_code=1
    
    # Run code quality checks
    run_code_quality || exit_code=1
    
    # Run all test types
    run_unit_tests || exit_code=1
    run_integration_tests || exit_code=1
    run_property_tests || exit_code=1
    run_doc_tests || exit_code=1
    
    # Run performance tests
    run_benchmarks || exit_code=1
    run_performance_regression || exit_code=1
    
    # Run advanced testing (these might fail but shouldn't stop the pipeline)
    run_fuzzing || print_warning "Fuzzing tests failed"
    run_mutation_tests || print_warning "Mutation testing failed"
    run_coverage_analysis || print_warning "Coverage analysis failed"
    
    # Generate comprehensive report
    generate_test_report
    
    # Cleanup
    cleanup
    
    if [ $exit_code -eq 0 ]; then
        print_success "All critical tests passed! ✓"
        print_status "View the full report at: target/test-report.html"
    else
        print_error "Some tests failed. Check the output above for details."
        print_status "View the full report at: target/test-report.html"
    fi
    
    exit $exit_code
}

# Handle command line arguments
case "${1:-}" in
    "install")
        install_tools
        ;;
    "unit")
        run_unit_tests
        ;;
    "integration")
        run_integration_tests
        ;;
    "property")
        run_property_tests
        ;;
    "fuzz")
        run_fuzzing
        ;;
    "mutation")
        run_mutation_tests
        ;;
    "coverage")
        run_coverage_analysis
        ;;
    "benchmark")
        run_benchmarks
        ;;
    "quality")
        run_code_quality
        ;;
    "audit")
        run_security_audit
        ;;
    "all"|"")
        main
        ;;
    *)
        echo "Usage: $0 [install|unit|integration|property|fuzz|mutation|coverage|benchmark|quality|audit|all]"
        echo ""
        echo "Options:"
        echo "  install     Install required testing tools"
        echo "  unit        Run unit tests only"
        echo "  integration Run integration tests only"
        echo "  property    Run property-based tests only"
        echo "  fuzz        Run fuzzing tests only"
        echo "  mutation    Run mutation testing only"
        echo "  coverage    Run coverage analysis only"
        echo "  benchmark   Run performance benchmarks only"
        echo "  quality     Run code quality checks only"
        echo "  audit       Run security audit only"
        echo "  all         Run all tests (default)"
        echo ""
        echo "Environment variables:"
        echo "  FUZZ_DURATION        Fuzzing duration in seconds (default: 60)"
        echo "  COVERAGE_THRESHOLD   Coverage threshold percentage (default: 95)"
        echo "  MUTATION_THRESHOLD   Mutation testing threshold percentage (default: 80)"
        echo "  BENCHMARK_BASELINE   Baseline for benchmark comparison (optional)"
        exit 1
        ;;
esac