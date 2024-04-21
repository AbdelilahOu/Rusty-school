use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct User {
    pub name: String,
    pub given_name: String,
    pub family_name: String,
    pub email: String,
    pub picture: Option<String>,
}
