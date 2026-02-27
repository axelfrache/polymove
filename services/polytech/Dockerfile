FROM rust:1.93.0-slim-bullseye AS builder

WORKDIR /app

RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl-dev \
    pkg-config \
    protobuf-compiler \
    && rm -rf /var/lib/apt/lists/*

COPY proto proto
COPY services/polytech/Cargo.toml services/polytech/Cargo.lock ./services/polytech/

RUN mkdir -p services/polytech/src && \
    echo "fn main() {println!(\"if you see this, the build broke\")}" > services/polytech/src/main.rs

WORKDIR /app/services/polytech
RUN cargo build --release
RUN rm -rf src

COPY services/polytech/src ./src
COPY services/polytech/build.rs ./
COPY services/polytech/migrations ./migrations

RUN touch src/main.rs
RUN cargo build --release

FROM debian:bullseye-slim

WORKDIR /app

RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/services/polytech/target/release/polytech /app/polytech
COPY --from=builder /app/services/polytech/migrations /app/migrations

EXPOSE 3000

CMD ["./polytech"]
