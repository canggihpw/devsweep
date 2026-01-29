//! Port Manager Module
//!
//! Provides functionality to list processes using network ports and kill them.
//! Useful for developers who need to free up ports for local development.

use std::collections::HashMap;
use std::process::Command;

/// Port category for grouping and display
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PortCategory {
    /// Development servers (React, Node, Python, etc.)
    DevServer,
    /// Database services (PostgreSQL, MySQL, MongoDB, Redis)
    Database,
    /// System services (SSH, DNS, etc.)
    System,
    /// Container/orchestration (Docker, Kubernetes)
    Container,
    /// Web servers (nginx, Apache)
    WebServer,
    /// Other/unknown
    Other,
}

impl PortCategory {
    pub fn name(&self) -> &'static str {
        match self {
            Self::DevServer => "Dev Servers",
            Self::Database => "Databases",
            Self::System => "System",
            Self::Container => "Containers",
            Self::WebServer => "Web Servers",
            Self::Other => "Other",
        }
    }

    pub fn icon(&self) -> &'static str {
        match self {
            Self::DevServer => "🚀",
            Self::Database => "🗄️",
            Self::System => "⚙️",
            Self::Container => "🐳",
            Self::WebServer => "🌐",
            Self::Other => "📡",
        }
    }
}

/// Safety level for killing a process
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum SafetyLevel {
    /// Safe to kill - dev servers, temporary processes
    Safe,
    /// Use caution - databases, may lose unsaved data
    Caution,
    /// Dangerous - system services, may break things
    Dangerous,
}

impl SafetyLevel {
    pub fn label(&self) -> &'static str {
        match self {
            Self::Safe => "Safe",
            Self::Caution => "Caution",
            Self::Dangerous => "Danger",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            Self::Safe => "Safe to kill, easily restartable",
            Self::Caution => "May lose unsaved data",
            Self::Dangerous => "System service, may cause issues",
        }
    }
}

/// Port information with category and safety
#[derive(Debug, Clone)]
pub struct PortInfo {
    pub port: u16,
    pub description: &'static str,
    pub category: PortCategory,
    pub safety: SafetyLevel,
}

