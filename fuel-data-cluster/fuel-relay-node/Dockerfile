# Stage 1: Cross-compilation tooling
FROM --platform=$BUILDPLATFORM tonistiigi/xx:1.2.0 AS xx
FROM --platform=$BUILDPLATFORM rust:1.81.0 AS chef

# Install cargo-chef and necessary targets
ARG TARGETPLATFORM
RUN cargo install cargo-chef && rustup target add wasm32-unknown-unknown
WORKDIR /build/

COPY --from=xx / /

# Consolidate apt installation to minimize layers
RUN apt-get update && apt-get install -y --no-install-recommends \
    lld \
    clang \
    libclang-dev \
    && xx-apt-get update \
    && xx-apt-get install -y libc6-dev g++ binutils \
    && apt-get clean && rm -rf /var/lib/apt/lists/*

# Stage 2: Dependency planning
FROM chef AS planner
ENV CARGO_NET_GIT_FETCH_WITH_CLI=true
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# Stage 3: Build application
FROM chef AS builder
ARG DEBUG_SYMBOLS=false
ENV CARGO_NET_GIT_FETCH_WITH_CLI=true
ENV CARGO_PROFILE_RELEASE_DEBUG=$DEBUG_SYMBOLS

COPY --from=planner /build/recipe.json recipe.json

# Cache dependencies
RUN --mount=type=cache,target=/usr/local/cargo/registry/index \
    --mount=type=cache,target=/usr/local/cargo/registry/cache \
    --mount=type=cache,target=/usr/local/cargo/git/db \
    --mount=type=cache,target=/build/target \
    xx-cargo chef cook --release --no-default-features -p fuel-relay-node --recipe-path recipe.json

# Copy source and build application
COPY . .
RUN --mount=type=cache,target=/usr/local/cargo/registry/index \
    --mount=type=cache,target=/usr/local/cargo/registry/cache \
    --mount=type=cache,target=/usr/local/cargo/git/db \
    --mount=type=cache,target=/build/target \
    xx-cargo build --release --no-default-features -p fuel-relay-node \
    && xx-verify ./target/$(xx-cargo --print-target-triple)/release/fuel-relay-node \
    && cp ./target/$(xx-cargo --print-target-triple)/release/fuel-relay-node /root/fuel-relay-node \
    && cp ./target/$(xx-cargo --print-target-triple)/release/fuel-relay-node.d /root/fuel-relay-node.d

# Stage 4: Runtime image
FROM ubuntu:22.04 AS run

ARG PORT=4000
ARG P2P_PORT=30333
ARG DB_PATH=/mnt/db
ENV PORT="${PORT}"

# Set working directory
WORKDIR /usr/src

# Install runtime dependencies
RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    curl \
    && apt-get autoremove -y && apt-get clean -y && rm -rf /var/lib/apt/lists/*

# Copy the binary and additional resources from the build stage
COPY --from=builder /root/fuel-relay-node .
COPY --from=builder /root/fuel-relay-node.d .

COPY /fuel-nodes/fuel-node/chain-config ./chain-config

# Expose necessary ports
EXPOSE ${PORT}
EXPOSE ${P2P_PORT}

# Set the command for the container
ENTRYPOINT ["/usr/src/fuel-relay-node"]
