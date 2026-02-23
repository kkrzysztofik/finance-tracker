use axum::{
    extract::State,
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
    Json,
};
use base64::Engine;

pub async fn basic_auth(
    State(config): State<crate::config::Config>,
    request: Request<axum::body::Body>,
    next: Next,
) -> Result<Response, (StatusCode, Json<serde_json::Value>)> {
    let unauthorized = || {
        (
            StatusCode::UNAUTHORIZED,
            Json(serde_json::json!({ "error": "unauthorized" })),
        )
    };

    let header_value = request
        .headers()
        .get("Authorization")
        .and_then(|v| v.to_str().ok())
        .ok_or_else(unauthorized)?;

    let encoded = header_value
        .strip_prefix("Basic ")
        .ok_or_else(unauthorized)?;

    let decoded_bytes = base64::engine::general_purpose::STANDARD
        .decode(encoded)
        .map_err(|_| unauthorized())?;

    let decoded = String::from_utf8(decoded_bytes).map_err(|_| unauthorized())?;

    let (user, pass) = decoded.split_once(':').ok_or_else(unauthorized)?;

    if user != config.auth_user || pass != config.auth_pass {
        return Err(unauthorized());
    }

    Ok(next.run(request).await)
}
