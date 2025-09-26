use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct File {
    pub id: String,
    pub path: String,
    pub name: String,
    pub size: i64,
    pub storage_type: String,
    pub is_private: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FileResponse {
    pub id: String,
    pub name: String,
    pub size: i64,
    pub storage_type: String,
    pub created_at: DateTime<Utc>,
}

impl From<File> for FileResponse {
    fn from(file: File) -> Self {
        Self {
            id: file.id,
            name: file.name,
            size: file.size,
            storage_type: file.storage_type,
            created_at: file.created_at,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UploadResponse {
    pub file_path: String,
    pub storage_type: String,
    pub data: File,
}

impl File {
    pub fn new(path: String, name: String, size: i64, storage_type: String) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v7(uuid::timestamp::Timestamp::now(uuid::NoContext)).to_string(),
            path,
            name,
            size,
            storage_type,
            is_private: false,
            created_at: now,
            updated_at: now,
        }
    }
}
