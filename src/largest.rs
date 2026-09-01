//! Largest-files / largest-folders index with a cached, cacheable result and a
//! simple treemap layout generator.
//!
//! This is intentionally a pure, testable module: `scan_largest` walks a set of
//! roots and produces a `SizeIndex`; `squarify` turns entries into rectangles;
//! `load_cache`/`save_cache` persist the index to disk with a TTL.

use serde::{Deserialize, Serialize};
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};
use walkdir::WalkDir;

/// A single measured file or directory.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct LargestEntry {
    pub name: String,
    pub path: PathBuf,
    pub size: u64,
}

// Order by size (largest first semantics via `Reverse`), tie-break by name so the
// ordering is deterministic.
impl PartialOrd for LargestEntry {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for LargestEntry {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.size
            .cmp(&other.size)
            .then_with(|| self.name.cmp(&other.name))
    }
}

/// Result of a largest files/folders scan.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SizeIndex {
    pub scan_timestamp: u64,
    pub roots: Vec<PathBuf>,
    pub top_dirs: Vec<LargestEntry>,
    pub top_files: Vec<LargestEntry>,
}

/// Tunables for a scan.
#[derive(Debug, Clone, Copy)]
pub struct ScanOptions {
    pub top_n: usize,
    pub max_depth: usize,
}

impl Default for ScanOptions {
    fn default() -> Self {
        Self {
            top_n: 20,
            max_depth: 5,
        }
    }
}

/// A rectangle produced by [`squarify`], in pixels.
#[derive(Debug, Clone, PartialEq)]
pub struct TreemapRect {
    pub name: String,
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}

/// Compute the largest subdirectories (of each root) by sizing every immediate
/// child directory in parallel. Returns them sorted largest-first.
pub fn scan_largest_folders(roots: &[PathBuf], top_n: usize) -> Vec<LargestEntry> {
    let top_n = top_n.max(1);
    let roots: Vec<PathBuf> = roots.iter().filter(|r| r.is_dir()).cloned().collect();
    if roots.is_empty() {
        return Vec::new();
    }

    use rayon::prelude::*;

    let dir_candidates: Vec<LargestEntry> = roots
        .iter()
        .flat_map(|root| {
            std::fs::read_dir(root)
                .ok()
                .into_iter()
                .flatten()
                .filter_map(|e| e.ok())
                .filter(|e| e.path().is_dir())
                .map(|e| LargestEntry {
                    name: e.file_name().to_string_lossy().into_owned(),
                    path: e.path(),
                    size: 0,
                })
                .collect::<Vec<_>>()
        })
        .collect();

    let mut top_dirs: Vec<LargestEntry> = dir_candidates
        .par_iter()
        .map(|d| LargestEntry {
            name: d.name.clone(),
            path: d.path.clone(),
            size: crate::utils::get_dir_size(&d.path),
        })
        .collect();
    top_dirs.sort_by_key(|e| std::cmp::Reverse(e.size));
    top_dirs.truncate(top_n);
    top_dirs
}

/// Compute the largest individual files across the roots, walking each tree up
/// to `max_depth` (symlinks never followed). Only the top-N files are kept via
/// a bounded min-heap, so memory stays flat even on huge trees.
pub fn scan_largest_files(roots: &[PathBuf], options: &ScanOptions) -> Vec<LargestEntry> {
    let top_n = options.top_n.max(1);
    let max_depth = options.max_depth.max(2);

    let mut heap: BinaryHeap<Reverse<LargestEntry>> = BinaryHeap::new();
    for root in roots.iter().filter(|r| r.is_dir()) {
        for entry in WalkDir::new(root)
            .max_depth(max_depth)
            .follow_links(false)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            if entry.file_type().is_file() {
                if let Ok(meta) = entry.metadata() {
                    let size = meta.len();
                    if size > 0 {
                        let item = LargestEntry {
                            name: entry.file_name().to_string_lossy().into_owned(),
                            path: entry.into_path(),
                            size,
                        };
                        if heap.len() < top_n {
                            heap.push(Reverse(item));
                        } else if let Some(peek) = heap.peek() {
                            if peek.0.size < size {
                                heap.pop();
                                heap.push(Reverse(item));
                            }
                        }
                    }
                }
            }
        }
    }
    let mut top_files: Vec<LargestEntry> = heap.into_iter().map(|r| r.0).collect();
    top_files.sort_by_key(|e| std::cmp::Reverse(e.size));
    top_files.truncate(top_n);
    top_files
}

