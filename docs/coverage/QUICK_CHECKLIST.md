# Coverage Improvement Quick Checklist

## 🎯 Goal: 30% → 60-70% Coverage

## Phase 1: Quick Wins ✅ COMPLETED (Coverage: 34.76%)

### Backend Tests (`tests/test_backend.rs`) - 28 tests ✅
- [x] `test_execute_cleanup_with_real_files` - Test actual file cleanup
- [x] `test_execute_cleanup_with_directory` - Test directory cleanup
- [x] `test_execute_cleanup_nonexistent_file` - Test missing file handling
- [x] `test_cache_usage_in_sequential_scans` - Test cache behavior
- [x] `test_scan_empty_environment` - Test scan with no caches found
- [x] `test_multiple_sequential_cleanups` - Test sequential operations
- [x] `test_backend_handles_zero_size_items` - Test zero-byte files
- [x] `test_category_data_with_large_item_count` - Test 100 items
- [x] Plus 20 more comprehensive tests

### Scan Cache Tests (`tests/test_scan_cache.rs`) - 29 tests ✅
- [x] `test_cache_ttl_expiration_short_duration` - Test TTL expiry with sleep
- [x] `test_cache_ttl_not_expired_within_window` - Test within TTL
- [x] `test_multiple_categories_independent_caching` - Test category isolation
- [x] `test_cache_with_zero_ttl` - Test immediate expiration
- [x] `test_cache_with_very_large_ttl` - Test long-term caching
- [x] `test_cache_handles_many_categories` - Test 100 categories
- [x] `test_path_tracker_track_file` - Test file tracking
- [x] Plus 22 more comprehensive tests

### Cleanup History Tests (`tests/test_cleanup_history.rs`) - 22 tests ✅
- [x] `test_restore_from_quarantine` - Test restore workflow
- [x] `test_restore_multiple_items` - Test multiple file restoration
- [x] `test_permanent_delete_from_quarantine` - Test permanent deletion
- [x] `test_quarantine_directory` - Test directory quarantine
- [x] `test_quarantine_preserves_file_content` - Test content integrity
- [x] `test_quarantine_with_unicode_filename` - Test unicode support
- [x] Plus 16 more comprehensive tests

### Node.js Checker Tests (`tests/test_nodejs_checker.rs`) - 17 tests ✅ NEW
- [x] `test_npm_cache_detection_with_real_cache` - npm _cacache structure
- [x] `test_yarn_cache_detection` - Yarn cache files
- [x] `test_pnpm_store_detection` - pnpm v3 store
- [x] `test_node_modules_detection` - Basic structure
- [x] `test_scoped_packages` - @babel/core packages
- [x] `test_large_node_modules_structure` - 50 packages
- [x] Plus 11 more comprehensive tests

**Actual Coverage: 34.76% (+4.76%)**
**Total Tests: 138 (was 80)**
**Status**: ✅ All tests passing, pushed to GitHub

---

## Phase 2: Checker Coverage (Target: 50%+ coverage) - NEXT

### Create New Test Files

#### `tests/test_nodejs_checker.rs`
- [ ] `test_npm_cache_detection_with_files` - Mock npm cache structure
- [ ] `test_yarn_cache_detection_with_files` - Mock yarn cache
- [ ] `test_node_modules_detection` - Mock node_modules
- [ ] `test_pnpm_cache_detection` - Mock pnpm store
- [ ] `test_global_modules_detection` - Mock global packages

#### `tests/test_docker_checker.rs`
- [ ] `test_container_cache_detection` - Mock Docker cache
- [ ] `test_image_size_calculation` - Test size reporting
- [ ] `test_build_cache_detection` - Mock build cache

#### `tests/test_python_checker.rs`
- [ ] `test_pycache_detection` - Mock __pycache__ dirs
- [ ] `test_pip_cache_detection` - Mock pip cache
- [ ] `test_virtualenv_detection` - Mock venv

#### `tests/test_rust_checker.rs`
- [ ] `test_cargo_registry_detection` - Mock cargo cache
- [ ] `test_target_dir_detection` - Mock target dirs
- [ ] `test_build_cache_detection` - Mock build cache

