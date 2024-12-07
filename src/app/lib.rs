mod auth;
mod reservations;

use auth::{auth_middleware::authorization_middleware, auth_router};
use axum::{
	http::{header, HeaderValue, Method},
	middleware::from_fn,
	Router,
};
use reservations::reservation_router;
use std::env;
use storage::upload_routes;
use tower_http::cors::CorsLayer;

pub async fn routes() -> Router {
	let cors_origin = match env::var("RUST_ENV").as_deref() {
		Ok("development") => "http://localhost:5173",
		_ => "https://wedding.fenny.me",
	};

	let cors_middleware = CorsLayer::new()
		.allow_origin(cors_origin.parse::<HeaderValue>().unwrap())
		.allow_methods([Method::GET, Method::POST, Method::OPTIONS])
		.allow_headers([header::AUTHORIZATION, header::CONTENT_TYPE])
		.allow_credentials(true);

	Router::new()
		.nest(
			"/api",
			Router::new()
				.nest("", protected_routes().await)
				.nest("/auth", auth_router()),
		)
		.layer(cors_middleware)
}

async fn protected_routes() -> Router {
	Router::new()
		.nest("/reservations", reservation_router())
		.nest("", upload_routes().await)
		.layer(from_fn(authorization_middleware))
}
