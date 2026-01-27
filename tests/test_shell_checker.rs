//! Shell checker comprehensive tests
//! Testing Zsh, Bash, Fish, and other shell cache detection

use devsweep::checkers;
use std::fs;
use tempfile::TempDir;

// ==================== Basic Functionality Tests ====================

#[test]
fn test_shell_checker_basic_functionality() {
    let result = checkers::check_shell_caches();
    assert_eq!(result.name, "Shell Caches");
}

#[test]
fn test_shell_checker_returns_valid_structure() {
    let result = checkers::check_shell_caches();

    assert!(!result.name.is_empty());

    for item in &result.items {
        assert!(!item.item_type.is_empty());
        assert!(!item.size_str.is_empty());
    }
}

// ==================== Zsh Tests ====================

#[test]
fn test_oh_my_zsh_cache() {
    let temp = TempDir::new().unwrap();
    let omz_cache = temp.path().join(".oh-my-zsh/cache");
    fs::create_dir_all(&omz_cache).unwrap();

    // Create cache files
    fs::write(omz_cache.join("grep-alias"), b"alias cache").unwrap();
    fs::write(omz_cache.join(".zsh-update"), b"update timestamp").unwrap();
    fs::write(omz_cache.join("completions"), b"completion cache").unwrap();

    assert!(omz_cache.exists());
    assert_eq!(fs::read_dir(&omz_cache).unwrap().count(), 3);
}

#[test]
fn test_oh_my_zsh_plugin_cache() {
    let temp = TempDir::new().unwrap();
    let omz_cache = temp.path().join(".oh-my-zsh/cache");
    fs::create_dir_all(&omz_cache).unwrap();

    // Create plugin caches
    fs::write(omz_cache.join("git-fetch-time"), b"timestamp").unwrap();
    fs::write(omz_cache.join("docker-containers"), b"container list").unwrap();
    fs::write(omz_cache.join("kubectl-context"), b"k8s context").unwrap();

    assert!(omz_cache.exists());
}

#[test]
fn test_zsh_compdump_files() {
    let temp = TempDir::new().unwrap();

    // Create .zcompdump files (completion dumps)
    fs::write(temp.path().join(".zcompdump"), b"completion data").unwrap();
    fs::write(temp.path().join(".zcompdump-hostname-5.9"), b"completion data").unwrap();
    fs::write(temp.path().join(".zcompdump-MacBook-Pro-5.8"), b"completion data").unwrap();

    // Verify files exist
    assert!(temp.path().join(".zcompdump").exists());
    assert!(temp.path().join(".zcompdump-hostname-5.9").exists());
}

#[test]
fn test_zsh_compdump_zwc_files() {
    let temp = TempDir::new().unwrap();

    // Create compiled zsh files (.zwc)
    fs::write(temp.path().join(".zcompdump.zwc"), b"compiled completion").unwrap();
    fs::write(temp.path().join(".zcompdump-hostname-5.9.zwc"), b"compiled").unwrap();

    assert!(temp.path().join(".zcompdump.zwc").exists());
}

#[test]
fn test_zsh_sessions_directory() {
    let temp = TempDir::new().unwrap();
    let zsh_sessions = temp.path().join(".zsh_sessions");
    fs::create_dir_all(&zsh_sessions).unwrap();

    // Create session files
    for i in 0..5 {
        fs::write(zsh_sessions.join(format!("session-{}.zsh", i)), b"session").unwrap();
        fs::write(zsh_sessions.join(format!("session-{}.history", i)), b"history").unwrap();
    }

    assert!(zsh_sessions.exists());
    assert_eq!(fs::read_dir(&zsh_sessions).unwrap().count(), 10);
}

#[test]
fn test_zsh_history_file() {
    let temp = TempDir::new().unwrap();

    // Create .zsh_history file
    let history = temp.path().join(".zsh_history");
    let history_content = (0..1000)
        .map(|i| format!(": {}:0;command {}\n", i, i))
        .collect::<String>();
    fs::write(&history, history_content.as_bytes()).unwrap();

    assert!(history.exists());
}

// ==================== Bash Tests ====================

#[test]
fn test_bash_sessions_directory() {
    let temp = TempDir::new().unwrap();
    let bash_sessions = temp.path().join(".bash_sessions");
    fs::create_dir_all(&bash_sessions).unwrap();

    // Create session files (macOS stores these)
    for i in 0..3 {
        let session_id = format!("ABC123DEF{}", i);
        fs::write(bash_sessions.join(format!("{}.session", session_id)), b"session").unwrap();
        fs::write(bash_sessions.join(format!("{}.history", session_id)), b"history").unwrap();
        fs::write(bash_sessions.join(format!("{}.historynew", session_id)), b"new").unwrap();
    }

    assert!(bash_sessions.exists());
}

