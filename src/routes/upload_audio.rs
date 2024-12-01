use aws_config::{BehaviorVersion, Region};
use aws_sdk_s3::config::Credentials;
use aws_sdk_s3::primitives::ByteStream;
use aws_sdk_s3::Client;
use rocket::data::ToByteUnit;
use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::serde::json::Json;
use rocket::Data;
use std::env;
use tokio::io::AsyncReadExt;
use uuid::Uuid;

#[derive(serde::Serialize)]
pub struct UploadResponse {
    pub message: String,
    pub file_url: String,
}

#[post("/api/greetings/upload-audio", data = "<audio_file>")]
pub async fn upload_audio(audio_file: Data<'_>) -> Result<Json<UploadResponse>, Custom<String>> {
    let bucket_name = env::var("S3_BUCKET_NAME").map_err(|_| {
        Custom(
            Status::InternalServerError,
            "Missing S3_BUCKET_NAME environment variable".to_string(),
        )
    })?;

    let access_key = env::var("S3_ACCESS_KEY").map_err(|_| {
        Custom(
            Status::InternalServerError,
            "Missing AWS_ACCESS_KEY_ID environment variable".to_string(),
        )
    })?;

    let secret_key = env::var("S3_SECRET_KEY").map_err(|_| {
        Custom(
            Status::InternalServerError,
            "Missing AWS_SECRET_ACCESS_KEY environment variable".to_string(),
        )
    })?;

    let s3_endpoint = "https://s3.nevaobjects.id";
    let custom_region = Region::new("id-jkt-1");

    let credentials = Credentials::new(access_key, secret_key, None, None, "static-credentials");

    let s3_config = aws_sdk_s3::Config::builder()
        .behavior_version(BehaviorVersion::latest())
        .region(custom_region)
        .endpoint_url(s3_endpoint)
        .credentials_provider(credentials)
        .build();

    let client = Client::from_conf(s3_config);

    let filename = format!("audio/{}.webm", Uuid::new_v4());

    let mut buffer = Vec::new();
    let mut stream = audio_file.open(25.megabytes());
    stream.read_to_end(&mut buffer).await.map_err(|e| {
        Custom(
            Status::InternalServerError,
            format!("Failed to read audio file: {}", e),
        )
    })?;

    // Upload to S3
    match client
        .put_object()
        .bucket(&bucket_name)
        .key(&filename)
        .body(ByteStream::from(buffer))
        .content_type("audio/webm")
        .send()
        .await
    {
        Ok(_) => {
            let file_url = format!("{}/{}/{}", s3_endpoint, bucket_name, filename);
            Ok(Json(UploadResponse {
                message: "Audio uploaded successfully".to_string(),
                file_url,
            }))
        }
        Err(e) => {
            eprintln!("Detailed S3 upload error: {:?}", e);
            Err(Custom(
                Status::InternalServerError,
                format!("S3 upload error: {}", e),
            ))
        }
    }
}