/// Known ports with their information
const KNOWN_PORTS: &[PortInfo] = &[
    // Dev Servers - Safe
    PortInfo {
        port: 3000,
        description: "React/Node.js dev server",
        category: PortCategory::DevServer,
        safety: SafetyLevel::Safe,
    },
    PortInfo {
        port: 3001,
        description: "React dev server (alt)",
        category: PortCategory::DevServer,
        safety: SafetyLevel::Safe,
    },
    PortInfo {
        port: 4000,
        description: "Phoenix/GraphQL",
        category: PortCategory::DevServer,
        safety: SafetyLevel::Safe,
    },
    PortInfo {
        port: 4200,
        description: "Angular dev server",
        category: PortCategory::DevServer,
        safety: SafetyLevel::Safe,
    },
    PortInfo {
        port: 5000,
        description: "Flask/Python dev",
        category: PortCategory::DevServer,
        safety: SafetyLevel::Safe,
    },
    PortInfo {
        port: 5173,
        description: "Vite dev server",
        category: PortCategory::DevServer,
        safety: SafetyLevel::Safe,
    },
    PortInfo {
        port: 5174,
        description: "Vite dev server (alt)",
        category: PortCategory::DevServer,
        safety: SafetyLevel::Safe,
    },
    PortInfo {
        port: 8000,
        description: "Django/Python",
        category: PortCategory::DevServer,
        safety: SafetyLevel::Safe,
    },
    PortInfo {
        port: 8080,
        description: "HTTP alternate",
        category: PortCategory::DevServer,
        safety: SafetyLevel::Safe,
    },
    PortInfo {
        port: 8081,
        description: "HTTP alternate",
        category: PortCategory::DevServer,
        safety: SafetyLevel::Safe,
    },
    PortInfo {
        port: 8443,
        description: "HTTPS alternate",
        category: PortCategory::DevServer,
        safety: SafetyLevel::Safe,
    },
    PortInfo {
        port: 9000,
        description: "PHP-FPM/SonarQube",
        category: PortCategory::DevServer,
        safety: SafetyLevel::Safe,
    },
    PortInfo {
        port: 9090,
        description: "Prometheus",
        category: PortCategory::DevServer,
        safety: SafetyLevel::Safe,
    },
    PortInfo {
        port: 4321,
        description: "Astro dev server",
        category: PortCategory::DevServer,
        safety: SafetyLevel::Safe,
    },
    PortInfo {
        port: 3030,
        description: "Dev server",
        category: PortCategory::DevServer,
        safety: SafetyLevel::Safe,
    },
    PortInfo {
        port: 8888,
        description: "Jupyter Notebook",
        category: PortCategory::DevServer,
        safety: SafetyLevel::Safe,
    },
    PortInfo {
        port: 9229,
        description: "Node.js debugger",
        category: PortCategory::DevServer,
        safety: SafetyLevel::Safe,
    },
    PortInfo {
        port: 35729,
        description: "LiveReload",
        category: PortCategory::DevServer,
        safety: SafetyLevel::Safe,
    },
    // Databases - Caution
    PortInfo {
        port: 5432,
        description: "PostgreSQL",
        category: PortCategory::Database,
        safety: SafetyLevel::Caution,
    },
    PortInfo {
        port: 3306,
        description: "MySQL",
        category: PortCategory::Database,
        safety: SafetyLevel::Caution,
    },
    PortInfo {
        port: 27017,
        description: "MongoDB",
        category: PortCategory::Database,
        safety: SafetyLevel::Caution,
    },
    PortInfo {
        port: 6379,
        description: "Redis",
        category: PortCategory::Database,
        safety: SafetyLevel::Caution,
    },
    PortInfo {
        port: 11211,
        description: "Memcached",
        category: PortCategory::Database,
        safety: SafetyLevel::Caution,
    },
    PortInfo {
        port: 9200,
        description: "Elasticsearch",
        category: PortCategory::Database,
        safety: SafetyLevel::Caution,
    },
    PortInfo {
        port: 9300,
        description: "Elasticsearch cluster",
        category: PortCategory::Database,
        safety: SafetyLevel::Caution,
    },
    PortInfo {
        port: 26257,
        description: "CockroachDB",
        category: PortCategory::Database,
        safety: SafetyLevel::Caution,
    },
    PortInfo {
        port: 8529,
        description: "ArangoDB",
        category: PortCategory::Database,
        safety: SafetyLevel::Caution,
    },
    PortInfo {
        port: 7474,
        description: "Neo4j",
        category: PortCategory::Database,
        safety: SafetyLevel::Caution,
    },
    PortInfo {
        port: 7687,
        description: "Neo4j Bolt",
        category: PortCategory::Database,
        safety: SafetyLevel::Caution,
    },
    // Container/Orchestration - Caution
    PortInfo {
        port: 2375,
        description: "Docker API (unencrypted)",
        category: PortCategory::Container,
        safety: SafetyLevel::Caution,
    },
    PortInfo {
        port: 2376,
        description: "Docker API (TLS)",
        category: PortCategory::Container,
        safety: SafetyLevel::Caution,
    },
    PortInfo {
        port: 2377,
        description: "Docker Swarm",
        category: PortCategory::Container,
        safety: SafetyLevel::Caution,
    },
    PortInfo {
        port: 6443,
        description: "Kubernetes API",
        category: PortCategory::Container,
        safety: SafetyLevel::Caution,
    },
    PortInfo {
        port: 10250,
        description: "Kubelet API",
        category: PortCategory::Container,
        safety: SafetyLevel::Caution,
    },
    // Web Servers - Caution
    PortInfo {
        port: 80,
        description: "HTTP",
        category: PortCategory::WebServer,
        safety: SafetyLevel::Caution,
    },
    PortInfo {
        port: 443,
        description: "HTTPS",
        category: PortCategory::WebServer,
        safety: SafetyLevel::Caution,
    },
    // System Services - Dangerous
    PortInfo {
        port: 22,
        description: "SSH",
        category: PortCategory::System,
        safety: SafetyLevel::Dangerous,
    },
    PortInfo {
        port: 53,
        description: "DNS",
        category: PortCategory::System,
        safety: SafetyLevel::Dangerous,
    },
    PortInfo {
        port: 123,
        description: "NTP",
        category: PortCategory::System,
        safety: SafetyLevel::Dangerous,
    },
    PortInfo {
        port: 631,
        description: "CUPS (Printing)",
        category: PortCategory::System,
        safety: SafetyLevel::Dangerous,
    },
    PortInfo {
        port: 5353,
        description: "mDNS (Bonjour)",
        category: PortCategory::System,
        safety: SafetyLevel::Dangerous,
    },
    PortInfo {
        port: 548,
        description: "AFP (Apple Filing)",
        category: PortCategory::System,
        safety: SafetyLevel::Dangerous,
    },
    PortInfo {
        port: 88,
        description: "Kerberos",
        category: PortCategory::System,
        safety: SafetyLevel::Dangerous,
    },
    PortInfo {
        port: 464,
        description: "Kerberos (kpasswd)",
        category: PortCategory::System,
        safety: SafetyLevel::Dangerous,
    },
    PortInfo {
        port: 749,
        description: "Kerberos admin",
        category: PortCategory::System,
        safety: SafetyLevel::Dangerous,
    },
];

