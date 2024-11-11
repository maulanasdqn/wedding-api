use super::{models, services};
use crate::commons::constants::endpoints;
use axum::{extract::Path, response::IntoResponse, Json};

pub async fn get_all() -> impl IntoResponse {
    services::fetch_authors().await
}

pub async fn get_detail(Path(id): Path<i32>) -> impl IntoResponse {
    services::fetch_author_detail(id).await
}

pub async fn post(Json(new_author): Json<models::Author>) -> impl IntoResponse {
    services::create_author(new_author).await
}

pub fn author_controller() -> axum::Router {
    axum::Router::new()
        .route(endpoints::endpoints::GET_ALL, axum::routing::get(get_all))
        .route(
            endpoints::endpoints::GET_DETAIL,
            axum::routing::get(get_detail),
        )
        .route(endpoints::endpoints::CREATE, axum::routing::post(post))
}
