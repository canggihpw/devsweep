use crate::types::{CheckResult, CleanupItem};
use crate::utils::{format_size, get_dir_size, home_dir};

pub fn check_gradle_maven() -> CheckResult {
    let mut result = CheckResult::new("Java Build Tools");

    let home = match home_dir() {
        Some(h) => h,
        None => return result,
    };

    // Gradle caches
    let gradle_cache = home.join(".gradle/caches");
    if gradle_cache.exists() {
        let size = get_dir_size(&gradle_cache);
        if size > 0 {
            let item = CleanupItem::new("Gradle caches", size, &format_size(size))
                .with_path(gradle_cache)
                .with_warning("Next build will need to re-download dependencies");
            result.add_item(item);
        }
    }

    // Gradle wrapper distributions
    let gradle_wrapper = home.join(".gradle/wrapper/dists");
    if gradle_wrapper.exists() {
        let size = get_dir_size(&gradle_wrapper);
        if size > 0 {
            let item = CleanupItem::new("Gradle wrapper distributions", size, &format_size(size))
                .with_path(gradle_wrapper)
                .with_warning("Contains Gradle distributions - projects may re-download");
            result.add_item(item);
        }
    }

    // Maven repository
    let maven_repo = home.join(".m2/repository");
    if maven_repo.exists() {
        let size = get_dir_size(&maven_repo);
        if size > 0 {
            let item = CleanupItem::new("Maven repository", size, &format_size(size))
                .with_path(maven_repo)
                .with_warning("Local Maven cache - projects may re-download dependencies");
            result.add_item(item);
        }
    }

    result
}
