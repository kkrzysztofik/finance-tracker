use axum::extract::State;
use axum::Json;
use sea_orm::DatabaseConnection;

use crate::error::AppError;

pub async fn categorize(
    State(db): State<DatabaseConnection>,
) -> Result<Json<serde_json::Value>, AppError> {
    let api_key = std::env::var("OPENAI_API_KEY")
        .map_err(|_| AppError::BadRequest("OPENAI_API_KEY not configured".into()))?;

    let result = crate::services::categorize::categorize_uncategorized(&db, &api_key)
        .await
        .map_err(AppError::Internal)?;

    Ok(Json(serde_json::json!({
        "total": result.total,
        "categorized": result.categorized,
        "failed": result.failed,
    })))
}