/// Compute the largest subdirectories and individual files across the roots.
pub fn scan_largest(roots: &[PathBuf], options: &ScanOptions) -> SizeIndex {
    let roots: Vec<PathBuf> = roots.iter().filter(|r| r.is_dir()).cloned().collect();
    SizeIndex {
        scan_timestamp: now_secs(),
        top_dirs: scan_largest_folders(&roots, options.top_n),
        top_files: scan_largest_files(&roots, options),
        roots,
    }
}

/// Generate a slice-and-dice treemap layout for the given entries inside a
/// `width`×`height` box. Entries are assumed to be sorted largest-first.
pub fn squarify(items: &[LargestEntry], width: f32, height: f32) -> Vec<TreemapRect> {
    let total: u64 = items.iter().map(|e| e.size).sum();
    if total == 0 || width <= 0.0 || height <= 0.0 {
        return Vec::new();
    }

    let area = width * height;
    let mut rects = Vec::with_capacity(items.len());
    let mut x = 0f32;
    let mut y = 0f32;
    let mut w = width;
    let mut h = height;

    for (i, entry) in items.iter().enumerate() {
        let item_area = area * (entry.size as f32) / (total as f32);
        if i % 2 == 0 {
            // Horizontal strip: full current height, width proportional to area.
            let strip_w = if h > 0.0 { item_area / h } else { 0.0 }.min(w);
            rects.push(TreemapRect {
                name: entry.name.clone(),
                x,
                y,
                w: strip_w,
                h,
            });
            x += strip_w;
            w -= strip_w;
        } else {
            // Vertical strip: full current width, height proportional to area.
            let strip_h = if w > 0.0 { item_area / w } else { 0.0 }.min(h);
            rects.push(TreemapRect {
                name: entry.name.clone(),
                x,
                y,
                w,
                h: strip_h,
            });
            y += strip_h;
            h -= strip_h;
        }
    }

    rects
}

/// The Largest tab treemap viewport size and top-N count.
pub const TREEMAP_W: f32 = 560.0;
pub const TREEMAP_H: f32 = 300.0;
pub const TREEMAP_N: usize = 20;

/// Default set of roots to scan, based on the home directory's top-level folders.
pub fn default_roots() -> Vec<PathBuf> {
    let home = match dirs::home_dir() {
        Some(h) => h,
        None => return Vec::new(),
    };

    let mut roots: Vec<PathBuf> = [
        "Desktop",
        "Documents",
        "Downloads",
        "Projects",
        "Developer",
        "Code",
    ]
    .iter()
    .map(|d| home.join(d))
    .filter(|p| p.is_dir())
    .collect();

    if roots.is_empty() {
        roots.push(home);
    }
    roots
}

const CACHE_TTL_SECS: u64 = 60 * 30; // 30 minutes

fn cache_path() -> PathBuf {
    let cache_dir = dirs::cache_dir().unwrap_or_else(|| PathBuf::from("/tmp"));
    cache_dir
        .join("development-cleaner")
        .join("largest_cache.json")
}

pub(crate) fn now_secs() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}

/// Load a still-valid cached index. Returns `None` when the file is missing,
/// stale, or unreadable.
pub fn load_cache() -> Option<SizeIndex> {
    let data = std::fs::read_to_string(cache_path()).ok()?;
    let index: SizeIndex = serde_json::from_str(&data).ok()?;
    if now_secs().saturating_sub(index.scan_timestamp) > CACHE_TTL_SECS {
        return None;
    }
    Some(index)
}

