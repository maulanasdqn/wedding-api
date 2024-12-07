mod auth;
mod reservations;
use auth::{auth_middleware::authorization_middleware, auth_router};
use axum::{http::HeaderValue, middleware::from_fn, Router};
use reservations::reservation_router;
use std::env;
use storage::upload_routes;
use tower_http::cors::CorsLayer;

pub async fn routes() -> Router {
	let origin = env::var("CORS_ORIGIN").expect("CORS_ORIGIN must be set");
	let cors_middleware =
		CorsLayer::new().allow_origin(origin.parse::<HeaderValue>().unwrap());

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
