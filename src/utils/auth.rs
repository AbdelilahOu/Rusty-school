use crate::types::config::Config;
use reqwest::{
    self,
    header::{AUTHORIZATION, CONTENT_LENGTH, CONTENT_TYPE},
};
use serde::Deserialize;
use url::Url;

pub type Res<T> = Result<T, String>;
// #1
pub async fn get_google_auth_url(client_id: String, redirect_uri: String) -> Url {
    let mut url = Url::parse("https://accounts.google.com/o/oauth2/v2/auth").unwrap();
    let params = [
        ("response_type", "code"),
        ("client_id", client_id.as_str()),
        ("scope", "email profile"),
        ("redirect_uri", redirect_uri.as_str()),
        ("access_type", "offline"),
        ("prompt", "consent"),
    ];
    let url = url.query_pairs_mut().extend_pairs(&params).finish();
    url.to_owned()
}
// #2
#[derive(Deserialize, Clone, Debug)]
pub struct TokenResponse {
    pub id_token: String,
    pub access_token: String,
    pub token_type: String,
    pub expires_in: i32,
    pub refresh_token: String,
    pub scope: String,
}
pub async fn request_tokens(code: String, conf: Config) -> Res<TokenResponse> {
    // base url
    let mut url = Url::parse("https://oauth2.googleapis.com/token").unwrap();
    // define params
    let params = [
        ("code", code.as_str()),
        ("client_id", conf.client_id.as_str()),
        ("client_secret", conf.client_secret.as_str()),
        ("redirect_uri", conf.redirect_uri.as_str()),
        ("grant_type", "authorization_code"),
    ];
    let url = url.query_pairs_mut().extend_pairs(&params).finish();
    // set headers
    let client = reqwest::Client::new();
    let res = client
        .post(url.as_str())
        .header(CONTENT_TYPE, "application/x-www-form-urlencoded")
        .header(CONTENT_LENGTH, 0)
        .send()
        .await
        .expect("Failed to send request");
    //
    if res.status().is_success() {
        let resp = res
            .json::<TokenResponse>()
            .await
            .expect("Failed to parse response");
        return Ok(resp);
    } else {
        Err("Error getting tokens".to_string())
    }
}
// #3
#[derive(Deserialize, Debug)]
pub struct GoogleUser {
    pub id: String,
    pub email: String,
    pub verified_email: bool,
    pub name: String,
    pub given_name: String,
    pub family_name: String,
    pub picture: String,
    pub locale: String,
}
pub async fn get_google_user(acc_token: String, id_token: String) -> Res<GoogleUser> {
    let mut url = Url::parse("https://www.googleapis.com/oauth2/v1/userinfo").unwrap();
    let params = [("alt", "json"), ("access_token", acc_token.as_str())];
    let url = url.query_pairs_mut().extend_pairs(&params).finish();
    let client = reqwest::Client::new();
    let req = client
        .get(url.as_str())
        .header(AUTHORIZATION, format!("Bearer {}", id_token.as_str()))
        .send()
        .await
        .expect("coudnt get user");
    //
    if req.status().is_success() {
        let resp = req.json::<GoogleUser>().await.expect("coudnt parse user");
        return Ok(resp);
    } else {
        Err("Error getting user".to_string())
    }
}
