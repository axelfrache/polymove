FROM rust:1.93.0-slim-bullseye AS builder

WORKDIR /app

COPY services/erasmumu/Cargo.toml services/erasmumu/Cargo.lock ./services/erasmumu/

RUN mkdir -p services/erasmumu/src && \
    echo "fn main() {println!(\"if you see this, the build broke\")}" > services/erasmumu/src/main.rs

WORKDIR /app/services/erasmumu
RUN cargo build --release
RUN rm -rf src

COPY services/erasmumu/src ./src

RUN touch src/main.rs
RUN cargo build --release

FROM debian:bullseye-slim

WORKDIR /app

RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/services/erasmumu/target/release/erasmumu /app/erasmumu

EXPOSE 3001

CMD ["./erasmumu"]
