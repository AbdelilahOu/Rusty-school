use crate::{types::token::Claims, utils::token::verify_token};
use actix_web::http::header::HeaderMap;

pub fn auth_guard(headers: &HeaderMap, secret: String) -> Option<Claims> {
    // check if auth header exists
    match headers.get("Authorization") {
        Some(header) => {
            //get header as str value
            let header = header.to_str().unwrap_or("");
            // check header is valid
            if header.is_empty() {
                return None;
            }
            // get token
            let splited_header = header.split(" ").collect::<Vec<&str>>();
            if splited_header.len() < 2 {
                return None;
            }
            let [authorization_type, token] = splited_header[..] else {
                return None;
            };
            // check if auth type is bearer
            if authorization_type == "Bearer".to_string() {
                match verify_token(token, secret) {
                    Ok(payload) => return Some(payload),
                    Err(err) => {
                        println!("Invalid token {}", err);
                        return None;
                    }
                }
            }
            return None;
        }
        None => None,
    }
}
