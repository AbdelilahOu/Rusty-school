use crate::{
    models::{
        auth::{AuthQuery, LogInResponse, RefreshAccessResponse, RefreshBody},
        commen::{ResponseData, State},
    },
    utils::{
        auth::{get_google_auth_url, get_google_user, request_tokens},
        token::{generate_tokens, verify_token},
    },
};
use actix_web::{
    http::{
        header::{self, ContentType},
        StatusCode,
    },
    HttpRequest, HttpResponse,
};
use service::{
    chrono::{Duration, NaiveDateTime, Utc},
    models::{CSession, CUser},
    mutation::MutationsService,
    query::QueriesService,
    transaction::TransactionsService,
};

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

pub async fn renew_access_token(body: RefreshBody, state: State) -> HttpResponse {
    match verify_token(&body.refresh_token, state.config.jwt_secret.clone()) {
        Ok(claims) => match QueriesService::get_session(&state.db_conn, claims.session_id).await {
            Ok(session_op) => match session_op {
                Some(session) => {
                    if session.is_blocked {
                        return HttpResponse::Unauthorized()
                            .status(StatusCode::UNAUTHORIZED)
                            .content_type(ContentType::json())
                            .json(ResponseData::<String> {
                                error: None,
                                message: Some("this user is blocked".to_string()),
                                data: None,
                            });
                    };

                    if !session.user_id.eq(&claims.user_id) {
                        return HttpResponse::Unauthorized()
                            .status(StatusCode::UNAUTHORIZED)
                            .content_type(ContentType::json())
                            .json(ResponseData::<String> {
                                error: None,
                                message: Some("user doesnt match".to_string()),
                                data: None,
                            });
                    };

                    if !session.refresh_token.eq(&body.refresh_token) {
                        return HttpResponse::Unauthorized()
                            .status(StatusCode::UNAUTHORIZED)
                            .content_type(ContentType::json())
                            .json(ResponseData::<String> {
                                error: None,
                                message: Some("refersh token doesnt match".to_string()),
                                data: None,
                            });
                    };
                    let now = Utc::now();
                    if session
                        .expires_at
                        .lt(&NaiveDateTime::new(now.date_naive(), now.time()))
                    {
                        return HttpResponse::Unauthorized()
                            .status(StatusCode::UNAUTHORIZED)
                            .content_type(ContentType::json())
                            .json(ResponseData::<String> {
                                error: None,
                                message: Some("refresh token expired".to_string()),
                                data: None,
                            });
                    };

                    let (access_token, access_expires_at, _) = generate_tokens(
                        session.user_id,
                        state.config.jwt_secret.clone(),
                        Duration::minutes(5),
                    );

                    HttpResponse::Ok()
                        .status(StatusCode::OK)
                        .content_type(ContentType::json())
                        .json(ResponseData::<RefreshAccessResponse> {
                            error: None,
                            message: Some("access token refreshed".to_string()),
                            data: Some(RefreshAccessResponse {
                                access_token,
                                access_token_expires_at: access_expires_at,
                            }),
                        })
                }
                None => HttpResponse::Unauthorized()
                    .status(StatusCode::UNAUTHORIZED)
                    .content_type(ContentType::json())
                    .json(ResponseData::<String> {
                        error: None,
                        message: Some("no session found".to_string()),
                        data: None,
                    }),
            },
            Err(e) => HttpResponse::Unauthorized()
                .status(StatusCode::UNAUTHORIZED)
                .content_type(ContentType::json())
                .json(ResponseData::<String> {
                    error: None,
                    message: Some(e.to_string()),
                    data: None,
                }),
        },
        Err(e) => HttpResponse::Unauthorized()
            .status(StatusCode::UNAUTHORIZED)
            .content_type(ContentType::json())
            .json(ResponseData::<String> {
                error: None,
                message: Some(e.to_string()),
                data: None,
            }),
    }
}

pub async fn google_auth_handler(req: HttpRequest, q: AuthQuery, state: State) -> HttpResponse {
    let res = request_tokens(q.code.clone(), state.config.clone()).await;
    match res {
        Ok(res) => {
            let user = get_google_user(res.access_token, res.id_token).await;
            match user {
                Ok(user) => {
                    let user_res = TransactionsService::upsert_user(
                        &state.db_conn,
                        CUser {
                            first_name: user.name.clone(),
                            last_name: "".to_string(),
                            email: user.email.clone(),
                            picture: Some(user.picture),
                        },
                    )
                    .await;
                    match user_res {
                        Ok(user_uuid) => {
                            // create access token
                            let (access_token, access_expires_at, _) = generate_tokens(
                                user_uuid,
                                state.config.jwt_secret.clone(),
                                Duration::minutes(5),
                            );
                            let (refresh_token, refresh_expires_at, session_id) = generate_tokens(
                                user_uuid,
                                state.config.jwt_secret.clone(),
                                Duration::hours(48),
                            );
                            // create session
                            let (user_agent, client_ip) =
                                match (req.headers().get(header::USER_AGENT), req.peer_addr()) {
                                    (Some(agent), Some(ip)) => (
                                        agent.to_str().unwrap_or("").to_string(),
                                        ip.ip().to_string(),
                                    ),
                                    _ => ("".to_string(), "".to_string()),
                                };
                            let create_session_res = MutationsService::create_session(
                                &state.db_conn,
                                CSession {
                                    id: session_id,
                                    user_id: user_uuid,
                                    user_agent,
                                    client_ip,
                                    is_blocked: false,
                                    refresh_token: refresh_token.clone(),
                                    expires_at: refresh_expires_at,
                                },
                            )
                            .await;
                            match create_session_res {
                                Ok(session_id) => HttpResponse::Ok()
                                    .status(StatusCode::OK)
                                    .content_type(ContentType::json())
                                    .json(ResponseData::<LogInResponse> {
                                        error: None,
                                        message: Some("user logged in successfully".to_string()),
                                        data: Some(LogInResponse {
                                            session_id,
                                            email: user.email,
                                            fullname: format!("{}", user.name),
                                            access_token,
                                            refresh_token,
                                            access_token_expires_at: access_expires_at,
                                            refresh_token_expires_at: refresh_expires_at,
                                        }),
                                    }),
                                Err(e) => HttpResponse::InternalServerError()
                                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                                    .content_type(ContentType::json())
                                    .json(ResponseData::<Option<String>> {
                                        error: Some(e.to_string()),
                                        message: Some("coudnt create session".to_string()),
                                        data: None,
                                    }),
                            }
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
