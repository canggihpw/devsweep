use crate::types::{CheckResult, CleanupItem, ItemDetail};
use crate::utils::{format_size, get_dir_size};
use dirs::home_dir;
use std::path::PathBuf;

pub fn check_system_logs() -> CheckResult {
    let mut result = CheckResult::new("System Logs & Crash Reports");

    if let Some(home) = home_dir() {
        // User Application Logs
        check_user_logs(&mut result, &home);

        // Crash Reports and Diagnostic Reports
        check_crash_reports(&mut result, &home);

        // System diagnostic data
        check_system_diagnostics(&mut result, &home);

        // Application-specific logs
        check_app_logs(&mut result, &home);
    }

    // System-level logs (readable without root)
    check_system_level_logs(&mut result);

    result
}

fn check_user_logs(result: &mut CheckResult, home: &PathBuf) {
    let user_logs = home.join("Library/Logs");
    if !user_logs.exists() {
        return;
    }

    let mut log_items: Vec<ItemDetail> = Vec::new();
    let mut total_size: u64 = 0;

    // Scan for large log directories
    if let Ok(entries) = std::fs::read_dir(&user_logs) {
        for entry in entries.flatten() {
            let path = entry.path();
            let name = entry.file_name().to_string_lossy().to_string();

            // Skip DiagnosticReports (handled separately)
            if name == "DiagnosticReports" {
                continue;
            }

            let size = if path.is_dir() {
                get_dir_size(&path)
            } else {
                std::fs::metadata(&path).map(|m| m.len()).unwrap_or(0)
            };

            // Only include items > 5MB
            if size > 5 * 1024 * 1024 {
                log_items.push(
                    ItemDetail::new(&name, size, &format_size(size))
                        .with_path(path)
                );
                total_size += size;
            }
        }
    }

    if !log_items.is_empty() {
        log_items.sort_by(|a, b| b.size.cmp(&a.size));
        result.add_item(
            CleanupItem::new("Application Logs (>5MB)", total_size, &format_size(total_size))
                .with_details(log_items)
                .with_safe_to_delete(true)
                .with_warning("Logs are useful for debugging - only delete if not investigating issues")
        );
    }
}

fn check_crash_reports(result: &mut CheckResult, home: &PathBuf) {
    let mut crash_items: Vec<ItemDetail> = Vec::new();
    let mut total_size: u64 = 0;

    // User Diagnostic Reports (crash reports)
    let diag_reports = home.join("Library/Logs/DiagnosticReports");
    if diag_reports.exists() {
        let size = get_dir_size(&diag_reports);
        if size > 1024 * 1024 { // > 1MB
            crash_items.push(
                ItemDetail::new("User Crash Reports", size, &format_size(size))
                    .with_path(diag_reports)
            );
            total_size += size;
        }
    }

    // Retired Diagnostic Reports
    let retired_reports = home.join("Library/Logs/DiagnosticReports/Retired");
    if retired_reports.exists() {
        let size = get_dir_size(&retired_reports);
        if size > 0 {
            // Already counted in DiagnosticReports, but note it exists
        }
    }

    // CoreAnalytics (system telemetry)
    let core_analytics = home.join("Library/Logs/CoreAnalytics");
    if core_analytics.exists() {
        let size = get_dir_size(&core_analytics);
        if size > 1024 * 1024 { // > 1MB
            crash_items.push(
                ItemDetail::new("CoreAnalytics Data", size, &format_size(size))
                    .with_path(core_analytics)
            );
            total_size += size;
        }
    }

    // Spotlight diagnostic logs
    let spotlight_diag = home.join("Library/Logs/Spotlight");
    if spotlight_diag.exists() {
        let size = get_dir_size(&spotlight_diag);
        if size > 5 * 1024 * 1024 { // > 5MB
            crash_items.push(
                ItemDetail::new("Spotlight Logs", size, &format_size(size))
                    .with_path(spotlight_diag)
            );
            total_size += size;
        }
    }

    if !crash_items.is_empty() {
        result.add_item(
            CleanupItem::new("Crash Reports & Diagnostics", total_size, &format_size(total_size))
                .with_details(crash_items)
                .with_safe_to_delete(true)
        );
    }
}

