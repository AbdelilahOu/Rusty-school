use serde::Serialize;

#[derive(Serialize)]
pub struct ResultResponse<T> {
    pub error: Option<String>,
    pub message: Option<String>,
    pub data: Option<T>,
}
