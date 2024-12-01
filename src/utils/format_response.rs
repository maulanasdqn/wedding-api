use crate::commons::structs::response::TMeta;
use crate::commons::structs::response::TResponse;

pub fn format_success<T>(data: T, meta: Option<TMeta>) -> TResponse<T> {
    TResponse::success(meta, data)
}

pub fn format_error<T>(message: String) -> TResponse<T> {
    TResponse::error(message)
}
