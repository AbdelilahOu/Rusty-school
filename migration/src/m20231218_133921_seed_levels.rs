use sea_orm_migration::prelude::*;

use crate::{m20231211_172237_c_levels::Level, utils::generate_random_level};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        for _ in 1..=10 {
            let level = generate_random_level();
            let insert = Query::insert()
                .into_table(Level::Table)
                .columns([Level::Name, Level::Description])
                .values_panic([level.level_name.into(), level.level_description.into()])
                .to_owned();

            manager.exec_stmt(insert).await?;
        }

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let delete_query = Query::delete().from_table(Level::Table).to_owned();
        manager.exec_stmt(delete_query).await?;
        Ok(())
    }
}
