use axum::Json;

pub async fn create_student(Json(payload): Json<String>) {}
pub async fn update_student(Json(payload): Json<String>) {}
pub async fn delete_student(Json(payload): Json<String>) {}
pub async fn list_students(Json(payload): Json<String>) {}
pub async fn get_student(Json(payload): Json<String>) {}
