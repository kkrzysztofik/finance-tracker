use axum::{middleware, Router};
use sea_orm::DatabaseConnection;
use tower_http::cors::{Any, CorsLayer};

use crate::config::Config;

pub mod accounts;
pub mod categories;
pub mod categorize;
pub mod import;
pub mod stats;
pub mod transactions;

pub fn create_router(pool: DatabaseConnection, config: Config) -> Router {
    let api = Router::new()
        .route("/api/transactions", axum::routing::get(transactions::list))
        .route(
            "/api/transactions/{id}",
            axum::routing::get(transactions::get_one),
        )
        .route(
            "/api/transactions/{id}/category",
            axum::routing::patch(transactions::update_category),
        )
        .route("/api/import", axum::routing::post(import::upload))
        .route("/api/categories", axum::routing::get(categories::list))
        .route("/api/accounts", axum::routing::get(accounts::list))
        .route("/api/stats/monthly", axum::routing::get(stats::monthly))
        .route(
            "/api/stats/categories",
            axum::routing::get(stats::by_category),
        )
        .route("/api/categorize", axum::routing::post(categorize::categorize))
        .layer(middleware::from_fn_with_state(config, crate::auth::basic_auth))
        .with_state(pool);

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    Router::new().merge(api).layer(cors)
}