/// System process names that should be marked as dangerous
const SYSTEM_PROCESSES: &[&str] = &[
    "launchd",
    "kernel_task",
    "WindowServer",
    "loginwindow",
    "systemd",
    "init",
    "kextd",
    "configd",
    "mds",
    "mds_stores",
    "diskarbitrationd",
    "coreaudiod",
    "bluetoothd",
    "airportd",
    "UserEventAgent",
    "coreservicesd",
    "Finder",
    "Dock",
    "SystemUIServer",
    "Control Center",
    "ssh",
    "sshd",
];

/// Common development ports for quick access buttons
pub const COMMON_DEV_PORTS: &[u16] = &[3000, 3001, 4000, 5000, 5173, 8000, 8080, 9000];

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
    /// Category of this port/process
    pub category: PortCategory,
    /// Safety level for killing this process
    pub safety: SafetyLevel,
    /// Description of what this port is typically used for
    pub description: Option<&'static str>,
}

impl PortProcess {
    /// Create a new PortProcess with automatic categorization
    fn new(
        port: u16,
        pid: u32,
        process_name: String,
        user: String,
        protocol: String,
        local_address: String,
        state: String,
    ) -> Self {
        let (category, safety, description) = categorize_port(port, &process_name);

        Self {
            port,
            pid,
            process_name,
            user,
            protocol,
            local_address,
            state,
            category,
            safety,
            description,
        }
    }
}

/// Categorize a port based on port number and process name
fn categorize_port(
    port: u16,
    process_name: &str,
) -> (PortCategory, SafetyLevel, Option<&'static str>) {
    // First check if it's a known system process (always dangerous)
    let process_lower = process_name.to_lowercase();
    if SYSTEM_PROCESSES
        .iter()
        .any(|&p| process_lower.contains(&p.to_lowercase()))
    {
        return (PortCategory::System, SafetyLevel::Dangerous, None);
    }

    // Check known ports
    if let Some(info) = KNOWN_PORTS.iter().find(|p| p.port == port) {
        return (info.category, info.safety, Some(info.description));
    }

    // Infer from process name
    let (category, safety) = categorize_by_process_name(&process_lower);

    (category, safety, None)
}

/// Categorize based on process name patterns
fn categorize_by_process_name(process_name: &str) -> (PortCategory, SafetyLevel) {
    // Dev servers
    if process_name.contains("node")
        || process_name.contains("npm")
        || process_name.contains("python")
        || process_name.contains("ruby")
        || process_name.contains("cargo")
        || process_name.contains("go")
        || process_name.contains("java")
        || process_name.contains("deno")
        || process_name.contains("bun")
        || process_name.contains("esbuild")
        || process_name.contains("vite")
        || process_name.contains("webpack")
        || process_name.contains("next")
        || process_name.contains("nuxt")
    {
        return (PortCategory::DevServer, SafetyLevel::Safe);
    }

    // Databases
    if process_name.contains("postgres")
        || process_name.contains("mysql")
        || process_name.contains("mongo")
        || process_name.contains("redis")
        || process_name.contains("elastic")
        || process_name.contains("memcache")
    {
        return (PortCategory::Database, SafetyLevel::Caution);
    }

    // Containers
    if process_name.contains("docker")
        || process_name.contains("containerd")
        || process_name.contains("kubectl")
        || process_name.contains("kubelet")
    {
        return (PortCategory::Container, SafetyLevel::Caution);
    }

    // Web servers
    if process_name.contains("nginx")
        || process_name.contains("apache")
        || process_name.contains("httpd")
        || process_name.contains("caddy")
    {
        return (PortCategory::WebServer, SafetyLevel::Caution);
    }

    // Default to Other with Safe (unknown dev process)
    (PortCategory::Other, SafetyLevel::Safe)
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

    // Sort by safety level (dangerous first), then by port number
    processes.sort_by(|a, b| match b.safety.cmp(&a.safety) {
        std::cmp::Ordering::Equal => a.port.cmp(&b.port),
        other => other,
    });

    // Deduplicate by (port, pid) - keep first occurrence
    let mut seen = std::collections::HashSet::new();
    processes.retain(|p| seen.insert((p.port, p.pid)));

    processes
}

