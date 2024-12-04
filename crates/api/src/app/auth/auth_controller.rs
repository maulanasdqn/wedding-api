use super::auth_dto::AuthDto;
use super::auth_service::login;
use axum::{extract::Json, http::StatusCode, response::IntoResponse};

pub async fn post_login(Json(credentials): Json<AuthDto>) -> impl IntoResponse {
	match login(Json(credentials)).await {
		Ok(token) => (StatusCode::OK, token).into_response(),
		Err(status) => (status.message).into_response(),
	}
}
