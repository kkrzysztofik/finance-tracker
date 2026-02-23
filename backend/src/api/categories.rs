use axum::extract::State;
use axum::Json;
use sqlx::PgPool;

use crate::error::AppError;
use crate::models::Category;

pub async fn list(
    State(pool): State<PgPool>,
) -> Result<Json<Vec<Category>>, AppError> {
    let categories = sqlx::query_as::<_, Category>(
        "SELECT id, name, name_pl FROM categories ORDER BY name",
    )
    .fetch_all(&pool)
    .await?;

    Ok(Json(categories))
}
