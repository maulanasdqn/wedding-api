use rocket::data::ToByteUnit;
use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::serde::json::Json;
use rocket::Data;
use std::fs;
use std::path::PathBuf;
use tokio::io::AsyncReadExt;
use uuid::Uuid;

#[derive(serde::Serialize)]
pub struct UploadResponse {
    pub message: String,
    pub file_url: String,
}

#[post("/api/greetings/upload-audio", data = "<audio_file>")]
pub async fn upload_audio(audio_file: Data<'_>) -> Result<Json<UploadResponse>, Custom<String>> {
    let upload_dir = "uploads/audio";
    fs::create_dir_all(upload_dir).map_err(|e| {
        Custom(
            Status::InternalServerError,
            format!("Failed to create upload directory: {}", e),
        )
    })?;

    let filename = format!("{}.webm", Uuid::new_v4());
    let file_path = PathBuf::from(upload_dir).join(&filename);

    let mut buffer = Vec::new();
    let mut stream = audio_file.open(25.megabytes());
    stream.read_to_end(&mut buffer).await.map_err(|e| {
        Custom(
            Status::InternalServerError,
            format!("Failed to read audio file: {}", e),
        )
    })?;

    tokio::fs::write(&file_path, &buffer).await.map_err(|e| {
        Custom(
            Status::InternalServerError,
            format!("Failed to save audio file: {}", e),
        )
    })?;

    let file_url = format!("/{}", file_path.to_string_lossy());

    Ok(Json(UploadResponse {
        message: "Audio uploaded successfully".to_string(),
        file_url,
    }))
}
