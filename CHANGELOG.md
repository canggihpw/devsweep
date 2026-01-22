# Changelog

All notable changes to DevSweep will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Comprehensive test suite with 332 tests achieving 57.33% line coverage
- Edge case tests for symlinks, unicode filenames, permissions, long paths
- Persistence tests for corrupted data recovery
- Integration workflow tests for end-to-end scenarios
- Single instance module (`src/single_instance.rs`) extracted for testability
- Library exports (`src/lib.rs`) for better code organization
- Codecov configuration with UI code exclusions

### Changed
- Refactored `main.rs` to use library module for single-instance handling
- Consolidated coverage documentation into single README
- Updated CI workflow to run only on pull requests

### Removed
- Obsolete planning documents (IMPROVEMENT_PLAN.md, PHASE1_COMPLETION.md, etc.)

## [0.1.0] - 2026-01-21

### Added
- Initial release
- Core scanning functionality for development tool caches
- GPUI-based desktop interface with modern UI
- Support for 16 development tool categories:
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
  - General caches
  - Trash
- Quarantine system for safe cache removal
- Scan caching for improved performance
- Settings tab for cache configuration
- About tab with app information
- Light/dark theme support (Catppuccin Latte/Mocha)
- Single-instance app behavior
- Build scripts for macOS app bundle creation
- DMG creation for distribution

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
