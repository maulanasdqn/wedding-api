FROM rust:latest AS builder

WORKDIR /app

COPY . .

RUN cargo build --release

FROM alpine:latest

WORKDIR /usr/local/bin

RUN apk add --no-cache openssl-dev pkgconf

COPY --from=builder /app/target/release/api .

CMD ["./api"]
