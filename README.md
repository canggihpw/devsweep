# DevSweep

[![Release](https://img.shields.io/github/v/release/canggihpw/devsweep)](https://github.com/canggihpw/devsweep/releases)
[![CI](https://img.shields.io/github/actions/workflow/status/canggihpw/devsweep/ci.yml?branch=main&label=CI)](https://github.com/canggihpw/devsweep/actions/workflows/ci.yml)
[![codecov](https://codecov.io/gh/canggihpw/devsweep/branch/main/graph/badge.svg)](https://codecov.io/gh/canggihpw/devsweep)
[![License](https://img.shields.io/github/license/canggihpw/devsweep)](LICENSE)
[![macOS](https://img.shields.io/badge/macOS-11.0+-blue)](https://github.com/canggihpw/devsweep)
[![Rust](https://img.shields.io/badge/rust-1.70+-orange)](https://www.rust-lang.org/)

A powerful macOS desktop application for cleaning up development caches and temporary files, built with Rust and GPUI.

## Overview

DevSweep helps you reclaim disk space by safely removing caches and temporary files created by development tools like Docker, Homebrew, Node.js, Python, Rust, Xcode, and more. Built with modern Rust and the GPUI framework (from Zed editor), it provides a fast, native macOS experience.

## Features

### 🔍 Scan Tab
- **Smart Scanning**: Primary "Scan" button uses cache for instant results
- **Full Rescan Option**: Secondary "Full Rescan" link for complete fresh analysis
- **Intelligent Caching**: Detects file changes and cache expiration automatically
- **17+ Categories**: Organized by tool (Docker, Node.js, Python, Xcode, Git, etc.)
- **Super Categories**: Items grouped into logical sections (Development Tools, Package Managers, Project Files, System & Browsers, Trash)
- **Size Threshold Filters**: Filter results by size (> 1MB, > 10MB, > 100MB, > 500MB, > 1GB)
- **Size Visualization**: Shows exact size for each item and category
- **Selective Cleanup**: Choose exactly what to delete with checkboxes
- **Scrollable Content**: Smooth scrolling through large result sets
- **Safety Warnings**: Highlights potentially risky items
- **Responsive UI**: Non-blocking operations with immediate visual feedback
- **Click Feedback**: All buttons provide tactile feedback on interaction

### 💾 Quarantine Tab
- **Safe Deletion**: Files are quarantined instead of permanently deleted
- **Undo Support**: Restore deleted files with one click (per record)
- **Individual Item Deletion**: Delete specific quarantined items permanently
- **Cleanup History**: View all past cleanup operations with timestamps
- **Expandable Records**: Click to see all items in each cleanup operation
- **Bulk Operations**: 
  - "Undo All" button per record to restore all items
  - "Delete All" button to clear entire quarantine
- **Storage Stats**: Real-time quarantine size and item count
- **Scrollable History**: Navigate through long cleanup history
- **Smart Cleanup**: Automatically removes old quarantine when exceeding 10GB

### 📈 Trends Tab
- **Storage Trends**: Visualize space usage patterns over time
- **Time Range Filters**: View data for Week, Month, Quarter, or All Time
- **Summary Cards**: Space freed, net change, and cleanup count at a glance
- **Bar Chart Visualization**: Visual representation of storage snapshots
- **Category Breakdown**: See which categories contribute most to storage
- **Automatic Tracking**: Snapshots recorded after each scan
- **Persistent History**: Trend data saved across app sessions

### 🔌 Ports Tab
- **Port Scanner**: View all processes listening on network ports
- **Search/Filter**: Filter by port number or process name
- **Common Ports**: Quick-access buttons for developer ports (3000, 5000, 8000, 8080, etc.)
- **Process Details**: See PID, user, protocol, and connection state
- **Kill Processes**: Terminate processes blocking ports with one click
- **Auto-Refresh**: Automatically rescans after killing a process

### ⚙️ Settings Tab
- **Cache TTL Configuration**: Customize how long scan results are cached per category
- **Per-Category Control**: Set different TTL values for each tool category
- **Grouped by Super Category**: TTL settings organized by Development Tools, Package Managers, etc.
- **Custom Scan Paths**: Add your own directories to include in scans
- **Browse or Type Paths**: Use native folder picker or enter paths manually
- **Toggle Custom Paths**: Enable/disable individual custom paths without removing them
- **Reset to Defaults**: Quick button to restore default settings
- **Persistent Settings**: All settings saved and restored on app restart
- **Scrollable Interface**: Easy navigation through all settings

### ℹ️ About Tab
- **App Information**: Version, description, and credits
- **Update Checker**: Automatically checks for new releases on GitHub
- **Download Updates**: Direct link to download new versions when available
- **Feature List**: Complete overview of capabilities
- **Technology Stack**: Built with Rust + GPUI
- **Clean Design**: Centered layout with app logo

### ⚡ Performance
- **Parallel Scanning**: Multi-threaded analysis using Rayon
- **Smart Caching**: Remembers previous results with configurable TTL
- **Path Tracking**: Detects file system changes to invalidate cache
- **Background Operations**: All I/O runs in background threads
- **Instant UI Response**: GPUI ensures smooth 60fps interface
- **Optimized Builds**: LTO and optimizations enabled for release builds

## Supported Categories

The app scans and cleans the following (17 categories), organized into super categories:

### Development Tools
1. **Docker** - Images, containers, volumes, build cache
2. **Homebrew** - Cache, downloads, old versions, logs
3. **Xcode** - DerivedData, archives, device support
4. **IDE Caches** - VS Code, JetBrains IDEs, Sublime Text

### Package Managers
5. **Node.js Package Managers** - npm, yarn, pnpm global caches
6. **Python** - pip cache, __pycache__ folders, virtualenvs
7. **Rust/Cargo** - Registry cache, git checkouts, target directories
8. **Go** - Build cache, module cache
9. **Java Build Tools** - Gradle cache, Maven repository

### Project Files
10. **node_modules in Projects** - Project dependencies (~/Documents, ~/Projects)
11. **Git Repositories** - Merged branches, stale remotes, large .git directories
12. **Custom Paths** - User-defined directories to scan

### System & Browsers
13. **System Logs & Crash Reports** - Crash reports, diagnostic reports, app logs
14. **Browser Caches** - Safari, Chrome, Brave, Firefox, Edge, Arc, Opera, Vivaldi
15. **Shell Caches** - Oh My Zsh, Powerlevel10k, zsh plugins
16. **Database Caches** - PostgreSQL, MySQL, MongoDB, Redis logs
17. **General Caches** - Large application caches

### Trash
18. **Trash** - User trash contents

## Installation

### Option 1: Download Release (Recommended)

1. Download the latest `.dmg` from releases
2. Open the DMG file
3. Drag "DevSweep.app" to Applications folder
4. Right-click the app and select "Open" (first time only, macOS Gatekeeper)
5. Grant Full Disk Access in System Preferences → Privacy & Security → Full Disk Access

### Option 2: Build from Source

**Requirements:**
- macOS 11.0 (Big Sur) or later
- Rust 1.70+ (install via [rustup](https://rustup.rs))
- Xcode Command Line Tools

```bash
# Clone the repository
git clone https://github.com/canggihpw/devsweep.git
cd devsweep

# Build release version
cargo build --release

# Run the app
./target/release/devsweep
```

### Option 3: Create App Bundle + DMG

```bash
# Build and create .app bundle + .dmg installer
./create-app-bundle.sh

# This creates:
# - DevSweep.app (macOS app bundle)
# - DevSweep-0.2.0.dmg (ready to distribute)
```

## Usage

### Quick Start

1. **Launch** the app from Applications or Spotlight
2. **Scan** your system:
   - Click **"Scan"** for quick cached scan (instant if cache valid)
   - Click **"Full Rescan"** for complete fresh analysis
3. **Review** the results in expandable categories
4. **Select** items you want to clean using checkboxes
   - Use "Select All" / "Deselect All" for bulk operations
5. **Clean** selected items with the "Clean Selected" button
6. **Undo** if needed from the Quarantine tab

### Tab Navigation

#### 🔍 Scan Tab
- **Primary action**: "Scan" button (uses cache for speed)
- **Secondary action**: "Full Rescan" link (bypasses cache)
- **Size filter dropdown**: Filter by size threshold (All, > 1MB, > 10MB, > 100MB, > 500MB, > 1GB)
- Click category headers to expand/collapse items
- Check categories to select all items within
- Individual item selection with checkboxes
- "Safe" badges indicate low-risk deletions
- Warning (⚠) badges for items needing caution

#### 💾 Quarantine Tab
- View all cleanup operations with timestamps
- Click record headers to expand and see individual items
- **Per-record actions**:
  - "Undo All" - restore all items from that cleanup
- **Per-item actions**:
  - "Delete" button - permanently delete individual items
- **Global actions**:
  - "Delete All" - clear entire quarantine
  - "Refresh" - update quarantine view
  - "Open in Finder" - browse quarantine directory
- ✓ Green checkmark = successful operation
- ✗ Red X = failed operation with error message

#### ⚙️ Settings Tab
- Configure cache TTL (Time To Live) for each category
- TTL settings grouped by super category for easier navigation
- 0 minutes = always rescan (no cache)
- Higher values = faster subsequent scans
- **Custom Scan Paths**:
  - Click "Browse Folder..." to select directories
  - Or "Enter Path..." to type a path manually
  - Toggle paths on/off without removing them
  - Remove paths with the X button
- "Reset to Defaults" restores recommended settings
- All settings persist between app launches

#### ℹ️ About Tab
- App version and description
- **Update checker**: Automatically checks GitHub for new releases
- **Download button**: Opens browser to download latest DMG when update available
- Feature highlights
- Technology stack information
- Credits and acknowledgments

### Understanding Quarantine

When you clean files, they're moved to quarantine (not deleted):

- **Location**: `~/Library/Caches/development-cleaner/quarantine/`
- **Duration**: Until you "Undo All" or "Delete All"
- **Auto-cleanup**: Quarantine cleared when exceeding 10GB
- **Individual control**: Delete specific items with "Delete" button
- **Record-level undo**: "Undo All" per cleanup operation
- **Safety**: Original paths preserved for accurate restoration

**Note**: Items deleted via commands (not file paths) cannot be quarantined and are permanent.

### Safety Tips

✅ **Safe to delete** (marked with "Safe" badge):
- npm/yarn/pip caches
- Docker build cache
- Homebrew downloads
- Browser caches
- Temporary build files

⚠️ **Use caution**:
- Docker images/containers you're actively using
- Python virtualenvs for current projects
- Database data (marked with warnings)
- node_modules in active projects

🛡️ **Always safe with quarantine**:
- All file-based deletions can be undone
- Review quarantine history before "Delete All"
- Individual item deletion for precise control

**Best practice**: 
1. Start with cached scans ("Scan" button)
2. Review results carefully
3. Clean conservatively at first
4. Check quarantine before permanent deletion
5. Use "Full Rescan" periodically for accuracy

## Building the App

### Requirements

- **macOS**: 11.0 (Big Sur) or later (GPUI requirement)
- **Rust**: 1.70 or later
- **Xcode Command Line Tools**: `xcode-select --install`
- **Git**: For cloning the repository

### Build Commands

```bash
# Development build (with debug symbols)
cargo build

# Release build (optimized)
cargo build --release

# Run directly
cargo run --release

# Run with logging
RUST_LOG=debug cargo run --release
```

### Create App Bundle

The `create-app-bundle.sh` script automates app bundle creation:

```bash
# Make script executable (first time)
chmod +x create-app-bundle.sh

# Build app bundle and DMG
./create-app-bundle.sh
```

This script:
1. ✅ Generates app icon from SVG (if needed)
2. ✅ Builds release binary
3. ✅ Creates .app bundle structure
4. ✅ Adds Info.plist with metadata
5. ✅ Code signs with entitlements (ad-hoc)
6. ✅ Creates distributable DMG
7. ✅ Adds README to DMG

### macOS Permissions

The app requires Full Disk Access to scan all directories:

1. Open **System Preferences** → **Privacy & Security**
2. Select **Full Disk Access** in the sidebar
3. Click the **+** button
4. Add **DevSweep.app**
5. Restart the app

## Architecture

```
devsweep/
├── src/
│   ├── main.rs              # App entry point
│   ├── lib.rs               # Library exports
│   ├── single_instance.rs   # Unix socket single-instance handling
│   ├── assets.rs            # Embedded assets (rust-embed + image loading)
│   ├── backend.rs           # Core scanning and cleanup logic
│   ├── types.rs             # Data structures
│   ├── utils.rs             # Helper functions
│   ├── scan_cache.rs        # Cache management with TTL
│   ├── cleanup_history.rs   # Quarantine and undo system
│   ├── cache_settings.rs    # Settings persistence
│   ├── update_checker.rs    # GitHub releases API, version comparison
│   ├── custom_paths.rs      # User-defined custom scan paths
│   ├── trends.rs            # Storage trends tracking and history
│   ├── app/                 # GPUI application components
│   │   ├── mod.rs           # Module exports
│   │   ├── state.rs         # Application state (DevSweep struct, SuperCategoryType)
│   │   ├── actions.rs       # Action handlers (scan, clean, update check, etc.)
│   │   ├── render.rs        # Main UI rendering, sidebar
│   │   └── tabs/            # Tab-specific UI
│   │       ├── mod.rs
│   │       ├── scan_tab.rs
│   │       ├── trends_tab.rs
│   │       ├── quarantine_tab.rs
│   │       ├── settings_tab.rs
│   │       └── about_tab.rs
│   ├── checkers/            # Category-specific scanners
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
│   │   └── git.rs           # Git repos: merged branches, stale remotes
│   └── ui/
│       ├── mod.rs
│       ├── sidebar.rs       # Tab definitions and icons
│       └── theme.rs         # Catppuccin Latte/Mocha colors
├── assets/
│   ├── image-dark.svg       # Logo for light theme
│   ├── image-light.svg      # Logo for dark theme
│   ├── icon-dark.png        # Generated icon for light theme
│   ├── icon-light.png       # Generated icon for dark theme
│   └── logo.icns            # macOS app bundle icon
├── scripts/
│   ├── create-app-bundle.sh # Bundle + DMG creator
│   ├── create-icon.sh       # Icon generator from SVG
│   └── entitlements.plist   # macOS permissions
├── docs/                    # Documentation
└── Cargo.toml               # Dependencies
```

## Key Technologies

- **[Rust](https://www.rust-lang.org/)** - Systems programming language
- **[GPUI](https://github.com/zed-industries/zed)** - GPU-accelerated UI framework from Zed
- **[Rayon](https://github.com/rayon-rs/rayon)** - Data parallelism
- **[Serde](https://serde.rs/)** - Serialization framework
- **[WalkDir](https://github.com/BurntSushi/walkdir)** - Recursive directory traversal
- **[Chrono](https://github.com/chronotope/chrono)** - Date and time handling
- **[rust-embed](https://github.com/pyrossh/rust-embed)** - Embed assets in binary
- **[image](https://github.com/image-rs/image)** - Image decoding for icons
- **[ureq](https://github.com/algesten/ureq)** - HTTP client for update checking
- **[semver](https://github.com/dtolnay/semver)** - Semantic version parsing and comparison
- **[Catppuccin](https://github.com/catppuccin/catppuccin)** - Latte/Mocha theme palettes

## How It Works

### Smart Scanning with Cache

1. **Initial Scan**: 
   - User clicks "Scan" button
   - Checks cache validity (TTL not expired, files unchanged)
   - Returns cached results instantly if valid

2. **Cache Invalidation**:
   - Tracks file metadata (modification times, sizes)
   - Detects when directories change
   - Respects per-category TTL settings

3. **Full Rescan**:
   - User clicks "Full Rescan" link
   - Bypasses cache completely
   - Scans all categories fresh
   - Updates cache with new results

4. **Parallel Processing**:
   - All category checks run in parallel using Rayon
   - Results collected and merged
   - UI updated on main thread

### Quarantine System

1. **Deletion**:
   - Files moved to `~/Library/Caches/development-cleaner/quarantine/`
   - Original paths stored in cleanup record
   - Timestamp and metadata preserved
   - Individual items tracked with indices

2. **Record Structure**:
   - Each cleanup operation creates a record
   - Contains: timestamp, item list, success/error counts
   - Items marked as: quarantined, permanently deleted, or failed

3. **Restoration**:
   - "Undo All" per record: restores all items from that cleanup
   - Items moved back to original locations
   - Parent directories created if missing
   - Errors reported for conflicts

4. **Individual Deletion**:
   - "Delete" button on each quarantined item
   - Permanently removes from quarantine
   - Updates record to mark as deleted
   - Cannot be undone after this point

5. **Automatic Cleanup**:
   - Monitors quarantine directory size
   - Removes oldest records when exceeding 10GB
   - Maintains 80% of limit after cleanup

### Data Persistence

- **Scan Cache**: `~/Library/Caches/development-cleaner/scan_cache.json`
- **Cleanup History**: `~/Library/Caches/development-cleaner/cleanup_history.json`
- **Trends History**: `~/Library/Caches/development-cleaner/trends_history.json`
- **Settings**: `~/Library/Application Support/development-cleaner/cache_settings.json`
- **Custom Paths**: `~/Library/Application Support/devsweep/custom_paths.json`
- **Quarantine Files**: `~/Library/Caches/development-cleaner/quarantine/`

## Troubleshooting

### App requires Full Disk Access

**Symptom**: Some directories show as empty or inaccessible

**Solution**:
1. System Preferences → Privacy & Security → Full Disk Access
2. Add DevSweep.app
3. Restart the app

### App won't open ("unidentified developer")

**Symptom**: macOS Gatekeeper blocks the app

**Solution**:
1. Right-click DevSweep.app
2. Select "Open"
3. Click "Open" in the dialog
4. For subsequent launches, double-click works normally

### Scan button stuck or unresponsive

**Symptom**: Clicking "Scan" doesn't respond

**Solution**:
- Wait for current operation to complete
- Look for status in header (e.g., "Scanning..." or "Cleaning...")
- Buttons are disabled during operations to prevent conflicts
- This is a feature, not a bug!

### Cache seems outdated

**Symptom**: Scan results don't reflect recent changes

**Solution**:
- Click "Full Rescan" to bypass cache
- Or adjust TTL in Settings tab to 0 for always-fresh results
- Cache auto-invalidates when files change, but may miss some updates

### Build errors

**Common issues**:

```bash
# GPUI dependency issue
cargo clean
cargo update
cargo build --release

# macOS SDK issue
xcode-select --install

# Rust version too old
rustup update stable
```

## Code Signing & Distribution

### For Personal Use

The app is automatically signed with ad-hoc signature:

```bash
# Already done by create-app-bundle.sh
codesign --force --deep --sign - --entitlements entitlements.plist "DevSweep.app"
```

### For Distribution

Requires Apple Developer account ($99/year):

```bash
# Sign with Developer ID
codesign --force --deep \
  --sign "Developer ID Application: Your Name (TEAM_ID)" \
  --entitlements entitlements.plist \
  --options runtime \
  "DevSweep.app"

# Sign DMG
codesign --sign "Developer ID Application: Your Name (TEAM_ID)" \
  "DevSweep-0.2.0.dmg"

# Notarize (required for distribution)
xcrun notarytool submit "DevSweep-0.2.0.dmg" \
  --apple-id "your@email.com" \
  --team-id "TEAM_ID" \
  --password "app-specific-password"

# Staple notarization ticket
xcrun stapler staple "DevSweep-0.2.0.dmg"
```

## Performance

### Scan Performance

- **Cached scan**: < 100ms (instant)
- **Full rescan**: 2-10 seconds (depending on system)
- **16 categories scanned in parallel**
- **~1-2 GB/sec throughput** on modern SSDs

### Storage Impact

- **App size**: ~15-20 MB
- **Cache storage**: < 1 MB typically
- **Quarantine**: Up to 10 GB (auto-cleanup)
- **Settings**: < 1 KB

**Typical space reclaimed**:
- Light users: 5-20 GB
- Active developers: 50-200 GB
- Heavy Docker/Node.js users: 200+ GB

### Resource Usage

- **Memory**: 50-100 MB during idle
- **CPU**: < 5% during UI interaction
- **Disk I/O**: Minimal (read-only scanning)
- **GPU**: GPUI uses Metal for rendering

## Roadmap

### Completed Features

- [x] **Dark mode**: Toggle between light/dark themes
  - Uses Catppuccin Latte (light) and Mocha (dark) palettes
  - Theme-aware icons that adapt to current mode
- [x] **Single-instance app**: Prevents multiple windows when reopening
- [x] **Comprehensive testing**: 332 tests with 57% line coverage (excluding UI)
- [x] **Performance benchmarks**: Cached scans complete in < 100ms
- [x] **Custom scan paths**: Add your own directories to scan via Settings tab
- [x] **Git repository cleanup**: Merged branches, stale remotes, large .git directories
- [x] **Update checker**: Automatic GitHub release checking with download links
- [x] **Super categories**: Logical grouping of scan results (Development Tools, Package Managers, etc.)
- [x] **Size threshold filters**: Filter scan results by size (> 1MB, > 10MB, > 100MB, > 500MB, > 1GB)
- [x] **Non-blocking UI**: Background processing for long-running operations
- [x] **Storage trends**: Visualize space usage over time with trend charts
  - Track storage consumption patterns across scans
  - View space freed and net changes over time
  - Time range filters (Week, Month, Quarter, All Time)
  - Category-level trend breakdown
- [x] **Port manager**: List processes using specific ports and kill them with one click
  - View all active port usage on the system
  - Search/filter by port number or process name
  - Kill processes blocking ports you need for local development
  - Common ports quick-access (3000, 5000, 8000, 8080, etc.)

### Planned Features

#### Developer Workflow
- [ ] **Language server caches**: Clean LSP data, TypeScript servers, Rust Analyzer caches
- [ ] **Build artifact cleanup**: Old build outputs, .o files, intermediate artifacts
- [ ] **Container image pruning**: Smart Docker/Podman image cleanup based on age and usage
- [ ] **Dependency audit**: Identify outdated or unused dependencies across projects
- [ ] **Project archiver**: Archive inactive projects (compress node_modules, target dirs)

#### Automation & Scheduling
- [ ] **Scheduled cleanups**: Automatic cleaning on schedule (daily, weekly, monthly)
- [ ] **Trash schedule**: Auto-empty trash older than X days
- [ ] **Cleanup profiles**: Save and load cleanup configurations for different workflows
- [ ] **CLI mode**: Run scans and cleanups from terminal for CI/CD integration

#### Analysis & Reporting
- [ ] **Export reports**: Save scan results as CSV/JSON for tracking
- [ ] **Smart recommendations**: AI-based cleanup suggestions based on usage patterns
- [ ] **Disk usage heatmap**: Visual representation of storage consumption by directory
- [ ] **Project size tracking**: Monitor how project sizes change over time

#### Integration & Access
- [ ] **Menu bar mode**: Quick access from menu bar with storage summary
- [ ] **Whitelist/blacklist**: Exclude specific paths from scans
- [ ] **Cloud storage cleanup**: Google Drive, Dropbox, iCloud caches
- [ ] **Spotlight integration**: Quick launch and search via Spotlight

#### Developer-Specific Cleanups
- [ ] **Test coverage artifacts**: Clean old coverage reports (.nyc_output, coverage/)
- [ ] **Debug symbols**: Remove .dSYM files and debug artifacts
- [ ] **Stale branches cleanup**: Interactive pruning of old Git branches
- [ ] **Lock file deduplication**: Identify duplicate lock files across projects
- [ ] **Virtual environment cleanup**: Find and remove orphaned venvs, conda envs
- [ ] **Compiler cache management**: ccache, sccache cleanup with size limits

### Contributing

Contributions are welcome! Please:

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests if applicable
5. Submit a pull request

## License

MIT License - see LICENSE file for details

## Acknowledgments

- **GPUI Framework**: Thanks to Zed team for the amazing UI framework
- **Catppuccin Latte**: Beautiful light mode color scheme
- **Rust Community**: Excellent ecosystem and tools
- **macOS Developer Community**: Inspiration from various cleanup tools

## Disclaimer

This tool modifies and deletes files on your system. While it includes safety features (quarantine, undo, warnings), use at your own risk. Always review what you're deleting and maintain backups of important data.

The quarantine system provides a safety net, but files can be permanently deleted with "Delete" or "Delete All" actions. These operations cannot be undone.

**Recommended workflow**:
1. Use "Scan" for quick cached results
2. Review selections carefully
3. Clean conservatively
4. Verify quarantine before permanent deletion
5. Use "Full Rescan" when you need fresh data

## FAQ

**Q: Is it safe to delete everything marked as "Safe"?**  
A: Generally yes, but review the list. These are caches and temporary files that can be regenerated.

**Q: Will cleaning break my development environment?**  
A: No, but you may need to re-download packages or rebuild projects. The quarantine system lets you undo if issues arise.

**Q: How often should I run this?**  
A: Weekly or monthly for active developers. Use "Scan" for quick checks, "Full Rescan" monthly.

**Q: Can I recover deleted files?**  
A: Yes! Use "Undo All" in the Quarantine tab before clicking "Delete All". Individual items can also be restored or deleted.

**Q: Why does it need Full Disk Access?**  
A: To scan system directories, caches, and your home folder. This permission is required by macOS.

**Q: What's the difference between "Scan" and "Full Rescan"?**  
A: "Scan" uses cached results for speed (instant if cache valid). "Full Rescan" bypasses cache for complete fresh analysis.

**Q: Does the quarantine take up space?**  
A: Yes, up to 10 GB. It auto-cleans when exceeding this limit. You can manually "Delete All" anytime.

**Q: Can I customize which categories to scan?**  
A: All built-in categories are always scanned, but you can add custom paths in Settings. Use checkboxes to select what to clean.

**Q: Is my data sent anywhere?**  
A: No. Everything runs locally. The only network request is the optional update checker that queries GitHub's public API for new releases. No personal data is collected or transmitted.

**Q: Why GPUI instead of other UI frameworks?**  
A: GPUI provides native macOS performance with GPU acceleration, smooth 60fps, and is built for developer tools.

---

**Built with ❤️ using Rust and GPUI**

For issues, feature requests, or contributions, visit the [GitHub repository](https://github.com/canggihpw/devsweep).