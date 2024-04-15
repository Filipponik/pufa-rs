FROM rust:alpine3.19 as build
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

FROM alpine:3.19
WORKDIR app
COPY --from=build /app/target/release/pufa-rs .

ENTRYPOINT ["./pufa-rs"]