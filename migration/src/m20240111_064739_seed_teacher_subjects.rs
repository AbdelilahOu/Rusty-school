use sea_orm::{DbBackend, Statement};
use sea_orm_migration::prelude::*;

use crate::m20231223_124755_c_teacher_subjects::TeacherSubjects;

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
                        DECLARE subject_id_v UUID;
                        DECLARE counter INTEGER = 1;
                        -- Logic block
                        BEGIN
                            WHILE counter > 0 LOOP
                                -- Select a random level_id from the levels table: 
                                SELECT id FROM levels ORDER BY random() LIMIT 1 INTO level_id_v;
                                
                                -- Select a random subject_id from the subjects table:
                                SELECT id FROM subjects WHERE level_id = level_id_v ORDER BY RANDOM() LIMIT 1 INTO subject_id_v;
                                
                                -- Seed the classes table
                                INSERT INTO
                                    teacher_subjects (subject_id, teacher_id)
                                VALUES
                                    (
                                        subject_id_v
                                        ,(
                                            SELECT id FROM teachers WHERE level_id = level_id_v ORDER BY RANDOM() LIMIT 1
                                        )
                                    );

                                -- check if all teachers has teacher_subjects
                                SELECT COUNT(id) FROM teachers WHERE id NOT IN (SELECT teacher_id FROM teacher_subjects) INTO counter;
                                -- Loop stuff
                                EXIT WHEN counter = 0;
                            END LOOP;
                        END;
                    $$;
                "#,
            )
        ).await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let delete_query = Query::delete()
            .from_table(TeacherSubjects::Table)
            .to_owned();
        manager.exec_stmt(delete_query).await?;
        Ok(())
    }
}
