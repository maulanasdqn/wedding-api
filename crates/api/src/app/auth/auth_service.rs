use super::auth_dto::AuthDto;
use axum::Json;
use std::env;
use utils::{jwt::encode_jwt, structs::response::MessageResponse};

pub async fn login(
	Json(credentials): Json<AuthDto>,
) -> Result<Json<String>, MessageResponse> {
	let user_email = env::var("USER_EMAIL").expect("USER_EMAIL must be set");
	let user_password =
		env::var("USER_PASSWORD").expect("USER_PASSWORD must be set");

	if credentials.email != user_email {
		return Err(MessageResponse {
			message: "Invalid email".to_string(),
		});
	}

	if credentials.password != user_password {
		return Err(MessageResponse {
			message: "Invalid password".to_string(),
		});
	}

	let token = encode_jwt(user_email).map_err(|_| MessageResponse {
		message: "Error encoding token".to_string(),
	})?;

	Ok(Json(token))
}