fn check_system_diagnostics(result: &mut CheckResult, home: &PathBuf) {
    let mut diag_items: Vec<ItemDetail> = Vec::new();
    let mut total_size: u64 = 0;

    // Apple System Logs aggregated data
    let asl_logs = home.join("Library/Logs/asl");
    if asl_logs.exists() {
        let size = get_dir_size(&asl_logs);
        if size > 10 * 1024 * 1024 { // > 10MB
            diag_items.push(
                ItemDetail::new("Apple System Logs (ASL)", size, &format_size(size))
                    .with_path(asl_logs)
            );
            total_size += size;
        }
    }

    // Console saved logs and reports
    let console_reports = home.join("Library/Logs/Console");
    if console_reports.exists() {
        let size = get_dir_size(&console_reports);
        if size > 1024 * 1024 { // > 1MB
            diag_items.push(
                ItemDetail::new("Console Saved Logs", size, &format_size(size))
                    .with_path(console_reports)
            );
            total_size += size;
        }
    }

    // Install logs
    let install_logs = home.join("Library/Logs/Install Application");
    if install_logs.exists() {
        let size = get_dir_size(&install_logs);
        if size > 5 * 1024 * 1024 { // > 5MB
            diag_items.push(
                ItemDetail::new("Install Logs", size, &format_size(size))
                    .with_path(install_logs)
            );
            total_size += size;
        }
    }

    // JetBrains logs (can get very large)
    let jetbrains_logs = home.join("Library/Logs/JetBrains");
    if jetbrains_logs.exists() {
        let size = get_dir_size(&jetbrains_logs);
        if size > 10 * 1024 * 1024 { // > 10MB
            diag_items.push(
                ItemDetail::new("JetBrains IDE Logs", size, &format_size(size))
                    .with_path(jetbrains_logs)
            );
            total_size += size;
        }
    }

    if !diag_items.is_empty() {
        result.add_item(
            CleanupItem::new("System Diagnostic Data", total_size, &format_size(total_size))
                .with_details(diag_items)
                .with_safe_to_delete(true)
        );
    }
}

fn check_app_logs(result: &mut CheckResult, home: &PathBuf) {
    let mut app_log_items: Vec<ItemDetail> = Vec::new();
    let mut total_size: u64 = 0;

    // Adobe logs
    let adobe_logs = home.join("Library/Logs/Adobe");
    if adobe_logs.exists() {
        let size = get_dir_size(&adobe_logs);
        if size > 10 * 1024 * 1024 { // > 10MB
            app_log_items.push(
                ItemDetail::new("Adobe Logs", size, &format_size(size))
                    .with_path(adobe_logs)
            );
            total_size += size;
        }
    }

    // Microsoft logs
    let microsoft_logs = home.join("Library/Logs/Microsoft");
    if microsoft_logs.exists() {
        let size = get_dir_size(&microsoft_logs);
        if size > 10 * 1024 * 1024 { // > 10MB
            app_log_items.push(
                ItemDetail::new("Microsoft Logs", size, &format_size(size))
                    .with_path(microsoft_logs)
            );
            total_size += size;
        }
    }

    // Google Chrome crash reports
    let chrome_crash = home.join("Library/Application Support/Google/Chrome/Crash Reports");
    if chrome_crash.exists() {
        let size = get_dir_size(&chrome_crash);
        if size > 5 * 1024 * 1024 { // > 5MB
            app_log_items.push(
                ItemDetail::new("Chrome Crash Reports", size, &format_size(size))
                    .with_path(chrome_crash)
            );
            total_size += size;
        }
    }

    // Firefox crash reports
    let firefox_crash = home.join("Library/Application Support/Firefox/Crash Reports");
    if firefox_crash.exists() {
        let size = get_dir_size(&firefox_crash);
        if size > 5 * 1024 * 1024 { // > 5MB
            app_log_items.push(
                ItemDetail::new("Firefox Crash Reports", size, &format_size(size))
                    .with_path(firefox_crash)
            );
            total_size += size;
        }
    }

    // Slack logs
    let slack_logs = home.join("Library/Application Support/Slack/logs");
    if slack_logs.exists() {
        let size = get_dir_size(&slack_logs);
        if size > 10 * 1024 * 1024 { // > 10MB
            app_log_items.push(
                ItemDetail::new("Slack Logs", size, &format_size(size))
                    .with_path(slack_logs)
            );
            total_size += size;
        }
    }

    // Discord logs
    let discord_logs = home.join("Library/Application Support/discord/logs");
    if discord_logs.exists() {
        let size = get_dir_size(&discord_logs);
        if size > 5 * 1024 * 1024 { // > 5MB
            app_log_items.push(
                ItemDetail::new("Discord Logs", size, &format_size(size))
                    .with_path(discord_logs)
            );
            total_size += size;
        }
    }

    // Zoom logs
    let zoom_logs = home.join("Library/Logs/zoom.us");
    if zoom_logs.exists() {
        let size = get_dir_size(&zoom_logs);
        if size > 10 * 1024 * 1024 { // > 10MB
            app_log_items.push(
                ItemDetail::new("Zoom Logs", size, &format_size(size))
                    .with_path(zoom_logs)
            );
            total_size += size;
        }
    }

    // Figma logs
    let figma_logs = home.join("Library/Application Support/Figma/logs");
    if figma_logs.exists() {
        let size = get_dir_size(&figma_logs);
        if size > 5 * 1024 * 1024 { // > 5MB
            app_log_items.push(
                ItemDetail::new("Figma Logs", size, &format_size(size))
                    .with_path(figma_logs)
            );
            total_size += size;
        }
    }

    // Spotify logs and cache (can be huge)
    let spotify_prefs = home.join("Library/Application Support/Spotify/PersistentCache");
    if spotify_prefs.exists() {
        let size = get_dir_size(&spotify_prefs);
        if size > 100 * 1024 * 1024 { // > 100MB
            app_log_items.push(
                ItemDetail::new("Spotify Cache", size, &format_size(size))
                    .with_path(spotify_prefs)
            );
            total_size += size;
        }
    }

    if !app_log_items.is_empty() {
        app_log_items.sort_by(|a, b| b.size.cmp(&a.size));
        result.add_item(
            CleanupItem::new("Application Logs & Crash Reports", total_size, &format_size(total_size))
                .with_details(app_log_items)
                .with_safe_to_delete(true)
        );
    }
}

