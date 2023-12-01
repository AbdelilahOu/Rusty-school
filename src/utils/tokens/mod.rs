use chrono::Utc;
use jsonwebtoken::{encode, EncodingKey, Header};
use sea_orm::Value;
use uuid::Uuid;

use crate::models::commen::Claims;

pub fn generate_tokens(id: Uuid) -> String {
    //
    let _ = dotenv::dotenv();
    // time
    let current_time = Utc::now();
    let expiration_time = current_time + chrono::Duration::hours(24);
    //
    let secret = std::env::var("RANDOM_KEY").unwrap();
    let mut header = Header::new(jsonwebtoken::Algorithm::HS512);
    let token = encode(
        &header,
        &Claims {
            uuid: id,
            exp: expiration_time.timestamp() as usize,
            sub: "user".to_string(),
        },
        &EncodingKey::from_secret(secret.as_ref()),
    )
    .expect("Failed to generate token");
    token
}

pub fn verify_token(token: String) {}
