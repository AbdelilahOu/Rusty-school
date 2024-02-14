use sea_orm::{DbBackend, Statement};
use sea_orm_migration::prelude::*;

use crate::m20240111_063739_c_timetable::{Lecture, TimeTable};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // create activities

        // create courses
        let db = manager.get_connection();
        db.execute(Statement::from_string(
            DbBackend::Postgres,
            r#"
                DO $$ 
                    -- Declare some variables
                    DECLARE timetable_id_v UUID;
                    DECLARE random_start_time_v TIME;
                    DECLARE random_end_time_v TIME;
                    DECLARE counter INTEGER = 50;
                    -- Logic block
                    BEGIN
                        WHILE counter > 0 LOOP
                            -- generate random time 
                            SELECT '08:00:00'::time + ((random() * 9)::INTEGER * INTERVAL '1 hour') INTO random_start_time_v;
                            SELECT random_start_time_v + (((random() * 1)::INTEGER + 1) * INTERVAL '30 minute') INTO random_end_time_v;
                            -- create a timetable
                            INSERT INTO 
                                time_table (type, day_of_week, start_time, end_time) 
                            VALUES 
                                (
                                    'lecture', 
                                    ((RANDOM() * 5)::INTEGER + 1)::TEXT::day_of_week_enum, 
                                    random_start_time_v, 
                                    random_end_time_v
                                )
                            RETURNING id
                            INTO timetable_id_v;
                            -- create a lecture
                            INSERT INTO
                                lectures (time_table_id, class_id)
                            VALUES 
                                (
                                    timetable_id_v, 
                                    (SELECT id FROM classes ORDER BY RANDOM() LIMIT 1)    
                                );
                            -- Loop stuff
                            counter := counter - 1;
                            EXIT WHEN counter = 0;
                        END LOOP;
                    END;
                $$;
            "#,
        ))
        .await?;
        // create events
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // delete lecture
        let delete_query = Query::delete().from_table(Lecture::Table).to_owned();
        manager.exec_stmt(delete_query).await?;

        // delete time table
        let delete_time_table = Query::delete().from_table(TimeTable::Table).to_owned();
        manager.exec_stmt(delete_time_table).await?;
        Ok(())
    }
}
