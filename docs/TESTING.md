# Testing Guide

## Quick Commands

```bash
cargo test                          # Run all tests
cargo test test_name                # Run specific test
cargo test -- --nocapture           # Show output
cargo clippy                        # Lint check
cargo llvm-cov --html --open        # Coverage report
```

## Coverage Status

| Metric | Value |
|--------|-------|
| Line Coverage | ~57% |
| Tests | 480+ |
| Pass Rate | 100% |

**Excluded from coverage**: UI code (`src/app/**`, `src/ui/**`, `src/main.rs`, `src/assets.rs`)

## Test Files (21 files)

| File | Tests | Focus |
|------|-------|-------|
| `test_edge_cases.rs` | 41 | Symlinks, unicode, permissions |
| `test_db_checker.rs` | 35 | PostgreSQL, MySQL, MongoDB, Redis, SQLite |
| `test_persistence.rs` | 35 | Corrupted data recovery |
| `test_ide_checker.rs` | 31 | VSCode, JetBrains, Android Studio |
| `test_scan_cache.rs` | 29 | Cache TTL, persistence |
| `test_backend.rs` | 28 | Backend operations |
| `test_shell_checker.rs` | 27 | Zsh, Bash, Fish, Starship |
| `test_homebrew_checker.rs` | 24 | Homebrew cache, cellar |
| `test_integration_workflows.rs` | 24 | End-to-end workflows |
| `test_xcode_checker.rs` | 23 | Xcode structures |
| `test_cleanup_history.rs` | 22 | Quarantine, restore |
| `test_python_checker.rs` | 21 | Python caches |
| `test_docker_checker.rs` | 30 | Docker images, containers, cache |
| `test_go_checker.rs` | 18 | Go module/build cache |
| `test_nodejs_checker.rs` | 17 | npm/yarn/pnpm |
| `test_checkers.rs` | 16 | Checker basics |
| `test_single_instance.rs` | 14 | Unix socket |
| `test_types.rs` | 13 | Type builders |
| `test_cache_settings.rs` | 8 | Cache settings |
| `test_utils.rs` | 6 | Utility functions |
| `performance_tests.rs` | 5 | Benchmarks |

## Test Pattern

```rust
use tempfile::TempDir;

#[test]
fn test_example() {
    let temp = TempDir::new().unwrap();
    let file = temp.path().join("test.txt");
    std::fs::write(&file, "content").unwrap();
    
    // Test logic here
    // TempDir auto-cleans on drop
}
```

## Adding Tests

1. Create `tests/test_<module>.rs`
2. Use `TempDir` for file operations
3. Test success and error paths
4. Run `cargo test` to verify
