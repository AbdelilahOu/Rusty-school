use serde::{Deserialize, Serialize};
use service::uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub session_id: Uuid,
    pub user_id: Uuid,
    pub exp: i64,
}
