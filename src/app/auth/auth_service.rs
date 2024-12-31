use axum::{
	http::StatusCode,
	response::{IntoResponse, Response},
	Json,
};
use serde_json::json;
use std::env;
use utils::{
	format_error, format_success, jwt::decode_refresh_token,
	jwt::encode_access_token, jwt::encode_refresh_token,
};

use super::auth_dto::AuthDto;

#[derive(serde::Serialize)]
pub struct ErrorResponse {
	pub message: String,
}

#[derive(serde::Serialize)]
pub struct LoginResponse {
	pub access_token: String,
	pub refresh_token: String,
}

pub async fn login(Json(credentials): Json<AuthDto>) -> Response {
	let email = env::var("USER_EMAIL").expect("USER_EMAIL must be set");
	let password = env::var("USER_PASSWORD").expect("USER_PASSWORD must be set");

	if credentials.email != email {
		return (
			StatusCode::UNAUTHORIZED,
			Json(format_error("Email or password is incorrect")),
		)
			.into_response();
	}

	if credentials.password != password {
		return (
			StatusCode::UNAUTHORIZED,
			Json(format_error("Email or password is incorrect")),
		)
			.into_response();
	}

	let access_token = encode_access_token(email.clone()).unwrap();
	let refresh_token = encode_refresh_token(email.clone()).unwrap();

	(
		StatusCode::OK,
		Json(format_success(
			json!({
				"access_token": access_token,
				"refresh_token": refresh_token,
			}),
			None,
		)),
	)
		.into_response()
}

pub async fn refresh_token(Json(payload): Json<serde_json::Value>) -> Response {
	let refresh_token = payload
		.get("refresh_token")
		.and_then(|token| token.as_str())
		.unwrap_or("");

	match decode_refresh_token(refresh_token.to_string()) {
		Ok(_email) => {
			let access_token =
				encode_access_token(env::var("USER_EMAIL").unwrap().to_string())
					.unwrap();
			(
				StatusCode::OK,
				Json(format_success(
					json!({
						"access_token": access_token
					}),
					None,
				)),
			)
				.into_response()
		}
		Err(_) => (
			StatusCode::UNAUTHORIZED,
			Json(format_error("Invalid or expired refresh token")),
		)
			.into_response(),
	}
}
