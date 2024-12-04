mod reservations;
use axum::Router;
use reservations::reservation_router;

pub fn create_routes() -> Router {
	Router::new().nest(
		"/api",
		Router::new().nest("/reservations", reservation_router()),
	)
}
