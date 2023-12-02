use crate::models::commen::ConfigObj;

pub fn load_config() -> ConfigObj {
    // load vars
    let _ = dotenv::dotenv();
    //
    let client_id = std::env::var("OAUTH_CLIENT_ID").expect("OAUTH_CLIENT_ID must be set");
    let client_secret = std::env::var("OAUTH_SECRET").expect("OAUTH_SECRET must be set");
    let redirect_uri = std::env::var("REDIRECT_URL").expect("REDIRECT_URL must be set");
    let jwt_secret = std::env::var("RANDOM_KEY").expect("RANDOM_KEY must be set");
    //
    return ConfigObj {
        client_id,
        client_secret,
        redirect_uri,
        jwt_secret,
    };
}
