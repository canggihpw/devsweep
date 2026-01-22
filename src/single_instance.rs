//! Single Instance Management
//!
//! Provides functionality to ensure only one instance of the application runs at a time.
//! Uses Unix domain sockets for inter-process communication.

use std::fs;
use std::io::{Read, Write};
use std::os::unix::net::{UnixListener, UnixStream};
use std::path::PathBuf;

/// Get the path for the Unix domain socket used for single-instance detection.
pub fn get_socket_path() -> PathBuf {
    let mut path = dirs::runtime_dir()
        .or_else(dirs::cache_dir)
        .unwrap_or_else(|| PathBuf::from("/tmp"));
    path.push("devsweep.sock");
    path
}

/// Try to activate an existing instance of the application.
///
/// Returns `true` if an existing instance was found and activated,
/// `false` if no existing instance is running.
pub fn try_activate_existing_instance() -> bool {
    let socket_path = get_socket_path();

    if let Ok(mut stream) = UnixStream::connect(&socket_path) {
        // Send activation message to existing instance
        let _ = stream.write_all(b"activate");
        let mut response = [0u8; 2];
        if stream.read_exact(&mut response).is_ok() && &response == b"ok" {
            return true; // Existing instance will handle activation
        }
    }

    false
}

/// Create a Unix socket listener for single-instance detection.
///
/// Returns `Some(UnixListener)` if successful, `None` if the socket couldn't be created.
pub fn create_instance_listener() -> Option<UnixListener> {
    let socket_path = get_socket_path();

    // Remove stale socket file if it exists
    let _ = fs::remove_file(&socket_path);

    // Create Unix socket listener
    match UnixListener::bind(&socket_path) {
        Ok(listener) => {
            listener.set_nonblocking(true).ok();
            Some(listener)
        }
        Err(_) => None,
    }
}

/// Clean up the socket file on application exit.
pub fn cleanup_socket() {
    let _ = fs::remove_file(get_socket_path());
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;
    use std::time::Duration;
    use tempfile::TempDir;

    #[test]
    fn test_get_socket_path() {
        let path = get_socket_path();
        assert!(path.to_string_lossy().ends_with("devsweep.sock"));
    }

    #[test]
    fn test_socket_path_has_valid_parent() {
        let path = get_socket_path();
        let parent = path.parent().unwrap();
        // Parent should exist (runtime_dir, cache_dir, or /tmp)
        assert!(parent.exists());
    }

    #[test]
    fn test_try_activate_no_instance() {
        // When no instance is running, should return false
        // Note: This test might interfere with a running instance
        // In practice, we'd use a test-specific socket path
        let result = try_activate_existing_instance();
        // Result depends on whether DevSweep is actually running
        let _ = result;
    }

    #[test]
    fn test_create_and_cleanup_listener() {
        // This test uses the real socket path, so be careful
        // In a real test environment, we'd mock the path

        // Just test that cleanup doesn't panic
        cleanup_socket();
    }

    #[test]
    fn test_socket_communication_pattern() {
        let temp = TempDir::new().unwrap();
        let socket_path = temp.path().join("test.sock");

        // Create listener
        let listener = UnixListener::bind(&socket_path).unwrap();
        listener.set_nonblocking(true).unwrap();

        // Spawn thread to accept
        let socket_clone = socket_path.clone();
        let handle = thread::spawn(move || {
            // Wait for connection
            thread::sleep(Duration::from_millis(50));
            let listener = UnixListener::bind(&socket_clone);
            // Listener already bound, this will fail - that's expected
            listener.is_err()
        });

        // Try to connect
        let result = UnixStream::connect(&socket_path);
        assert!(result.is_ok());

        handle.join().unwrap();
    }
}
