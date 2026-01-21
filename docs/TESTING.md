# DevSweep Testing Guide

## Overview

This document outlines the comprehensive testing strategy for DevSweep, ensuring code quality, reliability, and safety.

## Testing Philosophy

1. **Safety First**: Test quarantine and deletion operations thoroughly
2. **Fast Feedback**: Unit tests should run quickly
3. **Comprehensive Coverage**: Test all critical paths
4. **Real-world Scenarios**: Integration tests mimic actual usage
5. **Performance Validation**: Ensure speed targets are met

## Test Categories

### 1. Unit Tests
Test individual functions and modules in isolation.

**Coverage Areas**:
- ✅ Cache settings (TTL, presets, formatting)
- ✅ Cleanup history (records, persistence)
- ✅ Scan cache (metadata tracking, invalidation)
- ✅ Utility functions (formatting, sorting)
- 🔄 Individual checkers (browser, docker, nodejs, etc.)
- 🔄 Backend operations (scanning, quarantine)

### 2. Integration Tests
Test component interactions and workflows.

**Coverage Areas**:
- 🔄 Scan with cache workflow
- 🔄 Quarantine and restore operations
- 🔄 History tracking during cleanup
- 🔄 Multi-category scanning
- 🔄 Settings persistence across operations

### 3. Functional Tests
Test complete user workflows end-to-end.

**Coverage Areas**:
- 🔄 Complete scan → select → quarantine → restore flow
- 🔄 Settings modification and persistence
- 🔄 Cache invalidation scenarios
- 🔄 Error recovery

### 4. Performance Tests
Validate performance targets.

**Targets**:
- Scan with cache: < 5 seconds
- Full scan: < 30 seconds
- Memory usage: < 100MB

### 5. Safety Tests
Ensure data safety and error handling.

**Coverage Areas**:
- 🔄 Quarantine rollback functionality
- 🔄 File locking mechanisms
- 🔄 Permission errors
- 🔄 Invalid path handling

## Running Tests

### Quick Test
```bash
cargo test
```

### Verbose Output
```bash
cargo test -- --nocapture
```

### Specific Test
```bash
cargo test test_name
```

### With Coverage (requires cargo-tarpaulin)
```bash
cargo tarpaulin --out Html --output-dir coverage
```

### Run All Tests with Script
```bash
./scripts/run-tests.sh
```

## Test Structure

### Unit Test Location
Unit tests are placed in the same file as the code they test, in a `tests` module:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_something() {
        // Test implementation
    }
}
```

### Integration Test Location
Integration tests go in `tests/` directory (to be created):
- `tests/integration/`
- `tests/functional/`
- `tests/performance/`

## Test Data

### Mock Directories
Tests use temporary directories created with `tempfile` crate.

### Test Fixtures
- Sample cache directories
- Mock configuration files
- Predefined scan results

## Writing New Tests

### Guidelines
1. **Descriptive Names**: `test_quarantine_restores_all_files`
2. **Arrange-Act-Assert**: Clear test structure
3. **Isolated**: Tests don't depend on each other
4. **Deterministic**: Same input = same output
5. **Clean Up**: Remove temporary files

### Example
```rust
#[test]
fn test_format_size_kilobytes() {
    let size = 1024;
    let formatted = format_size(size);
    assert!(formatted.contains("Ki") || formatted.contains("KB"));
}
```

## Continuous Integration

Tests run automatically on:
- Every pull request
- Main branch commits
- Release tags

## Coverage Goals

- **Overall**: 70%+ code coverage
- **Critical paths**: 90%+ coverage
  - Quarantine operations
  - File deletion
  - Cache invalidation
- **Checkers**: 60%+ coverage (due to file system dependencies)

## Testing Tools

### Required
- `cargo test` - Built-in test runner
- `tempfile` - Temporary directories for tests

### Optional
- `cargo-tarpaulin` - Code coverage
- `cargo-nextest` - Faster test runner
- `mockall` - Mocking framework (if needed)

## Performance Benchmarking

### Micro-benchmarks
Use `criterion` crate for performance-critical functions:
```rust
#[bench]
fn bench_scan_performance(b: &mut Bencher) {
    b.iter(|| scan_directory(test_path));
}
```

### Real-world Benchmarks
Track scanning performance on actual development machines.

## Known Testing Challenges

### File System Operations
- Require real directories (harder to mock)
- Platform-specific (macOS-only)
- Permissions may vary

**Solution**: Use temporary directories and skip tests if permissions denied.

### GUI Tests
- GPUI framework doesn't have built-in testing
- Requires manual testing or screenshot comparison

**Solution**: Focus on backend logic testing; manual UI testing.

### Parallel Scanning
- Rayon makes timing non-deterministic
- Hard to test exact ordering

**Solution**: Test outcomes, not exact sequence.

## Testing Checklist

Before releasing:
- [ ] All unit tests pass
- [ ] Integration tests pass
- [ ] No regression in performance benchmarks
- [ ] Manual testing on clean macOS installation
- [ ] Test with and without Full Disk Access
- [ ] Verify quarantine system works
- [ ] Test with large directories (> 100GB scan)
- [ ] Verify cache invalidation works correctly
- [ ] Test theme switching
- [ ] Test single-instance behavior

## Future Testing Improvements

- [ ] Automated UI testing framework
- [ ] Property-based testing with `proptest`
- [ ] Fuzzing for parser functions
- [ ] Snapshot testing for UI components
- [ ] Load testing with extreme directory sizes
- [ ] Memory leak detection
- [ ] Cross-version compatibility tests

## Resources

- [Rust Testing Guide](https://doc.rust-lang.org/book/ch11-00-testing.html)
- [GPUI Testing](https://github.com/zed-industries/zed/tree/main/crates/gpui)
- [Cargo Nextest](https://nexte.st/)
- [Tarpaulin Coverage](https://github.com/xd009642/tarpaulin)

---

**Remember**: Good tests are the foundation of reliable software. Write tests for every new feature! 🧪
