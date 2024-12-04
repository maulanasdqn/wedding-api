pub mod reservation_controller;
pub mod reservation_dto;
pub mod reservation_endpoint;
pub mod reservation_service;

use axum::{
	routing::{get, post},
	Router,
};
use reservation_controller::{create, get_all};

pub fn reservation_router() -> Router {
	Router::new()
		.route("/", get(get_all))
		.route("/create", post(create))
}
