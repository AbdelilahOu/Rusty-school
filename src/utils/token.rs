use crate::types::token::Claims;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use service::chrono::{Duration, Utc};
use service::uuid::Uuid;

pub fn generate_tokens(user_id: Uuid, secret: String, duration: Duration) -> (String, Claims) {
    // time
    let current_time = Utc::now();
    let expiration_time = current_time + duration;
    //
    let header = Header::default();
    let session_id = Uuid::new_v4();
    let claims = Claims {
        session_id: session_id.clone(),
        user_id,
        exp: expiration_time.timestamp(),
    };
    let token = encode(&header, &claims, &EncodingKey::from_secret(secret.as_ref()))
        .expect("Failed to generate token");
    (token, claims)
}

pub fn verify_token(token: &str, secret: String) -> Result<Claims, String> {
    let token_res = decode::<Claims>(
        &token,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::new(Algorithm::HS256),
    );
    match token_res {
        Ok(t) => Ok(t.claims),
        Err(e) => Err(e.to_string()),
    }
}