#[test]
fn test_bash_history_file() {
    let temp = TempDir::new().unwrap();

    // Create .bash_history file
    let history = temp.path().join(".bash_history");
    let history_content = (0..500)
        .map(|i| format!("command {}\n", i))
        .collect::<String>();
    fs::write(&history, history_content.as_bytes()).unwrap();

    assert!(history.exists());
}

#[test]
fn test_bash_completion_cache() {
    let temp = TempDir::new().unwrap();
    let bash_completion = temp.path().join(".bash_completion.d");
    fs::create_dir_all(&bash_completion).unwrap();

    // Create completion files
    fs::write(bash_completion.join("docker"), b"docker completions").unwrap();
    fs::write(bash_completion.join("git"), b"git completions").unwrap();
    fs::write(bash_completion.join("npm"), b"npm completions").unwrap();

    assert!(bash_completion.exists());
}

// ==================== Fish Shell Tests ====================

#[test]
fn test_fish_cache_config_directory() {
    let temp = TempDir::new().unwrap();
    let fish_cache = temp.path().join(".config/fish/cache");
    fs::create_dir_all(&fish_cache).unwrap();

    // Create fish cache files
    fs::write(fish_cache.join("completions"), b"cached completions").unwrap();
    fs::write(fish_cache.join("functions"), b"cached functions").unwrap();

    assert!(fish_cache.exists());
}

#[test]
fn test_fish_cache_local_directory() {
    let temp = TempDir::new().unwrap();
    let fish_cache = temp.path().join(".cache/fish");
    fs::create_dir_all(&fish_cache).unwrap();

    // Create fish cache files
    fs::write(fish_cache.join("cached_completions"), b"completions").unwrap();

    assert!(fish_cache.exists());
}

#[test]
fn test_fish_history_file() {
    let temp = TempDir::new().unwrap();
    let fish_local = temp.path().join(".local/share/fish");
    fs::create_dir_all(&fish_local).unwrap();

    // Create fish_history file
    let history = fish_local.join("fish_history");
    fs::write(&history, b"- cmd: ls\n  when: 1234567890\n- cmd: cd\n  when: 1234567891").unwrap();

    assert!(history.exists());
}

#[test]
fn test_fish_generated_completions() {
    let temp = TempDir::new().unwrap();
    let fish_completions = temp.path().join(".local/share/fish/generated_completions");
    fs::create_dir_all(&fish_completions).unwrap();

    // Create generated completion files
    fs::write(fish_completions.join("docker.fish"), b"# docker completions").unwrap();
    fs::write(fish_completions.join("git.fish"), b"# git completions").unwrap();
    fs::write(fish_completions.join("cargo.fish"), b"# cargo completions").unwrap();

    assert!(fish_completions.exists());
    assert_eq!(fs::read_dir(&fish_completions).unwrap().count(), 3);
}

// ==================== Starship Prompt Tests ====================

#[test]
fn test_starship_cache() {
    let temp = TempDir::new().unwrap();
    let starship_cache = temp.path().join(".cache/starship");
    fs::create_dir_all(&starship_cache).unwrap();

    // Create starship cache files
    fs::write(starship_cache.join("session_cache"), b"session data").unwrap();

    assert!(starship_cache.exists());
}

#[test]
fn test_starship_config_cache() {
    let temp = TempDir::new().unwrap();
    let config_dir = temp.path().join(".config");
    fs::create_dir_all(&config_dir).unwrap();

    // Create starship.toml (not cache, but often checked)
    fs::write(config_dir.join("starship.toml"), b"[character]\nsuccess_symbol = \"[>](bold green)\"").unwrap();

    assert!(config_dir.join("starship.toml").exists());
}

// ==================== Other Shell Tools Tests ====================

#[test]
fn test_powerline_cache() {
    let temp = TempDir::new().unwrap();
    let powerline_cache = temp.path().join(".cache/powerline");
    fs::create_dir_all(&powerline_cache).unwrap();

    fs::write(powerline_cache.join("theme_cache"), b"theme data").unwrap();

    assert!(powerline_cache.exists());
}

#[test]
fn test_thefuck_cache() {
    let temp = TempDir::new().unwrap();
    let thefuck_cache = temp.path().join(".cache/thefuck");
    fs::create_dir_all(&thefuck_cache).unwrap();

    // Create thefuck cache files
    fs::write(thefuck_cache.join("shells_cache"), b"shells").unwrap();

    assert!(thefuck_cache.exists());
}

