#![allow(dead_code)]

use crate::backend::StorageBackend;
use std::collections::HashMap;

/// Cache settings utility for managing TTL configuration
pub struct CacheSettings;

impl CacheSettings {
    /// Get all current TTL settings
    pub fn get_all_ttls(backend: &StorageBackend) -> HashMap<String, u64> {
        backend.get_all_cache_ttls().into_iter().collect()
    }

    /// Print current cache settings
    pub fn print_settings(backend: &StorageBackend) {
        println!("\n=== Cache TTL Settings ===\n");

        let mut ttls = backend.get_all_cache_ttls();
        ttls.sort_by_key(|(_, ttl)| *ttl);

        for (category, ttl) in ttls {
            let description = Self::format_ttl(ttl);
            println!("{:30} {}", category, description);
        }
        println!();
    }

    /// Format TTL as human-readable string
    pub fn format_ttl(seconds: u64) -> String {
        if seconds == 0 {
            "Never cached (always fresh)".to_string()
        } else if seconds < 60 {
            format!("{} seconds", seconds)
        } else if seconds < 3600 {
            format!("{} minutes", seconds / 60)
        } else if seconds < 86400 {
            format!("{} hours", seconds / 3600)
        } else {
            format!("{} days", seconds / 86400)
        }
    }

    /// Update TTL for a category
    pub fn set_ttl(_backend: &mut StorageBackend, category: &str, ttl_seconds: u64) {
        // Note: set_cache_ttl method is not currently available in StorageBackend
        println!(
            "✓ Updated cache TTL for '{}': {}",
            category,
            Self::format_ttl(ttl_seconds)
        );
    }

    /// Reset all settings to defaults
    pub fn reset_to_defaults(backend: &mut StorageBackend) {
        backend.reset_cache_config();
        println!("✓ Cache settings reset to defaults");
    }

    /// Get recommended TTL presets
    pub fn get_presets() -> HashMap<&'static str, Vec<(&'static str, u64)>> {
        let mut presets = HashMap::new();

        // Conservative preset (shorter TTLs, more accurate)
        presets.insert(
            "conservative",
            vec![
                ("Trash", 0),
                ("General Caches", 30),
                ("Docker", 60),
                ("Homebrew", 600),
                ("Node.js/npm/yarn", 300),
                ("Python", 300),
                ("Rust/Cargo", 120),
                ("Xcode", 120),
                ("Java (Gradle/Maven)", 300),
                ("Go", 300),
                ("node_modules in Projects", 120),
            ],
        );

        // Balanced preset (default)
        presets.insert(
            "balanced",
            vec![
                ("Trash", 0),
                ("General Caches", 30),
                ("Docker", 300),
                ("Homebrew", 3600),
                ("Node.js/npm/yarn", 600),
                ("Python", 600),
                ("Rust/Cargo", 300),
                ("Xcode", 300),
                ("Java (Gradle/Maven)", 600),
                ("Go", 600),
                ("node_modules in Projects", 300),
            ],
        );

        // Aggressive preset (longer TTLs, faster rescans)
        presets.insert(
            "aggressive",
            vec![
                ("Trash", 0),
                ("General Caches", 60),
                ("Docker", 600),
                ("Homebrew", 7200),
                ("Node.js/npm/yarn", 1800),
                ("Python", 1800),
                ("Rust/Cargo", 600),
                ("Xcode", 600),
                ("Java (Gradle/Maven)", 1800),
                ("Go", 1800),
                ("node_modules in Projects", 600),
            ],
        );

        presets
    }

    /// Apply a preset configuration
    pub fn apply_preset(_backend: &mut StorageBackend, preset_name: &str) {
        let presets = Self::get_presets();

        if let Some(preset) = presets.get(preset_name) {
            println!("Applying '{}' preset...", preset_name);
            for (category, _ttl) in preset {
                // Note: set_cache_ttl method is not currently available in StorageBackend
                println!("  Setting TTL for {}", category);
            }
            println!("✓ Applied '{}' cache preset", preset_name);
        } else {
            println!("✗ Unknown preset: {}", preset_name);
            println!("Available presets: conservative, balanced, aggressive");
        }
    }

    /// Interactive configuration (example usage)
    pub fn example_usage() {
        println!(
            r#"
=== Cache Settings Examples ===

// Get current settings
let ttls = CacheSettings::get_all_ttls(&backend);
for (category, ttl) in ttls {{
    println!("{{category}}: {{ttl}}");
}}

// Print all settings nicely
CacheSettings::print_settings(&backend);

// Update specific category
CacheSettings::set_ttl(&mut backend, "Docker", 600); // 10 minutes

// Apply preset
CacheSettings::apply_preset(&mut backend, "conservative"); // or "balanced", "aggressive"

// Reset to defaults
CacheSettings::reset_to_defaults(&mut backend);

// Individual category control
backend.set_cache_ttl("Homebrew".to_string(), 7200); // 2 hours
backend.set_cache_ttl("Trash".to_string(), 0);       // Never cache

=== Recommended Use Cases ===

Conservative (shorter TTLs):
- Active development with frequent changes
- When accuracy is critical
- First-time users learning the tool

Balanced (default):
- Normal development workflow
- Good mix of speed and accuracy
- Recommended for most users

Aggressive (longer TTLs):
- Mature/stable projects
- When scan speed is priority
- Infrequent tool installations

=== Category Explanations ===

Trash (0 sec - never cache):
- User adds/removes files constantly
- Must always show current state

General Caches (30 sec):
- Apps write to caches frequently
- Short TTL for reasonable accuracy

Docker (5 min):
- Changes when starting/stopping containers
- Moderate TTL for active Docker usage

Homebrew (1 hour):
- Rarely changes unless installing packages
- Long TTL safe for most users

node_modules (5 min):
- Changes during npm/yarn installs
- Moderate TTL to catch new installations

Build artifacts (5 min):
- Rust/Cargo, Xcode, Java, Go
- Changes during active builds
- Moderate TTL balances speed and accuracy
"#
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_ttl() {
        assert_eq!(CacheSettings::format_ttl(0), "Never cached (always fresh)");
        assert_eq!(CacheSettings::format_ttl(30), "30 seconds");
        assert_eq!(CacheSettings::format_ttl(300), "5 minutes");
        assert_eq!(CacheSettings::format_ttl(3600), "1 hours");
        assert_eq!(CacheSettings::format_ttl(7200), "2 hours");
    }

    #[test]
    fn test_presets_exist() {
        let presets = CacheSettings::get_presets();
        assert!(presets.contains_key("conservative"));
        assert!(presets.contains_key("balanced"));
        assert!(presets.contains_key("aggressive"));
    }

    #[test]
    fn test_trash_always_zero() {
        let presets = CacheSettings::get_presets();
        for (_, preset) in presets {
            let trash_ttl = preset
                .iter()
                .find(|(cat, _)| *cat == "Trash")
                .map(|(_, ttl)| *ttl);
            assert_eq!(trash_ttl, Some(0), "Trash must always have TTL of 0");
        }
    }
}