/// Persist an index to the cache file.
pub fn save_cache(index: &SizeIndex) {
    let path = cache_path();
    if let Some(parent) = path.parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    if let Ok(json) = serde_json::to_string_pretty(index) {
        let _ = std::fs::write(path, json);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    fn make_tree() -> TempDir {
        let tmp = TempDir::new().unwrap();
        // root/sub_a/big.bin (large), root/sub_a/tiny.txt, root/sub_b/file.bin
        fs::create_dir_all(tmp.path().join("sub_a")).unwrap();
        fs::create_dir_all(tmp.path().join("sub_b")).unwrap();
        fs::write(tmp.path().join("sub_a/big.bin"), vec![0u8; 5000]).unwrap();
        fs::write(tmp.path().join("sub_a/tiny.txt"), b"hi").unwrap();
        fs::write(tmp.path().join("sub_b/file.bin"), vec![0u8; 2000]).unwrap();
        tmp
    }

    #[test]
    fn scan_largest_finds_top_dirs_and_files() {
        let tmp = make_tree();
        let roots = vec![tmp.path().to_path_buf()];
        let index = scan_largest(
            &roots,
            &ScanOptions {
                top_n: 10,
                max_depth: 5,
            },
        );

        assert_eq!(index.top_dirs.len(), 2);
        // sub_a is larger than sub_b
        assert_eq!(index.top_dirs[0].name, "sub_a");
        assert!(index.top_dirs[0].size > index.top_dirs[1].size);

        // big.bin (5000) is the largest file
        assert_eq!(index.top_files[0].name, "big.bin");
        assert_eq!(index.top_files[0].size, 5000);
        assert!(index.top_files.iter().any(|f| f.name == "file.bin"));
    }

    #[test]
    fn scan_largest_empty_roots() {
        let index = scan_largest(&[], &ScanOptions::default());
        assert!(index.top_dirs.is_empty());
        assert!(index.top_files.is_empty());
    }

    #[test]
    fn squarify_has_correct_invariants() {
        let items = vec![
            LargestEntry {
                name: "a".into(),
                path: PathBuf::from("a"),
                size: 50,
            },
            LargestEntry {
                name: "b".into(),
                path: PathBuf::from("b"),
                size: 30,
            },
            LargestEntry {
                name: "c".into(),
                path: PathBuf::from("c"),
                size: 20,
            },
        ];

        let width = 400.0;
        let height = 200.0;
        let rects = squarify(&items, width, height);

        assert_eq!(rects.len(), items.len());
        // First (largest) rect must dominate.
        assert!(rects[0].w >= rects[1].w);

        // All rects stay inside the box and have non-negative area.
        for r in &rects {
            assert!(r.x >= 0.0 && r.y >= 0.0);
            assert!(r.x + r.w <= width + 1e-3);
            assert!(r.y + r.h <= height + 1e-3);
            assert!(r.w > 0.0 && r.h > 0.0);
        }

        // Total filled area ≈ box area.
        let filled: f32 = rects.iter().map(|r| r.w * r.h).sum();
        assert!((filled - width * height).abs() < 1.0, "filled={filled}");
    }

    #[test]
    fn squarify_empty() {
        assert!(squarify(&[], 100.0, 100.0).is_empty());
    }

    #[test]
    fn cache_round_trip() {
        let index = SizeIndex {
            scan_timestamp: now_secs(),
            roots: vec![PathBuf::from("/x")],
            top_dirs: vec![LargestEntry {
                name: "d".into(),
                path: PathBuf::from("/x/d"),
                size: 10,
            }],
            top_files: Vec::new(),
        };
        save_cache(&index);
        let loaded = load_cache().expect("cache should load");
        assert_eq!(loaded.top_dirs[0].name, "d");
        // Cleanup the cache file so tests don't leak state.
        let _ = std::fs::remove_file(cache_path());
    }
}
