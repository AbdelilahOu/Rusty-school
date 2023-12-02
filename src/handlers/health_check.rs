use actix_web::HttpRequest;

pub async fn healthy(req: HttpRequest) -> String {
    println!("Token: {:?}", req.cookie("token"));
    String::from("healthy")
}
