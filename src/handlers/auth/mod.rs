use crate::models::commen::{AuthQuery, State, TokenResponse};
use actix_web::{http::StatusCode, HttpResponse};
use reqwest::{
    self,
    header::{CONTENT_LENGTH, CONTENT_TYPE},
};
use url::Url;

pub async fn login(state: State) -> HttpResponse {
    let mut url = Url::parse("https://accounts.google.com/o/oauth2/v2/auth").unwrap();
    let params = [
        ("response_type", "code"),
        ("client_id", state.client_id.as_str()),
        ("scope", "email profile"),
        ("redirect_uri", state.redirect_uri.as_str()),
        ("access_type", "offline"),
        ("prompt", "consent"),
    ];
    let url = url.query_pairs_mut().extend_pairs(&params).finish();
    HttpResponse::Found()
        .append_header(("Location", url.as_str()))
        .status(StatusCode::FOUND)
        .finish()
}

pub async fn google_auth_handler(q: AuthQuery, state: State) -> HttpResponse {
    println!("{:?}", q);
    // base url
    let mut url = Url::parse("https://oauth2.googleapis.com/token").unwrap();
    // define params
    let params = [
        ("code", q.code.as_str()),
        ("client_id", state.client_id.as_str()),
        ("client_secret", state.client_secret.as_str()),
        ("redirect_uri", state.redirect_uri.as_str()),
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
        .await
        .unwrap();

    println!("{:?}", res);
    match res.access_token {
        Some(token) => println!("{}", token),
        None => println!("No token"),
    }

    HttpResponse::Ok().body("ok")

    // HttpResponse::Found()
    //     .append_header(("Location", "https://callback-template.vercel.app/"))
    //     .status(StatusCode::FOUND)
    //     .finish()
}
