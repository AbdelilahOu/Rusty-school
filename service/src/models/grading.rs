use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Rubrics {
    pub title: String,
    pub description: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct CRubric {
    pub title: String,
    pub description: Option<String>,
    pub grading_criterias: Option<Vec<CCriteria>>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct CCriteria {
    pub performance: String,
    pub description: Option<String>,
}
