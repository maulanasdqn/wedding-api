pub mod reservation_controller;
pub mod reservation_dto;
pub mod reservation_service;

use axum::{
	routing::{delete, get, post, put},
	Router,
};
use reservation_controller::{
	create, delete as delete_c, get_all, get_by_id, update,
};

pub fn reservation_router() -> Router {
	Router::new()
		.route("/", get(get_all))
		.route("/:id", get(get_by_id))
		.route("/create", post(create))
		.route("/delete/:id", delete(delete_c))
		.route("/update/:id", put(update))
}
