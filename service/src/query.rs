// use ::entity::{student, student::Entity as Student};
use sea_orm::{prelude::Uuid, *};

pub struct Query;

impl Query {
    pub async fn list_students() -> Result<(), DbErr> {
        Ok(())
    }
    pub async fn get_student(_id: Uuid) -> Result<(), DbErr> {
        Ok(())
    }
}
