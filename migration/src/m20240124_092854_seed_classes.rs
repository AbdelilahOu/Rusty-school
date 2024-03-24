use sea_orm::{DbBackend, Statement};
use sea_orm_migration::prelude::*;

use crate::m20231223_094755_c_classes::Class;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        db.execute(
            Statement::from_string(
            DbBackend::Postgres,
                r#"
                    DO $$ 
                        -- Declare some variables
                        DECLARE level_id_v UUID;
                        DECLARE teacher_id_v UUID;
                        DECLARE subject_id_v UUID;
                        DECLARE counter INTEGER = 0;
                        -- Logic block
                        BEGIN
                            WHILE counter <= 50 LOOP
                                -- Select a random level_id from the levels table: 
                                SELECT id FROM levels ORDER BY random() LIMIT 1 INTO level_id_v;

                                -- Select a random teacher_id from the teachers table:
                                SELECT id FROM teachers WHERE level_id = level_id_v ORDER BY random() LIMIT 1 INTO teacher_id_v;

                                -- Select a random subject_id from the teacher_subjects table:
                                SELECT subject_id FROM teacher_subjects WHERE teacher_id = teacher_id_v ORDER BY random() LIMIT 1 INTO subject_id_v;

                                -- Seed the classes table
                                INSERT INTO
                                    classes (subject_id, teacher_id, group_id, room_id)
                                VALUES
                                    (
                                        subject_id_v,
                                        teacher_id_v,
                                        (
                                            SELECT id FROM groups WHERE level_id = level_id_v ORDER BY RANDOM() LIMIT 1
                                        ),(
                                            SELECT id FROM rooms ORDER BY RANDOM() LIMIT 1
                                        )
                                    )
                                ON CONFLICT DO NOTHING;

                                -- Loop stuff
                                counter := counter + 1;
                                EXIT WHEN counter > 50;
                            END LOOP;
                        END;
                    $$;
                "#,
            )
        ).await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let delete_query = Query::delete().from_table(Class::Table).to_owned();
        manager.exec_stmt(delete_query).await?;
        Ok(())
    }
}
