FROM rust:1.93.0-slim-bullseye AS builder

WORKDIR /app

COPY services/seeder/Cargo.toml ./services/seeder/

RUN mkdir -p services/seeder/src && echo "fn main() {}" > services/seeder/src/main.rs

WORKDIR /app/services/seeder
RUN cargo build --release
RUN rm -rf src

COPY services/seeder/src ./src
COPY services/seeder/data ./data
RUN touch src/main.rs && cargo build --release

FROM debian:bullseye-slim
WORKDIR /app
RUN apt-get update && apt-get install -y ca-certificates libssl-dev && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/services/seeder/target/release/seeder /app/seeder
CMD ["./seeder"]
