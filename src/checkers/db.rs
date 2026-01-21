use crate::types::{CheckResult, CleanupItem, ItemDetail};
use crate::utils::{format_size, get_dir_size};
use dirs::home_dir;
use std::path::PathBuf;

pub fn check_db_caches() -> CheckResult {
    let mut result = CheckResult::new("Database Caches");

    if let Some(home) = home_dir() {
        // PostgreSQL
        check_postgres_caches(&mut result, &home);

        // MySQL
        check_mysql_caches(&mut result, &home);

        // MongoDB
        check_mongodb_caches(&mut result, &home);

        // Redis
        check_redis_caches(&mut result, &home);

        // SQLite
        check_sqlite_caches(&mut result, &home);
    }

    // Homebrew database logs and data
    let brew_paths = vec![
        PathBuf::from("/opt/homebrew/var"),      // Apple Silicon
        PathBuf::from("/usr/local/var"),          // Intel
    ];

    for brew_var in brew_paths {
        if brew_var.exists() {
            check_brew_db_data(&mut result, &brew_var);
        }
    }

    result
}

fn check_postgres_caches(result: &mut CheckResult, home: &PathBuf) {
    let mut postgres_items: Vec<ItemDetail> = Vec::new();
    let mut total_size: u64 = 0;

    // Postgres.app data directories
    let postgres_app_support = home.join("Library/Application Support/Postgres");
    if postgres_app_support.exists() {
        if let Ok(entries) = std::fs::read_dir(&postgres_app_support) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    let dir_name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");

                    // Check for var-XX directories (data directories)
                    if dir_name.starts_with("var-") {
                        // PostgreSQL log file
                        let log_file = path.join("postgresql.log");
                        if log_file.exists() {
                            let size = std::fs::metadata(&log_file).map(|m| m.len()).unwrap_or(0);
                            if size > 1024 * 1024 { // > 1MB
                                postgres_items.push(
                                    ItemDetail::new(&format!("PostgreSQL Log ({})", dir_name), size, &format_size(size))
                                        .with_path(log_file)
                                );
                                total_size += size;
                            }
                        }

                        // pg_log directory (older versions)
                        let pg_log_dir = path.join("pg_log");
                        if pg_log_dir.exists() {
                            let size = get_dir_size(&pg_log_dir);
                            if size > 1024 * 1024 { // > 1MB
                                postgres_items.push(
                                    ItemDetail::new(&format!("PostgreSQL Logs Dir ({})", dir_name), size, &format_size(size))
                                        .with_path(pg_log_dir)
                                );
                                total_size += size;
                            }
                        }

                        // pg_wal directory (Write-Ahead Logs) - can be large but be careful
                        let pg_wal_dir = path.join("pg_wal");
                        if pg_wal_dir.exists() {
                            let size = get_dir_size(&pg_wal_dir);
                            if size > 100 * 1024 * 1024 { // > 100MB
                                postgres_items.push(
                                    ItemDetail::new(&format!("PostgreSQL WAL ({})", dir_name), size, &format_size(size))
                                        .with_path(pg_wal_dir)
                                        .with_extra_info("Warning: May affect database recovery")
                                );
                                total_size += size;
                            }
                        }
                    }
                }
            }
        }
    }

    // PostgreSQL cache in Library/Caches
    let pg_cache = home.join("Library/Caches/com.postgresapp.Postgres2");
    if pg_cache.exists() {
        let size = get_dir_size(&pg_cache);
        if size > 0 {
            postgres_items.push(
                ItemDetail::new("Postgres.app Cache", size, &format_size(size))
                    .with_path(pg_cache)
            );
            total_size += size;
        }
    }

    if !postgres_items.is_empty() {
        result.add_item(
            CleanupItem::new("PostgreSQL Logs & Cache", total_size, &format_size(total_size))
                .with_details(postgres_items)
                .with_safe_to_delete(true)
                .with_warning("Database logs can be safely deleted when not debugging")
        );
    }
}

