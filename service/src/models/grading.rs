use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Rubric {
    pub title: String,
    pub description: Option<String>,
    pub grading_criterias: Option<Vec<Criteria>>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct RubricQueries {
    pub page: u64,
    pub limit: u64,
    pub title: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Criteria {
    pub performance: String,
    pub description: Option<String>,
}
