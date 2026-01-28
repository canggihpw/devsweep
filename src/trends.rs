//! Storage Trends Module
//!
//! This module provides functionality for tracking and visualizing storage usage over time.
//! It maintains historical snapshots of storage data and computes trend information for display.

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, VecDeque};
use std::fs;
use std::path::PathBuf;
use std::time::SystemTime;

/// Maximum number of snapshots to keep in history
const MAX_SNAPSHOTS: usize = 365; // Keep up to 1 year of daily snapshots

/// Minimum interval between automatic snapshots (in seconds)
const MIN_SNAPSHOT_INTERVAL: u64 = 3600; // 1 hour

/// A single point in time capturing storage state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageSnapshot {
    /// When this snapshot was taken
    pub timestamp: SystemTime,
    /// Total reclaimable space across all categories (bytes)
    pub total_reclaimable: u64,
    /// Size per category at this point in time
    pub category_sizes: HashMap<String, u64>,
    /// Space freed by cleanup operations since last snapshot (if any)
    pub space_freed: Option<u64>,
    /// Available disk space at snapshot time
    pub disk_available: Option<u64>,
}

impl StorageSnapshot {
    pub fn new(
        total_reclaimable: u64,
        category_sizes: HashMap<String, u64>,
        disk_available: Option<u64>,
    ) -> Self {
        Self {
            timestamp: SystemTime::now(),
            total_reclaimable,
            category_sizes,
            space_freed: None,
            disk_available,
        }
    }

    /// Get the age of this snapshot in seconds
    pub fn age_seconds(&self) -> u64 {
        self.timestamp
            .elapsed()
            .map(|d| d.as_secs())
            .unwrap_or(0)
    }
}

/// Time range for viewing trends
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TrendTimeRange {
    /// Last 7 days
    #[default]
    Week,
    /// Last 30 days
    Month,
    /// Last 90 days
    Quarter,
    /// All available data
    AllTime,
}

impl TrendTimeRange {
    pub fn label(&self) -> &'static str {
        match self {
            Self::Week => "7 Days",
            Self::Month => "30 Days",
            Self::Quarter => "90 Days",
            Self::AllTime => "All Time",
        }
    }

    pub fn days(&self) -> Option<u32> {
        match self {
            Self::Week => Some(7),
            Self::Month => Some(30),
            Self::Quarter => Some(90),
            Self::AllTime => None,
        }
    }

    pub fn all_options() -> Vec<Self> {
        vec![Self::Week, Self::Month, Self::Quarter, Self::AllTime]
    }
}

/// Computed trend data for display
#[derive(Debug, Clone)]
pub struct TrendData {
    /// Data points for the chart (timestamp, value)
    pub points: Vec<(SystemTime, u64)>,
    /// Minimum value in the range
    pub min_value: u64,
    /// Maximum value in the range
    pub max_value: u64,
    /// Total space freed in the time range
    pub total_freed: u64,
    /// Net change in reclaimable space (positive = grew, negative = shrunk)
    pub net_change: i64,
    /// Number of cleanup operations in the range
    pub cleanup_count: u32,
}

impl TrendData {
    pub fn empty() -> Self {
        Self {
            points: Vec::new(),
            min_value: 0,
            max_value: 0,
            total_freed: 0,
            net_change: 0,
            cleanup_count: 0,
        }
    }
}

/// Per-category trend data
#[derive(Debug, Clone)]
pub struct CategoryTrendData {
    pub category: String,
    pub points: Vec<(SystemTime, u64)>,
    pub current_size: u64,
    pub change: i64,
}

/// Manages storage trend history
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrendsHistory {
    /// Historical snapshots ordered from newest to oldest
    pub snapshots: VecDeque<StorageSnapshot>,
    /// Timestamp of last snapshot
    pub last_snapshot_time: Option<SystemTime>,
    /// Running total of space freed (resets after 30 days of no activity)
    pub cumulative_freed: u64,
}

impl TrendsHistory {
    pub fn new() -> Self {
        Self {
            snapshots: VecDeque::new(),
            last_snapshot_time: None,
            cumulative_freed: 0,
        }
    }

    /// Load trends history from disk
    pub fn load() -> Self {
        let path = Self::history_file_path();
        if let Ok(data) = fs::read_to_string(&path) {
            if let Ok(history) = serde_json::from_str(&data) {
                return history;
            }
        }
        Self::new()
    }

