use sea_orm_migration::prelude::*;

use crate::{m20231109_190937_c_students::Student, utils::generate_random_student};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        for _ in 1..=200 {
            let student = generate_random_student();
            let insert = Query::insert()
                .into_table(Student::Table)
                .columns([Student::FirstName, Student::LastName])
                .values_panic([student.first_name.into(), student.last_name.into()])
                .to_owned();

            manager.exec_stmt(insert).await?;
        }

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let delete_query = Query::delete().from_table(Student::Table).to_owned();
        manager.exec_stmt(delete_query).await?;
        Ok(())
    }
}
