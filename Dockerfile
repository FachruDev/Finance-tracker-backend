FROM rustlang/rust:nightly-bullseye AS builder

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY src ./src
COPY migrations ./migrations

RUN cargo build --release

FROM debian:bullseye-slim AS runtime

RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates tzdata \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

ENV RUST_LOG=info \
    APP_HOST=0.0.0.0

COPY --from=builder /app/target/release/finance-backend /app/finance-backend
COPY migrations /app/migrations

EXPOSE 8080

CMD ["./finance-backend"]

