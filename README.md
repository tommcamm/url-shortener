# URL Shortener

A modern URL shortener service built with Rust, using Axum for the web framework, PostgreSQL for storage, and Redis for caching.

## Features

- Create short URLs with optional expiration
- Track visit statistics
- Protected admin endpoints
- Redis caching for fast URL lookups
- Docker Compose setup for development

## Prerequisites

- Rust (latest stable)
- Docker and Docker Compose
- sqlx-cli (optional, for database migrations)

## Setup

1. Clone the repository
2. Copy `.env.example` to `.env` and adjust values if needed
3. Start the services:
   ```bash
   docker-compose up -d
   ```
4. Run the application:
   ```bash
   cargo run
   ```

## API Endpoints

### Public Endpoints

- `POST /api/urls`
  Create a new short URL
  ```json
  {
    "url": "https://example.com",
    "expires_in_days": 7  // optional
  }
  ```

- `GET /:short_code`
  Redirect to the original URL

### Protected Endpoints (requires X-API-Key header)

- `GET /api/stats`
  Get URL statistics

## Example Usage

Create a short URL:
```bash
curl -X POST http://localhost:3000/api/urls \
  -H "Content-Type: application/json" \
  -d '{"url": "https://example.com"}'
```

Get statistics:
```bash
curl http://localhost:3000/api/stats \
  -H "X-API-Key: your-secret-api-key-here"
```

## Development

The project uses:
- Axum for the web framework
- SQLx for type-safe database operations
- Redis for caching
- Docker Compose for development environment

### Database Migrations

Migrations are automatically run at startup. The migration file is located in `migrations/`.

### Environment Variables

- `DATABASE_URL`: PostgreSQL connection string
- `REDIS_URL`: Redis connection string
- `API_KEY`: Secret key for protected endpoints
- `BASE_URL`: Base URL for generated short links
