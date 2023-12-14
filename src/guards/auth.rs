use crate::utils;
use actix_web::guard::GuardContext;

pub fn check_token(ctx: &GuardContext) -> bool {
    // get headers
    let headers = ctx.head().headers();
    let auth_header = headers.get("Authorization");
    // check if auth header exists
    match auth_header {
        Some(header) => {
            //get header as str value
            let header = header.to_str().unwrap();
            // get token
            let token = header.split(" ").collect::<Vec<&str>>()[1];
            // get auth type
            let authorization_type = header.split(" ").collect::<Vec<&str>>()[0];
            // check if auth type is bearer
            if authorization_type == "Bearer".to_string() {
                let payload_res = utils::token::verify_token(token, "".to_string());
                match payload_res {
                    Ok(_) => return true,
                    Err(_) => return false,
                }
            }
            false
        }
        None => false,
    }
}
