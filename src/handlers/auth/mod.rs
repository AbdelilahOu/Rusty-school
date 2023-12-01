use crate::{
    models::commen::{AuthQuery, ConfigObj, ResultResponse, State},
    utils::{self, get_google_auth_url},
};
use actix_web::{
    http::{header::ContentType, StatusCode},
    HttpResponse,
};
use service::{CUser, ServiceTransaction};

pub async fn login(state: State) -> HttpResponse {
    let url = get_google_auth_url(state.client_id.clone(), state.redirect_uri.clone()).await;
    HttpResponse::Found()
        .append_header(("Location", url.as_str()))
        .status(StatusCode::FOUND)
        .finish()
}

pub async fn google_auth_handler(q: AuthQuery, state: State) -> HttpResponse {
    let res = utils::request_tokens(
        q.code.clone(),
        ConfigObj {
            client_id: state.client_id.clone(),
            client_secret: state.client_secret.clone(),
            redirect_uri: state.redirect_uri.clone(),
        },
    )
    .await;

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
                        Ok(user) => {
                            // create sessions
                            // create access token
                            // create refresh token
                            HttpResponse::Created()
                                .status(StatusCode::CREATED)
                                .json(user)
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
