use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use validator::Validate;

#[derive(Serialize, Deserialize, Clone, Validate, FromRow)]
pub struct Greeting {
    pub id: Option<i32>,

    #[validate(length(min = 1, message = "Fullname must not be empty"))]
    pub fullname: String,

    #[validate(length(min = 1, message = "Attendance must not be empty"))]
    pub attendence: String,

    #[validate(length(min = 1, message = "Greeting must not be empty"))]
    pub greeting: String,

    #[validate(url(message = "Greeting audio must be a valid URL"))]
    pub greeting_audio: String,
}
