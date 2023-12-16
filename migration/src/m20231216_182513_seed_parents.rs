use sea_orm_migration::prelude::*;

use crate::{m20231116_165911_c_parents::Parent, utils::generate_random_parent};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        for _ in 1..=100 {
            let parent = generate_random_parent();
            let insert = Query::insert()
                .into_table(Parent::Table)
                .columns([Parent::FirstName, Parent::LastName])
                .values_panic([parent.first_name.into(), parent.last_name.into()])
                .to_owned();

            manager.exec_stmt(insert).await?;
        }

        Ok(())
    }

    async fn down(&self, _: &SchemaManager) -> Result<(), DbErr> {
        Ok(())
    }
}
