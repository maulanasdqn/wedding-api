FROM rust:latest AS builder

WORKDIR /app

COPY . .

RUN rustup target add x86_64-unknown-linux-musl \
    && apt-get update \
    && apt-get install -y musl-tools \
    && cargo build --release --target=x86_64-unknown-linux-musl

FROM alpine:latest

RUN apk add --no-cache openssl-dev pkgconf

WORKDIR /usr/local/bin

COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/api ./api

CMD ["./api"]