fn check_mysql_caches(result: &mut CheckResult, home: &PathBuf) {
    let mut mysql_items: Vec<ItemDetail> = Vec::new();
    let mut total_size: u64 = 0;

    // MySQL Workbench cache and logs
    let mysql_workbench = home.join("Library/Application Support/MySQL/Workbench");
    if mysql_workbench.exists() {
        // Log directory
        let log_dir = mysql_workbench.join("log");
        if log_dir.exists() {
            let size = get_dir_size(&log_dir);
            if size > 1024 * 1024 { // > 1MB
                mysql_items.push(
                    ItemDetail::new("MySQL Workbench Logs", size, &format_size(size))
                        .with_path(log_dir)
                );
                total_size += size;
            }
        }

        // SQL history
        let sql_history = mysql_workbench.join("sql_history");
        if sql_history.exists() {
            let size = get_dir_size(&sql_history);
            if size > 5 * 1024 * 1024 { // > 5MB
                mysql_items.push(
                    ItemDetail::new("MySQL Workbench SQL History", size, &format_size(size))
                        .with_path(sql_history)
                );
                total_size += size;
            }
        }
    }

    // MySQL cache
    let mysql_cache = home.join("Library/Caches/com.oracle.workbench.MySQLWorkbench");
    if mysql_cache.exists() {
        let size = get_dir_size(&mysql_cache);
        if size > 0 {
            mysql_items.push(
                ItemDetail::new("MySQL Workbench Cache", size, &format_size(size))
                    .with_path(mysql_cache)
            );
            total_size += size;
        }
    }

    // Sequel Pro / Sequel Ace
    let sequel_support = home.join("Library/Application Support/Sequel Pro");
    if sequel_support.exists() {
        let size = get_dir_size(&sequel_support);
        if size > 5 * 1024 * 1024 { // > 5MB
            mysql_items.push(
                ItemDetail::new("Sequel Pro Data", size, &format_size(size))
                    .with_path(sequel_support)
            );
            total_size += size;
        }
    }

    let sequel_ace_support = home.join("Library/Application Support/Sequel Ace");
    if sequel_ace_support.exists() {
        let size = get_dir_size(&sequel_ace_support);
        if size > 5 * 1024 * 1024 { // > 5MB
            mysql_items.push(
                ItemDetail::new("Sequel Ace Data", size, &format_size(size))
                    .with_path(sequel_ace_support)
            );
            total_size += size;
        }
    }

    if !mysql_items.is_empty() {
        result.add_item(
            CleanupItem::new("MySQL Logs & Cache", total_size, &format_size(total_size))
                .with_details(mysql_items)
                .with_safe_to_delete(true)
        );
    }
}

fn check_mongodb_caches(result: &mut CheckResult, home: &PathBuf) {
    let mut mongo_items: Vec<ItemDetail> = Vec::new();
    let mut total_size: u64 = 0;

    // MongoDB Compass cache
    let compass_cache = home.join("Library/Caches/mongodb-compass");
    if compass_cache.exists() {
        let size = get_dir_size(&compass_cache);
        if size > 0 {
            mongo_items.push(
                ItemDetail::new("MongoDB Compass Cache", size, &format_size(size))
                    .with_path(compass_cache)
            );
            total_size += size;
        }
    }

    // MongoDB Compass app support (logs, etc)
    let compass_support = home.join("Library/Application Support/MongoDB Compass");
    if compass_support.exists() {
        let logs_dir = compass_support.join("Logs");
        if logs_dir.exists() {
            let size = get_dir_size(&logs_dir);
            if size > 1024 * 1024 { // > 1MB
                mongo_items.push(
                    ItemDetail::new("MongoDB Compass Logs", size, &format_size(size))
                        .with_path(logs_dir)
                );
                total_size += size;
            }
        }
    }

    // MongoDB logs (if running locally via homebrew or manual install)
    let mongo_log = home.join("Library/Logs/MongoDB/mongo.log");
    if mongo_log.exists() {
        let size = std::fs::metadata(&mongo_log).map(|m| m.len()).unwrap_or(0);
        if size > 1024 * 1024 { // > 1MB
            mongo_items.push(
                ItemDetail::new("MongoDB Log", size, &format_size(size))
                    .with_path(mongo_log)
            );
            total_size += size;
        }
    }

    // MongoDB data directory (be very careful)
    let mongo_data = home.join("data/db");
    if mongo_data.exists() {
        // Only check for journal and diagnostic.data which can be cleaned
        let journal_dir = mongo_data.join("journal");
        if journal_dir.exists() {
            let size = get_dir_size(&journal_dir);
            if size > 100 * 1024 * 1024 { // > 100MB
                mongo_items.push(
                    ItemDetail::new("MongoDB Journal", size, &format_size(size))
                        .with_path(journal_dir)
                        .with_extra_info("Warning: Only delete when MongoDB is stopped")
                );
                total_size += size;
            }
        }

        let diag_dir = mongo_data.join("diagnostic.data");
        if diag_dir.exists() {
            let size = get_dir_size(&diag_dir);
            if size > 50 * 1024 * 1024 { // > 50MB
                mongo_items.push(
                    ItemDetail::new("MongoDB Diagnostic Data", size, &format_size(size))
                        .with_path(diag_dir)
                );
                total_size += size;
            }
        }
    }

    if !mongo_items.is_empty() {
        result.add_item(
            CleanupItem::new("MongoDB Logs & Cache", total_size, &format_size(total_size))
                .with_details(mongo_items)
                .with_safe_to_delete(true)
                .with_warning("Ensure MongoDB is stopped before cleaning journal")
        );
    }
}

