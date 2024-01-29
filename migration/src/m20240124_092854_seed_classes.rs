use sea_orm::{DbBackend, Statement};
use sea_orm_migration::prelude::*;

use crate::m20231223_094755_c_classes::Class;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        for _ in 1..40 {
            db.execute(
                Statement::from_string(
                DbBackend::Postgres,
                    r#"
                        DO $$ 
                            DECLARE level_id_v UUID;
                            BEGIN
                            -- Select a random level_id from the levels table: 
                            SELECT id FROM levels ORDER BY random() LIMIT 1 INTO level_id_v;

                            -- Use RAISE NOTICE to print the value of level_id_v for verification:
                            RAISE NOTICE 'Selected level_id: %',level_id_v;

                            -- Seed the classes table
                            INSERT INTO
                                classes (subject_id, teacher_id, group_id, room_id)
                            VALUES
                                (
                                    (
                                        SELECT id FROM subjects WHERE level_id = level_id_v ORDER BY RANDOM() LIMIT 1
                                    ),(
                                        SELECT id FROM teachers WHERE level_id = level_id_v ORDER BY RANDOM() LIMIT 1
                                    ),(
                                        SELECT id FROM groups WHERE level_id = level_id_v ORDER BY RANDOM() LIMIT 1
                                    ), (
                                        SELECT id FROM rooms LIMIT 1
                                    )
                                );
                            END;
                        $$;
                    "#,
                )
            ).await?;
        }
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let delete_query = Query::delete().from_table(Class::Table).to_owned();
        manager.exec_stmt(delete_query).await?;
        Ok(())
    }
}
