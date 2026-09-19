# Test build stage is independent from production.
FROM rust:1.89-bookworm AS test
WORKDIR /app
COPY Cargo.toml ./
COPY src ./src
RUN cargo test

# Production build stage: dev-dependencies are not compiled here.
FROM rust:1.89-bookworm AS production-builder

WORKDIR /app

COPY Cargo.toml ./
COPY src ./src

RUN cargo build --release

# Production stage
FROM debian:bookworm-slim AS production

RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    curl \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy only the production binary from builder
COPY --from=production-builder /app/target/release/backend-api .

EXPOSE 8080

CMD ["./backend-api"]