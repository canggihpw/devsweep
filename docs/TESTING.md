# DevSweep Testing Guide

## Overview

This document describes the testing strategy for DevSweep. The project has comprehensive test coverage with 332 tests achieving 57.33% line coverage on testable (non-UI) code.

## Testing Philosophy

1. **Safety First**: All file operations are thoroughly tested to prevent data loss
2. **Isolation**: Tests use `TempDir` for isolated file system operations
3. **No Hardcoded Paths**: All paths generated at runtime
4. **Edge Cases**: Unicode, symlinks, permissions, and error conditions are tested
5. **Automation**: All tests run in CI/CD via GitHub Actions

## Test Structure

```
tests/
├── test_backend.rs              # 28 tests - Core backend operations
├── test_scan_cache.rs           # 29 tests - Cache TTL, persistence
├── test_cleanup_history.rs      # 22 tests - Quarantine, restore
├── test_edge_cases.rs           # 41 tests - Edge cases & error handling
├── test_persistence.rs          # 35 tests - Data recovery
├── test_integration_workflows.rs # 24 tests - End-to-end workflows
├── test_single_instance.rs      # 14 tests - Unix socket handling
├── test_checkers.rs             # 16 tests - Checker functionality
├── test_types.rs                # 13 tests - Type structures
├── test_nodejs_checker.rs       # 17 tests - Node.js cache structures
├── test_docker_checker.rs       # 16 tests - Docker cache structures
├── test_python_checker.rs       # 21 tests - Python cache structures
├── test_xcode_checker.rs        # 23 tests - Xcode cache structures
├── test_cache_settings.rs       #  3 tests - Settings
├── test_utils.rs                #  2 tests - Utilities
└── performance_tests.rs         #  5 tests - Benchmarks
```

## Running Tests

```bash
# Run all tests
cargo test

# Run with output
cargo test -- --nocapture

# Run specific test file
cargo test --test test_backend

# Run specific test
cargo test test_scan_with_cache

# Run tests matching pattern
cargo test quarantine

# Run tests in parallel (default)
cargo test -- --test-threads=4
```

## Coverage Commands

```bash
# Generate coverage report (excluding UI)
cargo llvm-cov --ignore-filename-regex '(src/main\.rs|src/ui/|src/app/|src/assets\.rs|tests/)'

# HTML report
cargo llvm-cov --html --open

# Summary only
cargo llvm-cov --summary-only
```

## Test Categories

### 1. Unit Tests

Test individual components in isolation.

#### Backend (`test_backend.rs`)
- [x] Category data creation and cloning
- [x] Execute cleanup with real temp files
- [x] Execute cleanup with directories
- [x] Handle invalid/nonexistent paths
- [x] Get items for category
- [x] Quarantine records retrieval
- [x] Cache usage in sequential scans
- [x] Rescan consistency

#### Cache Settings (`test_cache_settings.rs`)
- [x] TTL formatting
- [x] Preset configurations
- [x] Trash settings validation

#### Cleanup History (`test_cleanup_history.rs`)
- [x] Record creation
- [x] History save/load
- [x] Quarantine operations
- [x] Restore from quarantine
- [x] Unicode filename handling
- [x] Directory quarantine

#### Scan Cache (`test_scan_cache.rs`)
- [x] Metadata change detection
- [x] Cache save/load
- [x] TTL expiration
- [x] Path tracking
- [x] Concurrent updates

#### Single Instance (`test_single_instance.rs`)
- [x] Socket path generation
- [x] Socket communication protocol
- [x] Connection timeout behavior
- [x] Stale socket cleanup

#### Types (`test_types.rs`)
- [x] CheckResult creation
- [x] CleanupItem builder pattern
- [x] ItemDetail structures

#### Utils (`test_utils.rs`)
- [x] Size formatting
- [x] Version sorting

### 2. Checker Tests

Each checker has dedicated tests for cache structure detection.

#### Node.js (`test_nodejs_checker.rs`)
- [x] node_modules detection
- [x] npm cache structures
- [x] yarn cache detection
- [x] pnpm store detection
- [x] Scoped packages
- [x] Large structure handling

