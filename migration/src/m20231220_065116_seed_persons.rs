use sea_orm::{prelude::Uuid, Statement};
use sea_orm_migration::prelude::*;

use crate::{
    m20231109_190937_c_student::Student, m20231113_170500_c_teacher::Teacher,
    m20231116_165911_c_parents::Parent, m20231118_095513_c_details::PersonDetails,
    m20231118_162555_c_person::Person,
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        //
        let (details_sql, details_values) = Query::select()
            .from(PersonDetails::Table)
            .column((Alias::new("person_details"), PersonDetails::Id))
            .left_join(
                Person::Table,
                Expr::col((Person::Table, Person::DetailsId))
                    .equals((PersonDetails::Table, PersonDetails::Id)),
            )
            .conditions(
                true,
                |x| {
                    x.and_where(Expr::col((Person::Table, Person::DetailsId)).is_null());
                },
                |_| {},
            )
            .limit(1)
            .to_owned()
            .build(PostgresQueryBuilder);

        let (parent_sql, parent_values) = Query::select()
            .from(Parent::Table)
            .column((Alias::new("parents"), Parent::Id))
            .left_join(
                Person::Table,
                Expr::col((Person::Table, Person::Id)).equals((Parent::Table, Parent::PersonId)),
            )
            .conditions(
                true,
                |x| {
                    x.and_where(Expr::col((Parent::Table, Parent::PersonId)).is_null());
                },
                |_| {},
            )
            .limit(1)
            .to_owned()
            .build(PostgresQueryBuilder);

        let (student_sql, student_values) = Query::select()
            .from(Student::Table)
            .column((Alias::new("students"), Student::Id))
            .left_join(
                Person::Table,
                Expr::col((Person::Table, Person::Id)).equals((Student::Table, Student::PersonId)),
            )
            .conditions(
                true,
                |x| {
                    x.and_where(Expr::col((Student::Table, Student::PersonId)).is_null());
                },
                |_| {},
            )
            .limit(1)
            .to_owned()
            .build(PostgresQueryBuilder);

        let (teacher_sql, teacher_values) = Query::select()
            .from(Teacher::Table)
            .column((Alias::new("teachers"), Teacher::Id))
            .left_join(
                Person::Table,
                Expr::col((Person::Table, Person::Id)).equals((Teacher::Table, Teacher::PersonId)),
            )
            .conditions(
                true,
                |x| {
                    x.and_where(Expr::col((Teacher::Table, Teacher::PersonId)).is_null());
                },
                |_| {},
            )
            .limit(1)
            .to_owned()
            .build(PostgresQueryBuilder);

        for _ in 0..200 {
            // SELECT DETAILS ID
            let statment = Statement::from_sql_and_values(
                sea_orm::DatabaseBackend::Postgres,
                details_sql.clone(),
                details_values.clone(),
            );
            let details_row = db.query_one(statment).await?;
            let details_id = details_row.unwrap().try_get::<Uuid>("", "id").unwrap();
            // SELECT STUDENT ID
            let statment = Statement::from_sql_and_values(
                sea_orm::DatabaseBackend::Postgres,
                student_sql.clone(),
                student_values.clone(),
            );
            let student_row = db.query_one(statment).await?;
            let student_id = student_row.unwrap().try_get::<Uuid>("", "id").unwrap();
            //
            let person_type = String::from("student");
            let (sql, values) = Query::insert()
                .into_table(Person::Table)
                .columns([Person::PersonType, Person::DetailsId])
                .values_panic([person_type.clone().into(), details_id.clone().into()])
                .returning(ReturningClause::Columns(vec![ColumnRef::Column(
                    SeaRc::new(Person::Id),
                )]))
                .build(PostgresQueryBuilder);

            let statment =
                Statement::from_sql_and_values(sea_orm::DatabaseBackend::Postgres, sql, values);

            let row = db.query_one(statment).await?;
            let person_id = row.unwrap().try_get::<Uuid>("", "id").unwrap();

            let update_student = Query::update()
                .table(Student::Table)
                .values([(Student::PersonId, person_id.clone().into())])
                .and_where(Expr::col((Student::Table, Student::Id)).eq(student_id))
                .to_owned();

            manager.exec_stmt(update_student).await?;
        }

        for _ in 0..100 {
            // SELECT DETAILS ID
            let statment = Statement::from_sql_and_values(
                sea_orm::DatabaseBackend::Postgres,
                details_sql.clone(),
                details_values.clone(),
            );
            let details_row = db.query_one(statment).await?;
            let details_id = details_row.unwrap().try_get::<Uuid>("", "id").unwrap();
            // SELECT pARENT ID
            let statment = Statement::from_sql_and_values(
                sea_orm::DatabaseBackend::Postgres,
                parent_sql.clone(),
                parent_values.clone(),
            );
            let parent_row = db.query_one(statment).await?;
            let parent_id = parent_row.unwrap().try_get::<Uuid>("", "id").unwrap();
            //
            let person_type = String::from("parent");
            let (sql, values) = Query::insert()
                .into_table(Person::Table)
                .columns([Person::PersonType, Person::DetailsId])
                .values_panic([person_type.clone().into(), details_id.clone().into()])
                .returning(ReturningClause::Columns(vec![ColumnRef::Column(
                    SeaRc::new(Person::Id),
                )]))
                .build(PostgresQueryBuilder);

            let statment =
                Statement::from_sql_and_values(sea_orm::DatabaseBackend::Postgres, sql, values);

            let row = db.query_one(statment).await?;
            let person_id = row.unwrap().try_get::<Uuid>("", "id").unwrap();

            let update_parent = Query::update()
                .table(Parent::Table)
                .values([(Parent::PersonId, person_id.clone().into())])
                .and_where(Expr::col((Parent::Table, Parent::Id)).eq(parent_id))
                .to_owned();

            manager.exec_stmt(update_parent).await?;
        }

        for _ in 0..50 {
            // SELECT DETAILS ID
            let statment = Statement::from_sql_and_values(
                sea_orm::DatabaseBackend::Postgres,
                details_sql.clone(),
                details_values.clone(),
            );
            let details_row = db.query_one(statment).await?;
            let details_id = details_row.unwrap().try_get::<Uuid>("", "id").unwrap();
            // SELECT TEACHER ID
            let statment = Statement::from_sql_and_values(
                sea_orm::DatabaseBackend::Postgres,
                teacher_sql.clone(),
                teacher_values.clone(),
            );
            let teacher_row = db.query_one(statment).await?;
            let teacher_id = teacher_row.unwrap().try_get::<Uuid>("", "id").unwrap();
            //
            let person_type = String::from("teacher");
            let (sql, values) = Query::insert()
                .into_table(Person::Table)
                .columns([Person::PersonType, Person::DetailsId])
                .values_panic([person_type.clone().into(), details_id.clone().into()])
                .returning(ReturningClause::Columns(vec![ColumnRef::Column(
                    SeaRc::new(Person::Id),
                )]))
                .build(PostgresQueryBuilder);

            let statment =
                Statement::from_sql_and_values(sea_orm::DatabaseBackend::Postgres, sql, values);

            let row = db.query_one(statment).await?;
            let person_id = row.unwrap().try_get::<Uuid>("", "id").unwrap();

            let update_teacher = Query::update()
                .table(Teacher::Table)
                .values([(Teacher::PersonId, person_id.clone().into())])
                .and_where(Expr::col((Teacher::Table, Teacher::Id)).eq(teacher_id))
                .to_owned();

            manager.exec_stmt(update_teacher).await?;
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
