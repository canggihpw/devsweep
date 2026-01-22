//! DevSweep Library
//!
//! This library provides the core functionality for DevSweep,
//! a macOS desktop application that helps developers reclaim disk space.

pub mod app;
pub mod assets;
pub mod backend;
pub mod cache_settings;
pub mod checkers;
pub mod cleanup_history;
pub mod scan_cache;
pub mod single_instance;
pub mod types;
pub mod ui;
pub mod utils;

// Re-export commonly used types for convenience
pub use backend::{CategoryData, StorageBackend};
pub use cache_settings::CacheSettings;
pub use cleanup_history::{CleanupHistory, CleanupRecord, HistoryStats};
pub use scan_cache::ScanCache;
pub use types::{CheckResult, CleanupItem, ItemDetail};
