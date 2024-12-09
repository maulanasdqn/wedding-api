use axum::http::StatusCode;
use chrono::{Duration, Utc};
use jsonwebtoken::{
	decode, encode, DecodingKey, EncodingKey, Header, TokenData, Validation,
};
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
	pub exp: usize,
	pub iat: usize,
	pub email: String,
}

pub fn encode_jwt(email: String) -> Result<String, StatusCode> {
	let secret: String = env::var("JWT_SECRET")
		.expect("JWT_SECRET must be set")
		.to_string();
	let now = Utc::now();
	let expire: chrono::TimeDelta = Duration::days(30);
	let exp: usize = (now + expire).timestamp() as usize;
	let iat: usize = now.timestamp() as usize;
	let claim = Claims { iat, exp, email };
	encode(
		&Header::default(),
		&claim,
		&EncodingKey::from_secret(secret.as_ref()),
	)
	.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

pub fn decode_jwt(jwt_token: String) -> Result<TokenData<Claims>, StatusCode> {
	let secret = env::var("JWT_SECRET")
		.expect("JWT_SECRET must be set")
		.to_string();
	let result: Result<TokenData<Claims>, StatusCode> = decode(
		&jwt_token,
		&DecodingKey::from_secret(secret.as_ref()),
		&Validation::default(),
	)
	.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR);
	result
}
