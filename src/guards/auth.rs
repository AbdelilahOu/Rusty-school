use crate::utils;
use actix_web::guard::GuardContext;

pub fn check_token(ctx: &GuardContext, secret: String) -> bool {
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
                return false;
            }
            // check header
            if header.split(" ").collect::<Vec<&str>>().len() != 2 {
                println!("header is not valid");
                return false;
            }
            // get token
            let token = header.split(" ").collect::<Vec<&str>>()[1];
            // get auth type
            let authorization_type = header.split(" ").collect::<Vec<&str>>()[0];
            // check if auth type is bearer
            if authorization_type == "Bearer".to_string() {
                let payload_res = utils::token::verify_token(token, secret);
                match payload_res {
                    Ok(_) => {
                        println!("Valid token");
                        return true;
                    }
                    Err(err) => {
                        println!("Invalid token {}", err);
                        return false;
                    }
                }
            }
            println!("Unsupported auth type");
            false
        }
        None => {
            println!("No auth header");
            return false;
        }
    }
}