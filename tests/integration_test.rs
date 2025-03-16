//! URL Shortener API Integration Tests
//!
//! This file demonstrates how to write integration tests for the API endpoints
//! without directly importing application code.

use reqwest::{Client, StatusCode};
use serde::{Deserialize, Serialize};
use std::process::{Child, Command};
use std::time::Duration;
use testcontainers::{clients::Cli, core::WaitFor, Container, GenericImage};
use tokio::time::sleep;
use uuid::Uuid;

// Use our port finder utility
use std::net::{SocketAddr, TcpListener};

// Find an available port on localhost
fn find_available_port() -> u16 {
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

// Define the data structures we need for testing
#[derive(Debug, Serialize)]
struct CreateUrlRequest {
    url: String,
    expires_in_days: Option<i32>,
}

#[derive(Debug, Deserialize)]
struct CreateUrlResponse {
    id: Uuid,
    original_url: String,
    short_url: String,
    expires_at: Option<String>,
}

// Global docker client for tests
static DOCKER: once_cell::sync::Lazy<Cli> = once_cell::sync::Lazy::new(Cli::default);

struct TestApp {
    port: u16,
    api_key: String,
    client: Client,
    pg_port: u16,
    redis_port: u16,
    app_process: Child,
}

impl TestApp {
    async fn new() -> Self {
        // Start PostgreSQL
        let pg_container = DOCKER.run(
            GenericImage::new("postgres", "14")
                .with_env_var("POSTGRES_PASSWORD", "postgres")
                .with_env_var("POSTGRES_DB", "test_db")
                .with_exposed_port(5432)
                .with_wait_for(WaitFor::message_on_stdout(
                    "database system is ready to accept connections",
                )),
        );

        let pg_port = pg_container.get_host_port_ipv4(5432);

        // Start Redis
        let redis_container = DOCKER.run(
            GenericImage::new("redis", "6")
                .with_exposed_port(6379)
                .with_wait_for(WaitFor::message_on_stdout("Ready to accept connections")),
        );

        let redis_port = redis_container.get_host_port_ipv4(6379);

        // Find an available port for our application
        let app_port = find_available_port();

        // Set environment variables for the test app
        let db_url = format!("postgres://postgres:postgres@localhost:{}/test_db", pg_port);
        let redis_url = format!("redis://localhost:{}", redis_port);
        let api_key = format!(
            "test-api-key-{}",
            Uuid::new_v4().to_string()[..8].to_string()
        );

        // Start the application as a separate process
        let app_process = Command::new("cargo")
            .arg("run")
            .env("POSTGRES_URL", &db_url)
            .env("REDIS_URL", &redis_url)
            .env("API_KEY", &api_key)
            .env("BASE_URL", &format!("http://localhost:{}", app_port))
            .env("ENVIRONMENT", "test")
            .spawn()
            .expect("Failed to start application process");

        // Wait for the application to start
        sleep(Duration::from_secs(2)).await;

        // Create HTTP client
        let client = Client::builder()
            .timeout(Duration::from_secs(10))
            .build()
            .expect("Failed to build HTTP client");

        Self {
            port: app_port,
            api_key,
            client,
            pg_port,
            redis_port,
            app_process,
        }
    }

    fn url(&self, path: &str) -> String {
        format!("http://localhost:{}{}", self.port, path)
    }

    fn admin_client(&self) -> Client {
        Client::builder()
            .default_headers({
                let mut headers = reqwest::header::HeaderMap::new();
                headers.insert(
                    "X-API-Key",
                    reqwest::header::HeaderValue::from_str(&self.api_key).unwrap(),
                );
                headers
            })
            .build()
            .expect("Failed to build admin client")
    }
}

impl Drop for TestApp {
    fn drop(&mut self) {
        // Terminate the application process
        let _ = self.app_process.kill();

        // Note: PostgreSQL and Redis containers are managed by the static DOCKER instance
        // and will be stopped when the tests complete
    }
}

// The actual integration tests
#[tokio::test]
async fn create_url_and_redirect() {
    // Arrange
    let app = TestApp::new().await;

    // Create a URL
    let create_response = app
        .client
        .post(&app.url("/api/urls"))
        .json(&CreateUrlRequest {
            url: "https://example.com".to_string(),
            expires_in_days: None,
        })
        .send()
        .await
        .expect("Failed to create URL");

    assert_eq!(create_response.status(), StatusCode::OK);

    let created_url: CreateUrlResponse = create_response
        .json()
        .await
        .expect("Failed to parse response");

    // Extract short code from URL
    let short_path = created_url
        .short_url
        .replace(&format!("http://localhost:{}", app.port), "");

    // Try to get the redirect
    let redirect_response = app
        .client
        .get(&app.url(&short_path))
        .send()
        .await
        .expect("Failed to get redirect");

    assert_eq!(redirect_response.status(), StatusCode::FOUND);

    // Verify redirect location
    let location = redirect_response
        .headers()
        .get("location")
        .expect("No location header");

    assert_eq!(location.to_str().unwrap(), "https://example.com");
}

#[tokio::test]
async fn access_stats_with_api_key() {
    // Arrange
    let app = TestApp::new().await;

    // Act - Try to access stats without API key
    let response_without_key = app
        .client
        .get(&app.url("/api/stats"))
        .send()
        .await
        .expect("Failed to request stats");

    // Assert - Should be unauthorized
    assert_eq!(response_without_key.status(), StatusCode::UNAUTHORIZED);

    // Act - Try to access stats with API key
    let response_with_key = app
        .admin_client()
        .get(&app.url("/api/stats"))
        .send()
        .await
        .expect("Failed to request stats with API key");

    // Assert - Should be successful
    assert_eq!(response_with_key.status(), StatusCode::OK);
}
