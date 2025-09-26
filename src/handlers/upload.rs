use axum::{
    extract::{Multipart, State},
    http::StatusCode,
    response::Json,
};
use bytes::Bytes;
use serde_json::{json, Value};
use std::sync::Arc;

use crate::{
    config::Config,
    database::{create_file, DbPool},
    models::{File, UploadResponse},
    storage::Storage,
};

#[derive(Clone)]
pub struct AppState {
    pub db: DbPool,
    pub storage: Storage,
    pub config: Arc<Config>,
}

pub async fn upload_file(
    State(state): State<AppState>,
    mut multipart: Multipart,
) -> Result<Json<UploadResponse>, (StatusCode, Json<Value>)> {
    let mut file_data: Option<(String, Bytes, u64)> = None;

    while let Some(field) = multipart.next_field().await.map_err(|_| {
        (
            StatusCode::BAD_REQUEST,
            Json(json!({"error": "Invalid multipart data"})),
        )
    })? {
        if field.name() == Some("file") {
            let filename = field
                .file_name()
                .ok_or_else(|| {
                    (
                        StatusCode::BAD_REQUEST,
                        Json(json!({"error": "No filename provided"})),
                    )
                })?
                .to_string();

            let data = field.bytes().await.map_err(|_| {
                (
                    StatusCode::BAD_REQUEST,
                    Json(json!({"error": "Failed to read file data"})),
                )
            })?;

            let size = data.len() as u64;
            file_data = Some((filename, data, size));
            break;
        }
    }

    let (filename, data, size) = file_data.ok_or_else(|| {
        (
            StatusCode::BAD_REQUEST,
            Json(json!({"error": "No file provided"})),
        )
    })?;

    if size > state.config.max_file_size {
        return Err((
            StatusCode::PAYLOAD_TOO_LARGE,
            Json(json!({"error": "File too large"})),
        ));
    }

    let allowed_types = state.config.allowed_file_types_vec();
    if !allowed_types.contains(&"*".to_string()) {
        let file_mime = mime_guess::from_path(&filename)
            .first_or_octet_stream()
            .to_string();

        if !allowed_types.contains(&file_mime) {
            return Err((
                StatusCode::BAD_REQUEST,
                Json(json!({"error": "File type not allowed"})),
            ));
        }
    }

    let file_path = state
        .storage
        .store_file(&filename, data)
        .await
        .map_err(|e| {
            tracing::error!("Failed to store file: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": "Failed to upload file"})),
            )
        })?;

    let file = File::new(
        file_path,
        filename,
        size as i64,
        state.storage.storage_type(),
    );

    let created_file = create_file(&state.db, &file).await.map_err(|e| {
        tracing::error!("Failed to save file metadata: {}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": "Failed to save file metadata"})),
        )
    })?;

    let response = UploadResponse {
        file_path: format!("/files/uploads/{}", created_file.id),
        storage_type: created_file.storage_type.clone(),
        data: created_file,
    };

    Ok(Json(response))
}
