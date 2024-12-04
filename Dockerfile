# Build stage
FROM rust:latest as builder

WORKDIR /app

COPY . .

RUN cargo build --release

FROM debian:bookworm-slim 

WORKDIR /usr/local/bin

RUN apt-get update && apt-get install -y libssl-dev pkg-config && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/api .

ENV AXUM_ADDRESS=0.0.0.0
ENV DATABASE_URL=your-database-url

EXPOSE 3000

CMD ["./api"]
