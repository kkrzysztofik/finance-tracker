use axum::extract::{Multipart, State};
use axum::Json;
use serde::Serialize;
use sqlx::PgPool;

use crate::error::AppError;
use crate::services;

#[derive(Serialize)]
pub struct ImportResponse {
    pub total_rows: i32,
    pub imported: i32,
    pub skipped: i32,
}

pub async fn upload(
    State(pool): State<PgPool>,
    mut multipart: Multipart,
) -> Result<Json<ImportResponse>, AppError> {
    let field = multipart
        .next_field()
        .await
        .map_err(|e| AppError::BadRequest(format!("Multipart error: {e}")))?
        .ok_or_else(|| AppError::BadRequest("No file provided".into()))?;

    let filename = field
        .file_name()
        .unwrap_or("upload.csv")
        .to_string();

    let bytes = field
        .bytes()
        .await
        .map_err(|e| AppError::BadRequest(format!("Failed to read file: {e}")))?;

    let content = String::from_utf8(bytes.to_vec())
        .map_err(|e| AppError::BadRequest(format!("File is not valid UTF-8: {e}")))?;

    let result = services::import::import_file(&pool, &filename, &content)
        .await
        .map_err(AppError::BadRequest)?;

    Ok(Json(ImportResponse {
        total_rows: result.total_rows,
        imported: result.imported,
        skipped: result.skipped,
    }))
}
