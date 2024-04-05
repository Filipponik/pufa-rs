FROM rust:alpine as build
LABEL authors="max"

WORKDIR app

RUN apk add \
    musl-dev \
    openssl \
    pkgconfig \
    libressl-dev \
    upx

COPY src/ ./src
COPY Cargo.lock Cargo.toml ./

RUN cargo build --release \
    && upx --best --lzma target/release/pufa-rs

FROM alpine:latest
WORKDIR app
COPY --from=build /app/target/release/pufa-rs .

ENTRYPOINT ["./pufa-rs"]