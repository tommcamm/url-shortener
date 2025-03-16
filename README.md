# Tommy URL Shortener

[![Rust](https://img.shields.io/badge/Rust-1.76+-orange)](https://www.rust-lang.org/)
[![Axum](https://img.shields.io/badge/Axum-0.8.1-blue)](https://github.com/tokio-rs/axum)
[![PostgreSQL](https://img.shields.io/badge/PostgreSQL-Latest-blue)](https://www.postgresql.org/)
[![Redis](https://img.shields.io/badge/Redis-Latest-red)](https://redis.io/)
[![OpenAPI](https://img.shields.io/badge/OpenAPI-3.0-green)](https://swagger.io/specification/)
[![License](https://img.shields.io/badge/License-MIT-yellow)](LICENSE)

A simple URL shortener service built with Rust, Axum, PostgreSQL, and Redis.

## Table of Contents

- [Features](#features)
- [Getting Started](#getting-started)
  - [Prerequisites](#prerequisites)
  - [Setup and Run](#setup-and-run)
  - [Running Locally](#running-locally-without-docker)
- [API Documentation](#api-documentation)
- [Error Handling](#error-handling)
- [Architecture](#architecture)
- [Roadmap](#roadmap)
- [License](#license)

## Features

- Create shortened URLs
- Set expiration dates for URLs
- View usage statistics 
- API key authentication for admin routes
- OpenAPI documentation with interactive Swagger UI
- Docker Compose setup for easy deployment
- Health check endpoint

## Getting Started

### Prerequisites

- Rust (latest stable version)
- Docker and Docker Compose
- Git

### Setup and Run

1. Clone the repository and navigate to the project directory:

```bash
git clone https://github.com/yourusername/tommy-shortener.git
cd tommy-shortener
```

2. Run the setup script to configure Git hooks for code quality:

```bash
./setup.sh
```

This sets up pre-commit hooks that prevent committing code with compilation errors or warnings.

3. The project comes with a `.env` file with default configuration. You can modify these values if needed:

```
DATABASE_URL=postgres://shortener:shortener123@postgres:5432/url_shortener
REDIS_URL=redis://redis:6379
API_KEY=developer_api_key_for_testing
BASE_URL=http://localhost:3000
```

4. Build and run the application using Docker Compose:

```bash
docker compose up
```

This will:
- Build the Rust application container
- Start PostgreSQL and Redis containers
- Connect all services together with proper networking
- Run database migrations automatically
- Start the web server on http://localhost:3000
- Provide health checks for all services

### Running Locally (Without Docker)

For local development, you can use:

```bash
# Start only the database containers
docker compose up -d postgres redis

# Run the application locally
cargo run
```

Note: If running locally, update the `.env` file to use localhost instead of service names:
```
DATABASE_URL=postgres://shortener:shortener123@localhost:5432/url_shortener
REDIS_URL=redis://localhost:6379
```

## API Documentation

This project includes OpenAPI documentation with Swagger UI for easy API exploration and testing:

- **Swagger UI Interface**: Available at `/swagger-ui` when the server is running
  - Example: http://localhost:3000/swagger-ui
  
- **OpenAPI JSON Schema**: Available at `/api-docs/openapi.json`
  - Example: http://localhost:3000/api-docs/openapi.json

The Swagger UI provides an interactive interface to:
- Explore all available endpoints
- View request/response schemas
- Test API endpoints directly from the browser
- Understand authentication requirements

## Error Handling

The API returns appropriate error codes and messages:

- 400 Bad Request - For invalid input
- 401 Unauthorized - For missing or invalid API key 
- 404 Not Found - For unknown short codes
- 500 Internal Server Error - For server-side issues

## Architecture

The project follows a clean architecture pattern:

```
┌───────────────┐      ┌───────────────┐      ┌───────────────┐
│               │      │               │      │               │
│  API Layer    │──────▶  Application  │──────▶   Domain      │
│  (Axum)       │      │   Services    │      │               │
│               │      │               │      │               │
└───────┬───────┘      └───────────────┘      └───────────────┘
        │                      │
┌───────▼───────┐      ┌───────▼───────┐
│               │      │               │
│  OpenAPI/     │      │ Infrastructure│
│  Swagger UI   │      │ (DB, Cache)   │
│               │      │               │
└───────────────┘      └───────────────┘
```

- `domain`: Core business logic and entities
- `application`: Services that implement business logic
- `infrastructure`: Database and cache implementations
- `api`: HTTP handlers, routes, and OpenAPI documentation

The application is containerized using Docker and orchestrated with Docker Compose:

- Rust application container (with health checks)
- PostgreSQL database container (with health checks)
- Redis cache container (with health checks)

## Roadmap

The following features are planned for future releases:

### Upcoming Features
- [ ] Rate limiting to prevent abuse
- [ ] Admin management endpoints for URL editing/removal
- [ ] API-based user authentication for personal URL management
- [ ] Custom short codes (allow users to specify their preferred short code)
- [ ] QR code generation API endpoint for shortened URLs
- [ ] Bulk URL shortening API endpoint for batch processing
- [ ] Comprehensive unit testing suite
- [ ] API integration testing framework

### In Progress
- [ ] Release of the first basic version
- [ ] Heartbeat monitoring system

Contributions are welcome! Feel free to pick up any of these items or suggest new features.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

MIT
