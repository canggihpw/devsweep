# Changelog

All notable changes to DevSweep will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Storage trends feature (new Trends tab)
  - Track storage consumption patterns across scans
  - View space freed and net changes over time
  - Time range filters (Week, Month, Quarter, All Time)
  - Bar chart visualization of storage snapshots
  - Category-level trend breakdown with progress bars
  - Automatic snapshot recording after each scan
  - Persistent history saved to `~/Library/Caches/development-cleaner/trends_history.json`
- Size threshold filters for scan results
  - Filter dropdown in scan tab stats bar
  - Options: All, > 1 MB, > 10 MB, > 100 MB, > 500 MB, > 1 GB
  - Dynamically filters items and updates totals
- Non-blocking UI for long-running operations
  - Scan, cleanup, undo, and quarantine operations run in background threads
  - Users can navigate between tabs while operations are in progress
- Comprehensive test coverage for checkers
  - Database checker tests (PostgreSQL, MySQL, MongoDB, Redis, SQLite)
  - Go checker tests (module cache, build cache)
  - Shell checker tests (Zsh, Bash, Fish, Starship)
  - IDE checker tests (VSCode, JetBrains, Android Studio)
  - Homebrew checker tests (cache, cellar)
  - Extended Docker checker tests

### Changed
- Improved scan tab UI layout
  - Stats on left, action buttons on right
  - Full Rescan as secondary outlined button
  - Better empty state messaging ("Ready to Sweep?")
  - Improved disabled button contrast
- Sidebar improvements
  - Theme toggle shrunk to icon-only style
  - Storage progress bar with color coding (green/yellow/red)
  - Added margin between sidebar and content area

### Fixed
- UI freezing during long-running scan and cleanup operations
- Dropdown overlay z-index issue (now renders on top of scan results)

## [0.3.0] - 2026-01-25

### Added
- Super categories for logical grouping of scan results and TTL settings
  - Development Tools (Docker, Homebrew, Xcode, IDE Caches)
  - Package Managers (Node.js, Python, Rust/Cargo, Go, Java)
  - Project Files (node_modules, Git Repositories, Custom Paths)
  - System & Browsers (System Logs, Browser Caches, Shell, Database, General)
  - Trash
- Update checker with GitHub releases API integration
  - Automatic version comparison using semver
  - Download button links to latest DMG
  - Non-blocking background checks
- Custom scan paths feature in Settings tab
  - Browse or manually enter directories
  - Toggle paths on/off without removing
  - Persistent configuration
- Git repository cleanup checker
  - Merged branches detection and cleanup
  - Stale remote-tracking branch detection
  - Large .git directory identification (>100MB)
  - Local-only operations (no network/credential prompts)
- New dependencies: `ureq` (HTTP client), `semver` (version comparison)

### Changed
- Settings tab TTL section now grouped by super category
- Scan tab now displays hierarchical super category > category > item structure
- Updated category names for consistency:
  - "Node.js/npm/yarn" → "Node.js Package Managers"
  - "Java (Gradle/Maven)" → "Java Build Tools"
  - "System Logs" → "System Logs & Crash Reports"
- Updated all documentation to reflect new features

### Fixed
- SuperCategoryType::from_category_name() now correctly maps all checker category names

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

[Unreleased]: https://github.com/canggihpw/devsweep/compare/v0.3.0...HEAD
[0.3.0]: https://github.com/canggihpw/devsweep/compare/v0.2.0...v0.3.0
[0.2.0]: https://github.com/canggihpw/devsweep/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/canggihpw/devsweep/releases/tag/v0.1.0
