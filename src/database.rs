use crate::models::File;
use anyhow::Result;
use sqlx::{Pool, Sqlite, SqlitePool};
use tokio::fs;

pub type DbPool = Pool<Sqlite>;

pub async fn create_pool(database_url: &str) -> Result<DbPool> {
    // Create the database file if it doesn't exist
    if database_url.starts_with("sqlite:") {
        let db_path = database_url.strip_prefix("sqlite:").unwrap_or(database_url);
        if let Some(parent) = std::path::Path::new(db_path).parent() {
            tokio::fs::create_dir_all(parent).await?;
        }
        if !std::path::Path::new(db_path).exists() {
            fs::File::create(db_path).await?;
        }
    }

    let pool = SqlitePool::connect(database_url).await?;

    sqlx::migrate!("./migrations").run(&pool).await?;

    Ok(pool)
}

pub async fn create_file(pool: &DbPool, file: &File) -> Result<File> {
    let result = sqlx::query_as::<_, File>(
        r#"
        INSERT INTO files (id, path, name, size, storage_type, is_private, created_at, updated_at)
        VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)
        RETURNING id, path, name, size, storage_type, is_private, created_at, updated_at
        "#,
    )
    .bind(&file.id)
    .bind(&file.path)
    .bind(&file.name)
    .bind(file.size)
    .bind(&file.storage_type)
    .bind(file.is_private)
    .bind(&file.created_at)
    .bind(&file.updated_at)
    .fetch_one(pool)
    .await?;

    Ok(result)
}

pub async fn get_file_by_id(pool: &DbPool, id: &str) -> Result<Option<File>> {
    let file = sqlx::query_as::<_, File>(
        r#"
        SELECT id, path, name, size, storage_type, is_private, created_at, updated_at
        FROM files
        WHERE id = ?1
        "#,
    )
    .bind(id)
    .fetch_optional(pool)
    .await?;

    Ok(file)
}

pub async fn get_files(pool: &DbPool, limit: i64) -> Result<Vec<File>> {
    let files = sqlx::query_as::<_, File>(
        r#"
        SELECT id, path, name, size, storage_type, is_private, created_at, updated_at
        FROM files
        WHERE is_private = false
        ORDER BY created_at DESC
        LIMIT ?1
        "#,
    )
    .bind(limit)
    .fetch_all(pool)
    .await?;

    Ok(files)
}

pub async fn delete_file_by_id(pool: &DbPool, id: &str) -> Result<bool> {
    let result = sqlx::query("DELETE FROM files WHERE id = ?1 AND is_private = false")
        .bind(id)
        .execute(pool)
        .await?;

    Ok(result.rows_affected() > 0)
}
