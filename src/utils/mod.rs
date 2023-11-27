use reqwest::{self, header::AUTHORIZATION};
use url::Url;

use crate::models::commen::GoogleUser;

pub async fn get_google_user(acc_token: String, id_token: String) -> GoogleUser {
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
        .await
        .unwrap();
    println!("{:?}", resp);
    resp
}
