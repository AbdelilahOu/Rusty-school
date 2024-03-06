use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Serialize, Debug)]
pub struct CSubject {
    pub name: String,
    pub description: Option<String>,
    pub level_id: Option<Uuid>,
}
