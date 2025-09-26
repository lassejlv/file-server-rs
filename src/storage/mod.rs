pub mod local;
pub mod s3;

use crate::config::{Config, StorageType};
use anyhow::Result;
use bytes::Bytes;

pub use local::LocalStorage;
pub use s3::S3Storage;

#[derive(Debug, Clone)]
pub enum Storage {
    Local(LocalStorage),
    S3(S3Storage),
}

impl Storage {
    pub async fn from_config(config: &Config) -> Result<Self> {
        match config.storage_type {
            StorageType::Local => {
                let storage = LocalStorage::new(config.storage_path.clone());
                Ok(Storage::Local(storage))
            }
            StorageType::S3 => {
                let bucket = config
                    .s3_bucket
                    .clone()
                    .ok_or_else(|| anyhow::anyhow!("S3 bucket not configured"))?;
                let storage = S3Storage::new(
                    bucket,
                    config.s3_region.clone(),
                    config.aws_endpoint_url.clone(),
                )
                .await?;
                Ok(Storage::S3(storage))
            }
        }
    }

    pub async fn store_file(&self, filename: &str, data: Bytes) -> Result<String> {
        match self {
            Storage::Local(storage) => storage.store_file(filename, data).await,
            Storage::S3(storage) => storage.store_file(filename, data).await,
        }
    }

    pub async fn get_file(&self, path: &str) -> Result<Vec<u8>> {
        match self {
            Storage::Local(storage) => storage.get_file(path).await,
            Storage::S3(storage) => storage.get_file(path).await,
        }
    }

    pub async fn delete_file(&self, path: &str) -> Result<()> {
        match self {
            Storage::Local(storage) => storage.delete_file(path).await,
            Storage::S3(storage) => storage.delete_file(path).await,
        }
    }

    pub fn get_mime_type(&self, path: &str) -> String {
        match self {
            Storage::Local(storage) => storage.get_mime_type(path),
            Storage::S3(storage) => storage.get_mime_type(path),
        }
    }

    pub fn storage_type(&self) -> String {
        match self {
            Storage::Local(_) => "local".to_string(),
            Storage::S3(_) => "s3".to_string(),
        }
    }
}
