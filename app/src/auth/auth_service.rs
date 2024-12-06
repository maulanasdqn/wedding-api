use super::auth_dto::{AuthDto, AuthResponse};
use axum::Json;
use std::env;
use utils::{
	format_error, format_success, jwt::encode_jwt, structs::response::TResponse,
};

pub async fn login(
	Json(credentials): Json<AuthDto>,
) -> Json<TResponse<AuthResponse>> {
	let email = env::var("USER_EMAIL").expect("USER_EMAIL must be set");
	let password = env::var("USER_PASSWORD").expect("USER_PASSWORD must be set");

	if credentials.email != email {
		let error_message = "Invalid email".to_string();
		return Json(format_error(error_message));
	}

	if credentials.password != password {
		let error_message = "Invalid password".to_string();
		return Json(format_error(error_message));
	}

	let token = encode_jwt(email).unwrap();
	let token_response = AuthResponse { token };
	Json(format_success(token_response, None))
}
