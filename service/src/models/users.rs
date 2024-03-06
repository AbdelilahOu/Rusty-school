use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct CUser {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub picture: Option<String>,
}
