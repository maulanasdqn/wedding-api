use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct CreateReservation {
    pub fullname: String,
    pub attendance: String,
    pub speech: String,
    pub speech_audio: Option<String>,
}
