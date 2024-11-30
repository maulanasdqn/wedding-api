#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct TMeta {
    pub page: i64,
    pub per_page: i64,
    pub total: i64,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct TResponse<T> {
    pub meta: TMeta,
    pub data: T,
}

impl<T> TResponse<T> {
    pub fn new(meta: TMeta, data: T) -> Self {
        TResponse { meta, data }
    }
}

pub fn format_response<T>(data: T, meta: TMeta) -> TResponse<T> {
    TResponse::new(meta, data)
}
