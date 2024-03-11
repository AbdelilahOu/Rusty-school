use crate::models::commen::ConfigObj;
use std::env;

pub fn load_config() -> ConfigObj {
    // load vars
    dotenv::from_filename("app.env").ok();
    //
    // set logging env
    env::set_var("RUST_LOG", "actix_web=info,actix_server=info");
    //
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set.");
    let client_id = env::var("OAUTH_CLIENT_ID").expect("OAUTH_CLIENT_ID must be set");
    let client_secret = env::var("OAUTH_SECRET").expect("OAUTH_SECRET must be set");
    let redirect_uri = env::var("REDIRECT_URL").expect("REDIRECT_URL must be set");
    let jwt_secret = env::var("RANDOM_KEY").expect("RANDOM_KEY must be set");
    let jwt_max_age = env::var("JWT_MAX_AGE")
        .expect("JWT_MAX_AGE must be set")
        .parse::<i64>()
        .expect("Failed to parse JWT_MAX_AGE");
    //
    return ConfigObj {
        client_id,
        client_secret,
        redirect_uri,
        jwt_secret,
        jwt_max_age,
        db_url,
    };
}
