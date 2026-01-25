# DevSweep Documentation

Welcome to the DevSweep documentation! This directory contains comprehensive guides and documentation for the project.

## Documentation Index

### For Users

- **[Installation Guide](../README.md#installation)** - How to install and set up DevSweep
- **[Usage Guide](../README.md#usage)** - How to use the application
- **[FAQ](../README.md#faq)** - Frequently asked questions
- **[Troubleshooting](../README.md#troubleshooting)** - Common issues and solutions

### For Contributors

- **[Contributing Guide](../CONTRIBUTING.md)** - How to contribute to the project
- **[Code of Conduct](../CODE_OF_CONDUCT.md)** - Community guidelines
- **[Git Workflow Guide](GIT_WORKFLOW.md)** - Branching strategy and workflow
- **[Versioning Guide](VERSIONING.md)** - Semantic versioning and release process
- **[Changelog](../CHANGELOG.md)** - Version history and changes

### Technical Documentation

- **[Project Summary](PROJECT_SUMMARY.md)** - High-level project overview and architecture
- **[Testing Guide](TESTING.md)** - Comprehensive testing strategy and guide
- **[Coverage Documentation](coverage/README.md)** - Test coverage status and improvement guide

## Project Structure

```
devsweep/
├── src/                    # Source code
│   ├── main.rs            # Application entry point
│   ├── lib.rs             # Library exports
│   ├── backend.rs         # Core scanning logic
│   ├── types.rs           # Type definitions
│   ├── utils.rs           # Utility functions
│   ├── scan_cache.rs      # Cache management
│   ├── cleanup_history.rs # Quarantine system
│   ├── cache_settings.rs  # Settings persistence
│   ├── single_instance.rs # Single-instance handling
│   ├── update_checker.rs  # GitHub releases API
│   ├── custom_paths.rs    # User-defined scan paths
│   ├── assets.rs          # Asset loading
│   ├── checkers/          # Tool-specific scanners
│   │   ├── mod.rs
│   │   ├── docker.rs
│   │   ├── homebrew.rs
│   │   ├── nodejs.rs
│   │   ├── python.rs
│   │   ├── rust_cargo.rs
│   │   ├── xcode.rs
│   │   ├── java.rs
│   │   ├── go.rs
│   │   ├── ide.rs
│   │   ├── shell.rs
│   │   ├── db.rs
│   │   ├── logs.rs
│   │   ├── browser.rs
│   │   ├── general.rs
│   │   └── git.rs         # Git repository cleanup
│   ├── app/               # GPUI application
│   │   ├── mod.rs
│   │   ├── state.rs       # App state + SuperCategoryType
│   │   ├── actions.rs
│   │   ├── render.rs
│   │   └── tabs/
│   └── ui/                # UI components
│       ├── mod.rs
│       ├── sidebar.rs
│       └── theme.rs
├── tests/                 # Test files (332 tests)
│   ├── test_backend.rs
│   ├── test_scan_cache.rs
│   ├── test_cleanup_history.rs
│   ├── test_edge_cases.rs
│   ├── test_persistence.rs
│   ├── test_integration_workflows.rs
│   ├── test_single_instance.rs
│   └── ... (more test files)
├── scripts/               # Build and utility scripts
├── assets/                # Icons and images
├── docs/                  # Documentation (you are here)
│   ├── README.md          # This file
│   ├── AI_CONTEXT.md      # AI assistant context
│   ├── PROJECT_SUMMARY.md # Project overview
│   ├── GIT_WORKFLOW.md    # Git workflow
│   ├── VERSIONING.md      # Versioning guide
│   ├── TESTING.md         # Testing guide
│   └── coverage/          # Coverage documentation
├── .github/               # GitHub-specific files
│   ├── workflows/         # CI/CD workflows
│   └── ISSUE_TEMPLATE/    # Issue templates
├── CONTRIBUTING.md        # Contribution guidelines
├── CODE_OF_CONDUCT.md     # Code of conduct
├── CHANGELOG.md           # Version history
├── LICENSE                # MIT License
├── README.md              # Main project README
├── Cargo.toml             # Rust dependencies
└── codecov.yml            # Coverage configuration
```

## Current Status

| Metric | Value |
|--------|-------|
| Version | 0.3.0 |
| Tests | 332+ |
| Coverage | ~57% |
| Pass Rate | 100% |
| Categories | 17+ |

## Quick Links

### Development

- [Rust Documentation](https://doc.rust-lang.org/)
- [GPUI Framework](https://github.com/zed-industries/zed)
- [Cargo Book](https://doc.rust-lang.org/cargo/)

### Standards & Conventions

- [Semantic Versioning](https://semver.org/)
- [Keep a Changelog](https://keepachangelog.com/)
- [Conventional Commits](https://www.conventionalcommits.org/)
- [Rust Style Guide](https://doc.rust-lang.org/style-guide/)

### macOS Development

- [macOS App Distribution](https://developer.apple.com/documentation/security/notarizing_macos_software_before_distribution)
- [Code Signing Guide](https://developer.apple.com/support/code-signing/)

## Getting Help

- **Issues**: Open an issue on GitHub for bugs or feature requests
- **Discussions**: Use GitHub Discussions for questions
- **Contributing**: See [CONTRIBUTING.md](../CONTRIBUTING.md) for guidelines

## License

This documentation is part of DevSweep and is licensed under the MIT License.

---

**Last Updated**: January 2026
**Project Repository**: https://github.com/canggihpw/devsweep
