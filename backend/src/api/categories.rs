use axum::extract::State;
use axum::Json;
use sea_orm::{DatabaseConnection, EntityTrait, QueryOrder};

use crate::entities::categories;
use crate::error::AppError;
use crate::models::Category;

pub async fn list(
    State(db): State<DatabaseConnection>,
) -> Result<Json<Vec<Category>>, AppError> {
    let categories = categories::Entity::find()
        .order_by_asc(categories::Column::Name)
        .all(&db)
        .await?;

    Ok(Json(categories))
}
