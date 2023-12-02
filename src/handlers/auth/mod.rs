use crate::{
    models::commen::{AuthQuery, ResultResponse, State},
    utils::{self, get_google_auth_url},
};
use actix_web::{
    cookie::{time::Duration, Cookie},
    http::{header::ContentType, StatusCode},
    HttpResponse,
};
use service::{CUser, ServiceTransaction};

pub async fn login(state: State) -> HttpResponse {
    let url =
        get_google_auth_url(state.env.client_id.clone(), state.env.redirect_uri.clone()).await;
    HttpResponse::Found()
        .append_header(("Location", url.as_str()))
        .status(StatusCode::FOUND)
        .finish()
}

pub async fn google_auth_handler(q: AuthQuery, state: State) -> HttpResponse {
    let res = utils::request_tokens(q.code.clone(), state.env.clone()).await;

    match res {
        Ok(res) => {
            let user = utils::get_google_user(res.access_token, res.id_token).await;
            match user {
                Ok(user) => {
                    let user_res = ServiceTransaction::upsert_user(
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
                        Ok(user_uuid) => {
                            // create access token
                            let token = utils::tokens::generate_tokens(
                                user_uuid,
                                state.env.jwt_secret.clone(),
                                state.env.jwt_max_age.clone(),
                            );
                            // create cookie
                            let cookie = Cookie::build("token", token)
                                .path("/")
                                .max_age(Duration::hours(48))
                                .http_only(true)
                                .finish();
                            let mut response = HttpResponse::Found();
                            response.append_header(("Location", "/"));
                            response.cookie(cookie);
                            response.finish()
                        }
                        Err(e) => HttpResponse::InternalServerError()
                            .status(StatusCode::INTERNAL_SERVER_ERROR)
                            .content_type(ContentType::json())
                            .json(ResultResponse::<Option<String>> {
                                error: Some(e.to_string()),
                                message: Some(String::from("coudnt insert user into db")),
                                data: None,
                            }),
                    }
                }
                Err(e) => HttpResponse::InternalServerError()
                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                    .content_type(ContentType::json())
                    .json(ResultResponse::<Option<String>> {
                        error: Some(e.to_string()),
                        message: Some(String::from("coudnt get user profile from google")),
                        data: None,
                    }),
            }
        }
        Err(e) => HttpResponse::InternalServerError()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .content_type(ContentType::json())
            .json(ResultResponse::<Option<String>> {
                error: Some(e.to_string()),
                message: Some(String::from("coudnt get access token from google")),
                data: None,
            }),
    }
}
