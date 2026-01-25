//! Update checker module for checking GitHub releases
//!
//! This module provides functionality to check for new releases of DevSweep
//! from GitHub and compare versions.

use semver::Version;
use serde::Deserialize;

/// GitHub API endpoint for the latest release
const GITHUB_API_URL: &str = "https://api.github.com/repos/canggihpw/devsweep/releases/latest";

/// User agent for GitHub API requests (required by GitHub)
const USER_AGENT: &str = "DevSweep-UpdateChecker";

/// Information about an available update
#[derive(Clone, Debug)]
pub struct UpdateInfo {
    /// The version number (e.g., "0.3.0")
    pub version: String,
    /// The git tag name (e.g., "v0.3.0")
    pub tag_name: String,
    /// URL to the GitHub release page
    pub release_url: String,
    /// URL to download the DMG file (if available)
    pub download_url: Option<String>,
    /// Release notes/changelog
    pub changelog: String,
    /// When the release was published
    pub published_at: String,
}

/// GitHub API response structure for a release
#[derive(Deserialize)]
struct GitHubRelease {
    tag_name: String,
    html_url: String,
    body: Option<String>,
    published_at: Option<String>,
    assets: Vec<GitHubAsset>,
}

/// GitHub API response structure for a release asset
#[derive(Deserialize)]
struct GitHubAsset {
    name: String,
    browser_download_url: String,
}

/// Error type for update checking
#[derive(Debug)]
pub enum UpdateError {
    /// Network request failed
    NetworkError(String),
    /// Failed to parse the response
    ParseError(String),
    /// No release found
    NoRelease,
}

impl std::fmt::Display for UpdateError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UpdateError::NetworkError(msg) => write!(f, "Network error: {}", msg),
            UpdateError::ParseError(msg) => write!(f, "Parse error: {}", msg),
            UpdateError::NoRelease => write!(f, "No release found"),
        }
    }
}

impl std::error::Error for UpdateError {}

/// Fetch the latest release information from GitHub
///
/// This function makes a blocking HTTP request to the GitHub API.
/// It should be called from a background thread to avoid blocking the UI.
pub fn fetch_latest_release() -> Result<UpdateInfo, UpdateError> {
    let response = ureq::get(GITHUB_API_URL)
        .set("User-Agent", USER_AGENT)
        .set("Accept", "application/vnd.github.v3+json")
        .call()
        .map_err(|e| UpdateError::NetworkError(e.to_string()))?;

    let release: GitHubRelease = response
        .into_json()
        .map_err(|e| UpdateError::ParseError(e.to_string()))?;

    // Extract version from tag (remove 'v' prefix if present)
    let version = release
        .tag_name
        .strip_prefix('v')
        .unwrap_or(&release.tag_name)
        .to_string();

    // Find the DMG asset
    let download_url = release
        .assets
        .iter()
        .find(|a| a.name.ends_with(".dmg"))
        .map(|a| a.browser_download_url.clone());

    Ok(UpdateInfo {
        version,
        tag_name: release.tag_name,
        release_url: release.html_url,
        download_url,
        changelog: release.body.unwrap_or_default(),
        published_at: release.published_at.unwrap_or_default(),
    })
}

/// Check if an update is available by comparing versions
///
/// Returns true if the latest version is greater than the current version.
pub fn is_update_available(current: &str, latest: &str) -> bool {
    let current = Version::parse(current).ok();
    let latest = Version::parse(latest).ok();

    match (current, latest) {
        (Some(c), Some(l)) => l > c,
        _ => false,
    }
}

/// Get the current application version from Cargo.toml
pub fn current_version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_update_available() {
        // Update available
        assert!(is_update_available("0.1.0", "0.2.0"));
        assert!(is_update_available("0.2.0", "0.3.0"));
        assert!(is_update_available("0.2.0", "1.0.0"));
        assert!(is_update_available("1.0.0", "1.0.1"));
        assert!(is_update_available("1.0.0", "1.1.0"));

        // No update available (same version)
        assert!(!is_update_available("0.2.0", "0.2.0"));
        assert!(!is_update_available("1.0.0", "1.0.0"));

        // No update available (current is newer)
        assert!(!is_update_available("0.3.0", "0.2.0"));
        assert!(!is_update_available("1.0.0", "0.9.0"));

        // Pre-release versions
        assert!(is_update_available("0.2.0-alpha.1", "0.2.0"));
        assert!(is_update_available("0.2.0-beta.1", "0.2.0"));
        assert!(!is_update_available("0.2.0", "0.2.0-alpha.1"));
    }

    #[test]
    fn test_is_update_available_invalid_versions() {
        // Invalid versions should return false
        assert!(!is_update_available("invalid", "0.2.0"));
        assert!(!is_update_available("0.2.0", "invalid"));
        assert!(!is_update_available("invalid", "also-invalid"));
    }

    #[test]
    fn test_current_version() {
        let version = current_version();
        // Should be a valid semver
        assert!(Version::parse(version).is_ok());
    }
}
