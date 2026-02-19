# ============================================================================
# BUILDER STAGE - Compile aplikasi Rust
# ============================================================================
FROM rust:1.93-slim AS builder

WORKDIR /app

# Install dependencies yang diperlukan untuk build
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    libpq-dev \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Copy manifest files
COPY Cargo.toml Cargo.lock ./

# Create dummy main.rs untuk cache dependencies
RUN mkdir src && echo "fn main() {}" > src/main.rs && \
    cargo build --release && \
    rm -rf src

# Copy source code
COPY src ./src

# Copy migrations dan config
# COPY migrations ./migrations
# COPY diesel.toml ./diesel.toml

RUN cargo install diesel_cli --no-default-features --features "postgres"

# RUN diesel setup && \
#     diesel migration generate qyl_migrations && \
#     diesel migration run 

# Build aplikasi dengan release optimizations
RUN cargo build --release

# ============================================================================
# RUNTIME STAGE - Minimal image untuk production
# ============================================================================
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y --no-install-recommends \
    libpq5 \
    ca-certificates \
    curl \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy binary dari builder stage
COPY --from=builder /app/target/release/quoteyourlife-be ./quoteyourlife-be

# Set logging level
ENV RUST_LOG=info

# Create non-root user untuk security
RUN useradd -m -u 1001 appuser && \
    chown -R appuser:appuser /app

USER appuser

EXPOSE 8080

# Health check untuk Docker daemon
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
    CMD curl -f http://localhost:8080/ || exit 1

CMD ["./quoteyourlife-be"]