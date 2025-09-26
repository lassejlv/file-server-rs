use anyhow::Result;
use aws_config::BehaviorVersion;
use aws_sdk_s3::primitives::ByteStream;
use aws_sdk_s3::Client;
use bytes::Bytes;
use std::path::Path;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct S3Storage {
    client: Client,
    bucket: String,
}

impl S3Storage {
    pub async fn new(
        bucket: String,
        region: Option<String>,
        endpoint_url: Option<String>,
    ) -> Result<Self> {
        let mut config_loader = aws_config::defaults(BehaviorVersion::latest());

        if let Some(region) = region {
            config_loader = config_loader.region(aws_config::Region::new(region));
        }

        let config = config_loader.load().await;

        let client = if let Some(endpoint) = endpoint_url {
            // For S3-compatible services like Cloudflare R2, MinIO, etc.
            Client::from_conf(
                aws_sdk_s3::config::Builder::from(&config)
                    .endpoint_url(endpoint)
                    .force_path_style(true) // Required for some S3-compatible services
                    .build(),
            )
        } else {
            Client::new(&config)
        };

        Ok(Self { client, bucket })
    }

    pub async fn store_file(&self, filename: &str, data: Bytes) -> Result<String> {
        let file_extension = Path::new(filename)
            .extension()
            .and_then(|ext| ext.to_str())
            .unwrap_or("");

        let name_without_ext = Path::new(filename)
            .file_stem()
            .and_then(|stem| stem.to_str())
            .unwrap_or("file");

        let unique_filename = if file_extension.is_empty() {
            format!(
                "{}-{}",
                name_without_ext,
                Uuid::new_v7(uuid::timestamp::Timestamp::now(uuid::NoContext))
            )
        } else {
            format!(
                "{}-{}.{}",
                name_without_ext,
                Uuid::new_v7(uuid::timestamp::Timestamp::now(uuid::NoContext)),
                file_extension
            )
        };

        let key = format!("uploads/{}", unique_filename);

        let content_type = mime_guess::from_path(filename)
            .first_or_octet_stream()
            .to_string();

        self.client
            .put_object()
            .bucket(&self.bucket)
            .key(&key)
            .body(ByteStream::from(data))
            .content_type(content_type)
            .send()
            .await?;

        Ok(format!("/{}", key))
    }

    pub async fn get_file(&self, path: &str) -> Result<Vec<u8>> {
        let key = path.trim_start_matches('/');

        let response = self
            .client
            .get_object()
            .bucket(&self.bucket)
            .key(key)
            .send()
            .await?;

        let data = response.body.collect().await?;
        Ok(data.into_bytes().to_vec())
    }

    pub async fn delete_file(&self, path: &str) -> Result<()> {
        let key = path.trim_start_matches('/');

        self.client
            .delete_object()
            .bucket(&self.bucket)
            .key(key)
            .send()
            .await?;

        Ok(())
    }

    pub fn get_mime_type(&self, path: &str) -> String {
        mime_guess::from_path(path)
            .first_or_octet_stream()
            .to_string()
    }
}
