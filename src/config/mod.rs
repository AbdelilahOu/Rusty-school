pub struct ConfigObj {
    pub client_id: String,
    pub client_secret: String,
    pub redirect_uri: String,
}

pub fn load_config() -> ConfigObj {
    // load vars
    dotenv::from_filename(".env").ok();
    //
    let client_id = std::env::var("CLIENT_ID").expect("CLIENT_ID must be set");
    let client_secret = std::env::var("CLIENT_SECRET").expect("CLIENT_SECRET must be set");
    let redirect_uri = std::env::var("REDIRECT_URI").expect("REDIRECT_URI must be set");
    //
    return ConfigObj {
        client_id,
        client_secret,
        redirect_uri,
    };
}
