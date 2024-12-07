use axum::{routing::post, Router};

pub mod auth_controller;
pub mod auth_dto;
pub mod auth_middleware;
pub mod auth_service;

pub fn auth_router() -> Router {
	Router::new().route("/login", post(auth_controller::post_login))
}
