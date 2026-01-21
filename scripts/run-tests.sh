#!/bin/bash

# DevSweep Comprehensive Test Runner
# This script runs all tests and generates reports

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Print header
echo -e "${BLUE}================================${NC}"
echo -e "${BLUE}  DevSweep Test Suite Runner${NC}"
echo -e "${BLUE}================================${NC}"
echo ""

# Check if we're in the right directory
if [ ! -f "Cargo.toml" ]; then
    echo -e "${RED}Error: Must be run from project root${NC}"
    exit 1
fi

# Parse arguments
VERBOSE=false
COVERAGE=false
BENCH=false

while [[ $# -gt 0 ]]; do
    case $1 in
        -v|--verbose)
            VERBOSE=true
            shift
            ;;
        -c|--coverage)
            COVERAGE=true
            shift
            ;;
        -b|--bench)
            BENCH=true
            shift
            ;;
        -h|--help)
            echo "Usage: $0 [OPTIONS]"
            echo ""
            echo "Options:"
            echo "  -v, --verbose    Show verbose test output"
            echo "  -c, --coverage   Generate code coverage report"
            echo "  -b, --bench      Run benchmarks"
            echo "  -h, --help       Show this help message"
            exit 0
            ;;
        *)
            echo -e "${RED}Unknown option: $1${NC}"
            exit 1
            ;;
    esac
done

# Test counter
TESTS_RUN=0
TESTS_PASSED=0
TESTS_FAILED=0

# Function to run test section
run_test_section() {
    local section_name=$1
    local test_command=$2
    
    echo -e "${YELLOW}Running: ${section_name}${NC}"
    echo "----------------------------------------"
    
    if eval "$test_command"; then
        echo -e "${GREEN}✓ ${section_name} passed${NC}"
        ((TESTS_PASSED++))
    else
        echo -e "${RED}✗ ${section_name} failed${NC}"
        ((TESTS_FAILED++))
    fi
    ((TESTS_RUN++))
    echo ""
}

# 1. Unit Tests
echo -e "${BLUE}[1/5] Running Unit Tests...${NC}"
echo ""

if [ "$VERBOSE" = true ]; then
    run_test_section "Unit Tests" "cargo test --lib -- --nocapture"
else
    run_test_section "Unit Tests" "cargo test --lib"
fi

# 2. Integration Tests (if they exist)
if [ -d "tests" ]; then
    echo -e "${BLUE}[2/5] Running Integration Tests...${NC}"
    echo ""
    
    if [ "$VERBOSE" = true ]; then
        run_test_section "Integration Tests" "cargo test --test '*' -- --nocapture"
    else
        run_test_section "Integration Tests" "cargo test --test '*'"
    fi
else
    echo -e "${YELLOW}[2/5] No integration tests found (skipping)${NC}"
    echo ""
fi

# 3. Documentation Tests
echo -e "${BLUE}[3/5] Running Doc Tests...${NC}"
echo ""
run_test_section "Documentation Tests" "cargo test --doc"

# 4. Run specific module tests
echo -e "${BLUE}[4/5] Running Module-Specific Tests...${NC}"
echo ""

run_test_section "Cache Settings Tests" "cargo test cache_settings::tests"
run_test_section "Cleanup History Tests" "cargo test cleanup_history::tests"
run_test_section "Scan Cache Tests" "cargo test scan_cache::tests"
run_test_section "Utils Tests" "cargo test utils::tests"

# 5. Code Coverage (optional)
if [ "$COVERAGE" = true ]; then
    echo -e "${BLUE}[5/5] Generating Coverage Report...${NC}"
    echo ""
    
    if command -v cargo-tarpaulin &> /dev/null; then
        run_test_section "Coverage Report" "cargo tarpaulin --out Html --output-dir coverage --exclude-files 'src/main.rs' 'src/ui/*'"
        
        if [ -f "coverage/index.html" ]; then
            echo -e "${GREEN}Coverage report generated: coverage/index.html${NC}"
        fi
    else
        echo -e "${YELLOW}cargo-tarpaulin not installed. Install with:${NC}"
        echo -e "${YELLOW}  cargo install cargo-tarpaulin${NC}"
        ((TESTS_FAILED++))
        ((TESTS_RUN++))
    fi
else
    echo -e "${YELLOW}[5/5] Coverage report skipped (use -c to enable)${NC}"
fi

echo ""

# Benchmarks (optional)
if [ "$BENCH" = true ]; then
    echo -e "${BLUE}Running Benchmarks...${NC}"
    echo ""
    cargo bench
    echo ""
fi

# Summary
echo -e "${BLUE}================================${NC}"
echo -e "${BLUE}  Test Summary${NC}"
echo -e "${BLUE}================================${NC}"
echo -e "Total test sections: ${TESTS_RUN}"
echo -e "${GREEN}Passed: ${TESTS_PASSED}${NC}"
echo -e "${RED}Failed: ${TESTS_FAILED}${NC}"
echo ""

# Final result
if [ $TESTS_FAILED -eq 0 ]; then
    echo -e "${GREEN}All tests passed! ✨${NC}"
    exit 0
else
    echo -e "${RED}Some tests failed. Please review the output above.${NC}"
    exit 1
fi
