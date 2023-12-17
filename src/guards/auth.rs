use crate::{models::commen::Claims, utils};
use actix_web::guard::GuardContext;

pub fn check_token(ctx: &GuardContext, secret: String) -> Option<Claims> {
    // get headers
    let headers = ctx.head().headers();
    let auth_header = headers.get("Authorization");
    // check if auth header exists
    match auth_header {
        Some(header) => {
            //get header as str value
            let header = header.to_str().unwrap();
            // check header is valid
            if header.is_empty() {
                println!("header is empty");
                return None;
            }
            // check header
            if header.split(" ").collect::<Vec<&str>>().len() != 2 {
                println!("header is not valid");
                return None;
            }
            // get token
            let token = header.split(" ").collect::<Vec<&str>>()[1];
            // get auth type
            let authorization_type = header.split(" ").collect::<Vec<&str>>()[0];
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
