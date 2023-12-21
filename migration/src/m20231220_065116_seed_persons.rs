use sea_orm::{prelude::Uuid, Statement};
use sea_orm_migration::prelude::*;

use crate::{
    m20231109_190937_c_student::Student, m20231118_095513_c_details::PersonDetails,
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
            let (student_sql, student_values) = Query::select()
                .from(Student::Table)
                .column((Alias::new("students"), Student::Id))
                .left_join(
                    Person::Table,
                    Expr::col((Person::Table, Person::Id))
                        .equals((Student::Table, Student::PersonId)),
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

        Ok(())
    }

    async fn down(&self, _: &SchemaManager) -> Result<(), DbErr> {
        Ok(())
    }
}
