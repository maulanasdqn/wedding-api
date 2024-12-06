use super::auth_dto::AuthDto;
use super::auth_service::login;
use axum::{extract::Json, response::IntoResponse};

pub async fn post_login(Json(payload): Json<AuthDto>) -> impl IntoResponse {
	login(Json(payload)).await
}
