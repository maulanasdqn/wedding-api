use super::models::CreateReservation;
use crate::commons::structs::response::{TMeta, TResponse};
use crate::libs::sea_orm::connect::connect_db;
use crate::libs::sea_orm::schemas::reservation::{ActiveModel, Entity, Model};
use crate::utils::format_response::format_error;
use crate::utils::format_response::format_success;
use axum::Json;
use sea_orm::sqlx::types::chrono;
use sea_orm::{ActiveModelTrait, EntityTrait, PaginatorTrait, Set};
use uuid::Uuid;

pub async fn create_reservation(
    new_reservation: Json<CreateReservation>,
) -> Json<TResponse<Model>> {
    let db = match connect_db().await {
        Ok(db) => db,
        Err(err) => {
            let error_message = format!("Failed to connect to the database: {}", err);
            let response = format_error(error_message);
            return Json(response);
        }
    };

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
    let db = match connect_db().await {
        Ok(db) => db,
        Err(err) => {
            let error_message = format!("Failed to connect to the database: {}", err);
            let response = format_error(error_message);
            return Json(response);
        }
    };

    let paginator = Entity::find().paginate(&db, params.per_page);

    let reservations = match paginator.fetch_page(params.page - 1).await {
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
        page: params.page,
        per_page: params.per_page,
        total: Some(total),
    };

    let response = format_success(reservations, Some(meta));
    Json(response)
}
