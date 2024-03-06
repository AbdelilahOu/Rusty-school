use serde::{Deserialize, Serialize};

use super::CPDetails;

pub struct ParentWithAddress {
    pub parent: CParent,
    pub details: CPDetails,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct CParent {
    pub first_name: String,
    pub last_name: String,
}
