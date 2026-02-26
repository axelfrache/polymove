FROM rust:1.93.0-slim-bullseye AS builder

WORKDIR /app

# Install protobuf compiler
RUN apt-get update && apt-get install -y protobuf-compiler && rm -rf /var/lib/apt/lists/*

# Copy proto directory (from root context)
COPY proto proto

# Copy service manifest
COPY services/mi8/Cargo.toml services/mi8/Cargo.lock ./services/mi8/

# Create empty project for caching dependencies
RUN mkdir -p services/mi8/src && \
    echo "fn main() {println!(\"if you see this, the build broke\")}" > services/mi8/src/main.rs

# Build dependencies
WORKDIR /app/services/mi8
RUN cargo build --release
RUN rm -rf src

# Copy source code
COPY services/mi8/src ./src
COPY services/mi8/build.rs ./

# Build application
RUN touch src/main.rs
RUN cargo build --release

FROM debian:bullseye-slim
WORKDIR /app
RUN apt-get update && apt-get install -y ca-certificates libssl-dev && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/services/mi8/target/release/mi8 /app/mi8
EXPOSE 50051
CMD ["./mi8"]
