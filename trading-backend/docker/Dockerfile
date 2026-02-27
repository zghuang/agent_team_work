FROM rust:1.75-alpine AS builder

WORKDIR /app

COPY Cargo.toml ./
COPY src ./src

RUN apk add --no-cache musl-dev && \
    cargo build --release

FROM alpine:latest

WORKDIR /app

COPY --from=builder /app/target/release/trading-backend /app/backend

EXPOSE 8080

CMD ["/app/backend"]
