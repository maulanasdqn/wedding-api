use axum::Json;
use database::get_db;
use database::schemas::reservation::{ActiveModel, Entity, Model};
use sea_orm::sqlx::types::chrono;
use sea_orm::{ActiveModelTrait, EntityTrait, PaginatorTrait, Set};
use utils::structs::response::{TMeta, TResponse};
use utils::{format_error, format_success};
use uuid::Uuid;

use super::reservation_dto::ReservationDto;

pub async fn create_reservation(
	new_reservation: Json<ReservationDto>,
) -> Json<TResponse<Model>> {
	let db = get_db().await;

	let active_model = ActiveModel {
		id: Set(Uuid::new_v4()),
		fullname: Set(new_reservation.fullname.clone()),
		attendance: Set(new_reservation.attendance.clone()),
		speech: Set(new_reservation.speech.clone()),
		speech_audio: Set(new_reservation.speech_audio.clone()),
		created_at: Set(chrono::Utc::now()),
	};

	match active_model.insert(&db).await {
		Ok(model) => {
			let response = format_success(model, None);
			Json(response)
		}
		Err(err) => {
			let error_message = format!("Failed to create reservations: {}", err);
			let response = format_error(error_message);
			Json(response)
		}
	}
}

pub async fn fetch_reservations(params: TMeta) -> Json<TResponse<Vec<Model>>> {
	let db = get_db().await;

	let page = params.page.unwrap_or(1);
	let per_page = params.per_page.unwrap_or(10);

	let paginator = Entity::find().paginate(&db, per_page);

	let reservations = match paginator.fetch_page(page - 1).await {
		Ok(reservations) => reservations,
		Err(err) => {
			let error_message = format!("Failed to fetch reservations: {}", err);
			let response = format_error(error_message);
			return Json(response);
		}
	};

	let total = match paginator.num_items().await {
		Ok(total) => total,
		Err(err) => {
			let error_message = format!("Failed to count reservations: {}", err);
			let response = format_error(error_message);
			return Json(response);
		}
	};

	let meta = TMeta {
		page: Some(page),
		per_page: Some(per_page),
		total: Some(total),
	};

	let response = format_success(reservations, Some(meta));
	Json(response)
}
