# Build stage
FROM rust:1.88-slim as builder

# Install required dependencies for building
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Set working directory
WORKDIR /app

# Copy manifests
COPY Cargo.toml Cargo.lock ./

# Copy source code
COPY src ./src
COPY migrations ./migrations
COPY static ./static

# Build the application
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Create app user
RUN useradd -r -s /bin/false -m -d /app appuser

# Set working directory
WORKDIR /app

# Copy the binary from builder stage
COPY --from=builder /app/target/release/file-server-rs /usr/local/bin/file-server-rs

# Copy static files and migrations
COPY --from=builder /app/static ./static
COPY --from=builder /app/migrations ./migrations

# Create directories for data
RUN mkdir -p /app/data /app/files && \
    chown -R appuser:appuser /app

# Switch to non-root user
USER appuser

ARG DATABASE_URL
ARG FILE_SERVER_STORAGE_PATH
ARG HOST
ARG PORT
ARG FILE_SERVER_STORAGE_TYPE
ARG FILE_SERVER_AUTH_TOKEN
ARG FILE_SERVER_DISABLE_UPLOAD_PAGE
ARG FILE_SERVER_ALLOWED_FILE_TYPES
ARG AWS_S3_BUCKET
ARG AWS_S3_REGION
ARG AWS_ACCESS_KEY_ID
ARG AWS_SECRET_ACCESS_KEY
ARG AWS_ENDPOINT_URL

EXPOSE 3000

CMD ["file-server-rs"]
