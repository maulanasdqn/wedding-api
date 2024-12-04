use structs::response::{MessageResponse, TMeta, TResponse};

pub mod structs;

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

pub fn format_success<T>(data: T, meta: Option<TMeta>) -> TResponse<T> {
	TResponse::success(meta, data)
}

pub fn format_error<T>(message: String) -> TResponse<T> {
	TResponse::error(message)
}
