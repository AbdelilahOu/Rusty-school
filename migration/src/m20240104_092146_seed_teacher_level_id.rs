use sea_orm::{prelude::Uuid, DbBackend, Statement};
use sea_orm_migration::prelude::*;

use crate::m20231113_170500_c_teachers::Teacher;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        for _ in 0..200 {
            db.execute(Statement::from_string(
                DbBackend::Postgres,
                r#"
                UPDATE
                    teachers
                SET
                    level_id = (
                        SELECT
                            id
                        FROM
                            levels
                        ORDER BY
                            random()
                        LIMIT
                            1
                    )
                WHERE
                    id = (
                        SELECT
                            id
                        FROM
                            teachers
                        WHERE
                            level_id IS NULL
                        ORDER BY
                            random()
                        LIMIT
                            1
                    )
                "#,
            ))
            .await?;
        }
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let delete_query = Query::update()
            .table(Teacher::Table)
            .value(Teacher::LevelId, Option::<Uuid>::None)
            .to_owned();
        manager.exec_stmt(delete_query).await?;
        Ok(())
    }
}
