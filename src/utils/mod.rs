use crate::models::commen::{ConfigObj, GoogleUser, TokenResponse};
use reqwest::{
    self,
    header::{AUTHORIZATION, CONTENT_LENGTH, CONTENT_TYPE},
};
use url::Url;

type Res<T> = Result<T, reqwest::Error>;

pub async fn get_google_user(acc_token: String, id_token: String) -> Res<GoogleUser> {
    let mut url = Url::parse("https://www.googleapis.com/oauth2/v1/userinfo").unwrap();
    let params = [("alt", "json"), ("access_token", acc_token.as_str())];
    let url = url.query_pairs_mut().extend_pairs(&params).finish();
    let client = reqwest::Client::new();
    let resp = client
        .get(url.as_str())
        .header(AUTHORIZATION, format!("Bearer {}", id_token.as_str()))
        .send()
        .await
        .expect("coudnt get user")
        .json::<GoogleUser>()
        .await;
    println!("{:?}", resp);
    resp
}

pub async fn get_google_tokens(code: String, secrets: ConfigObj) -> Res<TokenResponse> {
    // base url
    let mut url = Url::parse("https://oauth2.googleapis.com/token").unwrap();
    // define params
    let params = [
        ("code", code.as_str()),
        ("client_id", secrets.client_id.as_str()),
        ("client_secret", secrets.client_secret.as_str()),
        ("redirect_uri", secrets.redirect_uri.as_str()),
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
        .expect("Failed to send request")
        .json::<TokenResponse>()
        .await;

    res
}
