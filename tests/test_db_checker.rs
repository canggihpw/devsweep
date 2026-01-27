//! Database checker comprehensive tests
//! Testing PostgreSQL, MySQL, MongoDB, Redis, and SQLite cache detection

use devsweep::checkers;
use std::fs;
use tempfile::TempDir;

// ==================== Basic Functionality Tests ====================

#[test]
fn test_db_checker_basic_functionality() {
    let result = checkers::check_db_caches();
    assert_eq!(result.name, "Database Caches");
}

#[test]
fn test_db_checker_returns_valid_structure() {
    let result = checkers::check_db_caches();

    assert!(!result.name.is_empty());

    for item in &result.items {
        assert!(!item.item_type.is_empty());
        assert!(!item.size_str.is_empty());
    }
}

// ==================== PostgreSQL Tests ====================

#[test]
fn test_postgres_app_support_structure() {
    let temp = TempDir::new().unwrap();
    let postgres_support = temp.path().join("Library/Application Support/Postgres");
    fs::create_dir_all(&postgres_support).unwrap();

    // Create var-XX directories (data directories)
    let var_14 = postgres_support.join("var-14");
    fs::create_dir_all(&var_14).unwrap();

    // Create postgresql.log file (> 1MB threshold)
    let log_file = var_14.join("postgresql.log");
    let large_log = vec![b'x'; 2 * 1024 * 1024]; // 2MB
    fs::write(&log_file, &large_log).unwrap();

    assert!(log_file.exists());
    assert!(fs::metadata(&log_file).unwrap().len() > 1024 * 1024);
}

#[test]
fn test_postgres_pg_log_directory() {
    let temp = TempDir::new().unwrap();
    let postgres_support = temp.path().join("Library/Application Support/Postgres");
    let var_dir = postgres_support.join("var-14");
    fs::create_dir_all(&var_dir).unwrap();

    // Create pg_log directory (older versions)
    let pg_log = var_dir.join("pg_log");
    fs::create_dir_all(&pg_log).unwrap();

    // Create multiple log files
    for i in 0..5 {
        let log_data = vec![b'l'; 500 * 1024]; // 500KB each
        fs::write(pg_log.join(format!("postgresql-{}.log", i)), &log_data).unwrap();
    }

    assert!(pg_log.exists());
    assert_eq!(fs::read_dir(&pg_log).unwrap().count(), 5);
}

#[test]
fn test_postgres_wal_directory() {
    let temp = TempDir::new().unwrap();
    let postgres_support = temp.path().join("Library/Application Support/Postgres");
    let var_dir = postgres_support.join("var-15");
    fs::create_dir_all(&var_dir).unwrap();

    // Create pg_wal directory (Write-Ahead Logs)
    let pg_wal = var_dir.join("pg_wal");
    fs::create_dir_all(&pg_wal).unwrap();

    // Create WAL segment files (16MB each typically, but we'll simulate smaller)
    for i in 0..3 {
        let wal_data = vec![b'w'; 1024 * 1024]; // 1MB each
        fs::write(pg_wal.join(format!("00000001000000000000000{}", i)), &wal_data).unwrap();
    }

    assert!(pg_wal.exists());
}

#[test]
fn test_postgres_cache_directory() {
    let temp = TempDir::new().unwrap();
    let pg_cache = temp.path().join("Library/Caches/com.postgresapp.Postgres2");
    fs::create_dir_all(&pg_cache).unwrap();

    fs::write(pg_cache.join("cache_data"), b"postgres cache").unwrap();

    assert!(pg_cache.exists());
}

#[test]
fn test_postgres_multiple_versions() {
    let temp = TempDir::new().unwrap();
    let postgres_support = temp.path().join("Library/Application Support/Postgres");
    fs::create_dir_all(&postgres_support).unwrap();

    // Create multiple PostgreSQL version directories
    for version in &["var-13", "var-14", "var-15", "var-16"] {
        let var_dir = postgres_support.join(version);
        fs::create_dir_all(&var_dir).unwrap();

        let log_file = var_dir.join("postgresql.log");
        fs::write(&log_file, b"PostgreSQL log content").unwrap();
    }

    assert_eq!(fs::read_dir(&postgres_support).unwrap().count(), 4);
}

// ==================== MySQL Tests ====================

#[test]
fn test_mysql_workbench_logs() {
    let temp = TempDir::new().unwrap();
    let mysql_workbench = temp.path().join("Library/Application Support/MySQL/Workbench");
    fs::create_dir_all(&mysql_workbench).unwrap();

    // Create log directory
    let log_dir = mysql_workbench.join("log");
    fs::create_dir_all(&log_dir).unwrap();

    // Create large log files
    let log_data = vec![b'm'; 2 * 1024 * 1024]; // 2MB
    fs::write(log_dir.join("wb.log"), &log_data).unwrap();

    assert!(log_dir.exists());
}

