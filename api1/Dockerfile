FROM rust:1.72-bookworm AS builder
WORKDIR /usr/src/app
COPY ./src ./src
COPY ./Cargo.toml ./Cargo.toml
COPY ./Cargo.lock ./Cargo.lock
RUN cargo build --release

FROM debian:bookworm-slim
COPY --from=builder /usr/src/app/target/release/api1 /usr/local/bin/api1
CMD ["api1"]
