use sea_orm::{DbBackend, Statement};
use sea_orm_migration::prelude::*;

use crate::{m20231222_155651_c_groups::Group, utils::generate_random_group};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        for _ in 0..25 {
            let group = generate_random_group();
            db.execute(Statement::from_sql_and_values(
                DbBackend::Postgres,
                r#"
                INSERT INTO
                    groups (group_name, group_description, level_id)
                VALUES (
                    $1,
                    $2,
                    (
                        SELECT
                            id
                        FROM
                            levels
                        ORDER BY
                            random()
                        LIMIT
                            1
                    )
                )"#,
                [group.group_name.into(), group.group_description.into()],
            ))
            .await?;
        }
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let delete_query = Query::delete().from_table(Group::Table).to_owned();
        manager.exec_stmt(delete_query).await?;
        Ok(())
    }
}
