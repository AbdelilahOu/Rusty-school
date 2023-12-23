use sea_orm_migration::prelude::*;

use crate::{m20231113_170500_c_teachers::Teacher, utils::generate_random_teacher};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        for _ in 1..=50 {
            let teacher = generate_random_teacher();
            let insert = Query::insert()
                .into_table(Teacher::Table)
                .columns([Teacher::FirstName, Teacher::LastName])
                .values_panic([teacher.first_name.into(), teacher.last_name.into()])
                .to_owned();

            manager.exec_stmt(insert).await?;
        }
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let delete_query = Query::delete().from_table(Teacher::Table).to_owned();
        manager.exec_stmt(delete_query).await?;
        Ok(())
    }
}
