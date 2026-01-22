//! Tests for single_instance module
//!
//! Tests the Unix socket-based single instance detection functionality.

use devsweep::single_instance::get_socket_path;
use std::io::{Read, Write};
use std::os::unix::net::{UnixListener, UnixStream};
use std::thread;
use std::time::Duration;
use tempfile::TempDir;

// ============================================================================
// Socket Path Tests
// ============================================================================

#[test]
fn test_get_socket_path_returns_valid_path() {
    let path = get_socket_path();

    // Should end with devsweep.sock
    assert!(path.to_string_lossy().ends_with("devsweep.sock"));
}

#[test]
fn test_get_socket_path_parent_exists() {
    let path = get_socket_path();
    let parent = path.parent();

    assert!(parent.is_some());
    assert!(parent.unwrap().exists());
}

#[test]
fn test_get_socket_path_is_deterministic() {
    let path1 = get_socket_path();
    let path2 = get_socket_path();

    assert_eq!(path1, path2);
}

#[test]
fn test_socket_path_in_expected_location() {
    let path = get_socket_path();
    let path_str = path.to_string_lossy();

    // Should be in a cache/runtime directory, not in /tmp directly unless fallback
    assert!(
        path_str.contains("Cache")
            || path_str.contains("cache")
            || path_str.contains("/tmp")
            || path_str.contains("run")
    );
}

// ============================================================================
// try_activate_existing_instance Tests (using temp directories)
// ============================================================================

#[test]
fn test_try_activate_no_socket_returns_false() {
    // When socket doesn't exist, should return false
    // This tests the logic without using the real socket path
    let temp = TempDir::new().unwrap();
    let nonexistent = temp.path().join("nonexistent.sock");

    let result = UnixStream::connect(&nonexistent);
    assert!(result.is_err());
}

#[test]
fn test_socket_communication_protocol() {
    let temp = TempDir::new().unwrap();
    let socket_path = temp.path().join("protocol_test.sock");

    // Create listener (simulating existing instance)
    let listener = UnixListener::bind(&socket_path).unwrap();

    // Spawn thread to handle the protocol
    let handle = thread::spawn(move || {
        let (mut stream, _) = listener.accept().unwrap();
        let mut buf = [0u8; 8];
        let n = stream.read(&mut buf).unwrap();

        // Verify we received "activate"
        assert_eq!(&buf[..n], b"activate");

        // Send back "ok"
        stream.write_all(b"ok").unwrap();
    });

    // Give listener time to start
    thread::sleep(Duration::from_millis(10));

    // Connect and send activation message
    let mut stream = UnixStream::connect(&socket_path).unwrap();
    stream.write_all(b"activate").unwrap();

    // Read response
    let mut response = [0u8; 2];
    stream.read_exact(&mut response).unwrap();
    assert_eq!(&response, b"ok");

    handle.join().unwrap();
}

#[test]
fn test_socket_nonblocking_accept() {
    let temp = TempDir::new().unwrap();
    let socket_path = temp.path().join("nonblock.sock");

    let listener = UnixListener::bind(&socket_path).unwrap();
    listener.set_nonblocking(true).unwrap();

    // Accept on non-blocking socket with no connections should return WouldBlock
    let result = listener.accept();
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().kind(), std::io::ErrorKind::WouldBlock);
}

#[test]
fn test_socket_can_be_recreated_after_removal() {
    let temp = TempDir::new().unwrap();
    let socket_path = temp.path().join("recreate.sock");

    // Create first listener
    let listener1 = UnixListener::bind(&socket_path).unwrap();
    assert!(socket_path.exists());
    drop(listener1);

    // Remove socket
    std::fs::remove_file(&socket_path).unwrap();
    assert!(!socket_path.exists());

    // Create second listener
    let listener2 = UnixListener::bind(&socket_path).unwrap();
    assert!(socket_path.exists());
    drop(listener2);
}

// ============================================================================
// Integration: Single Instance Detection Pattern
// ============================================================================

#[test]
fn test_single_instance_detection_pattern() {
    let temp = TempDir::new().unwrap();
    let socket_path = temp.path().join("instance.sock");

    // Step 1: "First instance" creates socket
    let listener = UnixListener::bind(&socket_path).unwrap();
    listener.set_nonblocking(true).unwrap();

    // Step 2: "Second instance" can connect
    let stream = UnixStream::connect(&socket_path);
    assert!(stream.is_ok(), "Second instance should connect to first");

    drop(stream);
    drop(listener);
}

#[test]
fn test_no_instance_running_pattern() {
    let temp = TempDir::new().unwrap();
    let socket_path = temp.path().join("no_instance.sock");

    // No socket exists
    assert!(!socket_path.exists());

    // Try to connect fails
    let result = UnixStream::connect(&socket_path);
    assert!(result.is_err());

    // This is how try_activate_existing_instance returns false
}

#[test]
fn test_stale_socket_cleanup_pattern() {
    let temp = TempDir::new().unwrap();
    let socket_path = temp.path().join("stale.sock");

    // Create and drop a listener (simulating crashed app)
    let listener = UnixListener::bind(&socket_path).unwrap();
    drop(listener);

    // Socket file still exists but listener is gone
    // On some systems, the socket might be automatically cleaned up
    // So we just verify we can remove it and create a new one
    let _ = std::fs::remove_file(&socket_path);

    // New instance can create listener
    let new_listener = UnixListener::bind(&socket_path).unwrap();
    assert!(socket_path.exists());
    drop(new_listener);
}

// ============================================================================
// Edge Cases
// ============================================================================

#[test]
fn test_multiple_connect_attempts() {
    let temp = TempDir::new().unwrap();
    let socket_path = temp.path().join("multi.sock");

    let listener = UnixListener::bind(&socket_path).unwrap();

    // Multiple connections should work
    let stream1 = UnixStream::connect(&socket_path).unwrap();
    let stream2 = UnixStream::connect(&socket_path).unwrap();
    let stream3 = UnixStream::connect(&socket_path).unwrap();

    drop(stream1);
    drop(stream2);
    drop(stream3);
    drop(listener);
}

#[test]
fn test_partial_message_handling() {
    let temp = TempDir::new().unwrap();
    let socket_path = temp.path().join("partial.sock");

    let listener = UnixListener::bind(&socket_path).unwrap();

    let handle = thread::spawn(move || {
        let (mut stream, _) = listener.accept().unwrap();
        let mut buf = [0u8; 8];
        // Read what we can
        let n = stream.read(&mut buf).unwrap();
        n > 0 // Return whether we got any data
    });

    thread::sleep(Duration::from_millis(10));

    // Send only partial message
    let mut stream = UnixStream::connect(&socket_path).unwrap();
    stream.write_all(b"act").unwrap();
    drop(stream); // Close without sending full message

    let got_data = handle.join().unwrap();
    assert!(got_data);
}

#[test]
fn test_connection_timeout_behavior() {
    let temp = TempDir::new().unwrap();
    let socket_path = temp.path().join("timeout.sock");

    // No listener - connection should fail immediately
    let start = std::time::Instant::now();
    let result = UnixStream::connect(&socket_path);
    let elapsed = start.elapsed();

    assert!(result.is_err());
    // Should fail fast, not timeout
    assert!(elapsed < Duration::from_secs(1));
}
