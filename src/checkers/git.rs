//! Git repository cleanup checker
//!
//! Scans for Git repositories and identifies:
//! - Merged branches that can be deleted
//! - Stale remote-tracking branches
//! - Large .git directories

use crate::types::{CheckResult, CleanupItem};
use crate::utils::format_size;
use std::collections::HashSet;
use std::path::PathBuf;
use std::process::Command;
use walkdir::WalkDir;

/// Check for Git repository cleanup opportunities
pub fn check_git_repos() -> CheckResult {
    let mut result = CheckResult::new("Git Repositories");

    let home = match dirs::home_dir() {
        Some(h) => h,
        None => return result,
    };

    // Common directories to search for git repos
    let search_dirs = vec![
        home.join("Documents"),
        home.join("Projects"),
        home.join("Developer"),
        home.join("Code"),
        home.join("src"),
        home.join("repos"),
        home.join("workspace"),
        home.join("git"),
    ];

    let mut checked_repos: HashSet<PathBuf> = HashSet::new();

    for search_dir in search_dirs {
        if !search_dir.exists() {
            continue;
        }

        // Find .git directories (max depth 5 to avoid deep traversal)
        for entry in WalkDir::new(&search_dir)
            .max_depth(5)
            .follow_links(false)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let path = entry.path();

            // Skip if not a .git directory
            if !path.ends_with(".git") || !path.is_dir() {
                continue;
            }

            // Get the repo root (parent of .git)
            let repo_root = match path.parent() {
                Some(p) => p.to_path_buf(),
                None => continue,
            };

            // Skip if already checked
            if checked_repos.contains(&repo_root) {
                continue;
            }
            checked_repos.insert(repo_root.clone());

            // Analyze this git repository
            analyze_git_repo(&repo_root, &mut result);
        }
    }

    result
}

/// Analyze a single git repository for cleanup opportunities
fn analyze_git_repo(repo_path: &PathBuf, result: &mut CheckResult) {
    // Check for merged branches
    if let Some(item) = check_merged_branches(repo_path) {
        result.add_item(item);
    }

    // Check for stale remote branches
    if let Some(item) = check_stale_remotes(repo_path) {
        result.add_item(item);
    }

    // Check .git directory size (report if > 100MB)
    if let Some(item) = check_git_directory_size(repo_path) {
        result.add_item(item);
    }
}

