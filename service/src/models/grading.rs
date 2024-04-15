use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Rubric {
    pub title: String,
    pub description: Option<String>,
    pub grading_criterias: Option<Vec<Criteria>>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Criteria {
    pub performance: String,
    pub description: Option<String>,
}
