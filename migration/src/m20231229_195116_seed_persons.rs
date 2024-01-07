use sea_orm::{prelude::Uuid, Statement};
use sea_orm_migration::prelude::*;

use crate::{
    m20231109_190937_c_students::Student, m20231113_170500_c_teachers::Teacher,
    m20231116_165911_c_parents::Parent, m20231118_162555_c_persons::Person,
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        for _ in 0..200 {
            let get_person_id = Statement::from_string(
                sea_orm::DatabaseBackend::Postgres,
                r#"
                INSERT INTO
                    persons (person_type, details_id)
                VALUES
                    (
                        'student',(
                            SELECT
                                pd.id
                            FROM
                                person_details as pd
                            WHERE
                                pd.id not in (
                                    SELECT
                                        p.details_id
                                    from
                                        persons as p
                                )
                            order by
                                random()
                            limit
                                1
                        )
                    ) RETURNING id"#,
            );
            let row = db.query_one(get_person_id).await?;
            let person_id = row.unwrap().try_get::<Uuid>("", "id").unwrap();
            let update_student_person_id = Statement::from_sql_and_values(
                sea_orm::DatabaseBackend::Postgres,
                r#"
                UPDATE
                    students
                SET
                    person_id = $1
                WHERE
                    id IN (
                        SELECT
                            s.id
                        FROM
                            students as s
                        WHERE
                            s.person_id IS NULL 
                        LIMIT 1
                    );
                "#,
                [person_id.into()],
            );
            db.execute(update_student_person_id).await?;
        }

        for _ in 0..100 {
            let get_person_id = Statement::from_string(
                sea_orm::DatabaseBackend::Postgres,
                r#"
                INSERT INTO
                    persons (person_type, details_id)
                VALUES
                    (
                        'parent',(
                            SELECT
                                pd.id
                            FROM
                                person_details as pd
                            WHERE
                                pd.id not in (
                                    SELECT
                                        p.details_id
                                    from
                                        persons as p
                                )
                            order by
                                random()
                            limit
                                1
                        )
                    ) RETURNING id"#,
            );
            let row = db.query_one(get_person_id).await?;
            let person_id = row.unwrap().try_get::<Uuid>("", "id").unwrap();
            let update_parent_person_id = Statement::from_sql_and_values(
                sea_orm::DatabaseBackend::Postgres,
                r#"
                UPDATE
                    parents
                SET
                    person_id = $1
                WHERE
                    id = (
                        SELECT
                            id
                        FROM
                            parents
                        WHERE
                            person_id IS NULL 
                        LIMIT 1
                    );
                "#,
                [person_id.into()],
            );
            db.execute(update_parent_person_id).await?;
        }

        for _ in 0..50 {
            let get_person_id = Statement::from_string(
                sea_orm::DatabaseBackend::Postgres,
                r#"
                INSERT INTO
                    persons (person_type, details_id)
                VALUES
                    (
                        'teacher',(
                            SELECT
                                pd.id
                            FROM
                                person_details as pd
                            WHERE
                                pd.id not in (
                                    SELECT
                                        p.details_id
                                    from
                                        persons as p
                                )
                            order by
                                random()
                            limit
                                1
                        )
                    ) RETURNING id"#,
            );
            let row = db.query_one(get_person_id).await?;
            let person_id = row.unwrap().try_get::<Uuid>("", "id").unwrap();
            let update_teacher_person_id = Statement::from_sql_and_values(
                sea_orm::DatabaseBackend::Postgres,
                r#"
                UPDATE
                    teachers
                SET
                    person_id = $1
                WHERE id = 
                    (
                        SELECT
                            id
                        FROM
                            teachers
                        WHERE
                            person_id IS NULL 
                        LIMIT 1
                    );
                "#,
                [person_id.into()],
            );
            db.execute(update_teacher_person_id).await?;
        }

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let update_student = Query::update()
            .table(Student::Table)
            .value(Student::PersonId, None::<Uuid>)
            .to_owned();

        let update_parent = Query::update()
            .table(Parent::Table)
            .value(Parent::PersonId, None::<Uuid>)
            .to_owned();

        let teacher_update = Query::update()
            .table(Teacher::Table)
            .value(Teacher::PersonId, None::<Uuid>)
            .to_owned();

        let delete_person = Query::delete().from_table(Person::Table).to_owned();

        manager.exec_stmt(update_student).await?;
        manager.exec_stmt(update_parent).await?;
        manager.exec_stmt(teacher_update).await?;
        manager.exec_stmt(delete_person).await?;

        Ok(())
    }
}
