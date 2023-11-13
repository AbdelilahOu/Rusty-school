use serde::Serialize;

#[derive(Serialize)]
struct ResultResponse<T: for<'de> Serialize<'de>> {
    error: Option<String>,
    data: Option<T>,
}
