FROM rust:1.93.0-slim-bullseye AS builder

WORKDIR /app

COPY services/laposte/Cargo.toml services/laposte/Cargo.lock ./services/laposte/

RUN mkdir -p services/laposte/src && \
    echo "fn main() {println!(\"if you see this, the build broke\")}" > services/laposte/src/main.rs && \
    echo "pub fn placeholder() {}" > services/laposte/src/lib.rs

WORKDIR /app/services/laposte
RUN cargo build --release
RUN rm -rf src

COPY services/laposte/src ./src

RUN touch src/main.rs src/lib.rs
RUN cargo build --release

FROM debian:bullseye-slim

WORKDIR /app

RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/services/laposte/target/release/laposte /app/laposte

EXPOSE 3002

CMD ["./laposte"]
