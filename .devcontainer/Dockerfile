FROM rust:1.76

# Install additional development tools
RUN apt-get update && apt-get install -y \
    curl \
    postgresql-client \
    redis-tools \
    && rm -rf /var/lib/apt/lists/*

# Install Rust tooling
RUN rustup component add rustfmt clippy rust-analyzer

WORKDIR /workspace

# Pre-build dependencies
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs && cargo build && rm -rf src
