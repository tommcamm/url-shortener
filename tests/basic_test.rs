//! Basic integration tests for the URL Shortener service
//!
//! This file demonstrates the pattern for integration testing with containers.

use crate::helpers::test_api_key;
use reqwest::StatusCode;
use std::sync::Arc;
use testcontainers::{clients::Cli, core::WaitFor, GenericImage};
use tokio::sync::Mutex;
use uuid::Uuid;

mod helpers;

/// Global docker client for tests
static DOCKER_CLI: once_cell::sync::Lazy<Arc<Mutex<Cli>>> =
    once_cell::sync::Lazy::new(|| Arc::new(Mutex::new(Cli::default())));

/// A basic smoke test to verify our testing infrastructure works
#[tokio::test]
async fn basic_container_test() {
    // Start a simple postgres container
    let postgres = GenericImage::new("postgres", "14")
        .with_env_var("POSTGRES_PASSWORD", "postgres")
        .with_exposed_port(5432)
        .with_wait_for(WaitFor::message_on_stdout(
            "database system is ready to accept connections",
        ));

    // Get a lock on the docker client
    let docker = DOCKER_CLI.lock().await;

    // Start the container
    let container = docker.run(postgres);
    let port = container.get_host_port_ipv4(5432);

    // Verify we got a port mapped
    assert!(port > 0);

    // Construct a connection string (we won't actually connect, just verify formatting)
    let conn_string = format!("postgres://postgres:postgres@localhost:{}/postgres", port);
    assert!(conn_string.contains(&port.to_string()));

    println!("Successfully started PostgreSQL container on port {}", port);

    // Container will be automatically dropped and stopped when it goes out of scope
}

/// Test helper functions
#[test]
fn test_helpers() {
    // Test API key generation
    let key = test_api_key();
    assert!(key.starts_with("test-api-key-"));
    assert_eq!(key.len(), "test-api-key-".len() + 8);

    // Ensure we generate unique keys
    let key2 = test_api_key();
    assert_ne!(key, key2);
}

/// Test status code handling
#[test]
fn test_http_status_codes() {
    // We'll use these status codes in our API tests
    assert_eq!(StatusCode::OK.as_u16(), 200);
    assert_eq!(StatusCode::FOUND.as_u16(), 302); // For redirects
    assert_eq!(StatusCode::BAD_REQUEST.as_u16(), 400);
    assert_eq!(StatusCode::UNAUTHORIZED.as_u16(), 401);
    assert_eq!(StatusCode::NOT_FOUND.as_u16(), 404);

    // Just checking UUID generation for our fixtures
    let id = Uuid::new_v4();
    assert!(!id.to_string().is_empty());
}
