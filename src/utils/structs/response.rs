use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Deserialize, Clone, Debug, Serialize, ToSchema)]
pub struct TMeta {
	pub page: Option<u32>,
	pub per_page: Option<u32>,
	pub total: Option<u64>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MessageResponse {
	pub message: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TResponse<T> {
	pub meta: Option<TMeta>,
	pub data: Option<T>,
}
