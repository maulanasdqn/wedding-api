use std::convert::Infallible;
use std::env;

use axum::{
	body::Body,
	extract::Request,
	http::{self, Response, StatusCode},
	middleware::Next,
};
use serde_json::json;
use utils::{jwt::decode_jwt, structs::response::MessageResponse};

fn format_error(message: String) -> Response<Body> {
	let error_body = json!({
		"message": message
	});

	Response::builder()
		.status(StatusCode::FORBIDDEN)
		.header(http::header::CONTENT_TYPE, "application/json")
		.body(Body::from(error_body.to_string()))
		.unwrap()
}

pub async fn authorization_middleware(
	mut req: Request<Body>,
	next: Next,
) -> Result<Response<Body>, Infallible> {
	let auth_header = req.headers_mut().get(http::header::AUTHORIZATION);
	let auth_header: Result<String, MessageResponse> = match auth_header {
		Some(header) => {
			let header = header.to_str();
			match header {
				Ok(header) => Ok(header.to_string()),
				Err(_err) => {
					let message = "Invalid header".to_string();
					return Ok(format_error(message));
				}
			}
		}
		None => {
			let message = "Missing header".to_string();
			return Ok(format_error(message));
		}
	};

	let auth_header = match auth_header {
		Ok(header) => header,
		Err(auth_error) => {
			let error_response = format_error(auth_error.message);
			return Ok(error_response);
		}
	};

	let mut header = auth_header.split_whitespace();

	let token = match header.nth(1) {
		Some(token) => token,
		None => {
			let error_response = format_error("Invalid token".to_string());
			return Ok(error_response);
		}
	};

	let token_data = match decode_jwt(token.to_string()) {
		Ok(data) => data,
		Err(_) => {
			let message = "Invalid token".to_string();
			return Ok(format_error(message));
		}
	};

	let user_email = env::var("USER_EMAIL").expect("USER_EMAIL must be set");
	if token_data.claims.email != user_email {
		let error_response = format_error("Unauthorized user".to_string());
		return Ok(error_response);
	}

	req.extensions_mut().insert(user_email);
	let next = next.run(req).await;
	Ok(next)
}
