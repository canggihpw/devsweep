# Coverage Improvement Quick Checklist

## 🎯 Goal: 30% → 60-70% Coverage

## Phase 1: Quick Wins (Target: ~50% coverage)

### Backend Tests (`tests/test_backend.rs`)
- [ ] `test_execute_cleanup_with_real_files` - Test actual file cleanup
- [ ] `test_execute_cleanup_permission_denied` - Test error handling
- [ ] `test_execute_cleanup_nonexistent_file` - Test missing file handling
- [ ] `test_invalidate_cache_specific_items` - Test selective cache invalidation
- [ ] `test_invalidate_cache_all_items` - Test full cache clear
- [ ] `test_scan_empty_environment` - Test scan with no caches found

### Scan Cache Tests (`tests/test_scan_cache.rs`)
- [ ] `test_cache_ttl_expiration` - Test TTL expiry logic
- [ ] `test_needs_rescan_after_ttl` - Test rescan trigger
- [ ] `test_metadata_change_detection` - Test file size/time changes
- [ ] `test_multiple_categories_independent` - Test category isolation
- [ ] `test_cache_invalidation_preserves_others` - Test partial invalidation

### Cleanup History Tests (`tests/test_cleanup_history.rs`)
- [ ] `test_restore_single_item` - Test restore from quarantine
- [ ] `test_restore_all_items` - Test bulk restore
- [ ] `test_permanent_delete_item` - Test permanent deletion
- [ ] `test_permanent_delete_all` - Test delete all quarantine
- [ ] `test_quarantine_auto_cleanup_10gb` - Test size limit enforcement
- [ ] `test_history_record_persistence` - Test history save/load with data

**Expected Coverage: ~50%**

---

## Phase 2: Checker Coverage (Target: ~65% coverage)

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

**Expected Coverage: ~65%**

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
- [ ] 40% - Initial improvements
- [ ] 50% - Phase 1 complete
- [ ] 60% - Phase 2 halfway
- [ ] 65% - Phase 2 complete
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

**Current Status**: 30% coverage, 80 tests passing
**Target Status**: 65% coverage, ~120 tests passing

Let's do this! 💪