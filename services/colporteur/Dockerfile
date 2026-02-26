FROM rust:1.93.0-slim-bullseye AS builder

WORKDIR /app

RUN apt-get update && apt-get install -y protobuf-compiler && rm -rf /var/lib/apt/lists/*

COPY proto proto
COPY services/colporteur/Cargo.toml services/colporteur/Cargo.lock ./services/colporteur/

RUN mkdir -p services/colporteur/src && echo "fn main() {}" > services/colporteur/src/main.rs

WORKDIR /app/services/colporteur
RUN cargo build --release
RUN rm -rf src

COPY services/colporteur/src ./src
COPY services/colporteur/build.rs ./

RUN touch src/main.rs && cargo build --release

FROM debian:bullseye-slim
WORKDIR /app
RUN apt-get update && apt-get install -y ca-certificates libssl-dev && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/services/colporteur/target/release/colporteur /app/colporteur
CMD ["./colporteur"]
