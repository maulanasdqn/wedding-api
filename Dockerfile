# Build stage
FROM rust:latest as builder

WORKDIR /app

COPY . .

RUN cargo build --release

FROM debian:bookworm-slim 

WORKDIR /usr/local/bin

RUN apt-get update && apt-get install -y libssl-dev pkg-config && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/wedding-api .

ENV ROCKET_ADDRESS=0.0.0.0
ENV DATABASE_URL=your-database-url
ENV S3_ACCESS_KEY=your-access-key
ENV S3_SECRET_KEY=your-secret-key
ENV S3_REGION=your-region
ENV S3_BUCKET_NAME=your-bucket
ENV S3_ENDPOINT=your-endpoint

EXPOSE 8000

CMD ["./wedding-api"]
