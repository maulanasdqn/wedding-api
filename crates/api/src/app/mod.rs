mod auth;
mod reservations;
use axum::{http::HeaderValue, middleware, Router};
use reservations::reservation_router;
use tower_http::cors::CorsLayer;

pub fn create_routes() -> Router {
	let layer = CorsLayer::new()
		.allow_origin("https://wedding.fenny.me".parse::<HeaderValue>().unwrap());
	Router::new().nest(
		"/api",
		Router::new()
			.nest("/reservations", reservation_router())
			.layer(layer)
			.layer(middleware::from_fn(
				auth::auth_middleware::authorization_middleware,
			))
			.nest("/auth", auth::auth_router()),
	)
}
