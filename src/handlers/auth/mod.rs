use crate::{
    models::commen::{AuthQuery, State, TokenResponse},
    utils,
};
use actix_web::{http::StatusCode, HttpResponse};
use reqwest::{
    self,
    header::{CONTENT_LENGTH, CONTENT_TYPE},
};
use service::{CUser, ServiceMutation};
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

    match (res.access_token, res.id_token) {
        (Some(access_token), Some(id_token)) => {
            let user = utils::get_google_user(access_token, id_token).await;
            let user_res = ServiceMutation::create_user(
                &state.db_conn,
                CUser {
                    first_name: user.name,
                    last_name: user.family_name,
                    email: user.email,
                    picture: Some(user.picture),
                },
            )
            .await;
            match user_res {
                Ok(user) => HttpResponse::Created()
                    .status(StatusCode::CREATED)
                    .json(user),
                Err(e) => HttpResponse::InternalServerError()
                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                    .json(e.to_string()),
            }
        }
        (None, None) => HttpResponse::InternalServerError()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .finish(),

        _ => HttpResponse::InternalServerError()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .finish(),
    }
}
