use serde::Serialize;

#[derive(Serialize)]
pub struct SuccessResponse {
    pub message: String,
}
