use std::convert::Infallible;
use std::env;

use axum::{
	body::Body,
	extract::Request,
	http::{self, StatusCode},
	middleware::Next,
	response::Response,
};
use utils::jwt::decode_jwt;

struct AuthError {
	message: String,
	status_code: StatusCode,
}

impl AuthError {
	fn into_response(self) -> Response<Body> {
		Response::builder()
			.status(self.status_code)
			.body(Body::from(self.message))
			.unwrap()
	}
}

pub async fn authorization_middleware(
	mut req: Request<Body>,
	next: Next,
) -> Result<Response<Body>, Infallible> {
	let auth_header = req.headers_mut().get(http::header::AUTHORIZATION);
	let auth_header = match auth_header {
		Some(header) => header.to_str().map_err(|_| AuthError {
			message: "Invalid header".to_string(),
			status_code: StatusCode::FORBIDDEN,
		}),
		None => Err(AuthError {
			message: "Missing authorization header".to_string(),
			status_code: StatusCode::FORBIDDEN,
		}),
	};

	let auth_header = match auth_header {
		Ok(header) => header,
		Err(auth_error) => return Ok(auth_error.into_response()),
	};

	let mut header = auth_header.split_whitespace();

	let token = match header.nth(1) {
		Some(token) => token,
		None => {
			return Ok(AuthError {
				message: "Invalid token format".to_string(),
				status_code: StatusCode::UNAUTHORIZED,
			}
			.into_response())
		}
	};

	let token_data = match decode_jwt(token.to_string()) {
		Ok(data) => data,
		Err(_) => {
			return Ok(AuthError {
				message: "Unable to decode token".to_string(),
				status_code: StatusCode::UNAUTHORIZED,
			}
			.into_response())
		}
	};

	let user_email = env::var("USER_EMAIL").expect("USER_EMAIL must be set");
	if token_data.claims.email != user_email {
		return Ok(AuthError {
			message: "Unauthorized user".to_string(),
			status_code: StatusCode::UNAUTHORIZED,
		}
		.into_response());
	}

	req.extensions_mut().insert(user_email);
	Ok(next.run(req).await)
}