/// Get processes using a specific port
pub fn get_processes_on_port(port: u16) -> Vec<PortProcess> {
    let mut processes = Vec::new();

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
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() < 9 {
            continue;
        }

        let process_name = parts[0].to_string();
        let pid = parts[1].parse::<u32>().unwrap_or(0);
        let user = parts[2].to_string();

        let name_idx = parts.len() - 1;
        let name = parts[name_idx];

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

        if let Some(colon_pos) = name_str.rfind(':') {
            let addr = &name_str[..colon_pos];
            let port_str = &name_str[colon_pos + 1..];

            if let Ok(port) = port_str.parse::<u16>() {
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

                processes.push(PortProcess::new(
                    port,
                    pid,
                    process_name,
                    user,
                    protocol,
                    addr.to_string(),
                    conn_state,
                ));
            }
        }
    }

    processes
}

/// Kill a process by PID
pub fn kill_process(pid: u32) -> KillResult {
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

/// Get port info if it's a known port
pub fn get_port_info(port: u16) -> Option<&'static PortInfo> {
    KNOWN_PORTS.iter().find(|p| p.port == port)
}

/// Get common port description if available
pub fn get_port_description(port: u16) -> Option<&'static str> {
    KNOWN_PORTS
        .iter()
        .find(|p| p.port == port)
        .map(|p| p.description)
}

/// Group processes by category
pub fn group_by_category(processes: &[PortProcess]) -> HashMap<PortCategory, Vec<&PortProcess>> {
    let mut grouped: HashMap<PortCategory, Vec<&PortProcess>> = HashMap::new();
    for process in processes {
        grouped.entry(process.category).or_default().push(process);
    }
    grouped
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
    fn test_known_ports_not_empty() {
        assert!(!KNOWN_PORTS.is_empty());
    }

    #[test]
    fn test_get_port_description() {
        assert_eq!(get_port_description(3000), Some("React/Node.js dev server"));
        assert_eq!(get_port_description(5432), Some("PostgreSQL"));
        assert_eq!(get_port_description(12345), None);
    }

    #[test]
    fn test_port_categorization() {
        // Dev server port
        let (cat, safety, _) = categorize_port(3000, "node");
        assert_eq!(cat, PortCategory::DevServer);
        assert_eq!(safety, SafetyLevel::Safe);

        // Database port
        let (cat, safety, _) = categorize_port(5432, "postgres");
        assert_eq!(cat, PortCategory::Database);
        assert_eq!(safety, SafetyLevel::Caution);

        // System port
        let (cat, safety, _) = categorize_port(22, "sshd");
        assert_eq!(cat, PortCategory::System);
        assert_eq!(safety, SafetyLevel::Dangerous);
    }

    #[test]
    fn test_process_name_categorization() {
        let (cat, safety) = categorize_by_process_name("node");
        assert_eq!(cat, PortCategory::DevServer);
        assert_eq!(safety, SafetyLevel::Safe);

        let (cat, safety) = categorize_by_process_name("postgres");
        assert_eq!(cat, PortCategory::Database);
        assert_eq!(safety, SafetyLevel::Caution);

        let (cat, safety) = categorize_by_process_name("nginx");
        assert_eq!(cat, PortCategory::WebServer);
        assert_eq!(safety, SafetyLevel::Caution);
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
        assert_eq!(processes[0].category, PortCategory::DevServer);
        assert_eq!(processes[0].safety, SafetyLevel::Safe);
        assert_eq!(processes[1].port, 3001);
    }

    #[test]
    fn test_group_by_category() {
        let processes = vec![
            PortProcess::new(
                3000,
                1234,
                "node".to_string(),
                "user".to_string(),
                "TCP".to_string(),
                "*".to_string(),
                "LISTEN".to_string(),
            ),
            PortProcess::new(
                5432,
                5678,
                "postgres".to_string(),
                "user".to_string(),
                "TCP".to_string(),
                "*".to_string(),
                "LISTEN".to_string(),
            ),
        ];

        let grouped = group_by_category(&processes);
        assert!(grouped.contains_key(&PortCategory::DevServer));
        assert!(grouped.contains_key(&PortCategory::Database));
    }
}
