FROM rust:latest AS builder

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY server/Cargo.toml ./server/
COPY generator/Cargo.toml ./generator/

COPY ./server/src/main.rs ./server/src/main.rs
COPY ./generator/src/main.rs ./generator/src/main.rs

RUN cargo fetch

COPY . .

RUN cargo build --release


FROM debian
EXPOSE 8000

RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/server /usr/local/bin/server

ENV RUST_LOG=info

CMD ["sh", "-c", "/usr/local/bin/server"]