fn check_redis_caches(result: &mut CheckResult, home: &PathBuf) {
    let mut redis_items: Vec<ItemDetail> = Vec::new();
    let mut total_size: u64 = 0;

    // Redis dump files in common locations
    let redis_locations = vec![
        home.join("dump.rdb"),
        home.join(".redis/dump.rdb"),
        PathBuf::from("/opt/homebrew/var/db/redis/dump.rdb"),
        PathBuf::from("/usr/local/var/db/redis/dump.rdb"),
    ];

    for dump_path in redis_locations {
        if dump_path.exists() {
            let size = std::fs::metadata(&dump_path).map(|m| m.len()).unwrap_or(0);
            if size > 10 * 1024 * 1024 { // > 10MB
                let location = if dump_path.starts_with(home) {
                    "Home"
                } else {
                    "Homebrew"
                };
                redis_items.push(
                    ItemDetail::new(&format!("Redis Dump ({})", location), size, &format_size(size))
                        .with_path(dump_path)
                        .with_extra_info("Warning: Contains Redis data backup")
                );
                total_size += size;
            }
        }
    }

    // RedisInsight cache
    let redis_insight_cache = home.join("Library/Caches/RedisInsight");
    if redis_insight_cache.exists() {
        let size = get_dir_size(&redis_insight_cache);
        if size > 0 {
            redis_items.push(
                ItemDetail::new("RedisInsight Cache", size, &format_size(size))
                    .with_path(redis_insight_cache)
            );
            total_size += size;
        }
    }

    // Another Redis Desktop Manager
    let ardm_cache = home.join("Library/Caches/Another Redis Desktop Manager");
    if ardm_cache.exists() {
        let size = get_dir_size(&ardm_cache);
        if size > 0 {
            redis_items.push(
                ItemDetail::new("Another Redis Desktop Manager Cache", size, &format_size(size))
                    .with_path(ardm_cache)
            );
            total_size += size;
        }
    }

    if !redis_items.is_empty() {
        result.add_item(
            CleanupItem::new("Redis Logs & Cache", total_size, &format_size(total_size))
                .with_details(redis_items)
                .with_warning("Redis dump files contain data - ensure you have backups")
        );
    }
}

fn check_sqlite_caches(result: &mut CheckResult, home: &PathBuf) {
    let mut sqlite_items: Vec<ItemDetail> = Vec::new();
    let mut total_size: u64 = 0;

    // DB Browser for SQLite
    let db_browser_cache = home.join("Library/Caches/com.sqlitebrowser.sqlitebrowser");
    if db_browser_cache.exists() {
        let size = get_dir_size(&db_browser_cache);
        if size > 0 {
            sqlite_items.push(
                ItemDetail::new("DB Browser for SQLite Cache", size, &format_size(size))
                    .with_path(db_browser_cache)
            );
            total_size += size;
        }
    }

    // TablePlus cache
    let tableplus_cache = home.join("Library/Caches/com.tableplus.TablePlus");
    if tableplus_cache.exists() {
        let size = get_dir_size(&tableplus_cache);
        if size > 0 {
            sqlite_items.push(
                ItemDetail::new("TablePlus Cache", size, &format_size(size))
                    .with_path(tableplus_cache)
            );
            total_size += size;
        }
    }

    // DBeaver
    let dbeaver_cache = home.join("Library/Caches/DBeaverData");
    if dbeaver_cache.exists() {
        let size = get_dir_size(&dbeaver_cache);
        if size > 0 {
            sqlite_items.push(
                ItemDetail::new("DBeaver Cache", size, &format_size(size))
                    .with_path(dbeaver_cache)
            );
            total_size += size;
        }
    }

    let dbeaver_support = home.join("Library/DBeaverData");
    if dbeaver_support.exists() {
        // Check workspace/.metadata which can grow large
        let metadata_dir = dbeaver_support.join("workspace6/.metadata");
        if metadata_dir.exists() {
            let size = get_dir_size(&metadata_dir);
            if size > 50 * 1024 * 1024 { // > 50MB
                sqlite_items.push(
                    ItemDetail::new("DBeaver Workspace Metadata", size, &format_size(size))
                        .with_path(metadata_dir)
                );
                total_size += size;
            }
        }
    }

    if !sqlite_items.is_empty() {
        result.add_item(
            CleanupItem::new("Database Tools Cache", total_size, &format_size(total_size))
                .with_details(sqlite_items)
                .with_safe_to_delete(true)
        );
    }
}

