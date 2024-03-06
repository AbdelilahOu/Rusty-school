use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Serialize, Debug)]
pub struct CPDetails {
    pub email: Option<String>,
    pub phone: Option<String>,
    pub country_id: Option<Uuid>,
    pub state_id: Option<Uuid>,
    pub city_id: Option<Uuid>,
    pub district_id: Option<Uuid>,
    pub street_id: Option<Uuid>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct CCountry {
    pub name: String,
    pub initials: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct CState {
    pub name: String,
    pub initials: Option<String>,
    pub country_id: Option<Uuid>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct CCity {
    pub name: String,
    pub state_id: Option<Uuid>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct CDistrict {
    pub name: String,
    pub city_id: Option<Uuid>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct CStreet {
    pub name: String,
    pub zip_code: Option<String>,
    pub street_type: Option<String>,
    pub district_id: Option<Uuid>,
}
