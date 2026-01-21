use crate::types::{CheckResult, CleanupItem};
use crate::utils::{format_size, run_command};

pub fn check_docker() -> CheckResult {
    let mut result = CheckResult::new("Docker");

    // Check if Docker is installed and running
    let docker_info = run_command("docker", &["info"]);
    if docker_info.is_none() {
        result.status = Some("Docker not installed or not running".to_string());
        return result;
    }

    result.status = Some("installed".to_string());

    // Get Docker disk usage summary
    if let Some(df_output) = run_command("docker", &["system", "df"]) {
        result.extra_data.docker_summary = Some(df_output);
    }

    // Count dangling images
    if let Some(dangling) = run_command("docker", &["images", "-f", "dangling=true", "-q"]) {
        let count = dangling.lines().filter(|l| !l.is_empty()).count();
        if count > 0 {
            result.extra_data.dangling_images = Some(count);
        }
    }

    // Count stopped containers
    if let Some(stopped) = run_command("docker", &["ps", "-a", "-f", "status=exited", "-q"]) {
        let count = stopped.lines().filter(|l| !l.is_empty()).count();
        if count > 0 {
            result.extra_data.stopped_containers = Some(count);
        }
    }

    // Try to get reclaimable space from docker system df
    if let Some(df_output) = run_command(
        "docker",
        &[
            "system",
            "df",
            "--format",
            "{{.Type}}\t{{.Size}}\t{{.Reclaimable}}",
        ],
    ) {
        let mut total_reclaimable = 0u64;
        for line in df_output.lines() {
            let parts: Vec<&str> = line.split('\t').collect();
            if parts.len() >= 3 {
                // Parse reclaimable size (format like "1.2GB (50%)")
                let reclaimable_str = parts[2].split_whitespace().next().unwrap_or("0");
                if let Some(bytes) = parse_docker_size(reclaimable_str) {
                    total_reclaimable += bytes;
                }
            }
        }

        if total_reclaimable > 0 {
            let item = CleanupItem::new(
                "Docker Reclaimable Space",
                total_reclaimable,
                &format_size(total_reclaimable),
            )
            .with_safe_to_delete(true)
            .with_cleanup_command("docker system prune");
            result.add_item(item);
        }
    }

    result
}

fn parse_docker_size(s: &str) -> Option<u64> {
    let s = s.trim();
    if s == "0B" || s == "0" {
        return Some(0);
    }

    let (num_str, unit) = if s.ends_with("GB") {
        (s.trim_end_matches("GB"), 1024 * 1024 * 1024)
    } else if s.ends_with("MB") {
        (s.trim_end_matches("MB"), 1024 * 1024)
    } else if s.ends_with("KB") || s.ends_with("kB") {
        (s.trim_end_matches("KB").trim_end_matches("kB"), 1024)
    } else if s.ends_with('B') {
        (s.trim_end_matches('B'), 1)
    } else {
        return None;
    };

    num_str
        .parse::<f64>()
        .ok()
        .map(|n| (n * unit as f64) as u64)
}
