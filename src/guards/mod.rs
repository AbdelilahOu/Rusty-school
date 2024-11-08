use actix_web::http::header::HeaderMap;

use crate::{types::token::Claims, utils::token::verify_token};

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
            let split_header = header.split(" ").collect::<Vec<&str>>();
            if split_header.len() < 2 {
                return Err("auth headers length".to_string());
            }
            let [authorization_type, token] = split_header[..] else {
                return Err("auth headers format".to_string());
            };
            // check if auth type is bearer
            match authorization_type {
                "Bearer" => match verify_token(token, secret) {
                    Ok(payload) => Ok(payload),
                    Err(err) => Err(format!("Invalid token {}", err)),
                },
                _ => Err("auth type unsupported".to_string()),
            }
        }
        None => Err("no auth headers".to_string()),
    }
}

pub fn role_guard(role: String, roles: Vec<&str>) -> bool {
    for allowed_role in roles.into_iter() {
        if allowed_role == role.as_str() {
            return true;
        }
    }
    return false;
}
