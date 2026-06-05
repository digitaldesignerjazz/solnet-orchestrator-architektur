# Multi-stage Dockerfile for Solnet Orchestrator (Rust)
# Optimized for size and security

# === Builder Stage ===
FROM rust:1.78-slim AS builder

WORKDIR /app

# Install build dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Copy manifests first for better caching
COPY Cargo.toml Cargo.lock* ./

# Create dummy main to cache dependencies
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release && rm -rf src

# Copy real source
COPY src ./src

# Build the actual binary
RUN cargo build --release

# === Runtime Stage ===
FROM debian:bookworm-slim

WORKDIR /app

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Copy binary from builder
COPY --from=builder /app/target/release/solnet-orchestrator /app/solnet-orchestrator

# Copy default config if present
COPY config ./config

EXPOSE 8080

# Run as non-root for security
RUN useradd -m -u 1000 solnet
USER solnet

ENTRYPOINT ["./solnet-orchestrator"]
