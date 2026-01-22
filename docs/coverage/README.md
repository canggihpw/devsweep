# Test Coverage Documentation

## Current Status

| Metric | Value |
|--------|-------|
| **Line Coverage** | 57.33% |
| **Region Coverage** | 57.89% |
| **Functions Executed** | 70.59% |
| **Tests** | 332 |
| **Pass Rate** | 100% |

> **Note**: Coverage metrics exclude UI code (`src/app/**`, `src/ui/**`, `src/main.rs`, `src/assets.rs`) which requires GUI testing and cannot be unit tested.

## Coverage by Module

```
Module                      Coverage    Notes
------------------------------------------------------------
utils.rs                    98.55%      Fully tested
checkers/rust_cargo.rs      93.90%      Fully tested
scan_cache.rs               91.48%      Fully tested
checkers/nodejs.rs          82.64%      Well tested
checkers/homebrew.rs        79.64%      Well tested
single_instance.rs          76.19%      Well tested
cleanup_history.rs          68.73%      Good coverage
checkers/general.rs         64.59%      Good coverage
types.rs                    62.63%      Good coverage
checkers/browser.rs         59.36%      Adequate
checkers/java.rs            56.76%      Adequate
checkers/ide.rs             55.13%      Adequate
checkers/shell.rs           52.69%      Adequate
checkers/logs.rs            49.06%      Adequate
backend.rs                  48.05%      Needs improvement
checkers/xcode.rs           45.59%      Needs improvement
checkers/go.rs              41.94%      Needs improvement
cache_settings.rs           41.71%      Needs improvement
checkers/db.rs              34.03%      Low coverage
checkers/python.rs          24.51%      Low coverage
checkers/docker.rs           9.21%      Low coverage
```

## Test Files

| File | Tests | Description |
|------|-------|-------------|
| `test_edge_cases.rs` | 41 | Symlinks, unicode, permissions, long paths |
| `test_persistence.rs` | 35 | Corrupted data recovery, save/load cycles |
| `test_scan_cache.rs` | 29 | Cache TTL, persistence, path tracking |
| `test_backend.rs` | 28 | Backend operations, cleanup execution |
| `test_integration_workflows.rs` | 24 | End-to-end workflows |
| `test_xcode_checker.rs` | 23 | Xcode DerivedData, archives |
| `test_cleanup_history.rs` | 22 | Quarantine, restore, history management |
| `test_python_checker.rs` | 21 | Python cache structures |
| `test_nodejs_checker.rs` | 17 | npm/yarn/pnpm cache structures |
| `test_docker_checker.rs` | 16 | Docker cache structures |
| `test_checkers.rs` | 16 | Checker basic functionality |
| `test_single_instance.rs` | 14 | Unix socket single-instance |
| `test_types.rs` | 13 | Type builders and structures |
| `performance_tests.rs` | 5 | Performance benchmarks |
| `test_cache_settings.rs` | 3 | Cache settings |
| `test_utils.rs` | 2 | Utility functions |

## Running Coverage

```bash
# Generate coverage report (excluding UI code)
cargo llvm-cov --ignore-filename-regex '(src/main\.rs|src/ui/|src/app/|src/assets\.rs|tests/)'

# Generate HTML report
cargo llvm-cov --html --open --ignore-filename-regex '(src/main\.rs|src/ui/|src/app/|src/assets\.rs|tests/)'

# Text summary only
cargo llvm-cov --summary-only

# Run specific test file
cargo test --test test_backend
```

## Codecov Configuration

The project uses Codecov for coverage tracking. UI code is excluded via `codecov.yml`:

```yaml
ignore:
  - "src/main.rs"    # GUI initialization
  - "src/ui/**"      # UI components
  - "src/app/**"     # GPUI app modules
  - "src/assets.rs"  # Asset loading
  - "tests/**"       # Test files
```

## Improving Coverage

### High-Impact Areas

1. **Docker Checker (9.21%)** - Create mock Docker directory structures and call `check_docker()` with test paths

2. **Python Checker (24.51%)** - Test pip cache, virtualenv, and `__pycache__` detection with mock directories

3. **Database Checker (34.03%)** - Test Redis, PostgreSQL, MongoDB cache detection

4. **Go Checker (41.94%)** - Small module, 2-3 tests would significantly improve coverage

### Example Test Pattern

```rust
use devsweep::checkers::docker::check_docker;
use tempfile::TempDir;
use std::fs;

#[test]
fn test_check_docker_with_mock_cache() {
    let temp = TempDir::new().unwrap();
    
    // Create mock Docker directory structure
    let docker_dir = temp.path().join(".docker");
    fs::create_dir_all(docker_dir.join("buildx")).unwrap();
    fs::write(docker_dir.join("buildx/cache.db"), b"data").unwrap();
    
    // Test detection logic
    // ...
}
```

## Test Quality Highlights

- All tests use `TempDir` for isolated file system testing
- Zero hardcoded paths - all paths are generated at runtime
- Comprehensive edge cases: unicode filenames, symlinks, permissions
- Both success and failure paths tested
- Persistence tests cover corrupted data recovery
- Integration tests verify end-to-end workflows
- No flaky tests - all 332 tests pass consistently
