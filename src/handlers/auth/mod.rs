use actix_web::{
    http::header,
    web::{Json, Query},
    HttpRequest as Request, HttpResponse as Response,
};
use serde::{Deserialize, Serialize};

use service::{
    chrono::{DateTime, Duration, NaiveDateTime, Utc},
    models::{Session, User},
    mutation::MutationService,
    query::QueryService,
    transaction::TransactionService,
    uuid::Uuid,
};

use crate::{
    types::shared::{ResponseData, State},
    utils::{
        auth::{get_google_auth_url, get_google_user, request_tokens},
        token::{generate_tokens, verify_token},
    },
};

#[derive(Debug, Serialize, Deserialize)]
struct RefreshAccessResponse {
    pub access_token: String,
    pub access_token_expires_at: NaiveDateTime,
}

//
#[derive(Debug, Serialize, Deserialize)]
pub struct RenewAccess {
    pub refresh_token: String,
}

//
#[derive(Debug, Serialize, Deserialize)]
struct LogInResponse {
    pub session_id: Uuid,
    pub email: String,
    pub full_name: String,
    pub access_token: String,
    pub refresh_token: String,
    pub access_token_expires_at: NaiveDateTime,
    pub refresh_token_expires_at: NaiveDateTime,
}

#[derive(Deserialize, Debug)]
pub struct AuthQuery {
    pub code: String,
}

pub async fn login(state: State) -> Response {
    let url = get_google_auth_url(
        state.config.client_id.clone(),
        state.config.redirect_uri.clone(),
    )
    .await;
    Response::Found()
        .append_header(("Location", url.as_str()))
        .finish()
}

pub async fn renew_access_token(body: Json<RenewAccess>, state: State) -> Response {
    match verify_token(&body.refresh_token, state.config.jwt_secret.clone()) {
        Ok(claims) => match QueryService::get_session(&state.db_conn, claims.session_id).await {
            Ok(session_op) => match session_op {
                Some(session) => {
                    if session.is_blocked {
                        return Response::Unauthorized().json(ResponseData::<String> {
                            error: None,
                            message: Some("this user is blocked".to_string()),
                            data: None,
                        });
                    };

                    if !session.user_id.eq(&claims.user_id) {
                        return Response::Unauthorized().json(ResponseData::<String> {
                            error: None,
                            message: Some("user doesnt match".to_string()),
                            data: None,
                        });
                    };

                    if !session.refresh_token.eq(&body.refresh_token) {
                        return Response::Unauthorized().json(ResponseData::<String> {
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
                        return Response::Unauthorized().json(ResponseData::<String> {
                            error: None,
                            message: Some("refresh token expired".to_string()),
                            data: None,
                        });
                    };

                    let (access_token, claims) = generate_tokens(
                        session.user_id,
                        session.role,
                        state.config.jwt_secret.clone(),
                        Duration::minutes(5),
                    );

                    Response::Ok().json(ResponseData::<RefreshAccessResponse> {
                        error: None,
                        message: Some("access token refreshed".to_string()),
                        data: Some(RefreshAccessResponse {
                            access_token,
                            access_token_expires_at: DateTime::<Utc>::from_timestamp(claims.exp, 0)
                                .unwrap()
                                .naive_utc(),
                        }),
                    })
                }
                None => Response::Unauthorized().json(ResponseData::<String> {
                    error: None,
                    message: Some("no session found".to_string()),
                    data: None,
                }),
            },
            Err(e) => Response::Unauthorized().json(ResponseData::<String> {
                error: None,
                message: Some(e.to_string()),
                data: None,
            }),
        },
        Err(e) => Response::Unauthorized().json(ResponseData::<String> {
            error: None,
            message: Some(e.to_string()),
            data: None,
        }),
    }
}

pub async fn google_auth(req: Request, query: Query<AuthQuery>, state: State) -> Response {
    let res = request_tokens(query.code.clone(), state.config.clone()).await;
    match res {
        Ok(res) => {
            let user = get_google_user(res.access_token, res.id_token).await;
            match user {
                Ok(user) => {
                    let user_res = TransactionService::upsert_user(
                        &state.db_conn,
                        User {
                            name: user.name.clone(),
                            given_name: user.given_name.clone(),
                            family_name: user.family_name.clone(),
                            email: user.email.clone(),
                            picture: Some(user.picture),
                            role: "not_defined".to_string(),
                        },
                    )
                    .await;
                    match user_res {
                        Ok((user, id)) => {
                            // create access token
                            let (access_token, access_claims) = generate_tokens(
                                id,
                                user.role.clone(),
                                state.config.jwt_secret.clone(),
                                Duration::minutes(5),
                            );
                            let (refresh_token, refresh_claims) = generate_tokens(
                                id,
                                user.role,
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
                            let expires_at = DateTime::<Utc>::from_timestamp(refresh_claims.exp, 0)
                                .unwrap()
                                .naive_utc();
                            let create_session_res = MutationService::create_session(
                                &state.db_conn,
                                Session {
                                    id: refresh_claims.session_id,
                                    user_id: id,
                                    user_agent,
                                    client_ip,
                                    is_blocked: false,
                                    refresh_token: refresh_token.clone(),
                                    expires_at,
                                },
                            )
                            .await;
                            match create_session_res {
                                Ok(session_id) => {
                                    let access_token_expires_at =
                                        DateTime::<Utc>::from_timestamp(access_claims.exp, 0)
                                            .unwrap()
                                            .naive_utc();
                                    let refresh_token_expires_at =
                                        DateTime::<Utc>::from_timestamp(refresh_claims.exp, 0)
                                            .unwrap()
                                            .naive_utc();
                                    Response::Ok().json(ResponseData::<LogInResponse> {
                                        error: None,
                                        message: Some("user logged in successfully".to_string()),
                                        data: Some(LogInResponse {
                                            session_id,
                                            email: user.email,
                                            full_name: user.name,
                                            access_token,
                                            refresh_token,
                                            access_token_expires_at,
                                            refresh_token_expires_at,
                                        }),
                                    })
                                }
                                Err(e) => Response::InternalServerError().json(ResponseData::<
                                    Option<String>,
                                > {
                                    error: Some(e.to_string()),
                                    message: Some("couldn't create session".to_string()),
                                    data: None,
                                }),
                            }
                        }
                        Err(e) => Response::InternalServerError().json(ResponseData::<String> {
                            error: Some(e.to_string()),
                            message: Some("couldn't insert user into db".to_string()),
                            data: None,
                        }),
                    }
                }
                Err(e) => Response::InternalServerError().json(ResponseData::<String> {
                    error: Some(e.to_string()),
                    message: Some("couldn't get user profile from google".to_string()),
                    data: None,
                }),
            }
        }
        Err(e) => Response::InternalServerError().json(ResponseData::<String> {
            error: Some(e.to_string()),
            message: Some("couldn't get access token from google".to_string()),
            data: None,
        }),
    }
}
