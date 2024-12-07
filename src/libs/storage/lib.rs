mod service;

use axum::{
	extract::{Multipart, State},
	http::StatusCode,
	response::{IntoResponse, Json},
	routing::post,
	Router,
};
use log::error;
use serde_json::json;
use service::MinioClient;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Clone)]
struct AppState {
	minio: Arc<Mutex<MinioClient>>,
}

async fn upload_file_handler(
	State(state): State<AppState>,
	mut multipart: Multipart,
) -> Result<impl IntoResponse, StatusCode> {
	while let Some(field) = multipart
		.next_field()
		.await
		.map_err(|_| StatusCode::BAD_REQUEST)?
	{
		if let Some(file_name) = field.file_name() {
			let file_name = file_name.to_string();
			let data = field.bytes().await.map_err(|_| StatusCode::BAD_REQUEST)?;

			let minio_client = state.minio.lock().await;

			match minio_client.upload_file(&file_name, data.to_vec()).await {
				Ok(file_url) => {
					let response = json!({ "file_url": file_url });
					return Ok((StatusCode::OK, Json(response)));
				}
				Err(e) => {
					error!("Failed to upload file: {}", e);
					return Err(StatusCode::INTERNAL_SERVER_ERROR);
				}
			}
		}
	}

	Err(StatusCode::BAD_REQUEST)
}

async fn upload_state() -> AppState {
	let minio_client = MinioClient::new()
		.await
		.expect("Failed to create MinIO client");

	let shared_state = AppState {
		minio: Arc::new(Mutex::new(minio_client)),
	};

	shared_state
}

pub async fn upload_routes() -> Router {
	Router::new().route(
		"/upload",
		post(upload_file_handler).with_state(upload_state().await),
	)
}
