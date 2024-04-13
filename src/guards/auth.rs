use crate::{models::auth::Claims, utils};
use actix_web::http::header::HeaderMap;

pub fn check_token(headers: &HeaderMap, secret: String) -> Option<Claims> {
    // get headers
    let auth_header = headers.get("Authorization");
    // check if auth header exists
    match auth_header {
        Some(header) => {
            //get header as str value
            let header = header.to_str().unwrap_or("");
            // check header is valid
            if header.is_empty() {
                println!("header is empty");
                return None;
            }
            // get token
            let [authorization_type, token] = header.split(" ").collect::<Vec<&str>>()[..] else {
                println!("header is not valid");
                return None;
            };
            // check if auth type is bearer
            if authorization_type == "Bearer".to_string() {
                let payload_res = utils::token::verify_token(token, secret);
                match payload_res {
                    Ok(payload) => {
                        println!("Valid token");
                        return Some(payload);
                    }
                    Err(err) => {
                        println!("Invalid token {}", err);
                        return None;
                    }
                }
            }
            println!("Unsupported auth type");
            return None;
        }
        None => {
            println!("No auth header");
            return None;
        }
    }
}
