use crate::{
    m20231109_190937_c_student::Student, m20231116_165911_c_parents::Parent,
    m20231116_171406_c_pickups::Pickup,
};
use sea_orm::{prelude::Uuid, Statement};
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let parent_alias = Alias::new("parents");
        let (sql, values) = Query::select()
            .from(Parent::Table)
            .column((parent_alias, Parent::Id))
            .left_join(
                Pickup::Table,
                Expr::col((Parent::Table, Parent::Id)).equals((Pickup::Table, Pickup::ParentId)),
            )
            .conditions(
                true,
                |x| {
                    x.and_where(Expr::col((Pickup::Table, Pickup::Id)).is_null());
                },
                |_| {},
            )
            .to_owned()
            .build(PostgresQueryBuilder);
        //
        let statment =
            Statement::from_sql_and_values(sea_orm::DatabaseBackend::Postgres, sql, values);
        //
        let db = manager.get_connection();
        //
        let rows = db.query_all(statment).await?;
        //
        for row in rows {
            let id = row.try_get::<Uuid>("", "id").unwrap();
            let students_alias = Alias::new("students");
            let (sql, values) = Query::select()
                .from(Student::Table)
                .column((students_alias, Student::Id))
                .left_join(
                    Pickup::Table,
                    Expr::col((Student::Table, Student::Id))
                        .equals((Pickup::Table, Pickup::ParentId)),
                )
                .conditions(
                    true,
                    |x| {
                        x.and_where(Expr::col((Pickup::Table, Pickup::Id)).is_null());
                    },
                    |_| {},
                )
                .limit(2)
                .to_owned()
                .build(PostgresQueryBuilder);
            //
            let statment =
                Statement::from_sql_and_values(sea_orm::DatabaseBackend::Postgres, sql, values);
            let rows = db.query_all(statment).await?;
            for row in rows {
                let student_id = row.try_get::<Uuid>("", "id").unwrap();
                let insert = Query::insert()
                    .into_table(Pickup::Table)
                    .columns([Pickup::ParentId, Pickup::StudentId])
                    .values_panic([id.clone().into(), student_id.clone().into()])
                    .to_owned();
                manager.exec_stmt(insert).await?;
            }
        }

        Ok(())
    }

    async fn down(&self, _: &SchemaManager) -> Result<(), DbErr> {
        Ok(())
    }
}
