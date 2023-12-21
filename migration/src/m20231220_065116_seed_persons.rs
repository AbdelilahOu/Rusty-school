use sea_orm::Statement;
use sea_orm_migration::prelude::*;

use crate::{m20231118_095513_c_details::PersonDetails, m20231118_162555_c_person::Person};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
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
            //
            let statment = Statement::from_sql_and_values(
                sea_orm::DatabaseBackend::Postgres,
                details_sql.clone(),
                details_values.clone(),
            );
            let rows = db.query_one(statment).await?;
        }

        Ok(())
    }

    async fn down(&self, _: &SchemaManager) -> Result<(), DbErr> {
        Ok(())
    }
}
