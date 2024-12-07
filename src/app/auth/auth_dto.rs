use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Clone, Debug, Serialize, Deserialize, Validate)]
pub struct AuthDto {
	#[validate(email(message = "Invalid email format"))]
	pub email: String,

	#[validate(length(
		min = 8,
		message = "Password must be at least 8 characters long"
	))]
	pub password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthResponse {
	pub token: String,
}
