use axum::{extract::Query, response::IntoResponse, Json};
use utils::structs::response::TMeta;

use super::{
	reservation_dto::ReservationDto,
	reservation_service::{create_reservation, fetch_reservations},
};

pub async fn get_all(Query(params): Query<TMeta>) -> impl IntoResponse {
	fetch_reservations(params).await
}

pub async fn create(Json(payload): Json<ReservationDto>) -> impl IntoResponse {
	create_reservation(Json(payload)).await
}
