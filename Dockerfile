# Build stage
FROM rust:1.76 as builder
WORKDIR /app
COPY . .
RUN cargo build --release

# Runtime stage
FROM debian:bullseye-slim
WORKDIR /app
COPY --from=builder /app/target/release/tommy-shortener /app/
COPY migrations /app/migrations

# Install necessary dependencies for healthcheck
RUN apt-get update && apt-get install -y curl && rm -rf /var/lib/apt/lists/*

# Add healthcheck
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
  CMD curl -f http://localhost:3000/health || exit 1

EXPOSE 3000
CMD ["./tommy-shortener"]
