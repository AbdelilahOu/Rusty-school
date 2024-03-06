use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::CPDetails;

#[derive(Deserialize, Serialize, Debug)]
pub struct CStudent {
    pub first_name: String,
    pub last_name: String,
    pub group_id: Option<Uuid>,
}

pub struct StudentWithAddress {
    pub student: CStudent,
    pub details: CPDetails,
}
