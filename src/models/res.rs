use serde::Serialize;

#[derive(Serialize)]
struct ResultResponse<T> {
    error: Option<String>,
    data: Option<T>,
}
