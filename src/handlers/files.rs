use axum::{
    extract::{Path, Query, State},
    http::{header, HeaderMap, StatusCode},
    response::{Json, Response},
};
use serde::Deserialize;
use serde_json::{json, Value};

use crate::{
    database::{delete_file_by_id, get_file_by_id, get_files},
    models::FileResponse,
};

use super::upload::AppState;

#[derive(Deserialize)]
pub struct FilesQuery {
    limit: Option<i64>,
}

pub async fn get_file_by_id_handler(
    Path(id): Path<String>,
    State(state): State<AppState>,
) -> Result<Response, (StatusCode, Json<Value>)> {
    let file = get_file_by_id(&state.db, &id).await.map_err(|e| {
        tracing::error!("Database error: {}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": "Database error"})),
        )
    })?;

    let file = file.ok_or_else(|| {
        (
            StatusCode::NOT_FOUND,
            Json(json!({"error": "File not found"})),
        )
    })?;

    if file.is_private {
        return Err((
            StatusCode::NOT_FOUND,
            Json(json!({"error": "File not found"})),
        ));
    }

    let file_data = state.storage.get_file(&file.path).await.map_err(|e| {
        tracing::error!("Failed to retrieve file: {}", e);
        (
            StatusCode::NOT_FOUND,
            Json(json!({"error": "File not found"})),
        )
    })?;

    let content_type = state.storage.get_mime_type(&file.path);

    let mut headers = HeaderMap::new();
    headers.insert(header::CONTENT_TYPE, content_type.parse().unwrap());
    headers.insert(
        header::CONTENT_LENGTH,
        file.size.to_string().parse().unwrap(),
    );
    headers.insert(
        header::CACHE_CONTROL,
        "public, max-age=31536000".parse().unwrap(),
    );
    headers.insert(header::ACCEPT_RANGES, "bytes".parse().unwrap());
    headers.insert(header::ETAG, format!("\"{}\"", file.id).parse().unwrap());

    Ok(Response::builder()
        .status(StatusCode::OK)
        .body(file_data.into())
        .unwrap())
}

pub async fn list_files(
    Query(params): Query<FilesQuery>,
    State(state): State<AppState>,
) -> Result<Json<Vec<FileResponse>>, (StatusCode, Json<Value>)> {
    let limit = params.limit.unwrap_or(10).min(500);

    let files = get_files(&state.db, limit).await.map_err(|e| {
        tracing::error!("Database error: {}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": "Failed to fetch files"})),
        )
    })?;

    let file_responses: Vec<FileResponse> = files.into_iter().map(FileResponse::from).collect();

    Ok(Json(file_responses))
}

pub async fn delete_file(
    Path(id): Path<String>,
    State(state): State<AppState>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    let file = get_file_by_id(&state.db, &id).await.map_err(|e| {
        tracing::error!("Database error: {}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": "Database error"})),
        )
    })?;

    let file = file.ok_or_else(|| {
        (
            StatusCode::NOT_FOUND,
            Json(json!({"error": "File not found"})),
        )
    })?;

    if file.is_private {
        return Err((
            StatusCode::NOT_FOUND,
            Json(json!({"error": "File not found"})),
        ));
    }

    state.storage.delete_file(&file.path).await.map_err(|e| {
        tracing::error!("Failed to delete file from storage: {}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": "Failed to delete file"})),
        )
    })?;

    let deleted = delete_file_by_id(&state.db, &id).await.map_err(|e| {
        tracing::error!("Database error: {}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": "Database error"})),
        )
    })?;

    if !deleted {
        return Err((
            StatusCode::NOT_FOUND,
            Json(json!({"error": "File not found"})),
        ));
    }

    Ok(Json(json!({"message": "File deleted successfully"})))
}
