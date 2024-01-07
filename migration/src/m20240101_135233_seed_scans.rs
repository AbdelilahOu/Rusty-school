use sea_orm::{DbBackend, Statement};
use sea_orm_migration::prelude::*;

use crate::m20231116_214011_c_scans::Scan;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        for _ in 0..4000 {
            db.execute(Statement::from_string(
                DbBackend::Postgres,
                r#"
                INSERT INTO
                    scans (person_id, scan_date)
                VALUES
                (
                    (
                        SELECT
                            p.id
                        FROM
                            persons as p
                            LEFT JOIN students as s ON s.person_id = p.id
                            LEFT JOIN parents as pa ON pa.person_id = p.id
                            LEFT JOIN teachers as t ON t.person_id = p.id
                        ORDER BY
                            random()
                        LIMIT
                            1
                    ),
                    NOW() - (random() * (NOW() - NOW()+'90days')) + '30 days'
                )"#,
            ))
            .await?;
        }

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let delete_query = Query::delete().from_table(Scan::Table).to_owned();
        manager.exec_stmt(delete_query).await?;
        Ok(())
    }
}
