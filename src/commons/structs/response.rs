use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TMeta {
    pub page: u64,
    pub per_page: u64,
    pub total: Option<u64>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MessageResponse {
    pub message: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum TResponse<T> {
    Success { meta: Option<TMeta>, data: T },
    Error { error: MessageResponse },
}

impl<T> TResponse<T> {
    pub fn success(meta: Option<TMeta>, data: T) -> Self {
        TResponse::Success { meta, data }
    }

    pub fn error(message: String) -> Self {
        TResponse::Error {
            error: MessageResponse { message },
        }
    }
}
