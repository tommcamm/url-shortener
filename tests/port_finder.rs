//! Utility for finding available network ports for tests.
//!
//! This module provides functionality to find available TCP ports
//! that can be used for test servers to avoid port conflicts.

use std::net::{SocketAddr, TcpListener};

/// Find an available port on localhost.
///
/// This function attempts to bind to port 0, which causes the OS to assign
/// an available port. We then extract that port number and return it.
///
/// # Returns
///
/// The port number of an available TCP port.
///
/// # Panics
///
/// Panics if no available ports can be found.
pub fn find_available_port() -> u16 {
    let addr = SocketAddr::from(([127, 0, 0, 1], 0));
    let listener = TcpListener::bind(addr).expect("Failed to bind to address");
    let port = listener
        .local_addr()
        .expect("Failed to get local address")
        .port();

    // Explicitly drop the listener to free the port
    drop(listener);

    port
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_available_port() {
        let port1 = find_available_port();
        let port2 = find_available_port();

        // Different calls should return different ports
        assert_ne!(port1, port2);

        // Ports should be in a reasonable range
        assert!(port1 > 1024);
        assert!(port2 > 1024);
    }
}
