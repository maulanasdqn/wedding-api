use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
use validator::Validate;

#[derive(Clone, Debug, Serialize, Deserialize, Validate, ToSchema)]
pub struct ReservationDto {
	#[validate(length(min = 1, message = "Full name must not be empty"))]
	pub fullname: String,
	#[validate(length(min = 1, message = "Attendance must not be empty"))]
	pub attendance: String,
	#[validate(length(min = 1, message = "Speech must not be empty"))]
	pub speech: String,
	#[validate(url(message = "Speech audio must be a valid URL"))]
	pub speech_audio: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ReservationListResponse {
	pub data: Vec<ReservationDto>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, IntoParams)]
pub struct TMetas {
	pub page: Option<u32>,
	pub per_page: Option<u32>,
}
