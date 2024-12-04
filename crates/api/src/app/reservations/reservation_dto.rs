use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Clone, Debug, Serialize, Deserialize, Validate)]
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