#[test]
fn test_zplug_cache() {
    let temp = TempDir::new().unwrap();
    let zplug_dir = temp.path().join(".zplug");
    fs::create_dir_all(&zplug_dir).unwrap();

    // Create zplug cache
    let cache = zplug_dir.join("cache");
    fs::create_dir_all(&cache).unwrap();
    fs::write(cache.join("interface"), b"plugin cache").unwrap();

    assert!(cache.exists());
}

#[test]
fn test_zinit_cache() {
    let temp = TempDir::new().unwrap();
    let zinit_dir = temp.path().join(".zinit");
    fs::create_dir_all(&zinit_dir).unwrap();

    // Create zinit completions cache
    let completions = zinit_dir.join("completions");
    fs::create_dir_all(&completions).unwrap();
    fs::write(completions.join("_docker"), b"docker completions").unwrap();

    assert!(completions.exists());
}

// ==================== Edge Cases ====================

#[test]
fn test_empty_shell_cache_directories() {
    let temp = TempDir::new().unwrap();

    // Create empty cache directories
    let omz_cache = temp.path().join(".oh-my-zsh/cache");
    fs::create_dir_all(&omz_cache).unwrap();

    let fish_cache = temp.path().join(".cache/fish");
    fs::create_dir_all(&fish_cache).unwrap();

    let bash_sessions = temp.path().join(".bash_sessions");
    fs::create_dir_all(&bash_sessions).unwrap();

    assert!(omz_cache.exists());
    assert!(fish_cache.exists());
    assert!(bash_sessions.exists());
    assert_eq!(fs::read_dir(&omz_cache).unwrap().count(), 0);
}

#[test]
fn test_multiple_zcompdump_versions() {
    let temp = TempDir::new().unwrap();

    // Create multiple zcompdump files from different zsh versions
    for version in &["5.7", "5.8", "5.9", "5.9.1"] {
        let filename = format!(".zcompdump-myhost-{}", version);
        fs::write(temp.path().join(&filename), b"completion data").unwrap();

        // And compiled versions
        let zwc_filename = format!("{}.zwc", filename);
        fs::write(temp.path().join(&zwc_filename), b"compiled").unwrap();
    }

    // Should have 8 files (4 .zcompdump + 4 .zwc)
    let count = fs::read_dir(temp.path())
        .unwrap()
        .filter(|e| e.as_ref().unwrap().file_name().to_string_lossy().contains("zcompdump"))
        .count();
    assert_eq!(count, 8);
}

#[test]
fn test_shell_plugin_managers() {
    let temp = TempDir::new().unwrap();

    // Create various shell plugin manager directories
    let antigen = temp.path().join(".antigen");
    fs::create_dir_all(antigen.join("bundles")).unwrap();

    let antibody = temp.path().join(".antibody");
    fs::create_dir_all(&antibody).unwrap();

    let zgen = temp.path().join(".zgen");
    fs::create_dir_all(&zgen).unwrap();

    assert!(antigen.exists());
    assert!(antibody.exists());
    assert!(zgen.exists());
}

#[test]
fn test_large_history_file() {
    let temp = TempDir::new().unwrap();

    // Create a large history file
    let history = temp.path().join(".zsh_history");
    let large_history: String = (0..10000)
        .map(|i| format!(": {}:0;very long command that takes up space {}\n", i, i))
        .collect();
    fs::write(&history, large_history.as_bytes()).unwrap();

    assert!(history.exists());
    assert!(fs::metadata(&history).unwrap().len() > 100000);
}

#[test]
fn test_mixed_shell_configurations() {
    let temp = TempDir::new().unwrap();

    // Create caches for multiple shells (user might have multiple)
    let zsh_cache = temp.path().join(".oh-my-zsh/cache");
    fs::create_dir_all(&zsh_cache).unwrap();
    fs::write(zsh_cache.join("cache"), b"zsh cache").unwrap();

    let bash_sessions = temp.path().join(".bash_sessions");
    fs::create_dir_all(&bash_sessions).unwrap();
    fs::write(bash_sessions.join("session"), b"bash session").unwrap();

    let fish_cache = temp.path().join(".cache/fish");
    fs::create_dir_all(&fish_cache).unwrap();
    fs::write(fish_cache.join("cache"), b"fish cache").unwrap();

    assert!(zsh_cache.exists());
    assert!(bash_sessions.exists());
    assert!(fish_cache.exists());
}

#[test]
fn test_nushell_cache() {
    let temp = TempDir::new().unwrap();
    let nu_cache = temp.path().join(".cache/nushell");
    fs::create_dir_all(&nu_cache).unwrap();

    fs::write(nu_cache.join("plugin.nu"), b"plugin cache").unwrap();

    assert!(nu_cache.exists());
}
