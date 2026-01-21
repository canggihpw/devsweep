# Test Coverage Improvement Plan
**Target: Increase from ~30% to 60-70%**

## Current Status

- **Current Coverage**: ~30%
- **Target Coverage**: 60-70%
- **Tests Passing**: 80/80 (100%)
- **Main Gap**: Basic "smoke tests" that verify functions return, but don't test actual logic

## Coverage Analysis by Module

### Well-Covered Modules ✅ (Keep as-is)
- `types.rs` (~90%) - Comprehensive builder pattern tests
- `utils.rs` (~90%) - Good coverage of helper functions

### Needs Improvement 🔧

| Module | Current | Target | Priority | Effort |
|--------|---------|--------|----------|--------|
| `backend.rs` | ~30% | 70% | HIGH | Medium |
| `checkers/*.rs` | ~20% | 60% | HIGH | High |
| `cleanup_history.rs` | ~40% | 70% | HIGH | Medium |
| `scan_cache.rs` | ~35% | 65% | MEDIUM | Low |
| `cache_settings.rs` | ~80% | 80% | LOW | - |

## Implementation Plan

### Phase 1: Quick Wins (Est. +15-20% coverage)
**Timeline: 1-2 days**

#### 1.1 Backend Module Tests
**File**: `tests/test_backend.rs`

Add tests for:
- [ ] `invalidate_cache_for_items()` - Test cache invalidation logic
  ```rust
  #[test]
  fn test_invalidate_cache_specific_items() {
      // Create backend with cached data
      // Invalidate specific items
      // Verify only those items are invalidated
      // Verify other cache remains
  }
  ```

- [ ] `execute_cleanup()` with real temp files
  ```rust
  #[test]
  fn test_execute_cleanup_success() {
      // Create temp files
      // Execute cleanup
      // Verify files moved to quarantine
      // Verify cleanup record created
  }
  ```

- [ ] `execute_cleanup()` error scenarios
  ```rust
  #[test]
  fn test_execute_cleanup_readonly_file() {
      // Create read-only file
      // Attempt cleanup
      // Verify error handling
  }
  
  #[test]
  fn test_execute_cleanup_nonexistent_file() {
      // Try to clean file that doesn't exist
      // Verify graceful error handling
  }
  ```

- [ ] Parallel scanning edge cases
  ```rust
  #[test]
  fn test_scan_empty_categories() {
      // Mock environment with no caches
      // Verify returns empty results gracefully
  }
  ```

**Expected gain**: +8-10%

#### 1.2 Scan Cache Tests
**File**: `tests/test_scan_cache.rs` (expand existing)

Add tests for:
- [ ] TTL expiration logic
  ```rust
  #[test]
  fn test_cache_expires_after_ttl() {
      // Set short TTL
      // Add cache entry
      // Wait for expiration
      // Verify needs_rescan returns true
  }
  ```

- [ ] Cache invalidation on metadata change
  ```rust
  #[test]
  fn test_invalidate_on_size_change() {
      // Cache file metadata
      // Modify file size
      // Verify needs_rescan detects change
  }
  ```

- [ ] Multiple category caching
  ```rust
  #[test]
  fn test_multiple_categories_independent() {
      // Cache multiple categories
      // Invalidate one
      // Verify others unaffected
  }
  ```

**Expected gain**: +3-5%

#### 1.3 Cleanup History Tests
**File**: `tests/test_cleanup_history.rs` (expand existing)

Add tests for:
- [ ] Undo/restore operations
  ```rust
  #[test]
  fn test_restore_from_quarantine() {
      // Quarantine file
      // Restore it
      // Verify file back in original location
      // Verify record updated
  }
  ```

- [ ] Permanent deletion
  ```rust
  #[test]
  fn test_permanent_delete_from_quarantine() {
      // Quarantine file
      // Delete permanently
      // Verify file gone from quarantine
      // Verify record updated
  }
  ```

- [ ] History size limits
  ```rust
  #[test]
  fn test_quarantine_auto_cleanup_over_10gb() {
      // Mock quarantine over 10GB
      // Verify automatic cleanup triggers
      // Verify old records removed first
  }
  ```

**Expected gain**: +4-6%

---

### Phase 2: Checker Module Coverage (Est. +15-20% coverage)
**Timeline: 2-3 days**

Current checker tests only verify functions return results. We need to test actual detection logic.

