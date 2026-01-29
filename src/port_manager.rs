//! Port Manager Module
//!
//! Provides functionality to list processes using network ports and kill them.
//! Useful for developers who need to free up ports for local development.

use std::collections::HashMap;
use std::process::Command;

/// Common development ports with their typical usage
pub const COMMON_PORTS: &[(u16, &str)] = &[
    (3000, "React/Node.js dev server"),
    (3001, "React dev server (alternate)"),
    (4000, "Phoenix/GraphQL"),
    (5000, "Flask/Python dev server"),
    (5173, "Vite dev server"),
    (5174, "Vite dev server (alternate)"),
    (8000, "Django/Python"),
    (8080, "HTTP alternate/Tomcat"),
    (8081, "HTTP alternate"),
    (8443, "HTTPS alternate"),
    (9000, "PHP-FPM/SonarQube"),
    (9090, "Prometheus"),
    (27017, "MongoDB"),
    (5432, "PostgreSQL"),
    (3306, "MySQL"),
    (6379, "Redis"),
    (11211, "Memcached"),
];

/// Represents a process using a network port
#[derive(Debug, Clone)]
pub struct PortProcess {
    /// The port number
    pub port: u16,
    /// Process ID
    pub pid: u32,
    /// Process name/command
    pub process_name: String,
    /// User running the process
    pub user: String,
    /// Protocol (TCP/UDP)
    pub protocol: String,
    /// Local address (e.g., "127.0.0.1" or "*")
    pub local_address: String,
    /// Connection state (e.g., "LISTEN", "ESTABLISHED")
    pub state: String,
}

/// Result of a kill operation
#[derive(Debug, Clone)]
pub struct KillResult {
    pub pid: u32,
    pub port: u16,
    pub success: bool,
    pub message: String,
}

/// Get all processes listening on ports
pub fn get_listening_ports() -> Vec<PortProcess> {
    let mut processes = Vec::new();

    // Use lsof to get listening ports on macOS
    // -i: Select Internet addresses
    // -P: Inhibit port number to service name conversion
    // -n: Inhibit host name conversion
    // -sTCP:LISTEN: Only show TCP connections in LISTEN state
    let output = Command::new("lsof")
        .args(["-i", "-P", "-n", "-sTCP:LISTEN"])
        .output();

    if let Ok(output) = output {
        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            processes.extend(parse_lsof_output(&stdout, "TCP", "LISTEN"));
        }
    }

    // Also check UDP ports
    let output = Command::new("lsof")
        .args(["-i", "UDP", "-P", "-n"])
        .output();

    if let Ok(output) = output {
        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            processes.extend(parse_lsof_output(&stdout, "UDP", ""));
        }
    }

    // Sort by port number
    processes.sort_by_key(|p| p.port);

    // Deduplicate by (port, pid) - keep first occurrence
    let mut seen = std::collections::HashSet::new();
    processes.retain(|p| seen.insert((p.port, p.pid)));

    processes
}

/// Get processes using a specific port
pub fn get_processes_on_port(port: u16) -> Vec<PortProcess> {
    let mut processes = Vec::new();

    // Use lsof to find processes on specific port
    let output = Command::new("lsof")
        .args(["-i", &format!(":{}", port), "-P", "-n"])
        .output();

    if let Ok(output) = output {
        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            processes.extend(parse_lsof_output(&stdout, "", ""));
        }
    }

    processes
}

/// Parse lsof output into PortProcess structs
fn parse_lsof_output(
    output: &str,
    default_protocol: &str,
    default_state: &str,
) -> Vec<PortProcess> {
    let mut processes = Vec::new();

    for line in output.lines().skip(1) {
        // Skip header line
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() < 9 {
            continue;
        }

        // lsof output format:
        // COMMAND  PID  USER   FD   TYPE  DEVICE  SIZE/OFF  NODE  NAME
        // node    1234  user   23u  IPv4  0x1234  0t0       TCP   *:3000 (LISTEN)

        let process_name = parts[0].to_string();
        let pid = parts[1].parse::<u32>().unwrap_or(0);
        let user = parts[2].to_string();

        // Find the NAME column (usually last or second to last)
        let name_idx = parts.len() - 1;
        let name = parts[name_idx];

        // Check for state in parentheses
        let state = if name.starts_with('(') && name.ends_with(')') {
            let s = &name[1..name.len() - 1];
            let actual_name = if name_idx > 0 {
                parts[name_idx - 1]
            } else {
                ""
            };
            (actual_name, s.to_string())
        } else {
            (name, default_state.to_string())
        };

        let name_str = state.0;
        let conn_state = state.1;

        // Parse address:port from NAME field (e.g., "*:3000" or "127.0.0.1:8080")
        if let Some(colon_pos) = name_str.rfind(':') {
            let addr = &name_str[..colon_pos];
            let port_str = &name_str[colon_pos + 1..];

            if let Ok(port) = port_str.parse::<u16>() {
                // Determine protocol from TYPE column if available
                let protocol = if parts.len() > 4 {
                    let type_str = parts[4];
                    if type_str.contains("TCP") || parts.iter().any(|p| *p == "TCP") {
                        "TCP".to_string()
                    } else if type_str.contains("UDP") || parts.iter().any(|p| *p == "UDP") {
                        "UDP".to_string()
                    } else {
                        default_protocol.to_string()
                    }
                } else {
                    default_protocol.to_string()
                };

                processes.push(PortProcess {
                    port,
                    pid,
                    process_name,
                    user,
                    protocol,
                    local_address: addr.to_string(),
                    state: conn_state,
                });
            }
        }
    }

    processes
}