fn check_system_level_logs(result: &mut CheckResult) {
    let mut sys_log_items: Vec<ItemDetail> = Vec::new();
    let mut total_size: u64 = 0;

    // /var/log is usually readable but not writable without root
    // We can still show the sizes for awareness
    let var_log = PathBuf::from("/var/log");
    if var_log.exists() {
        // Check specific log files that might be readable
        let log_files = vec![
            ("system.log", "System Log"),
            ("wifi.log", "WiFi Log"),
            ("install.log", "Install Log"),
            ("fsck_apfs.log", "APFS Check Log"),
        ];

        for (filename, description) in log_files {
            let log_path = var_log.join(filename);
            if log_path.exists() {
                if let Ok(metadata) = std::fs::metadata(&log_path) {
                    let size = metadata.len();
                    if size > 10 * 1024 * 1024 { // > 10MB
                        sys_log_items.push(
                            ItemDetail::new(description, size, &format_size(size))
                                .with_path(log_path)
                                .with_extra_info("May require admin privileges to delete")
                        );
                        total_size += size;
                    }
                }
            }
        }

        // Archived logs (can be large)
        if let Ok(entries) = std::fs::read_dir(&var_log) {
            let mut archived_size: u64 = 0;
            let mut archived_count = 0;

            for entry in entries.flatten() {
                let path = entry.path();
                if let Some(ext) = path.extension() {
                    if ext == "gz" || ext == "bz2" {
                        if let Ok(metadata) = std::fs::metadata(&path) {
                            archived_size += metadata.len();
                            archived_count += 1;
                        }
                    }
                }
            }

            if archived_size > 50 * 1024 * 1024 { // > 50MB
                sys_log_items.push(
                    ItemDetail::new(&format!("Archived Logs ({} files)", archived_count), archived_size, &format_size(archived_size))
                        .with_path(var_log.clone())
                        .with_extra_info("May require admin privileges to delete")
                );
                total_size += archived_size;
            }
        }
    }

    // Private var logs
    let private_var_log = PathBuf::from("/private/var/log");
    if private_var_log.exists() {
        // DiagnosticMessages (can grow very large)
        let diag_messages = private_var_log.join("DiagnosticMessages");
        if diag_messages.exists() {
            let size = get_dir_size(&diag_messages);
            if size > 100 * 1024 * 1024 { // > 100MB
                sys_log_items.push(
                    ItemDetail::new("Diagnostic Messages", size, &format_size(size))
                        .with_path(diag_messages)
                        .with_extra_info("May require admin privileges to delete")
                );
                total_size += size;
            }
        }
    }

    if !sys_log_items.is_empty() {
        result.add_item(
            CleanupItem::new("System Logs (may need admin)", total_size, &format_size(total_size))
                .with_details(sys_log_items)
                .with_warning("Some items may require administrator privileges to delete")
        );
    }
}
