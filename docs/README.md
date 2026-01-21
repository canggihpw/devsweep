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
- **[Versioning Guide](VERSIONING.md)** - Semantic versioning and release process
- **[Changelog](../CHANGELOG.md)** - Version history and changes

### For Maintainers

- **[Maintainer Guide](MAINTAINER_GUIDE.md)** - Complete guide for project maintainers
- **[Testing Guide](TESTING.md)** - Comprehensive testing strategy and plan

### Technical Documentation

- **[Project Summary](PROJECT_SUMMARY.md)** - High-level project overview and architecture
- **[Architecture Overview](../README.md#architecture)** - System design and components
- **[How It Works](../README.md#how-it-works)** - Internal mechanisms
- **[Adding New Checkers](../CONTRIBUTING.md#adding-new-checkers)** - Guide for adding support for new tools

## Project Structure

```
devsweep/
├── src/                    # Source code
│   ├── main.rs            # Application entry point
│   ├── backend.rs         # Core scanning logic
│   ├── types.rs           # Type definitions
│   ├── checkers/          # Tool-specific scanners
│   └── ui/                # User interface components
├── scripts/               # Build and utility scripts
├── assets/                # Icons and images
├── docs/                  # Documentation (you are here)
│   ├── README.md          # This file
│   ├── PROJECT_SUMMARY.md # Project overview
│   ├── VERSIONING.md      # Versioning guide
│   ├── MAINTAINER_GUIDE.md # Maintainer guide
│   └── TESTING.md         # Testing strategy
├── .github/               # GitHub-specific files
│   ├── workflows/         # CI/CD workflows
│   └── ISSUE_TEMPLATE/    # Issue templates
├── CONTRIBUTING.md        # Contribution guidelines
├── CODE_OF_CONDUCT.md     # Code of conduct
├── CHANGELOG.md           # Version history
├── LICENSE                # MIT License
├── README.md              # Main project README
└── Cargo.toml            # Rust dependencies
```

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
- [App Bundle Documentation](https://developer.apple.com/library/archive/documentation/CoreFoundation/Conceptual/CFBundles/BundleTypes/BundleTypes.html)

## Getting Help

- **Issues**: Open an issue on GitHub for bugs or feature requests
- **Discussions**: Use GitHub Discussions for questions and general discussion
- **Contributing**: See [CONTRIBUTING.md](../CONTRIBUTING.md) for guidelines

## Documentation Contributions

We welcome improvements to our documentation! If you find:
- Typos or errors
- Unclear explanations
- Missing information
- Outdated content

Please submit a pull request or open an issue.

## License

This documentation is part of DevSweep and is licensed under the MIT License. See [LICENSE](../LICENSE) for details.

---

**Last Updated**: 2024
**Project Repository**: https://github.com/canggihpw/devsweep