#### `tests/test_homebrew_checker.rs`
- [ ] `test_brew_cache_detection` - Mock Homebrew cache
- [ ] `test_old_version_detection` - Mock old packages

#### `tests/test_xcode_checker.rs`
- [ ] `test_derived_data_detection` - Mock DerivedData
- [ ] `test_archives_detection` - Mock Xcode archives
- [ ] `test_ios_support_detection` - Mock device support

**Expected Coverage: 50-55%**

---

## Phase 3: Edge Cases (Target: ~70%+ coverage)

### Create `tests/test_edge_cases.rs`
- [ ] `test_symlink_handling` - Test symlink detection
- [ ] `test_permission_denied_graceful` - Test permission errors
- [ ] `test_unicode_filenames` - Test emoji/Chinese chars
- [ ] `test_very_long_paths` - Test deep nesting
- [ ] `test_empty_directories` - Test empty cache dirs
- [ ] `test_large_files` - Test >1GB files
- [ ] `test_readonly_files` - Test read-only handling

### Create `tests/test_persistence.rs`
- [ ] `test_corrupted_cache_recovery` - Test invalid JSON recovery
- [ ] `test_corrupted_history_recovery` - Test history recovery
- [ ] `test_missing_cache_file` - Test cache file missing
- [ ] `test_partial_write_recovery` - Test interrupted writes

**Expected Coverage: ~70-75%**

---

## 📊 Progress Tracking

After each test batch, run:
```bash
# Check coverage
cargo llvm-cov --html --open

# Or text summary
cargo llvm-cov --summary-only
```

### Coverage Milestones
- [x] 30% - Starting point
- [x] 34.76% - Phase 1 complete ✅
- [ ] 40% - Early Phase 2
- [ ] 50% - Phase 2 complete
- [ ] 60% - Phase 3 halfway
- [ ] 70% - Phase 3 complete ✨

---

## 🚀 Quick Start Commands

```bash
# Start improvement branch
git checkout -b improve-test-coverage

# Run tests with coverage
cargo llvm-cov --html --open

# Run specific test file
cargo test --test test_backend

# Check coverage of specific file
cargo llvm-cov --text | grep backend.rs

# Commit progress
git add tests/
git commit -m "test: improve coverage to X%"
```

---

## 💡 Test Template

```rust
use devsweep::*;
use tempfile::TempDir;
use std::fs;

#[test]
fn test_your_feature() {
    // Arrange - Set up test data
    let temp = TempDir::new().unwrap();
    let test_file = temp.path().join("test.txt");
    fs::write(&test_file, b"data").unwrap();
    
    // Act - Execute the code
    let result = your_function(&test_file);
    
    // Assert - Verify results
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), expected_value);
}
```

---

## ✅ Daily Goals

### Day 1: Quick Wins
- [ ] Complete Backend tests (6 tests)
- [ ] Complete Scan Cache tests (5 tests)
- Target: ~45% coverage

### Day 2: More Quick Wins + Checkers
- [ ] Complete Cleanup History tests (6 tests)
- [ ] Start Node.js checker tests (3 tests)
- Target: ~55% coverage

### Day 3: Checker Coverage
- [ ] Complete Node.js tests (2 more)
- [ ] Add Docker, Python tests (6 tests)
- Target: ~62% coverage

### Day 4: More Checkers
- [ ] Add Rust, Homebrew, Xcode tests (9 tests)
- Target: ~68% coverage

### Day 5: Edge Cases (Optional)
- [ ] Add edge case tests (7 tests)
- Target: ~70%+ coverage

---

## 🎉 Success Criteria

- ✅ Coverage reaches 60-70%
- ✅ All tests passing
- ✅ No flaky tests
- ✅ Codecov badge shows correct percentage
- ✅ CI workflows green

---

**Current Status**: 34.76% coverage, 138 tests passing ✅
**Target Status**: 60-70% coverage, ~200 tests passing

**Phase 1**: ✅ COMPLETED - See `docs/coverage/PHASE1_COMPLETION.md` for details

Let's do this! 💪