# Test Coverage Documentation

## Current Status

| Metric | Value |
|--------|-------|
| **Coverage** | 35.87% |
| **Tests** | 332 |
| **Pass Rate** | 100% |

## Coverage by Module

```
Module                      Coverage    Notes
────────────────────────────────────────────────────────
utils.rs                    97.17%      Fully tested
checkers/rust_cargo.rs      93.89%      Fully tested
scan_cache.rs               92.38%      Fully tested
checkers/homebrew.rs        81.01%      Well tested
checkers/nodejs.rs          79.69%      Well tested
single_instance.rs          77.14%      Well tested
checkers/general.rs         67.10%      Good coverage
cleanup_history.rs          64.76%      Good coverage
cache_settings.rs           63.47%      Good coverage
types.rs                    60.38%      Good coverage
checkers/browser.rs         54.55%      Adequate
checkers/ide.rs             55.10%      Adequate
checkers/logs.rs            53.29%      Adequate
checkers/java.rs            50.70%      Adequate
checkers/shell.rs           50.00%      Adequate
backend.rs                  46.86%      Needs improvement
checkers/xcode.rs           38.17%      Needs improvement
checkers/go.rs              37.29%      Needs improvement
checkers/db.rs              36.45%      Needs improvement
checkers/python.rs          23.20%      Low coverage
checkers/docker.rs          11.76%      Low coverage
```

**UI modules (0% coverage)**: `app/*`, `ui/*`, `main.rs`, `assets.rs` - These are excluded from coverage targets as they require GUI testing.

## Test Files

| File | Tests | Description |
|------|-------|-------------|
| `test_backend.rs` | 28 | Backend operations, cleanup execution |
| `test_scan_cache.rs` | 29 | Cache TTL, persistence, path tracking |
| `test_cleanup_history.rs` | 22 | Quarantine, restore, history management |
| `test_nodejs_checker.rs` | 17 | npm/yarn/pnpm cache structures |
| `test_docker_checker.rs` | 16 | Docker cache structures |
| `test_python_checker.rs` | 21 | Python cache structures |
| `test_xcode_checker.rs` | 23 | Xcode DerivedData, archives |
| `test_edge_cases.rs` | 41 | Symlinks, unicode, permissions |
| `test_persistence.rs` | 35 | Corrupted data recovery |
| `test_integration_workflows.rs` | 24 | End-to-end workflows |
| `test_single_instance.rs` | 14 | Unix socket single-instance |
| `test_checkers.rs` | 16 | Checker basic functionality |
| `test_types.rs` | 13 | Type builders and structures |
| `test_cache_settings.rs` | 3 | Cache settings |
| `test_utils.rs` | 2 | Utility functions |
| `performance_tests.rs` | 5 | Performance benchmarks |

## How to Improve Coverage to 50%+

The current coverage is 35.21%. To reach 50%, focus on these high-impact areas:

### 1. Checker Functions with Mock Paths (Expected: +8-12%)

The checker modules have structure tests but don't call the actual checker functions with mock directories. Adding these tests would significantly improve coverage.

**Target modules:**
- `checkers/docker.rs` (11.76% -> 60%+)
- `checkers/python.rs` (23.20% -> 60%+)
- `checkers/db.rs` (36.45% -> 60%+)

**Example approach:**
```rust
use devsweep::checkers::docker::check_docker;
use tempfile::TempDir;

#[test]
fn test_check_docker_with_mock_cache() {
    let temp = TempDir::new().unwrap();
    
    // Create mock Docker directory structure
    let docker_dir = temp.path().join(".docker");
    fs::create_dir_all(docker_dir.join("buildx")).unwrap();
    fs::write(docker_dir.join("buildx/cache.db"), b"data").unwrap();
    
    // Set HOME to temp dir and call checker
    // Verify results contain expected items
}
```

**Why this helps:** The checker tests currently create directory structures but don't call the actual `check_*` functions because they look at system paths. Using environment variable overrides or path injection would test the real detection logic.

### 2. Backend Parallel Scanning (Expected: +3-5%)

The `backend.rs` module has untested parallel scanning code paths.

**Target functions:**
- `scan_categories_parallel()` - parallel execution paths
- Error handling in parallel scans
- Category aggregation logic

**Example:**
```rust
#[test]
fn test_parallel_scan_with_failures() {
    // Mock a checker that fails
    // Verify other checkers still complete
    // Verify error is captured but doesn't crash
}
```

### 3. Database Checker (Expected: +2-3%)

`checkers/db.rs` at 36.45% has several untested paths for Redis, PostgreSQL, and MongoDB detection.

**Create `tests/test_db_checker.rs`:**
```rust
#[test]
fn test_redis_dump_detection() {
    let temp = TempDir::new().unwrap();
    fs::write(temp.path().join("dump.rdb"), b"REDIS...").unwrap();
    // Test detection
}

#[test]
fn test_postgres_data_detection() {
    let temp = TempDir::new().unwrap();
    let pg_data = temp.path().join("postgres/data");
    fs::create_dir_all(&pg_data).unwrap();
    // Test detection
}
```

### 4. Xcode Checker Improvements (Expected: +2-3%)

`checkers/xcode.rs` at 38.17% - the existing structure tests don't exercise the actual detection code.

**Improvements needed:**
- Call `check_xcode()` with mocked paths
- Test simulator device detection
- Test archive parsing

### 5. Go Checker (Expected: +1-2%)

`checkers/go.rs` at 37.29% is a small module. Adding 2-3 tests would bring it to 70%+.

```rust
#[test]
fn test_go_module_cache_detection() {
    let temp = TempDir::new().unwrap();
    let go_path = temp.path().join("go/pkg/mod");
    fs::create_dir_all(&go_path).unwrap();
    // Add mock module cache files
}
```

## Coverage Commands

```bash
# Generate HTML report
cargo llvm-cov --html --open

# Text summary
cargo llvm-cov --summary-only

# Check specific file
cargo llvm-cov --text | grep backend.rs

# Run specific test file
cargo test --test test_backend
```

## Why Coverage is Limited at ~35%

1. **UI Code (0%)**: The `app/` and `ui/` modules are GUI code that requires integration testing with a display, which is excluded from unit test coverage.

2. **System-Dependent Checkers**: Many checkers look at system paths like `~/.docker`, `~/.cargo`, `/usr/local/Cellar`. Testing these requires either:
   - Environment variable overrides (not always supported)
   - Path injection (requires code changes)
   - Integration tests with real system state

3. **main.rs (0%)**: The application entry point requires running the full GUI.

## Realistic Coverage Targets

| Target | Achievable? | Effort |
|--------|-------------|--------|
| 40% | Yes | Low - Add checker function tests |
| 50% | Yes | Medium - Add DB, Go, Xcode tests |
| 60% | Difficult | High - Requires code refactoring for testability |
| 70%+ | Not practical | Would need to exclude UI from total |

## Test Quality Highlights

- 100+ tests use real file system (TempDir)
- Zero hardcoded paths
- Comprehensive edge cases: unicode, symlinks, permissions
- Both success and failure paths tested
- No flaky tests