#### 2.1 Node.js Checker (Complete Example)
**File**: `tests/test_nodejs_checker.rs` (new file)

```rust
use devsweep::checkers::nodejs::*;
use std::fs;
use tempfile::TempDir;

#[test]
fn test_npm_cache_detection_with_real_cache() {
    let temp = TempDir::new().unwrap();
    let npm_cache = temp.path().join(".npm");
    fs::create_dir_all(&npm_cache).unwrap();
    
    // Create mock cache files
    fs::write(npm_cache.join("package1.tar"), b"data").unwrap();
    fs::write(npm_cache.join("package2.tar"), b"data").unwrap();
    
    // Test detection
    let result = check_npm_cache(&npm_cache);
    
    assert!(result.items.len() > 0);
    assert!(result.total_size > 0);
}

#[test]
fn test_node_modules_detection() {
    let temp = TempDir::new().unwrap();
    let project = temp.path().join("my-project");
    let node_modules = project.join("node_modules");
    fs::create_dir_all(&node_modules).unwrap();
    
    // Create mock node_modules structure
    fs::create_dir_all(node_modules.join("package1")).unwrap();
    fs::write(node_modules.join("package1/index.js"), b"code").unwrap();
    
    // Test detection
    // ...
}
```

#### 2.2 Apply Same Pattern to Other Checkers
For each checker module, add 2-3 focused tests:

- [ ] **Docker** (`checkers/docker.rs`)
  - Test container cache detection with mock data
  - Test image size calculation
  - Test "safe to delete" flag logic

- [ ] **Python** (`checkers/python.rs`)
  - Test `__pycache__` detection in temp dirs
  - Test pip cache detection
  - Test virtualenv detection

- [ ] **Rust/Cargo** (`checkers/rust_cargo.rs`)
  - Test cargo registry cache detection
  - Test target directory detection
  - Test size calculation for large caches

- [ ] **Homebrew** (`checkers/homebrew.rs`)
  - Test brew cache detection
  - Test old version cleanup detection

- [ ] **Xcode** (`checkers/xcode.rs`)
  - Test DerivedData detection
  - Test archive detection
  - Test iOS device support detection

**Expected gain per checker**: ~2-3%
**Total expected gain**: +15-20%

---

### Phase 3: Edge Cases & Error Handling (Est. +10-15% coverage)
**Timeline: 1-2 days**

#### 3.1 File System Edge Cases
**File**: `tests/test_edge_cases.rs` (new file)

```rust
#[test]
fn test_symlink_handling() {
    // Create symlink to directory
    // Verify detection doesn't follow symlinks infinitely
    // Verify size calculation correct
}

#[test]
fn test_permission_denied_handling() {
    // Create directory without read permissions
    // Verify graceful handling
    // Verify error reported, not crash
}

#[test]
fn test_unicode_filenames() {
    // Create files with emoji, Chinese chars, etc.
    // Verify detection works
    // Verify display correct
}

#[test]
fn test_very_long_paths() {
    // Create deeply nested directory structure
    // Verify no path length errors
}

#[test]
fn test_empty_directories() {
    // Create empty cache directories
    // Verify reported as 0 bytes
    // Verify still detected
}
```

**Expected gain**: +5-8%

#### 3.2 Concurrent Operations
**File**: `tests/test_concurrency.rs` (new file)

```rust
#[test]
fn test_concurrent_scans() {
    // Run multiple scans in parallel
    // Verify no race conditions
    // Verify results consistent
}

#[test]
fn test_scan_during_cleanup() {
    // Start scan
    // Start cleanup in parallel
    // Verify both complete successfully
}
```

**Expected gain**: +2-3%

#### 3.3 Data Persistence & Recovery
**File**: `tests/test_persistence.rs` (new file)

```rust
#[test]
fn test_corrupted_cache_recovery() {
    // Write invalid JSON to cache file
    // Attempt to load
    // Verify graceful fallback to empty cache
}

#[test]
fn test_corrupted_history_recovery() {
    // Write invalid history data
    // Attempt to load
    // Verify recovery to empty history
}

#[test]
fn test_partial_write_recovery() {
    // Simulate interrupted write
    // Verify recovery mechanism
}
```

**Expected gain**: +3-4%

---

### Phase 4: Integration & Workflow Tests (Est. +5-10% coverage)
**Timeline: 1 day**

#### 4.1 Full Workflow Tests
**File**: `tests/integration/test_full_workflows.rs`

