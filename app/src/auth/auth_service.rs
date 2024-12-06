use super::auth_dto::AuthDto;
use axum::Json;
use serde_json::json;
use std::env;
use utils::{format_error, format_success, jwt::encode_jwt};

#[derive(serde::Serialize)]
pub struct ErrorResponse {
	pub message: String,
}

pub async fn login(Json(credentials): Json<AuthDto>) -> Json<serde_json::Value> {
	let email = env::var("USER_EMAIL").expect("USER_EMAIL must be set");
	let password = env::var("USER_PASSWORD").expect("USER_PASSWORD must be set");

	if credentials.email != email {
		return Json(format_error("Email or password is incorrect"));
	}

	if credentials.password != password {
		return Json(format_error("Email or password is incorrect"));
	}

	let token = encode_jwt(email).unwrap();

	Json(format_success(json!({ "access_token": token }), None))
}
