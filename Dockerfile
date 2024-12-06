FROM rust:latest AS builder

WORKDIR /app

COPY . .

RUN cargo build --release

FROM alpine:latest

WORKDIR /usr/local/bin

RUN apk add --no-cache openssl-dev pkgconf

COPY --from=builder /app/target/release/api .

ENV AXUM_ADDRESS=0.0.0.0
ENV DATABASE_URL=your-database-url
ENV CORS_ORIGIN=your-cors-origin

EXPOSE 3000

CMD ["./api"]
