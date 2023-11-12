use ::entity::{student, student::Entity as Student};
use sea_orm::*;

pub struct Mutation;

impl Mutation {
    pub async fn create_student() -> Result<(), DbErr> {
        Ok(())
    }
}
