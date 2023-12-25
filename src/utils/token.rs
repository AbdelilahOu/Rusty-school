use super::auth::Res;
use crate::models::commen::Claims;
use chrono::Utc;
use jsonwebtoken::{encode, EncodingKey, Header};
use uuid::Uuid;

pub fn generate_tokens(user_uuid: Uuid, secret: String, age: i64) -> String {
    // time
    let current_time = Utc::now();
    let expiration_time = current_time + chrono::Duration::hours(age);
    //
    let header = Header::default();
    let token = encode(
        &header,
        &Claims {
            user_uuid,
            exp: expiration_time.timestamp() as usize,
            sub: "user".to_string(),
        },
        &EncodingKey::from_secret(secret.as_ref()),
    )
    .expect("Failed to generate token");
    token
}

pub fn verify_token(token: &str, secret: String) -> Res<Claims> {
    let token_res = jsonwebtoken::decode::<Claims>(
        &token,
        &jsonwebtoken::DecodingKey::from_secret(secret.as_ref()),
        &jsonwebtoken::Validation::new(jsonwebtoken::Algorithm::HS256),
    );
    match token_res {
        Ok(t) => Ok(t.claims),
        Err(e) => Err(e.to_string()),
    }
}
