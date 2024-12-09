mod auth;
mod reservations;

use auth::{auth_middleware::authorization_middleware, auth_router};
use axum::{
	http::{header, HeaderValue, Method},
	middleware::from_fn,
	Router,
};
use docs::docs_router;
use reservations::reservation_router;
use std::env;
use storage::upload_routes;
use tower_http::cors::CorsLayer;

pub async fn routes() -> Router {
	let cors_origins = match env::var("RUST_ENV").as_deref() {
		Ok("development") => vec!["http://localhost:5173"],
		Ok("production") => {
			vec!["https://wedding.fenny.me", "https://wedding.msdqn.dev"]
		}
		_ => vec!["https://wedding.fenny.me"],
	};

	let allowed_origins: Vec<HeaderValue> = cors_origins
		.into_iter()
		.filter_map(|origin| origin.parse::<HeaderValue>().ok())
		.collect();

	let cors_middleware = CorsLayer::new()
		.allow_origin(allowed_origins)
		.allow_methods([Method::GET, Method::POST, Method::OPTIONS])
		.allow_headers([header::AUTHORIZATION, header::CONTENT_TYPE])
		.allow_credentials(true);

	Router::new()
		.nest(
			"/api",
			Router::new()
				.nest("", protected_routes().await)
				.nest("/auth", auth_router())
				.nest("", docs_router().await),
		)
		.layer(cors_middleware)
}

async fn protected_routes() -> Router {
	Router::new()
		.nest("/reservations", reservation_router())
		.nest("", upload_routes().await)
		.layer(from_fn(authorization_middleware))
}