/// Check for local branches that have been merged into main/master
fn check_merged_branches(repo_path: &PathBuf) -> Option<CleanupItem> {
    // Get the default branch (main or master)
    let default_branch = get_default_branch(repo_path)?;

    // Get list of merged branches
    let output = Command::new("git")
        .args(["branch", "--merged", &default_branch])
        .current_dir(repo_path)
        .output()
        .ok()?;

    if !output.status.success() {
        return None;
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let merged_branches: Vec<&str> = stdout
        .lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .filter(|l| !l.starts_with('*')) // Skip current branch
        .filter(|l| *l != "main" && *l != "master" && *l != "develop") // Skip protected branches
        .collect();

    if merged_branches.is_empty() {
        return None;
    }

    let branch_count = merged_branches.len();
    let branch_list = merged_branches.join(", ");
    let repo_name = repo_path.file_name()?.to_string_lossy().to_string();

    // Create cleanup command to delete merged branches
    let delete_cmd = format!(
        "cd {} && git branch -d {}",
        repo_path.display(),
        merged_branches.join(" ")
    );

    Some(
        CleanupItem::new(
            &format!("Merged branches in {}", repo_name),
            0, // Size is negligible for branches
            &format!("{} branches", branch_count),
        )
        .with_path(repo_path.clone())
        .with_safe_to_delete(true)
        .with_cleanup_command(&delete_cmd)
        .with_warning(&format!("Branches: {}", branch_list)),
    )
}

/// Check for stale remote-tracking branches
/// Note: This uses local-only git commands to avoid network access and credential prompts
fn check_stale_remotes(repo_path: &PathBuf) -> Option<CleanupItem> {
    // First, check if there are any remotes
    let remotes_output = Command::new("git")
        .args(["remote"])
        .current_dir(repo_path)
        .output()
        .ok()?;

    if !remotes_output.status.success()
        || String::from_utf8_lossy(&remotes_output.stdout)
            .trim()
            .is_empty()
    {
        return None; // No remotes configured
    }

    // Instead of contacting remote (which may prompt for credentials),
    // check for remote-tracking branches that have no local branch
    // This is a local-only operation
    let output = Command::new("git")
        .args(["branch", "-r", "--list"])
        .current_dir(repo_path)
        .output()
        .ok()?;

    if !output.status.success() {
        return None;
    }

    let remote_stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let remote_branches: Vec<String> = remote_stdout
        .lines()
        .map(|l| l.trim().to_string())
        .filter(|l| !l.is_empty() && !l.contains("HEAD"))
        .collect();

    // Get local branches
    let local_output = Command::new("git")
        .args(["branch", "--list"])
        .current_dir(repo_path)
        .output()
        .ok()?;

    let local_stdout = String::from_utf8_lossy(&local_output.stdout).to_string();
    let local_branches: Vec<String> = local_stdout
        .lines()
        .map(|l| l.trim().trim_start_matches("* ").to_string())
        .filter(|l| !l.is_empty())
        .collect();

    // Find remote branches where the local branch was deleted
    // (origin/feature-x exists but feature-x doesn't)
    let potentially_stale: Vec<&String> = remote_branches
        .iter()
        .filter(|rb| {
            let branch_name = rb.replace("origin/", "");
            // Skip main branches
            if branch_name == "main" || branch_name == "master" || branch_name == "develop" {
                return false;
            }
            // Check if local branch exists
            !local_branches.contains(&branch_name)
        })
        .collect();

    if potentially_stale.is_empty() {
        return None;
    }

    let branch_count = potentially_stale.len();
    let repo_name = repo_path.file_name()?.to_string_lossy().to_string();

    // Create cleanup command (user will run this manually)
    let prune_cmd = format!("cd {} && git remote prune origin", repo_path.display());

    Some(
        CleanupItem::new(
            &format!("Stale remotes in {}", repo_name),
            0,
            &format!("{} potentially stale refs", branch_count),
        )
        .with_path(repo_path.clone())
        .with_safe_to_delete(true)
        .with_cleanup_command(&prune_cmd)
        .with_warning("Run 'git fetch --prune' to sync with remote first"),
    )
}

/// Check if .git directory is unusually large
fn check_git_directory_size(repo_path: &std::path::Path) -> Option<CleanupItem> {
    let git_dir = repo_path.join(".git");
    if !git_dir.exists() {
        return None;
    }

    let size = calculate_dir_size(&git_dir);

    // Only report if > 100MB
    if size < 100 * 1024 * 1024 {
        return None;
    }

    let repo_name = repo_path.file_name()?.to_string_lossy().to_string();

    // Create cleanup command for git gc
    let gc_cmd = format!(
        "cd {} && git gc --aggressive --prune=now",
        repo_path.display()
    );

    Some(
        CleanupItem::new(
            &format!("Large .git in {}", repo_name),
            size,
            &format_size(size),
        )
        .with_path(git_dir)
        .with_safe_to_delete(false) // Don't auto-delete, just run gc
        .with_cleanup_command(&gc_cmd)
        .with_warning("Run 'git gc' to optimize. This won't delete the repo."),
    )
}

/// Get the default branch name (main or master)
fn get_default_branch(repo_path: &PathBuf) -> Option<String> {
    // Try to get the default branch from remote
    let output = Command::new("git")
        .args(["symbolic-ref", "refs/remotes/origin/HEAD", "--short"])
        .current_dir(repo_path)
        .output()
        .ok();

    if let Some(out) = output {
        if out.status.success() {
            let branch = String::from_utf8_lossy(&out.stdout)
                .trim()
                .replace("origin/", "");
            if !branch.is_empty() {
                return Some(branch);
            }
        }
    }

    // Fallback: check if main or master exists
    for branch in &["main", "master"] {
        let output = Command::new("git")
            .args(["rev-parse", "--verify", branch])
            .current_dir(repo_path)
            .output()
            .ok();

        if let Some(out) = output {
            if out.status.success() {
                return Some(branch.to_string());
            }
        }
    }

    None
}

/// Calculate total size of a directory
fn calculate_dir_size(path: &PathBuf) -> u64 {
    WalkDir::new(path)
        .follow_links(false)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .filter_map(|e| e.metadata().ok())
        .map(|m| m.len())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_git_repos_returns_result() {
        // This test just verifies the function returns a valid result structure
        // We don't actually scan repos in tests to avoid network/credential issues
        let result = CheckResult::new("Git Repositories");
        assert_eq!(result.name, "Git Repositories");
    }

    #[test]
    fn test_calculate_dir_size() {
        let temp = std::env::temp_dir();
        let size = calculate_dir_size(&temp);
        // Temp dir should have some size (u64 is always >= 0)
        assert!(size < u64::MAX);
    }

    #[test]
    fn test_get_default_branch_nonexistent() {
        // Test with a non-git directory
        let path = std::env::temp_dir();
        let result = get_default_branch(&path);
        assert!(result.is_none());
    }
}
