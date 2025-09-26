# File Server RS

A high-performance file server written in Rust

## Features

- **Fast & Efficient**: Built with Axum and Tokio for excellent performance
- **Multiple Storage Backends**: Support for local filesystem and AWS S3
- **File Management**: Upload, download, list, and delete files
- **Authentication**: Optional bearer token authentication
- **Modern UI**: Beautiful web interface with drag & drop uploads
- **File Validation**: Configurable file size limits and type restrictions
- **Database Integration**: SQLite database for metadata storage
- **Production Ready**: Comprehensive error handling and logging

## Quick Start

1. **Build the project**:

   ```bash
   cargo build --release
   ```

2. **Run with default settings**:

   ```bash
   cargo run
   ```

3. **Access the web interface**:
   Open http://localhost:3000 in your browser

## Configuration

Configure the server using environment variables:

### Basic Settings

- `HOST`: Server host (default: 0.0.0.0)
- `PORT`: Server port (default: 3000)
- `DATABASE_URL`: SQLite database URL (default: sqlite:./files.db)

### File Upload Settings

- `FILE_SERVER_MAX_FILE_SIZE`: Maximum file size in bytes (default: 52428800 = 50MB)
- `FILE_SERVER_ALLOWED_FILE_TYPES`: Comma-separated list of allowed MIME types (default: all files allowed)
- `FILE_SERVER_STORAGE_TYPE`: Storage backend - "local" or "s3" (default: local)
- `FILE_SERVER_STORAGE_PATH`: Local storage directory (default: ./files)

### Security Settings

- `FILE_SERVER_AUTH_TOKEN`: Bearer token for upload authentication (optional)
- `FILE_SERVER_DISABLE_UPLOAD_PAGE`: Disable the web interface (default: false)

### AWS S3 Settings (when using S3 storage)

- `AWS_S3_BUCKET`: S3 bucket name
- `AWS_S3_REGION`: AWS region (or compatible region)
- `AWS_ACCESS_KEY_ID`: AWS access key (or compatible access key)
- `AWS_SECRET_ACCESS_KEY`: AWS secret key (or compatible secret key)
- `AWS_ENDPOINT_URL`: Custom endpoint URL for S3-compatible services (optional)

#### S3-Compatible Services

The server supports any S3-compatible storage service by setting the `AWS_ENDPOINT_URL`:

- **Cloudflare R2**: Use your account's R2 endpoint
- **MinIO**: Use your MinIO server endpoint
- **DigitalOcean Spaces**: Use your Spaces endpoint
- **Wasabi**: Use Wasabi's endpoint
- Any other S3-compatible service

## API Endpoints

### Upload File

```
POST /upload
Content-Type: multipart/form-data
Authorization: Bearer <token> (if auth enabled)

Form field: file
```

### List Files

```
GET /files/uploads?limit=50
```

### Download File

```
GET /files/uploads/:id
```

### Delete File

```
DELETE /files/uploads/:id
```

## Examples

### Local Storage with Authentication

```bash
export FILE_SERVER_AUTH_TOKEN="your-secret-token"
export FILE_SERVER_STORAGE_PATH="/var/uploads"
cargo run
```

### S3 Storage

## Development

1. **Install dependencies**:

   ```bash
   cargo check
   ```

2. **Run in development mode**:

   ```bash
   cargo run
   ```

3. **Run tests**:
   ```bash
   cargo test
   ```

```

```