    /// Save trends history to disk
    pub fn save(&self) -> Result<(), String> {
        let path = Self::history_file_path();

        // Ensure parent directory exists
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)
                .map_err(|e| format!("Failed to create trends directory: {}", e))?;
        }

        let json = serde_json::to_string_pretty(self)
            .map_err(|e| format!("Failed to serialize trends: {}", e))?;

        fs::write(&path, json).map_err(|e| format!("Failed to write trends file: {}", e))?;

        Ok(())
    }

    /// Get the history file path
    fn history_file_path() -> PathBuf {
        let cache_dir = dirs::cache_dir().unwrap_or_else(|| PathBuf::from("/tmp"));
        cache_dir
            .join("development-cleaner")
            .join("trends_history.json")
    }

    /// Add a new snapshot to history
    pub fn add_snapshot(&mut self, snapshot: StorageSnapshot) {
        // Check if enough time has passed since last snapshot
        if let Some(last_time) = self.last_snapshot_time {
            if let Ok(elapsed) = last_time.elapsed() {
                if elapsed.as_secs() < MIN_SNAPSHOT_INTERVAL {
                    // Update the most recent snapshot instead of adding new one
                    if let Some(latest) = self.snapshots.front_mut() {
                        latest.total_reclaimable = snapshot.total_reclaimable;
                        latest.category_sizes = snapshot.category_sizes;
                        latest.disk_available = snapshot.disk_available;
                        latest.timestamp = snapshot.timestamp;
                    }
                    return;
                }
            }
        }

        self.snapshots.push_front(snapshot);
        self.last_snapshot_time = Some(SystemTime::now());

        // Limit history size
        while self.snapshots.len() > MAX_SNAPSHOTS {
            self.snapshots.pop_back();
        }
    }

    /// Record space freed from a cleanup operation
    pub fn record_cleanup(&mut self, space_freed: u64) {
        self.cumulative_freed += space_freed;

        // Update the most recent snapshot if it exists
        if let Some(latest) = self.snapshots.front_mut() {
            latest.space_freed = Some(latest.space_freed.unwrap_or(0) + space_freed);
        }
    }

    /// Get trend data for the specified time range
    pub fn get_trend_data(&self, range: TrendTimeRange) -> TrendData {
        if self.snapshots.is_empty() {
            return TrendData::empty();
        }

        let now = SystemTime::now();
        let cutoff = range.days().map(|days| {
            now.checked_sub(std::time::Duration::from_secs(days as u64 * 24 * 3600))
                .unwrap_or(SystemTime::UNIX_EPOCH)
        });

        let filtered: Vec<_> = self
            .snapshots
            .iter()
            .filter(|s| {
                if let Some(cutoff_time) = cutoff {
                    s.timestamp >= cutoff_time
                } else {
                    true
                }
            })
            .collect();

        if filtered.is_empty() {
            return TrendData::empty();
        }

        let points: Vec<_> = filtered
            .iter()
            .map(|s| (s.timestamp, s.total_reclaimable))
            .collect();

        let values: Vec<u64> = points.iter().map(|(_, v)| *v).collect();
        let min_value = values.iter().copied().min().unwrap_or(0);
        let max_value = values.iter().copied().max().unwrap_or(0);

        let total_freed: u64 = filtered.iter().filter_map(|s| s.space_freed).sum();

        let cleanup_count = filtered.iter().filter(|s| s.space_freed.is_some()).count() as u32;

        // Calculate net change (first snapshot vs last snapshot in range)
        let net_change = if let (Some(oldest), Some(newest)) = (filtered.last(), filtered.first()) {
            newest.total_reclaimable as i64 - oldest.total_reclaimable as i64
        } else {
            0
        };

        TrendData {
            points,
            min_value,
            max_value,
            total_freed,
            net_change,
            cleanup_count,
        }
    }

    /// Get per-category trend data
    pub fn get_category_trends(&self, range: TrendTimeRange) -> Vec<CategoryTrendData> {
        if self.snapshots.is_empty() {
            return Vec::new();
        }

        let now = SystemTime::now();
        let cutoff = range.days().map(|days| {
            now.checked_sub(std::time::Duration::from_secs(days as u64 * 24 * 3600))
                .unwrap_or(SystemTime::UNIX_EPOCH)
        });

        let filtered: Vec<_> = self
            .snapshots
            .iter()
            .filter(|s| {
                if let Some(cutoff_time) = cutoff {
                    s.timestamp >= cutoff_time
                } else {
                    true
                }
            })
            .collect();

        if filtered.is_empty() {
            return Vec::new();
        }

        // Collect all category names
        let mut all_categories: std::collections::HashSet<String> = std::collections::HashSet::new();
        for snapshot in &filtered {
            for cat in snapshot.category_sizes.keys() {
                all_categories.insert(cat.clone());
            }
        }

        // Build trend data for each category
        let mut result: Vec<CategoryTrendData> = all_categories
            .into_iter()
            .map(|category| {
                let points: Vec<_> = filtered
                    .iter()
                    .filter_map(|s| {
                        s.category_sizes
                            .get(&category)
                            .map(|&size| (s.timestamp, size))
                    })
                    .collect();

                let current_size = filtered
                    .first()
                    .and_then(|s| s.category_sizes.get(&category))
                    .copied()
                    .unwrap_or(0);

                let oldest_size = filtered
                    .last()
                    .and_then(|s| s.category_sizes.get(&category))
                    .copied()
                    .unwrap_or(0);

                let change = current_size as i64 - oldest_size as i64;

                CategoryTrendData {
                    category,
                    points,
                    current_size,
                    change,
                }
            })
            .collect();

        // Sort by current size (largest first)
        result.sort_by(|a, b| b.current_size.cmp(&a.current_size));

        result
    }

    /// Get the most recent snapshot
    pub fn latest_snapshot(&self) -> Option<&StorageSnapshot> {
        self.snapshots.front()
    }

    /// Get the number of snapshots
    pub fn snapshot_count(&self) -> usize {
        self.snapshots.len()
    }

    /// Check if we have enough data for meaningful trends
    pub fn has_sufficient_data(&self) -> bool {
        self.snapshots.len() >= 2
    }
}

