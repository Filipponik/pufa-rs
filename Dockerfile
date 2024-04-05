FROM rust:latest as build
LABEL authors="max"

WORKDIR src

COPY src/ ./src
COPY Cargo.lock Cargo.toml ./

RUN cargo build --release

ENTRYPOINT ["./target/release/pufa-rs"]
