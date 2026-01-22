# Changelog

All notable changes to DevSweep will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.2.0] - 2026-01-22

### Added
- Comprehensive test suite with 332 tests achieving 57.33% line coverage
- Edge case tests for symlinks, unicode filenames, permissions, long paths
- Persistence tests for corrupted data recovery
- Integration workflow tests for end-to-end scenarios
- Performance tests for scan and cache operations
- Single instance module (`src/single_instance.rs`) extracted for testability
- Library exports (`src/lib.rs`) for better code organization
- Codecov configuration and integration for coverage tracking
- Testing guide documentation (`docs/TESTING.md`)
- Git workflow documentation (`docs/GIT_WORKFLOW.md`)
- Coverage documentation (`docs/coverage/README.md`)
- Theme-aware icons for light and dark modes
- Dedicated test files for checkers (Node.js, Docker, Python, Xcode)

### Changed
- Refactored `main.rs` into modular app directory structure with separate tabs
- Refactored single-instance handling to use dedicated library module
- Consolidated coverage documentation into single comprehensive README
- Updated CI workflow to run only on pull requests for efficiency
- Improved documentation to reflect current implementation
- Optimized workflows and removed redundant jobs
- Combined test and coverage workflows into single CI run for faster execution

### Fixed
- Clippy warnings and linting errors throughout codebase
- Workflow branch name issues
- Unsigned comparison warning in Node.js tests
- CI workflow configuration for better performance

### Removed
- Obsolete planning documents (IMPROVEMENT_PLAN.md, PHASE1_COMPLETION.md, etc.)
- Redundant test jobs from CI workflow
- Separate coverage.yml workflow (now integrated into ci.yml)

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

[Unreleased]: https://github.com/canggihpw/devsweep/compare/v0.2.0...HEAD
[0.2.0]: https://github.com/canggihpw/devsweep/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/canggihpw/devsweep/releases/tag/v0.1.0
