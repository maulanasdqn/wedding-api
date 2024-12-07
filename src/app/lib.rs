mod auth;
mod reservations;
use auth::{auth_middleware::authorization_middleware, auth_router};
use axum::{http::HeaderValue, middleware::from_fn, Router};
use reservations::reservation_router;
use std::env;
use tower_http::cors::CorsLayer;

pub fn routes() -> Router {
	let origin = env::var("CORS_ORIGIN").expect("CORS_ORIGIN must be set");
	let cors_middleware =
		CorsLayer::new().allow_origin(origin.parse::<HeaderValue>().unwrap());

	Router::new()
		.nest(
			"/api",
			Router::new()
				.nest("", protected_routes())
				.nest("/auth", auth_router()),
		)
		.layer(cors_middleware)
}

fn protected_routes() -> Router {
	Router::new()
		.nest("/reservations", reservation_router())
		.layer(from_fn(authorization_middleware))
}
