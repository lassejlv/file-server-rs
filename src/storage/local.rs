use anyhow::Result;
use bytes::Bytes;
use std::path::{Path, PathBuf};
use tokio::fs;
use tokio::io::AsyncWriteExt;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct LocalStorage {
    base_path: PathBuf,
}

impl LocalStorage {
    pub fn new(base_path: PathBuf) -> Self {
        Self { base_path }
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

        let file_path = self.base_path.join(&unique_filename);

        if let Some(parent) = file_path.parent() {
            fs::create_dir_all(parent).await?;
        }

        let mut file = fs::File::create(&file_path).await?;
        file.write_all(&data).await?;
        file.sync_all().await?;

        Ok(file_path.to_string_lossy().to_string())
    }

    pub async fn get_file(&self, path: &str) -> Result<Vec<u8>> {
        let file_path = Path::new(path);
        let data = fs::read(file_path).await?;
        Ok(data)
    }

    pub async fn delete_file(&self, path: &str) -> Result<()> {
        let file_path = Path::new(path);
        fs::remove_file(file_path).await?;
        Ok(())
    }

    pub fn get_mime_type(&self, path: &str) -> String {
        mime_guess::from_path(path)
            .first_or_octet_stream()
            .to_string()
    }
}
