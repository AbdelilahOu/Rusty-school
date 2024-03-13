use actix_web::HttpRequest;

pub async fn healthy(req: HttpRequest) -> String {
    println!("Token: {:?}", req.cookie("token"));
    "healthy".to_string()
}
