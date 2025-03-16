# Tommy URL Shortener - API Testing

This directory contains automated tests for the URL shortener API.

## Test Structure

- `helpers.rs` - Utilities and helper functions for tests
- `basic_test.rs` - Basic smoke tests to verify testing infrastructure
- `integration_test.rs` - Black-box API tests that run against a live server instance

## Testing Approach

Our API testing follows a black-box approach, where we:

1. Start containers for required dependencies (PostgreSQL and Redis)
2. Launch the actual application with test configuration
3. Run HTTP tests against the running instance
4. Verify API behavior through HTTP status codes and response bodies

## Running Tests

```bash
# Run all tests
cargo test

# Run specific test file
cargo test --test integration_test

# Run a specific test function
cargo test --test integration_test create_url_and_redirect
```

## Test Dependencies

The tests require:
- Docker installed and running (for testcontainers)
- Cargo/Rust toolchain
- Network connectivity to download container images if not already present

## Test Environment

Tests use their own isolated environment with:
- Containerized PostgreSQL
- Containerized Redis 
- Application running on a dynamically assigned port
- Auto-generated API key for authorized endpoint testing
