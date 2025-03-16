# Tommy URL Shortener

A simple URL shortener service built with Rust, Axum, PostgreSQL, and Redis.

## Features

- Create shortened URLs
- Set expiration dates for URLs
- View usage statistics
- API key authentication for admin routes
- Docker Compose setup for easy deployment

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

## API Endpoints

### Create Short URL

```
POST /api/urls
Content-Type: application/json

{
  "url": "https://example.com/very/long/url/that/needs/shortening",
  "expires_in_days": 30  // Optional
}
```

Response:

```json
{
  "id": "uuid",
  "original_url": "https://example.com/very/long/url/that/needs/shortening",
  "short_url": "http://localhost:3000/abcd1234",
  "expires_at": "2023-04-15T00:00:00Z"  // Optional
}
```

### Redirect to Original URL

```
GET /:short_code
```

This will redirect to the original URL.

### Get Statistics (Admin only)

```
GET /api/stats
X-API-Key: developer_api_key_for_testing
```

Response:

```json
{
  "total_urls": 42,
  "total_visits": 123,
  "urls": [
    {
      "id": "uuid",
      "original_url": "https://example.com",
      "short_code": "abcd1234",
      "visits": 10,
      "created_at": "2023-03-15T00:00:00Z",
      "expires_at": "2023-04-15T00:00:00Z"
    },
    // ... more URLs ...
  ]
}
```

## Error Handling

The API returns appropriate error codes and messages:

- 400 Bad Request - For invalid input
- 401 Unauthorized - For missing or invalid API key 
- 404 Not Found - For unknown short codes
- 500 Internal Server Error - For server-side issues

## Architecture

The project follows a clean architecture pattern:

- `domain`: Core business logic and entities
- `application`: Services that implement business logic
- `infrastructure`: Database and cache implementations
- `api`: HTTP handlers and routes

The application is containerized using Docker and orchestrated with Docker Compose:

- Rust application container (with health checks)
- PostgreSQL database container (with health checks)
- Redis cache container (with health checks)

## License

MIT