fn check_brew_db_data(result: &mut CheckResult, brew_var: &PathBuf) {
    let mut brew_db_items: Vec<ItemDetail> = Vec::new();
    let mut total_size: u64 = 0;

    // PostgreSQL logs
    let pg_log = brew_var.join("log/postgres.log");
    if pg_log.exists() {
        let size = std::fs::metadata(&pg_log).map(|m| m.len()).unwrap_or(0);
        if size > 1024 * 1024 { // > 1MB
            brew_db_items.push(
                ItemDetail::new("PostgreSQL Log (Homebrew)", size, &format_size(size))
                    .with_path(pg_log)
            );
            total_size += size;
        }
    }

    // PostgreSQL log directory
    let pg_log_dir = brew_var.join("log/postgresql@14");
    if pg_log_dir.exists() {
        let size = get_dir_size(&pg_log_dir);
        if size > 1024 * 1024 { // > 1MB
            brew_db_items.push(
                ItemDetail::new("PostgreSQL Logs Dir (Homebrew)", size, &format_size(size))
                    .with_path(pg_log_dir)
            );
            total_size += size;
        }
    }

    // MySQL error log
    let mysql_log = brew_var.join("log/mysql.log");
    if mysql_log.exists() {
        let size = std::fs::metadata(&mysql_log).map(|m| m.len()).unwrap_or(0);
        if size > 1024 * 1024 { // > 1MB
            brew_db_items.push(
                ItemDetail::new("MySQL Log (Homebrew)", size, &format_size(size))
                    .with_path(mysql_log)
            );
            total_size += size;
        }
    }

    // MySQL error log (alternate name)
    let mysql_err_log = brew_var.join("log/mysql/error.log");
    if mysql_err_log.exists() {
        let size = std::fs::metadata(&mysql_err_log).map(|m| m.len()).unwrap_or(0);
        if size > 1024 * 1024 { // > 1MB
            brew_db_items.push(
                ItemDetail::new("MySQL Error Log (Homebrew)", size, &format_size(size))
                    .with_path(mysql_err_log)
            );
            total_size += size;
        }
    }

    // MongoDB log
    let mongo_log = brew_var.join("log/mongodb/mongo.log");
    if mongo_log.exists() {
        let size = std::fs::metadata(&mongo_log).map(|m| m.len()).unwrap_or(0);
        if size > 1024 * 1024 { // > 1MB
            brew_db_items.push(
                ItemDetail::new("MongoDB Log (Homebrew)", size, &format_size(size))
                    .with_path(mongo_log)
            );
            total_size += size;
        }
    }

    // Redis log
    let redis_log = brew_var.join("log/redis.log");
    if redis_log.exists() {
        let size = std::fs::metadata(&redis_log).map(|m| m.len()).unwrap_or(0);
        if size > 1024 * 1024 { // > 1MB
            brew_db_items.push(
                ItemDetail::new("Redis Log (Homebrew)", size, &format_size(size))
                    .with_path(redis_log)
            );
            total_size += size;
        }
    }

    if !brew_db_items.is_empty() {
        result.add_item(
            CleanupItem::new("Homebrew Database Logs", total_size, &format_size(total_size))
                .with_details(brew_db_items)
                .with_safe_to_delete(true)
        );
    }
}
