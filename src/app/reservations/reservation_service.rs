use super::reservation_dto::{ReservationDto, TMetas};
use axum::Json;
use database::get_db;
use database::schemas::reservation::{ActiveModel, Entity};
use sea_orm::sqlx::types::chrono;
use sea_orm::{ActiveModelTrait, EntityTrait, PaginatorTrait, Set};
use serde_json::Value;
use utils::structs::response::TMeta;
use utils::{format_error, format_success};
use uuid::Uuid;

pub async fn create_reservation(
	new_reservation: Json<ReservationDto>,
) -> Json<Value> {
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
		Ok(model) => Json(format_success(model, None)),
		Err(err) => Json(format_error(&format!(
			"Failed to create reservations: {}",
			err
		))),
	}
}

pub async fn fetch_reservations(params: TMetas) -> Json<Value> {
	let db = get_db().await;

	let page = params.page.unwrap_or_else(|| 1);
	let per_page = params.per_page.unwrap_or(1000);
	let paginator = Entity::find().paginate(&db, per_page.into());

	let reservations = match paginator.fetch_page((page - 1).into()).await {
		Ok(reservations) => reservations,
		Err(err) => {
			return Json(format_error(&format!(
				"Failed to fetch reservations: {}",
				err
			)));
		}
	};

	let total = match paginator.num_items().await {
		Ok(total) => total,
		Err(err) => {
			return Json(format_error(&format!(
				"Failed to count reservations: {}",
				err
			)));
		}
	};

	let meta = TMeta {
		page: Some(page),
		per_page: Some(per_page),
		total: Some(total),
	};

	Json(format_success(reservations, Some(meta)))
}