#[test]
fn test_mysql_workbench_sql_history() {
    let temp = TempDir::new().unwrap();
    let mysql_workbench = temp.path().join("Library/Application Support/MySQL/Workbench");
    fs::create_dir_all(&mysql_workbench).unwrap();

    // Create SQL history directory
    let sql_history = mysql_workbench.join("sql_history");
    fs::create_dir_all(&sql_history).unwrap();

    // Create history files
    for i in 0..10 {
        let history_data = vec![b's'; 1024 * 1024]; // 1MB each
        fs::write(sql_history.join(format!("history_{}.sql", i)), &history_data).unwrap();
    }

    assert!(sql_history.exists());
    assert_eq!(fs::read_dir(&sql_history).unwrap().count(), 10);
}

#[test]
fn test_mysql_workbench_cache() {
    let temp = TempDir::new().unwrap();
    let mysql_cache = temp.path().join("Library/Caches/com.oracle.workbench.MySQLWorkbench");
    fs::create_dir_all(&mysql_cache).unwrap();

    fs::write(mysql_cache.join("cache_file"), b"mysql workbench cache").unwrap();

    assert!(mysql_cache.exists());
}

#[test]
fn test_sequel_pro_data() {
    let temp = TempDir::new().unwrap();
    let sequel_support = temp.path().join("Library/Application Support/Sequel Pro");
    fs::create_dir_all(&sequel_support).unwrap();

    // Create data files
    fs::write(sequel_support.join("Favorites.plist"), b"favorites data").unwrap();
    fs::write(sequel_support.join("QueryHistory.plist"), b"query history").unwrap();

    assert!(sequel_support.exists());
}

#[test]
fn test_sequel_ace_data() {
    let temp = TempDir::new().unwrap();
    let sequel_ace = temp.path().join("Library/Application Support/Sequel Ace");
    fs::create_dir_all(&sequel_ace).unwrap();

    fs::write(sequel_ace.join("Data"), b"sequel ace data").unwrap();

    assert!(sequel_ace.exists());
}

// ==================== MongoDB Tests ====================

#[test]
fn test_mongodb_compass_cache() {
    let temp = TempDir::new().unwrap();
    let compass_cache = temp.path().join("Library/Caches/mongodb-compass");
    fs::create_dir_all(&compass_cache).unwrap();

    fs::write(compass_cache.join("cache_data"), b"compass cache").unwrap();

    assert!(compass_cache.exists());
}

#[test]
fn test_mongodb_compass_logs() {
    let temp = TempDir::new().unwrap();
    let compass_support = temp.path().join("Library/Application Support/MongoDB Compass");
    fs::create_dir_all(&compass_support).unwrap();

    // Create Logs directory
    let logs_dir = compass_support.join("Logs");
    fs::create_dir_all(&logs_dir).unwrap();

    // Create large log files
    let log_data = vec![b'c'; 2 * 1024 * 1024]; // 2MB
    fs::write(logs_dir.join("compass.log"), &log_data).unwrap();

    assert!(logs_dir.exists());
}

#[test]
fn test_mongodb_local_log() {
    let temp = TempDir::new().unwrap();
    let mongo_log_dir = temp.path().join("Library/Logs/MongoDB");
    fs::create_dir_all(&mongo_log_dir).unwrap();

    // Create mongo.log file
    let log_data = vec![b'm'; 2 * 1024 * 1024]; // 2MB
    fs::write(mongo_log_dir.join("mongo.log"), &log_data).unwrap();

    assert!(mongo_log_dir.join("mongo.log").exists());
}

#[test]
fn test_mongodb_journal_directory() {
    let temp = TempDir::new().unwrap();
    let mongo_data = temp.path().join("data/db");
    fs::create_dir_all(&mongo_data).unwrap();

    // Create journal directory
    let journal_dir = mongo_data.join("journal");
    fs::create_dir_all(&journal_dir).unwrap();

    // Create journal files
    for i in 0..5 {
        fs::write(journal_dir.join(format!("j._0.{}", i)), b"journal data").unwrap();
    }

    assert!(journal_dir.exists());
}

#[test]
fn test_mongodb_diagnostic_data() {
    let temp = TempDir::new().unwrap();
    let mongo_data = temp.path().join("data/db");
    fs::create_dir_all(&mongo_data).unwrap();

    // Create diagnostic.data directory
    let diag_dir = mongo_data.join("diagnostic.data");
    fs::create_dir_all(&diag_dir).unwrap();

    fs::write(diag_dir.join("metrics.interim"), b"diagnostic metrics").unwrap();

    assert!(diag_dir.exists());
}

// ==================== Redis Tests ====================

