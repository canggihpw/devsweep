//! Verifies the category → super-category mapping stays correct as new
//! categories are added or renamed. Walks the actual canonical list from
//! `all_category_checks()` so it can't silently drift.

use devsweep::app::state::SuperCategoryType;
use devsweep::backend::all_category_checks;

fn expected_super_category(name: &str) -> SuperCategoryType {
    match name {
        // Development Tools
        "Docker"
        | "Homebrew"
        | "Swift Package Manager"
        | "Xcode"
        | "Flutter"
        | "Android"
        | "IDE Caches" => SuperCategoryType::DevelopmentTools,
        // Package Managers
        "Node.js/npm/yarn" | "Bun" | "Python" | "Rust/Cargo" | "Go" | "Java (Gradle/Maven)" => {
            SuperCategoryType::PackageManagers
        }
        // Project Files
        "node_modules in Projects" | "Git Repositories" | "Custom Paths" => {
            SuperCategoryType::ProjectFiles
        }
        // System & Browsers
        "System Logs" | "Browser Caches" | "Shell Caches" | "Database Caches"
        | "General Caches" => SuperCategoryType::SystemAndBrowsers,
        // Trash
        "Trash" => SuperCategoryType::Trash,
        _ => panic!("category '{name}' missing from expected mapping"),
    }
}

#[test]
fn every_canonical_category_maps_to_the_right_super_category() {
    let checks = all_category_checks();

    // Sanity: we have exactly the intended number of categories.
    assert_eq!(
        checks.len(),
        22,
        "unexpected number of categories registered"
    );

    for (name, _) in checks {
        let got = SuperCategoryType::from_category_name(name);
        let want = expected_super_category(name);
        assert_eq!(
            got, want,
            "category '{name}' mapped to {got:?}, expected {want:?}"
        );
    }
}

#[test]
fn package_manager_categories_are_not_mis_bucketed() {
    // Regression: these previously fell through to the "System & Browsers"
    // fallback because the matcher used stale names.
    for name in ["Node.js/npm/yarn", "Java (Gradle/Maven)"] {
        assert_eq!(
            SuperCategoryType::from_category_name(name),
            SuperCategoryType::PackageManagers,
            "'{name}' must be a Package Manager"
        );
    }
}
