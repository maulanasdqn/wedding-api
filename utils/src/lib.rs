pub mod jwt;
pub mod password;
pub mod structs;

use serde_json::json;
use serde_json::Value;
use structs::response::TMeta;

pub fn format_success<T: serde::Serialize>(data: T, meta: Option<TMeta>) -> Value {
	match meta {
		Some(meta) => json!({
			"meta": {
				"page": meta.page,
				"per_page": meta.per_page,
				"total": meta.total
			},
			"data": data
		}),
		None => json!({ "data": data }),
	}
}

pub fn format_error(message: &str) -> Value {
	json!({ "message": message })
}