#### Docker (`test_docker_checker.rs`)
- [x] Build cache structure
- [x] Image cache structure
- [x] Container logs
- [x] Volumes structure
- [x] Overlay2 structure

#### Python (`test_python_checker.rs`)
- [x] `__pycache__` detection
- [x] pip cache structures
- [x] virtualenv detection
- [x] pytest/mypy cache
- [x] Jupyter/IPython cache

#### Xcode (`test_xcode_checker.rs`)
- [x] DerivedData detection
- [x] Archives detection
- [x] Device support
- [x] Module cache
- [x] Simulator devices

### 3. Edge Case Tests (`test_edge_cases.rs`)

- [x] Symlink handling (circular, broken)
- [x] Permission denied scenarios
- [x] Unicode filenames (emoji, CJK, Arabic)
- [x] Long paths (>255 chars)
- [x] Empty directories
- [x] Zero-size files
- [x] Large file metadata
- [x] Special characters in paths
- [x] Deeply nested directories

### 4. Persistence Tests (`test_persistence.rs`)

- [x] Corrupted JSON recovery
- [x] Truncated file handling
- [x] Empty file recovery
- [x] Missing directory creation
- [x] Binary garbage in files
- [x] Wrong JSON structure handling
- [x] Unicode in paths/names
- [x] Save/load roundtrip

### 5. Integration Tests (`test_integration_workflows.rs`)

- [x] Full scan workflow
- [x] Scan with cache
- [x] Quarantine and restore workflow
- [x] Cleanup history tracking
- [x] Multiple cleanup sessions
- [x] Cache invalidation
- [x] Concurrent access safety
- [x] Error recovery

### 6. Performance Tests (`performance_tests.rs`)

- [x] Scan performance targets
- [x] Cached scan performance
- [x] Large directory handling
- [x] Parallel scanning efficiency
- [x] Memory usage

## Test Patterns

### Using TempDir for Isolation

```rust
use tempfile::TempDir;
use std::fs;

#[test]
fn test_example() {
    let temp = TempDir::new().unwrap();
    let test_file = temp.path().join("test.txt");
    fs::write(&test_file, "content").unwrap();
    
    // Test logic here
    
    // TempDir automatically cleaned up when dropped
}
```

### Testing File Operations

```rust
#[test]
fn test_quarantine_file() {
    let temp = TempDir::new().unwrap();
    let file_path = temp.path().join("to_quarantine.txt");
    fs::write(&file_path, "test content").unwrap();
    
    let mut history = CleanupHistory::default();
    history.quarantine_item(&file_path).unwrap();
    
    assert!(!file_path.exists());
    // File now in quarantine
}
```

### Testing Error Conditions

```rust
#[test]
fn test_corrupted_json_recovery() {
    let temp = TempDir::new().unwrap();
    let cache_file = temp.path().join("cache.json");
    fs::write(&cache_file, "{ invalid json }}}").unwrap();
    
    let cache = ScanCache::load_from(&cache_file);
    // Should return default, not panic
    assert!(cache.categories.is_empty());
}
```

## CI/CD Integration

Tests run automatically on pull requests via GitHub Actions:

```yaml
# .github/workflows/ci.yml
- name: Run tests
  run: cargo test --all-features

- name: Generate coverage
  run: cargo llvm-cov --lcov --output-path lcov.info

- name: Upload to Codecov
  uses: codecov/codecov-action@v4
```

## Coverage Exclusions

UI code is excluded from coverage as it requires GUI testing:

- `src/main.rs` - Application entry point
- `src/app/**` - GPUI app modules
- `src/ui/**` - UI components
- `src/assets.rs` - Asset loading

See `codecov.yml` for full configuration.

## Adding New Tests

1. Create test file in `tests/` directory
2. Follow naming convention: `test_<module>.rs`
3. Use `TempDir` for file operations
4. Test both success and error paths
5. Run `cargo test` to verify
6. Check coverage with `cargo llvm-cov`

## Current Coverage Status

| Metric | Value |
|--------|-------|
| Line Coverage | 57.33% |
| Tests | 332 |
| Pass Rate | 100% |

See [coverage/README.md](coverage/README.md) for detailed breakdown.
