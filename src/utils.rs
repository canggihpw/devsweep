use bytesize::ByteSize;
use rayon::prelude::*;
use std::path::Path;
use std::process::Command;
use walkdir::WalkDir;

/// Convert bytes to human-readable string
pub fn format_size(bytes: u64) -> String {
    ByteSize(bytes).to_string_as(true)
}

/// Get total size of a directory using parallel iteration for better performance
pub fn get_dir_size<P: AsRef<Path>>(path: P) -> u64 {
    let path = path.as_ref();
    if !path.exists() {
        return 0;
    }

    WalkDir::new(path)
        .into_iter()
        .par_bridge()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .filter_map(|e| e.metadata().ok())
        .map(|m| m.len())
        .sum()
}

/// Run a command and return stdout as String
pub fn run_command(cmd: &str, args: &[&str]) -> Option<String> {
    Command::new(cmd)
        .args(args)
        .output()
        .ok()
        .filter(|o| o.status.success())
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
}

/// Get home directory path
pub fn home_dir() -> Option<std::path::PathBuf> {
    dirs::home_dir()
}

/// Sort version strings naturally (handles semantic versioning)
pub fn sort_versions(versions: &mut [String]) {
    versions.sort_by(|a, b| {
        let parse_version = |s: &str| -> Vec<u64> {
            s.split(|c: char| !c.is_ascii_digit())
                .filter_map(|part| part.parse().ok())
                .collect()
        };
        let va = parse_version(a);
        let vb = parse_version(b);
        va.cmp(&vb)
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_size() {
        assert!(format_size(1024).contains("Ki") || format_size(1024).contains("KB"));
    }

    #[test]
    fn test_sort_versions() {
        let mut versions = vec![
            "1.0.0".to_string(),
            "2.0.0".to_string(),
            "1.10.0".to_string(),
            "1.2.0".to_string(),
        ];
        sort_versions(&mut versions);
        assert_eq!(versions[0], "1.0.0");
        assert_eq!(versions[1], "1.2.0");
        assert_eq!(versions[2], "1.10.0");
        assert_eq!(versions[3], "2.0.0");
    }
}
