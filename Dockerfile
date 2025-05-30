# --- Builder ---
FROM rust:latest as builder

WORKDIR /

# Install sqlx-cli (optional if you want migrations in Docker)
RUN cargo install sqlx-cli --no-default-features --features postgres

# Copy workspace files
COPY . .

# Set environment for offline mode or sqlx prepare (if using)
ENV SQLX_OFFLINE=true

# Build only the API crate
RUN cargo build --release -p api

# --- Runtime ---
FROM debian:bookworm-slim

WORKDIR /

RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

COPY --from=builder /target/release/api .

COPY api/.env .env
COPY /migration ./migration

EXPOSE 3000

CMD ["./api"]
