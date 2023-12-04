use actix_web::{
    http::{header::ContentType, StatusCode},
    HttpResponse,
};
use serde::Serialize;

use crate::models::commen::ResponseData;

pub fn create_response<T: Serialize>(status: StatusCode, data: ResponseData<T>) -> HttpResponse {
    let mut response = HttpResponse::build(status);
    response.append_header(ContentType::json());
    response.json(data);
    response.finish()
}
