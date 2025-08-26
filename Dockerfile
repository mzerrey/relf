# Build stage
FROM rust:latest as builder

WORKDIR /app

# Install Trunk
RUN cargo install trunk

# Add wasm32 target
RUN rustup target add wasm32-unknown-unknown

# Copy source files
COPY Cargo.toml Cargo.lock ./
COPY src ./src
COPY static ./static
COPY index.html ./
COPY Trunk.toml ./

# Build the WASM application with Trunk
RUN trunk build --release

# Build the server
RUN cargo build --release --bin server --features server

# Runtime stage
FROM debian:bookworm-slim

WORKDIR /app

# Install necessary runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Copy the built server binary
COPY --from=builder /app/target/release/server /app/server

# Copy the Trunk build output
COPY --from=builder /app/dist /app/dist

# Copy only robots.txt
COPY --from=builder /app/static/robots.txt /app/static/robots.txt

# Expose port
EXPOSE 5000

# Run the server
CMD ["./server", "0.0.0.0", "5000"]