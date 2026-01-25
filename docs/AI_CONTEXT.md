# AI Context Document - DevSweep

This document is designed for AI assistants to quickly understand the DevSweep codebase without reading all source files. Read this first before making changes.

## Quick Facts

- **Language**: Rust (2021 edition)
- **UI Framework**: GPUI (Zed's GPU-accelerated UI framework, not Tauri/Electron)
- **Platform**: macOS only (11.0+)
- **Purpose**: Desktop app to clean development caches (node_modules, cargo target, pip cache, etc.)
- **Current Version**: Check `Cargo.toml` for actual version

## Project Structure

```
src/
├── main.rs              # Entry point, window creation, single-instance handling
├── lib.rs               # Module exports
├── backend.rs           # Core scanning logic, StorageBackend struct
├── types.rs             # CleanupItem, CheckResult, ItemDetail structs
├── utils.rs             # format_size(), sort_versions()
├── scan_cache.rs        # ScanCache for caching scan results
├── cleanup_history.rs   # CleanupHistory, quarantine system
├── cache_settings.rs    # CacheSettings, TTL configuration
├── single_instance.rs   # Unix socket for single-instance app
├── update_checker.rs    # GitHub releases API, version comparison
├── custom_paths.rs      # User-defined custom scan paths
├── assets.rs            # Icon loading with rust-embed
├── app/
│   ├── mod.rs           # Re-exports DevSweep
│   ├── state.rs         # DevSweep struct, SuperCategoryType enum
│   ├── actions.rs       # All action methods (scan, cleanup, update check, etc.)
│   ├── render.rs        # Render trait impl, sidebar rendering
│   └── tabs/
│       ├── mod.rs
│       ├── scan_tab.rs      # Main scanning UI with super category grouping
│       ├── quarantine_tab.rs # Quarantine management
│       ├── settings_tab.rs   # Cache TTL settings + custom paths
│       └── about_tab.rs      # App info, update checker UI
├── checkers/            # Scanner modules for each tool type
│   ├── mod.rs           # Exports all checkers
│   ├── nodejs.rs        # npm, yarn, pnpm, node_modules
│   ├── python.rs        # pip, __pycache__, virtualenv
│   ├── rust_cargo.rs    # cargo registry, target dirs
│   ├── docker.rs        # Docker images, containers
│   ├── xcode.rs         # DerivedData, archives
│   ├── homebrew.rs      # Homebrew caches
│   ├── java.rs          # Maven, Gradle
│   ├── go.rs            # Go build cache
│   ├── git.rs           # Git repos: merged branches, stale remotes, large .git
│   ├── ide.rs           # VSCode, JetBrains
│   ├── browser.rs       # Chrome, Firefox, Safari
│   ├── shell.rs         # Shell history, caches
│   ├── db.rs            # PostgreSQL, MySQL logs
│   ├── logs.rs          # System logs
│   └── general.rs       # Trash, general caches
└── ui/
    ├── mod.rs           # Re-exports
    ├── theme.rs         # Theme struct (Catppuccin colors)
    └── sidebar.rs       # Tab enum definition
```

## Key Types and Patterns

### Main App State (`src/app/state.rs`)

```rust
// Super category for grouping related categories
pub enum SuperCategoryType {
    DevelopmentTools,   // Docker, Homebrew, Xcode, IDE Caches
    PackageManagers,    // Node.js, Python, Rust/Cargo, Go, Java
    ProjectFiles,       // node_modules, Git Repositories, Custom Paths
    SystemAndBrowsers,  // System Logs, Browser Caches, Shell, DB, General
    Trash,              // User trash
}

pub struct DevSweep {
    // Core
    pub backend: Arc<Mutex<StorageBackend>>,
    pub active_tab: Tab,
    pub theme_mode: ThemeMode,
    
    // Operation flags
    pub is_scanning: bool,
    pub is_cleaning: bool,
    
    // Scan results (hierarchical)
    pub super_categories: Vec<SuperCategoryItem>, // Top-level grouping
    pub categories: Vec<CategoryItem>,            // UI display data
    pub category_data: Vec<CategoryData>,         // Backend data
    pub all_items: Vec<CleanupItemData>,          // Flattened items
    pub selected_items: Vec<CleanupItem>,         // Items to clean
    
    // Quarantine
    pub quarantine_records: Vec<QuarantineRecordData>,
    pub quarantine_items: Vec<QuarantineItemData>,
    
    // Cache TTL settings (grouped by SuperCategoryType in UI)
    pub cache_ttls: Vec<CacheTTLSetting>,
    
    // Update checker
    pub is_checking_update: bool,
    pub update_info: Option<UpdateInfo>,
    pub update_error: Option<String>,
    pub update_check_completed: bool,
    
    // Custom paths
    pub custom_paths: Vec<CustomPath>,
    pub new_custom_path_input: String,
    pub new_custom_path_label: String,
}
```

### GPUI Patterns

**Rendering**: Uses builder pattern with method chaining:
```rust
div()
    .flex()
    .w_full()
    .bg(Theme::base(theme))
    .child(other_element)
```

**Interactive elements need `.id()`**:
```rust
div()
    .id("my-button")  // Required for on_click!
    .on_click(cx.listener(|this, _event, cx| {
        // handle click
        cx.notify();  // Trigger re-render
    }))
```

**Async operations**:
```rust
pub fn do_async_work(&mut self, cx: &mut ViewContext<Self>) {
    self.is_loading = true;
    cx.notify();
    
    cx.spawn(|this, mut cx| async move {
        // Do work (can use std::thread::spawn for blocking)
        let result = heavy_computation();
        
        // Update UI on main thread
        let _ = cx.update(|cx| {
            let _ = this.update(cx, |this, cx| {
                this.is_loading = false;
                this.result = result;
                cx.notify();
            });
        });
    }).detach();
}
```

**Return types for render methods**:
- `-> Div` for simple static elements
- `-> impl IntoElement` for elements with conditional structure
- `-> AnyElement` when branches return different types (use `.into_any_element()`)

### Checker Pattern (`src/checkers/*.rs`)

Each checker follows this pattern:
```rust
pub fn check_something() -> CheckResult {
    let mut result = CheckResult::new("Category Name");
    
    let path = dirs::home_dir().unwrap().join(".cache/something");
    if path.exists() {
        let size = calculate_size(&path);
        result.add_item(
            CleanupItem::new("Item Type", size)
                .with_path(path)
                .with_safe_to_delete(true)
        );
    }
    
    result
}
```

### Backend Flow

1. `StorageBackend::scan_with_cache(use_cache)` - Runs all checkers
2. Returns `Vec<CategoryData>` with items grouped by category
3. User selects items
4. `StorageBackend::execute_cleanup_with_history(items, use_quarantine)`
5. Items moved to quarantine (can be restored) or deleted

## File Locations

- **Config/Cache files**: `~/.config/devsweep/` (created by the app)
  - `scan_cache.json` - Cached scan results
  - `cleanup_history.json` - Quarantine records
  - `cache_settings.json` - TTL settings
  - `custom_paths.json` - User-defined scan paths
- **Quarantine**: `~/.config/devsweep/quarantine/`
- **Single instance socket**: `/tmp/devsweep-{uid}.sock`

## Dependencies (Key Ones)

- `gpui` - UI framework (from Zed repo, rev v0.167.2)
- `walkdir` - Directory traversal
- `rayon` - Parallel processing
- `serde/serde_json` - Serialization
- `ureq` - HTTP client (for update checker)
- `semver` - Version comparison
- `chrono` - Date/time
- `fs2` - File system operations
- `rust-embed` - Embed assets in binary
- `image` - Image decoding for icons

## Common Tasks

### Adding a New Checker

1. Create `src/checkers/newchecker.rs`
2. Implement `pub fn check_newchecker() -> CheckResult`
3. Add `pub mod newchecker;` and `pub use newchecker::check_newchecker;` to `src/checkers/mod.rs`
4. Add to `all_checks` vector in `src/backend.rs` `scan_with_cache()` method

### Adding UI State

1. Add field to `DevSweep` struct in `src/app/state.rs`
2. Initialize in `DevSweep::new()`
3. Add action method in `src/app/actions.rs`
4. Update relevant tab in `src/app/tabs/`

### Adding a New Super Category

1. Add variant to `SuperCategoryType` enum in `src/app/state.rs`
2. Update `name()`, `icon()`, and `from_category_name()` methods
3. Add to `SuperCategoryType::all()` vector in display order

### Adding a New Tab

1. Add variant to `Tab` enum in `src/ui/sidebar.rs`
2. Implement `icon()` and `label()` for the new variant
3. Create `src/app/tabs/newtab.rs` with `render_newtab()` method
4. Add to match in `src/app/render.rs`
5. Add to sidebar in `render_sidebar()`

## Testing

- Unit tests: `cargo test`
- Test files in `tests/` directory
- Coverage: ~57% (UI excluded)

## Build & Release

- Build: `cargo build --release`
- Create DMG: `./scripts/create-app-bundle.sh`
- CI: `.github/workflows/ci.yml` (tests on PR)
- Release: `.github/workflows/release.yml` (on tag push `v*.*.*`)

## Theme System

Uses Catppuccin colors via `Theme` struct:
- `Theme::base(mode)` - Background
- `Theme::text(mode)` - Primary text
- `Theme::subtext0/1(mode)` - Secondary text
- `Theme::surface0/1/2(mode)` - Surface colors
- `Theme::green/red/blue(mode)` - Accent colors
- `ThemeMode::Light` / `ThemeMode::Dark`

## Update Checker

- Checks GitHub API: `https://api.github.com/repos/canggihpw/devsweep/releases/latest`
- Compares with `env!("CARGO_PKG_VERSION")`
- Shows in sidebar (below app name) and About tab
- Download button opens browser to DMG URL

## Custom Paths System

Users can add custom directories to scan via Settings tab:
- **Config file**: `~/.config/devsweep/custom_paths.json`
- **Data structure**: `CustomPath { path, label, enabled, recursive }`
- **UI**: Settings tab has "Custom Scan Paths" section with Browse/Add/Toggle/Remove
- **Scanning**: `custom_paths::check_custom_paths()` is called alongside other checkers

## Git Repository Cleanup

The git checker (`src/checkers/git.rs`) scans for:
1. **Merged branches**: Local branches already merged into main/master
2. **Stale remote refs**: Remote-tracking branches with no local counterpart
3. **Large .git directories**: Repos with .git > 100MB (suggests `git gc`)

**Important**: All git operations are local-only to avoid credential prompts. The stale remote detection compares local vs remote-tracking branches without contacting the server.

## Important Notes

1. **GPUI is NOT like React/Vue** - No virtual DOM, direct GPU rendering
2. **Single-threaded UI** - Use `cx.spawn()` for async, `std::thread::spawn` for blocking
3. **macOS only** - Uses macOS-specific paths and features
4. **Not code-signed** - Users must bypass Gatekeeper (`xattr -cr`)
5. **Quarantine before delete** - Two-stage deletion for safety
6. **Git operations are local-only** - No network calls to avoid credential prompts in tests/scans
7. **Update checker uses ureq** - Blocking HTTP, must run in background thread
8. **Super categories group related checkers** - TTL settings and scan results use same grouping
9. **Category names must match** - `SuperCategoryType::from_category_name()` must match actual checker names
