use super::{
	auth_dto::AuthDto,
	auth_service::{login, refresh_token},
};
use axum::{extract::Json, response::IntoResponse};

#[utoipa::path(
    post,
    path = "/api/auth/login",
    request_body = AuthDto,
    responses(
        (status = 200, description = "Login successful", body = AuthResponse),
        (status = 401, description = "Unauthorized", body = MessageResponse)
    ),
    tag = "Authentication"
)]

pub async fn post_login(Json(payload): Json<AuthDto>) -> impl IntoResponse {
	login(Json(payload)).await
}

#[utoipa::path(
    post,
    path = "/api/auth/refresh",
    request_body(content = String, description = "Refresh token"),
    responses(
        (status = 200, description = "Token refreshed", body = AuthResponse),
        (status = 400, description = "Invalid refresh token", body = MessageResponse)
    ),
    tag = "Authentication"
)]

pub async fn post_refresh(Json(payload): Json<String>) -> impl IntoResponse {
	refresh_token(Json(serde_json::Value::String(payload))).await
}
