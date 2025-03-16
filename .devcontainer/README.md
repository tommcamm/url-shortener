# Dev Container Setup for Rust URL Shortener

This directory contains configuration files for VS Code Dev Containers, allowing you to develop and test the application directly from VS Code without manually running `docker-compose up`.

## What's Included

- **Rust 1.76** development environment
- **PostgreSQL 16** database
- **Redis 7** cache server
- Various development tools (PostgreSQL client, Redis tools, etc.)
- VS Code extensions for Rust development

## How to Use

1. Install the "Dev Containers" extension in VS Code
2. Open the project in VS Code
3. Click the green button in the bottom-left corner of VS Code (or press F1 and select "Dev Containers: Reopen in Container")
4. Wait for the container to build and start (this may take a few minutes the first time)
5. Once inside the container, you can:
   - Run `cargo run` to start the application
   - Run `cargo test` to run tests
   - Access PostgreSQL on port 5432
   - Access Redis on port 6379

## Environment Configuration

The dev container is pre-configured with the following environment variables:

- `DATABASE_URL=postgres://shortener:shortener123@postgres:5432/url_shortener`
- `REDIS_URL=redis://redis:6379`
- `API_KEY=developer_api_key_for_testing`
- `BASE_URL=http://localhost:3000`

## Persistence

- PostgreSQL and Redis data are persisted in Docker volumes
- Your code changes are immediately available in the container
