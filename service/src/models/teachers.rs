use serde::{Deserialize, Serialize};

use super::CPDetails;

pub struct TeacherWithAddress {
    pub teacher: CTeacher,
    pub details: CPDetails,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct CTeacher {
    pub first_name: String,
    pub last_name: String,
}
