use super::{
	auth_dto::AuthDto,
	auth_service::{login, refresh_token},
};
use axum::{extract::Json, response::IntoResponse};

pub async fn post_login(Json(payload): Json<AuthDto>) -> impl IntoResponse {
	login(Json(payload)).await
}

pub async fn post_refresh(Json(payload): Json<String>) -> impl IntoResponse {
	refresh_token(Json(serde_json::Value::String(payload))).await
}