impl Default for TrendsHistory {
    fn default() -> Self {
        Self::new()
    }
}

/// Renders a simple ASCII bar chart for the terminal or as visual representation
pub fn render_ascii_chart(data: &TrendData, width: usize, height: usize) -> Vec<String> {
    if data.points.is_empty() || height == 0 || width == 0 {
        return vec!["No data available".to_string()];
    }

    let mut lines = Vec::new();
    let range = if data.max_value > data.min_value {
        data.max_value - data.min_value
    } else {
        1
    };

    // Sample points to fit width
    let points = &data.points;
    let step = if points.len() > width {
        points.len() / width
    } else {
        1
    };

    let sampled: Vec<u64> = points.iter().step_by(step).map(|(_, v)| *v).collect();

    // Build chart rows (top to bottom)
    for row in 0..height {
        let threshold = data.max_value - (range * row as u64 / height as u64);
        let mut line = String::new();

        for &value in &sampled {
            if value >= threshold {
                line.push('█');
            } else if value >= threshold.saturating_sub(range / (height as u64 * 2)) {
                line.push('▄');
            } else {
                line.push(' ');
            }
        }

        lines.push(line);
    }

    // Add bottom border
    lines.push("─".repeat(sampled.len().min(width)));

    lines
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_snapshot_creation() {
        let mut sizes = HashMap::new();
        sizes.insert("Docker".to_string(), 1024 * 1024 * 100);
        sizes.insert("Node.js".to_string(), 1024 * 1024 * 50);

        let snapshot = StorageSnapshot::new(1024 * 1024 * 150, sizes, Some(1024 * 1024 * 1024 * 50));

        assert_eq!(snapshot.total_reclaimable, 1024 * 1024 * 150);
        assert_eq!(snapshot.category_sizes.len(), 2);
        assert!(snapshot.space_freed.is_none());
    }

    #[test]
    fn test_trends_history() {
        let mut history = TrendsHistory::new();
        assert!(history.snapshots.is_empty());

        let snapshot = StorageSnapshot::new(1000, HashMap::new(), None);
        history.add_snapshot(snapshot);

        assert_eq!(history.snapshots.len(), 1);
        assert!(history.latest_snapshot().is_some());
    }

    #[test]
    fn test_trend_time_range() {
        assert_eq!(TrendTimeRange::Week.days(), Some(7));
        assert_eq!(TrendTimeRange::Month.days(), Some(30));
        assert_eq!(TrendTimeRange::AllTime.days(), None);
    }
}
