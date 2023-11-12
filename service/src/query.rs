use ::entity::{student, student::Entity as Student};
use sea_orm::*;

pub struct Query;

impl Query {
    pub async fn get_students() -> Result<(), DbErr> {
        Ok(())
    }
}
