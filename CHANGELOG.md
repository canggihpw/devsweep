# Changelog

All notable changes to DevSweep will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Initial project setup and structure
- Core scanning functionality for development tool caches
- GPUI-based desktop interface with modern UI
- Support for multiple development tool categories:
  - Node.js (node_modules, npm/yarn/pnpm caches)
  - Python (pip cache, __pycache__, virtual environments)
  - Rust/Cargo (target directories, cargo cache)
  - Docker (images, containers, build cache)
  - Xcode (DerivedData, archives)
  - Go (build cache, module cache)
  - Java/Maven/Gradle caches
  - Homebrew caches
  - Browser caches (Chrome, Firefox, Safari, Edge)
  - IDE caches (VSCode, IntelliJ, etc.)
  - Database caches (PostgreSQL, MySQL, Redis)
  - Shell history and logs
  - System logs
- Quarantine system for safe cache removal
- Scan caching for improved performance
- Settings tab for cache configuration
- About tab with app information
- Build scripts for macOS app bundle creation
- DMG creation for distribution

### Changed
- N/A

### Deprecated
- N/A

### Removed
- N/A

### Fixed
- N/A

### Security
- N/A

## [0.1.0] - YYYY-MM-DD

### Added
- Initial release
- Basic scanning functionality
- Quarantine system
- macOS app support

---

## Version History Guidelines

### Categories

- **Added** for new features
- **Changed** for changes in existing functionality
- **Deprecated** for soon-to-be removed features
- **Removed** for now removed features
- **Fixed** for any bug fixes
- **Security** for vulnerability fixes

### Version Links

[Unreleased]: https://github.com/canggihpw/devsweep/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/canggihpw/devsweep/releases/tag/v0.1.0

### Notes

- Update this file with every significant change
- Group changes by category
- Keep descriptions clear and concise
- Link to relevant issues/PRs when applicable
- Update version links when creating new releases
- Date format: YYYY-MM-DD (ISO 8601)