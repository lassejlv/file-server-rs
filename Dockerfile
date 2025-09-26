FROM rust:1.88-slim as builder

# Install required dependencies for building
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY Cargo.toml ./

COPY src ./src
COPY migrations ./migrations
COPY static ./static

RUN cargo build --release

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

RUN useradd -r -s /bin/false -m -d /app appuser

WORKDIR /app

COPY --from=builder /app/target/release/file-server-rs /usr/local/bin/file-server-rs

COPY --from=builder /app/static ./static
COPY --from=builder /app/migrations ./migrations

RUN mkdir -p /app/data /app/files && \
    chown -R appuser:appuser /app && \
    chmod -R 755 /app

USER appuser

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
