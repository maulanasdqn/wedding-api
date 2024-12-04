use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TMeta {
	pub page: Option<u64>,
	pub per_page: Option<u64>,
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

impl Default for TMeta {
	fn default() -> Self {
		TMeta {
			page: Some(1),
			per_page: Some(10),
			total: None,
		}
	}
}
