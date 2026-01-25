# Contributing to DevSweep

## Quick Start

```bash
# Fork and clone
git clone https://github.com/YOUR_USERNAME/devsweep.git
cd devsweep

# Build and test
cargo build
cargo test

# Create feature branch
git checkout -b feature/your-feature
```

## Development

### Prerequisites
- Rust 1.70+ ([rustup](https://rustup.rs/))
- Xcode Command Line Tools: `xcode-select --install`

### Commands
```bash
cargo build              # Build
cargo run                # Run app
cargo test               # Run tests
cargo fmt                # Format code
cargo clippy             # Lint
cargo build --release    # Release build
```

## Adding a New Checker

1. Create `src/checkers/yourtool.rs`:
```rust
use crate::types::{CheckResult, CleanupItem};

pub fn check_yourtool() -> CheckResult {
    let mut result = CheckResult::new("Your Tool");
    // Add items...
    result
}
```

2. Register in `src/checkers/mod.rs`:
```rust
pub mod yourtool;
pub use yourtool::check_yourtool;
```

3. Add to `src/backend.rs` in `scan_with_cache()`

4. Update `SuperCategoryType::from_category_name()` in `src/app/state.rs`

## Pull Request Process

1. Follow [GIT_WORKFLOW.md](docs/GIT_WORKFLOW.md)
2. Run `cargo fmt && cargo clippy && cargo test`
3. Update CHANGELOG.md [Unreleased] section
4. Create PR with clear description

## Commit Messages

Follow [Conventional Commits](https://www.conventionalcommits.org/):
- `feat:` New feature
- `fix:` Bug fix
- `docs:` Documentation
- `test:` Tests
- `refactor:` Refactoring
- `chore:` Maintenance

## Code Style

- Run `cargo fmt` before committing
- Fix all `cargo clippy` warnings
- Use `Result` types, avoid `.unwrap()` in production code
- Add tests for new functionality

## Questions?

Open an issue on GitHub.