/// Kill a process by PID
pub fn kill_process(pid: u32) -> KillResult {
    // First try SIGTERM (graceful)
    let output = Command::new("kill").arg(pid.to_string()).output();

    match output {
        Ok(result) => {
            if result.status.success() {
                KillResult {
                    pid,
                    port: 0,
                    success: true,
                    message: format!("Process {} terminated successfully", pid),
                }
            } else {
                let stderr = String::from_utf8_lossy(&result.stderr);
                KillResult {
                    pid,
                    port: 0,
                    success: false,
                    message: format!("Failed to kill process {}: {}", pid, stderr.trim()),
                }
            }
        }
        Err(e) => KillResult {
            pid,
            port: 0,
            success: false,
            message: format!("Failed to execute kill command: {}", e),
        },
    }
}

/// Force kill a process by PID (SIGKILL)
pub fn force_kill_process(pid: u32) -> KillResult {
    let output = Command::new("kill").args(["-9", &pid.to_string()]).output();

    match output {
        Ok(result) => {
            if result.status.success() {
                KillResult {
                    pid,
                    port: 0,
                    success: true,
                    message: format!("Process {} force killed successfully", pid),
                }
            } else {
                let stderr = String::from_utf8_lossy(&result.stderr);
                KillResult {
                    pid,
                    port: 0,
                    success: false,
                    message: format!("Failed to force kill process {}: {}", pid, stderr.trim()),
                }
            }
        }
        Err(e) => KillResult {
            pid,
            port: 0,
            success: false,
            message: format!("Failed to execute kill command: {}", e),
        },
    }
}

/// Kill all processes on a specific port
pub fn kill_processes_on_port(port: u16, force: bool) -> Vec<KillResult> {
    let processes = get_processes_on_port(port);
    let mut results = Vec::new();

    for process in processes {
        let mut result = if force {
            force_kill_process(process.pid)
        } else {
            kill_process(process.pid)
        };
        result.port = port;
        results.push(result);
    }

    if results.is_empty() {
        results.push(KillResult {
            pid: 0,
            port,
            success: false,
            message: format!("No processes found on port {}", port),
        });
    }

    results
}

/// Get common port description if available
pub fn get_port_description(port: u16) -> Option<&'static str> {
    COMMON_PORTS
        .iter()
        .find(|(p, _)| *p == port)
        .map(|(_, desc)| *desc)
}

/// Group processes by port
pub fn group_by_port(processes: &[PortProcess]) -> HashMap<u16, Vec<&PortProcess>> {
    let mut grouped: HashMap<u16, Vec<&PortProcess>> = HashMap::new();
    for process in processes {
        grouped.entry(process.port).or_default().push(process);
    }
    grouped
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_common_ports_not_empty() {
        assert!(!COMMON_PORTS.is_empty());
    }

    #[test]
    fn test_get_port_description() {
        assert_eq!(get_port_description(3000), Some("React/Node.js dev server"));
        assert_eq!(get_port_description(5432), Some("PostgreSQL"));
        assert_eq!(get_port_description(12345), None); // Port not in common list
    }

    #[test]
    fn test_parse_lsof_output() {
        let sample_output = r#"COMMAND   PID   USER   FD   TYPE             DEVICE SIZE/OFF NODE NAME
node    12345   user   23u  IPv4 0x1234567890      0t0  TCP *:3000 (LISTEN)
node    12346   user   24u  IPv6 0x1234567891      0t0  TCP *:3001 (LISTEN)
"#;
        let processes = parse_lsof_output(sample_output, "TCP", "LISTEN");
        assert_eq!(processes.len(), 2);
        assert_eq!(processes[0].port, 3000);
        assert_eq!(processes[0].pid, 12345);
        assert_eq!(processes[1].port, 3001);
    }

    #[test]
    fn test_group_by_port() {
        let processes = vec![
            PortProcess {
                port: 3000,
                pid: 1234,
                process_name: "node".to_string(),
                user: "user".to_string(),
                protocol: "TCP".to_string(),
                local_address: "*".to_string(),
                state: "LISTEN".to_string(),
            },
            PortProcess {
                port: 3000,
                pid: 1235,
                process_name: "node".to_string(),
                user: "user".to_string(),
                protocol: "TCP".to_string(),
                local_address: "*".to_string(),
                state: "LISTEN".to_string(),
            },
            PortProcess {
                port: 8080,
                pid: 5678,
                process_name: "java".to_string(),
                user: "user".to_string(),
                protocol: "TCP".to_string(),
                local_address: "*".to_string(),
                state: "LISTEN".to_string(),
            },
        ];

        let grouped = group_by_port(&processes);
        assert_eq!(grouped.len(), 2);
        assert_eq!(grouped.get(&3000).unwrap().len(), 2);
        assert_eq!(grouped.get(&8080).unwrap().len(), 1);
    }
}
