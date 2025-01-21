# Stage 1: Chef Setup
FROM rust:1.81 AS chef
RUN cargo install cargo-chef
WORKDIR /app

# Copy workspace Cargo.toml and member manifests
COPY Cargo.toml Cargo.lock ./
COPY fuel-data-crates/fuel-data-nats/Cargo.toml fuel-data-crates/fuel-data-nats/
COPY fuel-data-crates/fuel-data-types/Cargo.toml fuel-data-crates/fuel-data-types/
COPY fuel-data-edges/fuel-data-edge/Cargo.toml fuel-data-edges/fuel-data-edge/
COPY fuel-data-edges/fuel-data-grpc-edge/Cargo.toml fuel-data-edges/fuel-data-grpc-edge/
COPY fuel-data-protos/Cargo.toml fuel-data-protos/
COPY fuel-data-cluster/Cargo.toml fuel-data-cluster/
COPY fuel-nodes/fuel-node-types/Cargo.toml fuel-nodes/fuel-node-types/
COPY fuel-data-sdks/fuel-data-rs/Cargo.toml fuel-data-sdks/fuel-data-rs/

# Generate the dependency recipe
RUN cargo chef prepare --recipe-path recipe.json

# Stage 2: Build Dependencies
FROM chef AS builder
COPY --from=chef /app/recipe.json ./recipe.json
RUN cargo chef cook --release --no-default-features --recipe-path recipe.json -p fuel-data-grpc-edge

# Copy the full source code
COPY . .

WORKDIR /app/fuel-data-edges/fuel-data-grpc-edge

# Build the binary and specify the target directory
RUN cargo build --release --no-default-features -p fuel-data-grpc-edge --target-dir /app/target

# Debug: Verify binary existence
RUN ls -l /app/target/release

# Stage 3: Runtime
FROM ubuntu:22.04
WORKDIR /usr/src

# Install runtime dependencies
RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates curl \
    && rm -rf /var/lib/apt/lists/*

# Set environment variables for NATS
ENV NATS_URL=nats://localhost:4222

# Copy the built binary
COPY --from=builder /app/target/release/fuel-data-grpc-edge /usr/src/fuel-data-grpc-edge

# Expose the gRPC server port
EXPOSE 50051

# Set the entry point
ENTRYPOINT ["/usr/src/fuel-data-grpc-edge"]
