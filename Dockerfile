# Build stage
FROM rust:latest AS builder

# Install dependencies yang diperlukan untuk build
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    libpq-dev \
    && rm -rf /var/lib/apt/lists/*

# Set working directory
WORKDIR /app

# Copy Cargo files terlebih dahulu untuk cache dependencies
COPY Cargo.toml ./
# Copy Cargo.lock jika ada
COPY Cargo.loc[k] ./

# Create dummy main.rs untuk build dependencies
RUN mkdir src && echo "fn main() {}" > src/main.rs

# Build dependencies (akan di-cache jika Cargo.toml tidak berubah)
RUN cargo build --release && rm src/main.rs

# Copy source code
COPY src ./src

# Build aplikasi
RUN touch src/main.rs && cargo build --release

# Runtime stage
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    libpq5 \
    && rm -rf /var/lib/apt/lists/* \
    && apt-get clean

# Create non-root user
RUN useradd -r -s /bin/false -m -d /app appuser

# Set working directory
WORKDIR /app

# Copy binary dari build stage
COPY --from=builder /app/target/release/backend /app/backend

# Change ownership ke appuser
RUN chown -R appuser:appuser /app

# Switch ke non-root user
USER appuser

# Expose port
EXPOSE 3000

# Health check
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
    CMD curl -f http://localhost:3000/health || exit 1

# Run aplikasi
CMD ["./backend"]