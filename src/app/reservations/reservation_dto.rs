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
	pub meta: Option<TMetas>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ReservationDetailResponse {
	pub data: ReservationDto,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, IntoParams)]
pub struct TMetas {
	pub page: Option<u32>,
	pub per_page: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, IntoParams)]
pub struct TMetaParam {
	pub page: Option<u32>,
	pub per_page: Option<u32>,
	pub search: Option<String>,
	pub order_by: Option<String>,
	pub sort: Option<String>,
}
