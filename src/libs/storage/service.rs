use bytes::Bytes;
use log::info;
use minio_rsc::client::Minio;
use minio_rsc::error::Error;
use minio_rsc::provider::StaticProvider;
use std::env;
use std::sync::Arc;
use uuid::Uuid;

pub struct MinioClient {
	client: Arc<Minio>,
	bucket_name: String,
}

impl MinioClient {
	pub async fn new() -> Result<Self, Box<dyn std::error::Error>> {
		let bucket_name =
			env::var("MINIO_BUCKET_NAME").expect("MINIO_BUCKET_NAME not set");
		let endpoint = env::var("MINIO_ENDPOINT").expect("MINIO_ENDPOINT not set");
		let access_key =
			env::var("MINIO_ACCESS_KEY").expect("MINIO_ACCESS_KEY not set");
		let secret_key =
			env::var("MINIO_SECRET_KEY").expect("MINIO_SECRET_KEY not set");

		let provider = StaticProvider::new(&access_key, &secret_key, None);
		let client = Minio::builder()
			.endpoint(&endpoint)
			.provider(provider)
			.secure(true)
			.build()?;

		if !client.bucket_exists(&bucket_name).await? {
			client.make_bucket(&bucket_name, false).await?;
			info!("Bucket `{}` created successfully.", bucket_name);
		}

		Ok(Self {
			client: Arc::new(client),
			bucket_name,
		})
	}
	pub async fn upload_file(
		&self,
		original_filename: &str,
		data: Vec<u8>,
	) -> Result<String, Error> {
		let sanitized_filename = original_filename
			.chars()
			.filter(|c| c.is_alphanumeric() || *c == '.' || *c == '_' || *c == '-')
			.collect::<String>();

		let file_ext = std::path::Path::new(&sanitized_filename)
			.extension()
			.and_then(|ext| ext.to_str())
			.unwrap_or("webm");

		let unique_filename = format!(
			"{}-{}.{}",
			Uuid::new_v4(),
			sanitized_filename.replace('.', "_"),
			file_ext
		);

		info!(
			"Uploading file: {} ({})",
			unique_filename,
			data.len() / 1024
		);

		let endpoint =
			env::var("MINIO_ENDPOINT").expect("MINIO_ENDPOINT must be set");

		self.client
			.put_object(&self.bucket_name, &unique_filename, Bytes::from(data))
			.await?;

		let file_url = format!(
			"https://{}/{}/{}",
			endpoint, self.bucket_name, unique_filename
		);

		Ok(file_url)
	}
}
