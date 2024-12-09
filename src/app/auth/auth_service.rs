use super::auth_dto::AuthDto;
use axum::Json;
use serde_json::json;
use std::env;
use utils::{
	format_error, format_success, jwt::decode_refresh_token,
	jwt::encode_access_token, jwt::encode_refresh_token,
};

#[derive(serde::Serialize)]
pub struct ErrorResponse {
	pub message: String,
}

#[derive(serde::Serialize)]
pub struct LoginResponse {
	pub access_token: String,
	pub refresh_token: String,
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

	let access_token = encode_access_token(email.clone()).unwrap();
	let refresh_token = encode_refresh_token(email.clone()).unwrap();

	Json(format_success(
		json!({
			"access_token": access_token,
			"refresh_token": refresh_token,
		}),
		None,
	))
}

pub async fn refresh_token(
	Json(payload): Json<serde_json::Value>,
) -> Json<serde_json::Value> {
	let refresh_token = payload
		.get("refresh_token")
		.and_then(|token| token.as_str())
		.unwrap_or("");

	match decode_refresh_token(refresh_token.to_string()) {
		Ok(_email) => {
			let access_token =
				encode_access_token(env::var("USER_EMAIL").unwrap().to_string())
					.unwrap();
			Json(format_success(
				json!({
					"access_token": access_token
				}),
				None,
			))
		}
		Err(_) => Json(format_error("Invalid or expired refresh token")),
	}
}
