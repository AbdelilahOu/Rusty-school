use crate::{
    models::commen::{AuthQuery, ResponseData, State},
    utils::{
        auth::{get_google_auth_url, get_google_user, request_tokens},
        token::generate_tokens,
    },
};
use actix_web::{
    cookie::{time::Duration, Cookie},
    http::{header::ContentType, StatusCode},
    HttpResponse,
};
use service::{models::CUser, transaction::TransactionsService};

pub async fn login(state: State) -> HttpResponse {
    let url = get_google_auth_url(
        state.config.client_id.clone(),
        state.config.redirect_uri.clone(),
    )
    .await;
    HttpResponse::Found()
        .append_header(("Location", url.as_str()))
        .status(StatusCode::FOUND)
        .finish()
}

pub async fn google_auth_handler(q: AuthQuery, state: State) -> HttpResponse {
    let res = request_tokens(q.code.clone(), state.config.clone()).await;

    match res {
        Ok(res) => {
            let user = get_google_user(res.access_token, res.id_token).await;
            match user {
                Ok(user) => {
                    let user_res = TransactionsService::upsert_user(
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
                            let token = generate_tokens(
                                user_uuid,
                                state.config.jwt_secret.clone(),
                                state.config.jwt_max_age.clone(),
                            );
                            // create cookie
                            let cookie = Cookie::build("token", token.clone())
                                .path("/")
                                .max_age(Duration::hours(state.config.jwt_max_age.clone()))
                                .http_only(true)
                                .finish();
                            let mut response = HttpResponse::Found();
                            response.append_header(("Location", "/"));
                            response.cookie(cookie);
                            response.json(ResponseData::<String> {
                                error: None,
                                message: Some("user logged in successfully".to_string()),
                                data: Some(token.clone()),
                            });
                            response.finish()
                        }
                        Err(e) => HttpResponse::InternalServerError()
                            .status(StatusCode::INTERNAL_SERVER_ERROR)
                            .content_type(ContentType::json())
                            .json(ResponseData::<Option<String>> {
                                error: Some(e.to_string()),
                                message: Some("coudnt insert user into db".to_string()),
                                data: None,
                            }),
                    }
                }
                Err(e) => HttpResponse::InternalServerError()
                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                    .content_type(ContentType::json())
                    .json(ResponseData::<Option<String>> {
                        error: Some(e.to_string()),
                        message: Some("coudnt get user profile from google".to_string()),
                        data: None,
                    }),
            }
        }
        Err(e) => HttpResponse::InternalServerError()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .content_type(ContentType::json())
            .json(ResponseData::<Option<String>> {
                error: Some(e.to_string()),
                message: Some("coudnt get access token from google".to_string()),
                data: None,
            }),
    }
}