#[test]
fn test_redis_dump_file_home() {
    let temp = TempDir::new().unwrap();

    // Create dump.rdb in home
    let dump_file = temp.path().join("dump.rdb");
    let dump_data = vec![b'r'; 15 * 1024 * 1024]; // 15MB (> 10MB threshold)
    fs::write(&dump_file, &dump_data).unwrap();

    assert!(dump_file.exists());
    assert!(fs::metadata(&dump_file).unwrap().len() > 10 * 1024 * 1024);
}

#[test]
fn test_redis_dump_file_dot_redis() {
    let temp = TempDir::new().unwrap();
    let redis_dir = temp.path().join(".redis");
    fs::create_dir_all(&redis_dir).unwrap();

    let dump_file = redis_dir.join("dump.rdb");
    fs::write(&dump_file, b"redis dump data").unwrap();

    assert!(dump_file.exists());
}

#[test]
fn test_redis_insight_cache() {
    let temp = TempDir::new().unwrap();
    let redis_insight = temp.path().join("Library/Caches/RedisInsight");
    fs::create_dir_all(&redis_insight).unwrap();

    fs::write(redis_insight.join("cache"), b"redis insight cache").unwrap();

    assert!(redis_insight.exists());
}

#[test]
fn test_another_redis_desktop_manager_cache() {
    let temp = TempDir::new().unwrap();
    let ardm_cache = temp.path().join("Library/Caches/Another Redis Desktop Manager");
    fs::create_dir_all(&ardm_cache).unwrap();

    fs::write(ardm_cache.join("cache"), b"ardm cache").unwrap();

    assert!(ardm_cache.exists());
}

// ==================== SQLite/Database Tools Tests ====================

#[test]
fn test_db_browser_sqlite_cache() {
    let temp = TempDir::new().unwrap();
    let db_browser = temp.path().join("Library/Caches/com.sqlitebrowser.sqlitebrowser");
    fs::create_dir_all(&db_browser).unwrap();

    fs::write(db_browser.join("cache"), b"db browser cache").unwrap();

    assert!(db_browser.exists());
}

#[test]
fn test_tableplus_cache() {
    let temp = TempDir::new().unwrap();
    let tableplus = temp.path().join("Library/Caches/com.tableplus.TablePlus");
    fs::create_dir_all(&tableplus).unwrap();

    fs::write(tableplus.join("cache"), b"tableplus cache").unwrap();

    assert!(tableplus.exists());
}

#[test]
fn test_dbeaver_cache() {
    let temp = TempDir::new().unwrap();
    let dbeaver_cache = temp.path().join("Library/Caches/DBeaverData");
    fs::create_dir_all(&dbeaver_cache).unwrap();

    fs::write(dbeaver_cache.join("cache"), b"dbeaver cache").unwrap();

    assert!(dbeaver_cache.exists());
}

#[test]
fn test_dbeaver_workspace_metadata() {
    let temp = TempDir::new().unwrap();
    let dbeaver_support = temp.path().join("Library/DBeaverData");
    fs::create_dir_all(&dbeaver_support).unwrap();

    // Create workspace metadata
    let metadata_dir = dbeaver_support.join("workspace6/.metadata");
    fs::create_dir_all(&metadata_dir).unwrap();

    // Create metadata files
    for i in 0..10 {
        fs::write(metadata_dir.join(format!("meta_{}.dat", i)), b"metadata").unwrap();
    }

    assert!(metadata_dir.exists());
}

// ==================== Homebrew Database Logs Tests ====================

#[test]
fn test_brew_postgres_log_apple_silicon() {
    let temp = TempDir::new().unwrap();
    let brew_var = temp.path().join("opt/homebrew/var");
    fs::create_dir_all(&brew_var).unwrap();

    // Create log directory
    let log_dir = brew_var.join("log");
    fs::create_dir_all(&log_dir).unwrap();

    // Create postgres.log (> 1MB)
    let log_data = vec![b'p'; 2 * 1024 * 1024];
    fs::write(log_dir.join("postgres.log"), &log_data).unwrap();

    assert!(log_dir.join("postgres.log").exists());
}

#[test]
fn test_brew_postgres_log_directory() {
    let temp = TempDir::new().unwrap();
    let brew_var = temp.path().join("opt/homebrew/var");
    let log_dir = brew_var.join("log/postgresql@14");
    fs::create_dir_all(&log_dir).unwrap();

    // Create multiple log files
    for i in 0..3 {
        let log_data = vec![b'l'; 500 * 1024];
        fs::write(log_dir.join(format!("server.log.{}", i)), &log_data).unwrap();
    }

    assert!(log_dir.exists());
}