```rust
#[test]
fn test_scan_quarantine_restore_workflow() {
    // 1. Full scan
    // 2. Select items
    // 3. Quarantine
    // 4. Verify quarantined
    // 5. Restore
    // 6. Verify restored
}

#[test]
fn test_scan_cache_rescan_workflow() {
    // 1. Initial scan (builds cache)
    // 2. Verify cache used on second scan
    // 3. Modify filesystem
    // 4. Verify invalidation detected
    // 5. Rescan rebuilds cache
}

#[test]
fn test_multiple_cleanup_sessions() {
    // 1. Cleanup batch 1
    // 2. Cleanup batch 2
    // 3. Verify both in history
    // 4. Undo batch 1
    // 5. Verify batch 2 unaffected
}
```

**Expected gain**: +5-10%

---

## Summary Timeline

| Phase | Duration | Coverage Gain | New Coverage |
|-------|----------|---------------|--------------|
| Starting Point | - | - | ~30% |
| Phase 1: Quick Wins | 1-2 days | +15-20% | ~45-50% |
| Phase 2: Checkers | 2-3 days | +15-20% | ~60-70% |
| Phase 3: Edge Cases | 1-2 days | +10-15% | ~70-85% |
| Phase 4: Integration | 1 day | +5-10% | ~75-95% |

**Recommended Focus**: **Phases 1-2** to hit 60-70% target

---

## Test Writing Best Practices

### DO ✅
1. **Use temp directories** for file operations
   ```rust
   let temp = TempDir::new().unwrap();
   let test_file = temp.path().join("test.txt");
   ```

2. **Test one thing per test**
   - Clear test names: `test_cache_expires_after_ttl`
   - Single assertion focus

3. **Clean up resources**
   - Use RAII (TempDir auto-cleans)
   - Use `defer` patterns if needed

4. **Test both success and failure paths**
   ```rust
   #[test]
   fn test_cleanup_success() { /* happy path */ }
   
   #[test]
   fn test_cleanup_nonexistent_file() { /* error path */ }
   ```

5. **Use realistic test data**
   - Mock actual cache structures
   - Use realistic file sizes

### DON'T ❌
1. **Don't test excluded code** (UI, main.rs)
2. **Don't compare unsigned with >= 0** (clippy error)
3. **Don't use hardcoded paths** (`/Users/...`)
4. **Don't depend on system state** (installed tools)
5. **Don't leave temp files** around

---

## Measuring Progress

### After Each Phase:
```bash
# Generate coverage report
cargo llvm-cov --html --open

# Check overall percentage
cargo llvm-cov --summary-only

# Check specific file
cargo llvm-cov --text | grep backend.rs
```

### CI Integration:
- Coverage automatically reported on every PR
- Codecov will show coverage diff
- Block PRs that decrease coverage >2%

---

## Quick Start: Implement Phase 1 Now

To immediately improve coverage to ~50%, implement these 5 tests:

1. **`test_execute_cleanup_with_temp_files`** - Backend cleanup with real files
2. **`test_restore_from_quarantine`** - Cleanup history restore operation  
3. **`test_cache_ttl_expiration`** - Scan cache TTL logic
4. **`test_invalidate_specific_items`** - Backend cache invalidation
5. **`test_nodejs_cache_detection_real_files`** - Node.js checker with mock cache

**Estimated time**: 2-3 hours
**Coverage gain**: +15-18%

---

## Next Steps

1. ✅ Review this plan
2. ⬜ Choose starting phase (recommend Phase 1)
3. ⬜ Create feature branch: `git checkout -b improve-test-coverage`
4. ⬜ Implement tests iteratively
5. ⬜ Run coverage after each test: `cargo llvm-cov --html --open`
6. ⬜ Commit when reaching milestones (40%, 50%, 60%, etc.)
7. ⬜ Monitor Codecov dashboard for progress

---

## Questions?

- **Q: Should we test UI code?**  
  A: No, UI is excluded in `codecov.yml`. Focus on business logic.

- **Q: How do we test macOS-specific functions?**  
  A: Use conditional compilation or mock the OS calls where possible.

- **Q: What about flaky tests?**  
  A: Use deterministic test data. Avoid timing-dependent tests.

- **Q: Should every line be covered?**  
  A: No. 60-70% is excellent. Focus on critical paths, not every edge case.

---

**Ready to start?** Begin with Phase 1, Quick Wins! 🚀