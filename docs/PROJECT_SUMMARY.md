# DevSweep - Project Summary

## Overview

**DevSweep** is a modern macOS desktop application built with Rust and GPUI that helps developers reclaim disk space by identifying and safely removing development tool caches and build artifacts.

## The Problem

Developers accumulate gigabytes of cached data from various development tools:
- Node.js `node_modules` directories
- Python `__pycache__` and virtual environments
- Rust `target` directories and cargo caches
- Docker images and containers
- Xcode derived data
- And many more...

These caches grow rapidly and consume significant disk space, often without the developer's awareness.

## The Solution

DevSweep provides:
- **Smart Scanning**: Fast detection of development caches across 16+ tool categories
- **Safe Removal**: Quarantine system with undo capabilities
- **Modern UI**: Native macOS experience built with GPUI framework
- **Performance**: Intelligent caching for faster subsequent scans
- **Transparency**: Clear breakdown of what will be deleted and why

## Key Features

### Intelligent Scanning
- Detects caches from 17+ development tools and frameworks
- Super categories for logical grouping (Development Tools, Package Managers, etc.)
- Smart caching system for improved performance
- Parallel scanning for speed
- Real-time size calculation

### Safe Quarantine System
- Move files to quarantine before permanent deletion
- Undo capability to restore quarantined files
- Individual item management
- Automatic cleanup of old quarantined items

### Customizable Settings
- Configurable cache TTL (Time-To-Live) per category
- TTL settings grouped by super category
- Custom scan paths - add your own directories
- Enable/disable cache functionality

### Update Checker
- Automatic GitHub release checking
- Version comparison with semver
- Direct download links for new versions
- Non-blocking background checks

### Modern User Interface
- Native macOS look and feel
- Light and dark theme support (Catppuccin Latte/Mocha)
- Responsive design with tab-based navigation
- Real-time feedback
- Single-instance app behavior

## Supported Development Tools

### Development Tools
- **Docker**: Images, containers, build cache
- **Homebrew**: Package caches
- **Xcode**: DerivedData, archives
- **IDEs**: VSCode, IntelliJ, JetBrains caches

### Package Managers
- **Node.js**: npm, yarn, pnpm caches
- **Python**: pip cache, __pycache__, virtualenvs
- **Rust**: Cargo registry, target directories
- **Go**: Build cache, module cache
- **Java**: Maven, Gradle caches

### Project Files
- **node_modules**: Project dependencies in Documents/Projects
- **Git Repositories**: Merged branches, stale remotes, large .git dirs
- **Custom Paths**: User-defined directories

### System & Browsers
- **Browsers**: Chrome, Firefox, Safari, Edge, Arc caches
- **Databases**: PostgreSQL, MySQL, Redis logs
- **System**: Logs, crash reports, shell history
- **General**: Large application caches

### Trash
- User trash contents

## Technical Stack

- **Language**: Rust (2021 edition)
- **UI Framework**: GPUI (Zed's GPU-accelerated UI framework)
- **Platform**: macOS 11.0+
- **Build System**: Cargo
- **Asset Embedding**: rust-embed for bundling icons
- **Image Processing**: image crate for icon loading
- **Theme**: Catppuccin (Latte for light, Mocha for dark)

## Architecture Highlights

### Modular Design
- **Checkers**: Pluggable scanner modules for each tool
- **Backend**: Core scanning and quarantine logic
- **UI**: Separate presentation layer
- **Cache System**: Persistent scan results
- **Single Instance**: Unix socket-based instance detection

### Performance Optimizations
- Parallel directory traversal with Rayon
- Incremental scanning with cache
- Efficient size calculations
- Minimal memory footprint

### Safety First
- Two-stage deletion (quarantine -> permanent)
- File locking to prevent conflicts
- Atomic operations
- Comprehensive error handling

## Project Status

**Current Version**: 0.2.0

**Test Coverage**: 57.33% (332 tests, 100% pass rate)

**Stability**: Beta - Core functionality stable, comprehensive test coverage

### Completed Features
- Core scanning functionality with 17+ categories
- Super categories for logical grouping of scan results
- Quarantine system with restore capabilities
- Modern GPUI-based UI with tab navigation
- Light/dark theme toggle with theme-aware icons
- Single-instance app behavior
- Embedded app icons (light/dark variants)
- Comprehensive test suite (332 tests, 57.33% coverage)
- Scan caching for improved performance
- CI/CD with automated testing and coverage reporting
- Complete documentation (testing guide, Git workflow, coverage tracking)
- Update checker with GitHub releases API
- Custom scan paths (user-defined directories)
- Git repository cleanup (merged branches, stale remotes, large .git)

### In Progress
- Performance optimization
- Additional checker improvements

### Planned
- Scheduled scanning
- Advanced filtering (size thresholds)
- Export reports (CSV/JSON)
- Menu bar mode
- Cross-platform support (Linux, Windows)

## Use Cases

### For Individual Developers
- Quickly reclaim disk space when storage runs low
- Regular maintenance of development machine
- Clean up after switching between projects

### For Development Teams
- Standardize cache cleanup across team members
- Maintain clean development environments
- Reduce onboarding friction for new machines

## Metrics & Goals

### Performance Targets
- Scan typical developer machine in < 5 seconds (with cache)
- Full scan in < 30 seconds (without cache)
- Memory usage < 100MB during operation

### Test Coverage Goals
- Current: 57.33% line coverage (excluding UI)
- Target: Maintain above 50% coverage

## Links

- **Repository**: https://github.com/canggihpw/devsweep
- **Issues**: https://github.com/canggihpw/devsweep/issues
- **Releases**: https://github.com/canggihpw/devsweep/releases

## License

MIT License - See [LICENSE](../LICENSE) for details.
