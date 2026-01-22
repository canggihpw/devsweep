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
- Detects caches from 16+ development tools and frameworks
- Smart caching system for improved performance
- Parallel scanning for speed
- Real-time size calculation

### Safe Quarantine System
- Move files to quarantine before permanent deletion
- Undo capability to restore quarantined files
- Individual item management
- Automatic cleanup of old quarantined items

### Customizable Settings
- Configurable cache TTL (Time-To-Live)
- Enable/disable cache functionality
- Adjustable scan behavior

### Modern User Interface
- Native macOS look and feel
- Light and dark theme support (Catppuccin Latte/Mocha)
- Responsive design with tab-based navigation
- Real-time feedback
- Single-instance app behavior

## Supported Development Tools

- **Node.js**: npm, yarn, pnpm caches and node_modules
- **Python**: pip cache, __pycache__, virtualenvs
- **Rust**: Cargo registry, target directories
- **Docker**: Images, containers, build cache
- **Xcode**: DerivedData, archives
- **Go**: Build cache, module cache
- **Java**: Maven, Gradle caches
- **Homebrew**: Package caches
- **Browsers**: Chrome, Firefox, Safari, Edge caches
- **IDEs**: VSCode, IntelliJ, JetBrains caches
- **Databases**: PostgreSQL, MySQL, Redis logs
- **System**: Logs, shell history
- And more...

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

**Current Version**: 0.1.0

**Test Coverage**: 57.33% (332 tests, 100% pass rate)

**Stability**: Alpha - Core functionality working, API may change

### Completed Features
- Core scanning functionality
- Quarantine system
- Modern GPUI-based UI
- Support for major development tools
- Light/dark theme toggle
- Single-instance app behavior
- Embedded app icons
- Comprehensive test suite

### In Progress
- Additional checker improvements
- Performance optimization

### Planned
- Additional tool support
- Scheduled scanning
- Advanced filtering
- Plugin system for custom checkers
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
