# Contributing to DevSweep

Thank you for your interest in contributing to DevSweep! This document provides guidelines and instructions for contributing to the project.

## Table of Contents

- [Code of Conduct](#code-of-conduct)
- [Getting Started](#getting-started)
- [How Can I Contribute?](#how-can-i-contribute)
- [Development Setup](#development-setup)
- [Coding Standards](#coding-standards)
- [Commit Guidelines](#commit-guidelines)
- [Pull Request Process](#pull-request-process)
- [Testing](#testing)
- [Adding New Checkers](#adding-new-checkers)
- [Documentation](#documentation)

## Code of Conduct

This project adheres to a Code of Conduct that all contributors are expected to follow. Please be respectful and constructive in all interactions.

## Getting Started

1. **Fork the repository** on GitHub
2. **Clone your fork** locally:
   ```bash
   git clone https://github.com/YOUR_USERNAME/devsweep.git
   cd devsweep
   ```
3. **Add upstream remote**:
   ```bash
   git remote add upstream https://github.com/canggihpw/devsweep.git
   ```
4. **Create a branch** for your changes:
   ```bash
   git checkout -b feature/your-feature-name
   ```

## How Can I Contribute?

### Reporting Bugs

Before creating bug reports, please check existing issues to avoid duplicates. When creating a bug report, include:

- **Clear title and description**
- **Steps to reproduce** the issue
- **Expected behavior** vs. **actual behavior**
- **macOS version** and **app version**
- **Screenshots** if applicable
- **Error messages** or logs

### Suggesting Features

Feature suggestions are welcome! Please:

- **Check existing issues** for similar suggestions
- **Provide detailed explanation** of the feature
- **Explain why this feature would be useful** to most users
- **Provide examples** or mockups if possible

### Contributing Code

We welcome code contributions! Here are areas where you can help:

1. **New Checkers**: Add support for more development tools
2. **Bug Fixes**: Fix reported issues
3. **Performance**: Improve scanning speed or memory usage
4. **UI/UX**: Enhance user interface and experience
5. **Documentation**: Improve README, comments, or guides
6. **Testing**: Add unit tests or integration tests

## Development Setup

### Prerequisites

- **Rust** 1.70+ (install via [rustup](https://rustup.rs/))
- **Xcode Command Line Tools** (macOS):
  ```bash
  xcode-select --install
  ```
- **Git** for version control

### Building the Project

1. **Clone and navigate** to the project:
   ```bash
   git clone https://github.com/YOUR_USERNAME/devsweep.git
   cd devsweep
   ```

2. **Build the project**:
   ```bash
   cargo build
   ```

3. **Run the application**:
   ```bash
   cargo run
   ```

4. **Run tests**:
   ```bash
   cargo test
   ```

5. **Build release version**:
   ```bash
   cargo build --release
   ```

### Project Structure

```
devsweep/
├── src/
│   ├── main.rs              # Application entry point
│   ├── backend.rs           # Core scanning logic
│   ├── types.rs             # Type definitions
│   ├── utils.rs             # Utility functions
│   ├── scan_cache.rs        # Scan caching system
│   ├── cache_settings.rs    # Cache configuration
│   ├── cleanup_history.rs   # Quarantine history
│   ├── checkers/            # Scanner modules
│   │   ├── mod.rs           # Checker registry
│   │   ├── nodejs.rs        # Node.js checker
│   │   ├── python.rs        # Python checker
│   │   ├── rust_cargo.rs    # Rust/Cargo checker
│   │   └── ...              # Other checkers
│   └── ui/                  # UI components
│       ├── mod.rs           # UI entry point
│       ├── sidebar.rs       # Sidebar navigation
│       └── theme.rs         # Theme definitions
├── scripts/                 # Build and utility scripts
├── assets/                  # Icons and images
├── docs/                    # Documentation
└── Cargo.toml              # Rust dependencies
```

## Coding Standards

### Rust Style Guide

Follow the official [Rust Style Guide](https://doc.rust-lang.org/style-guide/):

1. **Format code** with `rustfmt`:
   ```bash
   cargo fmt
   ```

2. **Check for issues** with `clippy`:
   ```bash
   cargo clippy
   ```

3. **Fix all warnings** before submitting PR

### Code Principles

- **Keep it simple**: Prefer clarity over cleverness
- **DRY**: Don't repeat yourself
- **Single Responsibility**: Each function/module should do one thing well
- **Error Handling**: Use `Result` types, avoid unwrap in production code
- **Comments**: Write self-documenting code; add comments for "why", not "what"
- **Performance**: Consider performance, but prioritize correctness first

### Naming Conventions

- **Variables/Functions**: `snake_case`
- **Types/Structs**: `PascalCase`
- **Constants**: `SCREAMING_SNAKE_CASE`
- **Modules**: `snake_case`

Example:
```rust
const MAX_SCAN_DEPTH: usize = 10;

struct ScanResult {
    total_size: u64,
    file_count: usize,
}

fn calculate_directory_size(path: &Path) -> Result<u64, ScanError> {
    // Implementation
}
```

## Commit Guidelines

We follow [Conventional Commits](https://www.conventionalcommits.org/):

### Commit Format

```
<type>(<scope>): <subject>

<body>

<footer>
```

### Types

- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation changes
- `style`: Code style (formatting, missing semicolons, etc.)
- `refactor`: Code refactoring
- `perf`: Performance improvements
- `test`: Adding or updating tests
- `chore`: Maintenance tasks
- `ci`: CI/CD changes

### Examples

```
feat(checker): add Ruby gems cache support

Implements scanning for Ruby gems cache directory.
Includes size calculation and quarantine support.

Closes #123
```

```
fix(ui): prevent crash when path contains special characters

Added proper escaping for filesystem paths.

Fixes #45
```

```
docs(readme): update installation instructions

Added troubleshooting section for common issues.
```

### Commit Best Practices

- Use **present tense** ("add feature" not "added feature")
- Use **imperative mood** ("move cursor to..." not "moves cursor to...")
- **First line**: 72 characters or less
- **Reference issues**: Use "Closes #123" or "Fixes #456"
- **Break commits**: One logical change per commit

## Pull Request Process

### Before Submitting

1. **Update your branch** with latest upstream:
   ```bash
   git fetch upstream
   git rebase upstream/main
   ```

2. **Run tests**:
   ```bash
   cargo test
   ```

3. **Format and lint**:
   ```bash
   cargo fmt
   cargo clippy
   ```

4. **Update documentation** if needed

5. **Update CHANGELOG.md** (for significant changes)

### Submitting PR

1. **Push to your fork**:
   ```bash
   git push origin feature/your-feature-name
   ```

2. **Open PR** on GitHub with:
   - **Clear title** following commit conventions
   - **Description** of changes and motivation
   - **Related issues** (e.g., "Closes #123")
   - **Screenshots** for UI changes
   - **Testing done** and results

3. **PR template**:
   ```markdown
   ## Description
   Brief description of changes
   
   ## Type of Change
   - [ ] Bug fix
   - [ ] New feature
   - [ ] Breaking change
   - [ ] Documentation update
   
   ## Testing
   - [ ] Tests pass locally
   - [ ] Added new tests
   - [ ] Manual testing performed
   
   ## Checklist
   - [ ] Code follows project style
   - [ ] Self-review completed
   - [ ] Comments added where needed
   - [ ] Documentation updated
   - [ ] No new warnings
   ```

### Review Process

- Maintainers will review your PR
- Address feedback by pushing new commits
- Once approved, maintainers will merge
- PRs may take several days to review

## Testing

### Running Tests

```bash
# Run all tests
cargo test

# Run specific test
cargo test test_name

# Run with output
cargo test -- --nocapture
```

### Writing Tests

Add tests for new features or bug fixes:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_size() {
        let result = calculate_directory_size(Path::new("/tmp"));
        assert!(result.is_ok());
    }

    #[test]
    fn test_invalid_path() {
        let result = calculate_directory_size(Path::new("/nonexistent"));
        assert!(result.is_err());
    }
}
```

### Manual Testing

For UI changes:
1. Build and run the app
2. Test all affected workflows
3. Test edge cases
4. Check performance impact
5. Verify on different macOS versions if possible

## Adding New Checkers

To add support for a new development tool:

### 1. Create Checker File

Create `src/checkers/your_tool.rs`:

```rust
use crate::types::ScanItem;
use std::path::PathBuf;

pub fn check() -> Vec<ScanItem> {
    let mut items = Vec::new();
    
    // Define paths to check
    let cache_path = dirs::home_dir()
        .map(|h| h.join(".your_tool/cache"));
    
    if let Some(path) = cache_path {
        if path.exists() {
            items.push(ScanItem {
                name: "Your Tool Cache".to_string(),
                path: path.to_string_lossy().to_string(),
                size: calculate_size(&path),
                category: "Your Tool".to_string(),
                description: "Cached data for Your Tool".to_string(),
            });
        }
    }
    
    items
}

fn calculate_size(path: &PathBuf) -> u64 {
    // Use existing utility functions
    crate::utils::calculate_directory_size(path).unwrap_or(0)
}
```

### 2. Register Checker

In `src/checkers/mod.rs`:

```rust
pub mod your_tool;

// In the registry function
pub fn get_all_checkers() -> Vec<Box<dyn Fn() -> Vec<ScanItem>>> {
    vec![
        // ... existing checkers
        Box::new(your_tool::check),
    ]
}
```

### 3. Document Checker

Add to README.md under "Supported Categories":

```markdown
- **Your Tool**: Description of what is scanned
  - Cache directory
  - Build artifacts
```

### 4. Test Checker

- Test with tool installed and uninstalled
- Test with empty and full caches
- Verify size calculations
- Test quarantine functionality

## Documentation

### Code Documentation

Add doc comments for public APIs:

```rust
/// Calculates the total size of a directory recursively.
///
/// # Arguments
///
/// * `path` - Path to the directory
///
/// # Returns
///
/// Total size in bytes, or error if path is invalid
///
/// # Examples
///
/// ```
/// let size = calculate_directory_size(Path::new("/tmp"))?;
/// println!("Size: {} bytes", size);
/// ```
pub fn calculate_directory_size(path: &Path) -> Result<u64, ScanError> {
    // Implementation
}
```

### README Updates

Update README.md when:
- Adding new features
- Changing installation process
- Adding new supported tools
- Changing usage instructions

### Changelog

Update `CHANGELOG.md` following [Keep a Changelog](https://keepachangelog.com/):

```markdown
## [Unreleased]

### Added
- Support for Your Tool cache scanning

### Fixed
- Fixed crash when scanning network drives
```

## Questions or Need Help?

- **Open an issue** for questions
- **Join discussions** in existing issues
- **Read documentation** in `/docs` folder
- **Check examples** in existing code

## License

By contributing, you agree that your contributions will be licensed under the same license as the project.

---

Thank you for contributing to DevSweep! 🎉