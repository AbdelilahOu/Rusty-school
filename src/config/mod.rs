use std::env;

use crate::types::config::Config;

pub fn load_config() -> Config {
    // load vars
    dotenv::dotenv().ok();
    // set logging env
    env::set_var("RUST_LOG", "actix_web=info,actix_server=info");
    //
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let client_id = env::var("OAUTH_CLIENT_ID").expect("OAUTH_CLIENT_ID must be set");
    let client_secret = env::var("OAUTH_SECRET").expect("OAUTH_SECRET must be set");
    let redirect_uri = env::var("REDIRECT_URL").expect("REDIRECT_URL must be set");
    let jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    //
    return Config {
        client_id,
        client_secret,
        redirect_uri,
        jwt_secret,
        db_url,
    };
}
