FROM rust:1.93.0-slim-bullseye AS builder

WORKDIR /app

COPY Cargo.toml Cargo.lock ./

RUN mkdir src && \
    echo "fn main() {println!(\"if you see this, the build broke\")}" > src/main.rs && \
    crate_name=$(grep 'name =' Cargo.toml | head -n1 | cut -d '"' -f 2) && \
    cargo build --release && \
    rm -rf src

COPY . .

RUN touch src/main.rs

RUN cargo build --release

FROM debian:bullseye-slim

WORKDIR /app

RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/polytech /app/polytech

COPY --from=builder /app/migrations /app/migrations

EXPOSE 3000

CMD ["./polytech"]
