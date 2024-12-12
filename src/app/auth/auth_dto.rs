use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

#[derive(Clone, Debug, Serialize, Deserialize, Validate, ToSchema)]
pub struct AuthDto {
	#[validate(email(message = "Invalid email format"))]
	pub email: String,

	#[validate(length(
		min = 8,
		message = "Password must be at least 8 characters long"
	))]
	pub password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct AuthData {
	pub access_token: String,
	pub refresh_token: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct AuthResponse {
	pub data: AuthData,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct MessageResponse {
	pub message: String,
}