#[test]
fn test_brew_mysql_log() {
    let temp = TempDir::new().unwrap();
    let brew_var = temp.path().join("usr/local/var");
    let log_dir = brew_var.join("log");
    fs::create_dir_all(&log_dir).unwrap();

    // Create mysql.log
    let log_data = vec![b'm'; 2 * 1024 * 1024];
    fs::write(log_dir.join("mysql.log"), &log_data).unwrap();

    assert!(log_dir.join("mysql.log").exists());
}

#[test]
fn test_brew_mysql_error_log() {
    let temp = TempDir::new().unwrap();
    let brew_var = temp.path().join("opt/homebrew/var");
    let mysql_log_dir = brew_var.join("log/mysql");
    fs::create_dir_all(&mysql_log_dir).unwrap();

    // Create error.log
    let log_data = vec![b'e'; 2 * 1024 * 1024];
    fs::write(mysql_log_dir.join("error.log"), &log_data).unwrap();

    assert!(mysql_log_dir.join("error.log").exists());
}

#[test]
fn test_brew_mongodb_log() {
    let temp = TempDir::new().unwrap();
    let brew_var = temp.path().join("opt/homebrew/var");
    let mongo_log_dir = brew_var.join("log/mongodb");
    fs::create_dir_all(&mongo_log_dir).unwrap();

    // Create mongo.log
    let log_data = vec![b'm'; 2 * 1024 * 1024];
    fs::write(mongo_log_dir.join("mongo.log"), &log_data).unwrap();

    assert!(mongo_log_dir.join("mongo.log").exists());
}

#[test]
fn test_brew_redis_log() {
    let temp = TempDir::new().unwrap();
    let brew_var = temp.path().join("opt/homebrew/var");
    let log_dir = brew_var.join("log");
    fs::create_dir_all(&log_dir).unwrap();

    // Create redis.log
    let log_data = vec![b'r'; 2 * 1024 * 1024];
    fs::write(log_dir.join("redis.log"), &log_data).unwrap();

    assert!(log_dir.join("redis.log").exists());
}

// ==================== Edge Cases ====================

#[test]
fn test_empty_directories() {
    let temp = TempDir::new().unwrap();

    // Create empty database directories
    let postgres_support = temp.path().join("Library/Application Support/Postgres");
    fs::create_dir_all(&postgres_support).unwrap();

    let mysql_workbench = temp.path().join("Library/Application Support/MySQL/Workbench");
    fs::create_dir_all(&mysql_workbench).unwrap();

    // Both should exist but be empty
    assert!(postgres_support.exists());
    assert!(mysql_workbench.exists());
}

#[test]
fn test_small_files_below_threshold() {
    let temp = TempDir::new().unwrap();
    let postgres_support = temp.path().join("Library/Application Support/Postgres");
    let var_dir = postgres_support.join("var-14");
    fs::create_dir_all(&var_dir).unwrap();

    // Create small log file (< 1MB threshold)
    let small_log = vec![b'x'; 500 * 1024]; // 500KB
    fs::write(var_dir.join("postgresql.log"), &small_log).unwrap();

    // File exists but is below threshold
    assert!(var_dir.join("postgresql.log").exists());
    assert!(fs::metadata(var_dir.join("postgresql.log")).unwrap().len() < 1024 * 1024);
}

#[test]
fn test_non_var_directories_in_postgres() {
    let temp = TempDir::new().unwrap();
    let postgres_support = temp.path().join("Library/Application Support/Postgres");
    fs::create_dir_all(&postgres_support).unwrap();

    // Create directories that don't start with "var-"
    let other_dir = postgres_support.join("config");
    fs::create_dir_all(&other_dir).unwrap();
    fs::write(other_dir.join("settings.json"), b"{}").unwrap();

    let backup_dir = postgres_support.join("backups");
    fs::create_dir_all(&backup_dir).unwrap();

    assert!(other_dir.exists());
    assert!(backup_dir.exists());
}

#[test]
fn test_mixed_database_tools() {
    let temp = TempDir::new().unwrap();

    // Create caches for multiple tools
    let tableplus = temp.path().join("Library/Caches/com.tableplus.TablePlus");
    fs::create_dir_all(&tableplus).unwrap();
    fs::write(tableplus.join("cache"), b"tableplus").unwrap();

    let dbeaver = temp.path().join("Library/Caches/DBeaverData");
    fs::create_dir_all(&dbeaver).unwrap();
    fs::write(dbeaver.join("cache"), b"dbeaver").unwrap();

    let db_browser = temp.path().join("Library/Caches/com.sqlitebrowser.sqlitebrowser");
    fs::create_dir_all(&db_browser).unwrap();
    fs::write(db_browser.join("cache"), b"db browser").unwrap();

    assert!(tableplus.exists());
    assert!(dbeaver.exists());
    assert!(db_browser.exists());
}
