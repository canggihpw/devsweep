# Changelog

All notable changes to DevSweep will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.5.0] - 2026-09-01

### Added
- New **📊 Largest** tab showing where your disk space is going:
  - A **treemap** of the largest folders (color-coded, filled rectangle layout).
  - Ranked lists of the **largest folders** and the **largest files**
    (rank, name, full path, size).
- The largest index is **cached** to disk (30-minute TTL) and loaded instantly
  when you open the tab, with a **Rescan** button to force a fresh index.
- The largest scan streams **live progress** — folder sizes first, then the
  largest files — so results appear as they're found instead of a blank screen.
- The largest scan runs on a **dedicated background thread** (non-blocking), so
  scanning, cleaning, and tab switching stay responsive while it runs.
- Dev-focused default roots (Desktop, Documents, Downloads, Projects, Developer,
  Code) with a bounded walk (depth 5, symlinks never followed) for fast results.

## [0.4.0] - 2026-08-31

### Added
- Streaming scan results: categories appear live as each check completes, with a
  "Scanned X/Y categories" progress indicator in the header — no more waiting for
  the whole scan to finish before anything shows up.
- New scan categories (now **22** in total):
  - **Bun** (global + install caches)
  - **Flutter** (Dart pub cache, `~/.pub-cache`)
  - **Android** (Gradle + Android SDK caches)
  - **Swift Package Manager** caches (`org.swift.swiftpm`)
  - Python now also detects the **uv** cache
  - Rust/Cargo now also detects **sccache**
- Single source of truth for category names, with tests guarding against drift
  between the scanner, the UI grouping, and the TTL defaults.

### Changed
- Performance:
  - Faster scans — cached categories are reused by TTL without re-`stat()`-ing
    every path; path validation only runs where no TTL is set (in parallel).
  - Scan results render via pre-grouped lists and stored item indices
    (O(n²) → O(n)), which also fixes duplicate row ids / wrong-toggle bugs.
  - No redundant clones in the scan pipeline (results are moved, not copied).
  - Selection/select-all is O(1) via a key set instead of O(n) scans.
  - Directory sizing is sequential again to avoid starving the parallel pool.
- UI: "Full Rescan" is now a proper secondary button; category badges
  pluralize correctly (e.g. `1 category, 1 item`).
- Internals: scan pipeline refactored onto a small plan/commit API that both the
  blocking (`scan_with_cache`) and streaming paths share.

### Fixed
- Node.js and Java caches now land under **Package Managers** instead of being
  mis-grouped under "System & Browsers" (the UI no longer matches stale names).
- Removed per-scan debug `stdout` spam from the Trash checker.
- Dropped a dead `last_full_scan` field from the scan cache.

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

[Unreleased]: https://github.com/canggihpw/devsweep/compare/v0.5.0...HEAD
[0.5.0]: https://github.com/canggihpw/devsweep/compare/v0.4.0...v0.5.0
[0.4.0]: https://github.com/canggihpw/devsweep/compare/v0.3.0...v0.4.0
[0.3.0]: https://github.com/canggihpw/devsweep/compare/v0.2.0...v0.3.0
[0.2.0]: https://github.com/canggihpw/devsweep/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/canggihpw/devsweep/releases/tag/v0.1.0
