use sea_orm::{prelude::Uuid, DbBackend, Statement};
use sea_orm_migration::prelude::*;

use crate::m20231109_190937_c_students::Student;

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
                    students
                SET
                    group_id = (
                        SELECT
                            id
                        FROM
                            groups
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
                            students
                        WHERE
                            group_id IS NULL
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
            .table(Student::Table)
            .value(Student::GroupId, Option::<Uuid>::None)
            .to_owned();
        manager.exec_stmt(delete_query).await?;
        Ok(())
    }
}
