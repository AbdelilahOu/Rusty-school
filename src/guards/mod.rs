use crate::{types::token::Claims, utils::token::verify_token};
use actix_web::http::header::HeaderMap;

pub fn auth_guard(headers: &HeaderMap, secret: String) -> Result<Claims, String> {
    // check if auth header exists
    match headers.get("Authorization") {
        Some(header) => {
            //get header as str value
            let header = header.to_str().unwrap_or("");
            // check header is valid
            if header.is_empty() {
                return Err("auth headers empty".to_string());
            }
            // get token
            let splited_header = header.split(" ").collect::<Vec<&str>>();
            if splited_header.len() < 2 {
                return Err("auth headers length".to_string());
            }
            let [authorization_type, token] = splited_header[..] else {
                return Err("auth headers format".to_string());
            };
            // check if auth type is bearer
            if authorization_type == "Bearer".to_string() {
                match verify_token(token, secret) {
                    Ok(payload) => return Ok(payload),
                    Err(err) => return Err(format!("Invalid token {}", err)),
                }
            }
            return Err("auth type unsupported".to_string());
        }
        None => return Err("no auth headers".to_string()),
    }
}
