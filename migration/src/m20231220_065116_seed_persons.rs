use sea_orm::Statement;
use sea_orm_migration::prelude::*;

use crate::m20231118_095513_c_details::PersonDetails;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // let db = manager.get_connection();
        for _ in 0..200 {
            let sql = Query::select()
                .from(PersonDetails::Table)
                .column((Alias::new("person_details"), PersonDetails::Id))
                .left_join(
                    PersonDetails::Table,
                    Expr::col((PersonDetails::Table, PersonDetails::Id))
                        .equals((PersonDetails::Table, PersonDetails::Id)),
                )
                .conditions(
                    true,
                    |x| {
                        x.and_where(Expr::col((PersonDetails::Table, PersonDetails::Id)).is_null());
                    },
                    |_| {},
                )
                .limit(2)
                .to_owned()
                .to_string(PostgresQueryBuilder);
            println!("{}", sql);
            // .build(PostgresQueryBuilder);
            //
            // let statment =
            //     Statement::from_sql_and_values(sea_orm::DatabaseBackend::Postgres, sql, values);
            // let rows = db.query_all(statment).await?;
        }

        Ok(())
    }

    async fn down(&self, _: &SchemaManager) -> Result<(), DbErr> {
        Ok(())
    }
}
