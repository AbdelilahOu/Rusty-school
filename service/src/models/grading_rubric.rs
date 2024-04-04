use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct CRubric {
    pub title: String,
    pub description: Option<String>,
}
