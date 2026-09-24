# ==============================================================================
# STAGE 1: Builder (Compiles the code using the official Rust image)
# ==============================================================================
FROM rust:1.97-slim AS builder

WORKDIR /usr/src/smpc

# Copy all your project files into the container
COPY . .

# Build the release binary
RUN cargo build --release

# ==============================================================================
# STAGE 2: Runner (A tiny, clean runtime environment)
# ==============================================================================
FROM debian:bookworm-slim

WORKDIR /app

# Install basic SSL/TLS certificates (required if your Rust app makes HTTPS requests)
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

# Copy the compiled 'smpc' binary from the builder stage
# (Note: cargo places release binaries inside target/release/)
COPY --from=builder /usr/src/smpc/target/release/smpc /app/smpc

# Command to execute your binary when the container boots
CMD ["./smpc"]
