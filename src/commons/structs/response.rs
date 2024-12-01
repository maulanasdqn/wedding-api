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
pub struct TResponse<T> {
    pub meta: Option<TMeta>,
    pub data: Option<T>,
    pub error: Option<MessageResponse>,
}

impl<T> TResponse<T> {
    pub fn success(meta: Option<TMeta>, data: T) -> Self {
        TResponse {
            meta,
            data: Some(data),
            error: None,
        }
    }

    pub fn error(message: String) -> Self {
        TResponse {
            meta: None,
            data: None,
            error: Some(MessageResponse { message }),
        }
    }
}
