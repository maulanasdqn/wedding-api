#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct TMeta {
    pub page: u32,
    pub per_page: u32,
    pub total: u32,
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
