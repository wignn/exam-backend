FROM rust:latest AS builder

WORKDIR /usr/src/app
COPY . .

# Build dependencies first (faster cache)
RUN cargo build --release

# Stage 2: Runtime
FROM debian:bookworm-slim

# Install libssl for SQLx with Postgres
RUN apt-get update && apt-get install -y libpq-dev ca-certificates && rm -rf /var/lib/apt/lists/*

# Copy the built binary
COPY --from=builder /usr/src/app/target/release/backend /usr/local/bin/app

# Set environment variables (if any)
ENV RUST_LOG=info
ENV ROCKET_ENV=production

# Run the binary
CMD ["app"]
