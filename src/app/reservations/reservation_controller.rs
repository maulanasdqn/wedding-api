use crate::reservations::reservation_dto::TMetas;

use super::{
	reservation_dto::ReservationDto,
	reservation_service::{create_reservation, fetch_reservations},
};
use axum::{extract::Query, response::IntoResponse, Json};

#[utoipa::path(
    get,
    path = "/api/reservations",
    params(TMetas),
    security(
        ("Bearer" = [])
    ),
    responses(
        (status = 201, description = "List Reservation", body = ReservationListResponse),
        (status = 400, description = "Invalid reservation data", body = MessageResponse)
    ),
    tag = "Reservations"
)]

pub async fn get_all(Query(params): Query<TMetas>) -> impl IntoResponse {
	println!("Raw query params: {:?}", params);
	fetch_reservations(params).await
}

#[utoipa::path(
    post,
    path = "/api/reservations/create",
    request_body = ReservationDto,
    security(
        ("Bearer" = [])
    ),
    responses(
        (status = 201, description = "Reservation created", body = ReservationDto),
        (status = 400, description = "Invalid reservation data", body = MessageResponse)
    ),
    tag = "Reservations"
)]

pub async fn create(Json(payload): Json<ReservationDto>) -> impl IntoResponse {
	create_reservation(Json(payload)).await
}
