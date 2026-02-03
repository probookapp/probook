# Build stage
FROM rust:1.75-slim-bookworm AS builder

WORKDIR /app

# Install system dependencies for building
RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*

# Copy workspace files
COPY Cargo.toml ./
COPY crates/ crates/

# Copy src-tauri Cargo.toml for workspace resolution (but exclude Tauri source)
COPY src-tauri/Cargo.toml src-tauri/Cargo.toml
RUN mkdir -p src-tauri/src && echo "fn main() {}" > src-tauri/src/main.rs

# Build only the API server
RUN cargo build --release -p probook-api

# Runtime stage
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y ca-certificates libssl3 && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/probook-api /usr/local/bin/probook-api

# Copy migrations
COPY crates/probook-core/migrations/ /app/migrations/

EXPOSE 3001

CMD ["probook-api"]
