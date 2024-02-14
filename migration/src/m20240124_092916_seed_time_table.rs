use sea_orm::{DbBackend, Statement};
use sea_orm_migration::prelude::*;

use crate::{
    m20240111_063739_c_timetable::{Event, Lecture, TimeTable},
    utils::generate_random_event,
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        // create activities

        // create events
        db.execute(Statement::from_string(
            DbBackend::Postgres,
            r#"
                DO $$ 
                    -- Declare some variables
                    DECLARE random_start_time_v TIME;
                    DECLARE random_date_v DATE;
                    DECLARE counter INTEGER = 10;
                    -- Logic block
                    BEGIN
                        WHILE counter > 0 LOOP
                            -- generate random time 
                            SELECT '08:00:00'::time + ((random() * 9)::INTEGER * INTERVAL '1 hour') + (((random() * 1)::INTEGER + 1) * INTERVAL '30 minute') INTO random_start_time_v;
                            SELECT NOW()::date + ((random() * 5)::INTEGER * INTERVAL '1 day') INTO random_date_v;
                            -- create a timetable
                            INSERT INTO 
                                time_table (type, full_date, start_time) 
                            VALUES 
                                (
                                    'event', 
                                    random_date_v, 
                                    random_start_time_v
                                );
                            counter := counter - 1;
                            EXIT WHEN counter = 0;
                        END LOOP;
                    END;
                $$;
            "#,
        ))
        .await?;

        for _ in 0..10 {
            let random_event = generate_random_event();
            db.execute(Statement::from_sql_and_values(
                DbBackend::Postgres,
                r#"
                    INSERT INTO 
                        events (time_table_id, event_title, event_description) 
                    VALUES 
                        (
                            (
                                SELECT id FROM time_table WHERE type = 'event' AND id NOT IN (SELECT time_table_id FROM events) LIMIT 1
                            ), 
                            $1, 
                            $2
                        );
                "#,
                [random_event.title.into(),random_event.description.into()]
            ))
            .await?;
        }

        // create courses
        db.execute(Statement::from_string(
            DbBackend::Postgres,
            r#"
                DO $$ 
                    -- Declare some variables
                    DECLARE timetable_id_v UUID;
                    DECLARE random_start_time_v TIME;
                    DECLARE random_end_time_v TIME;
                    DECLARE counter INTEGER = 300;
                    -- Logic block
                    BEGIN
                        WHILE counter > 0 LOOP
                            -- generate random time 
                            SELECT '08:00:00'::time + ((random() * 9)::INTEGER * INTERVAL '1 hour') + (((random() * 1)::INTEGER + 1) * INTERVAL '30 minute') INTO random_start_time_v;
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

        // delete lecture
        let delete_query = Query::delete().from_table(Event::Table).to_owned();
        manager.exec_stmt(delete_query).await?;

        // delete time table
        let delete_time_table = Query::delete().from_table(TimeTable::Table).to_owned();
        manager.exec_stmt(delete_time_table).await?;
        Ok(())
    }
}
