use crate::commons::structs::response::TMeta;
use crate::commons::structs::response::TResponse;

pub fn format_success<T>(data: T, meta: Option<TMeta>) -> TResponse<T> {
    let res = TResponse::success(meta, data);
    res
}

pub fn format_error<T>(message: String) -> TResponse<T> {
    TResponse::error(message)
}
