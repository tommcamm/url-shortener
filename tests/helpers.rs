//! Helper functions and types for tests

use std::path::PathBuf;

/// Get the project root directory
pub fn project_root() -> PathBuf {
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not set");
    PathBuf::from(manifest_dir)
}

/// Get path to migrations directory
pub fn migrations_path() -> PathBuf {
    project_root().join("migrations")
}

/// Create a test database URL with a unique name for parallel test execution
pub fn test_db_url(db_name: &str) -> String {
    format!("postgres://postgres:postgres@localhost:5432/{}", db_name)
}

/// Create a unique test URL for Redis
pub fn test_redis_url() -> String {
    "redis://localhost:6379/1".to_string()
}

/// Generate a random API key for tests
pub fn test_api_key() -> String {
    "test-api-key-".to_string() + &uuid::Uuid::new_v4().to_string()[..8]
}
