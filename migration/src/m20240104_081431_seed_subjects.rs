use sea_orm::{DbBackend, Statement};
use sea_orm_migration::prelude::*;

use crate::{m20231215_142739_c_subjects::Subject, utils::generate_random_subject};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        for _ in 0..40 {
            let subject = generate_random_subject();
            db.execute(Statement::from_sql_and_values(
                DbBackend::Postgres,
                r#"
                INSERT INTO
                    subjects (subject_name, subject_description, level_id)
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
                [
                    subject.subject_name.into(),
                    subject.subject_description.into(),
                ],
            ))
            .await?;
        }
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let delete_query = Query::delete().from_table(Subject::Table).to_owned();
        manager.exec_stmt(delete_query).await?;
        Ok(())
    }
}
