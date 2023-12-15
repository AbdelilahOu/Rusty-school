use sea_orm_migration::{prelude::*, sea_query::extension::postgres::PgExpr};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Student::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Student::Id)
                            .uuid()
                            .not_null()
                            .default(Expr::cust("gen_random_uuid()"))
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Student::FirstName).string().not_null())
                    .col(ColumnDef::new(Student::LastName).string().not_null())
                    .col(ColumnDef::new(Student::Level).string().not_null())
                    .col(
                        ColumnDef::new(Student::FullName).string().generated(
                            Expr::col(Student::FirstName)
                                .concat(" ")
                                .concat(Expr::col(Student::LastName)),
                            true,
                        ),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Student::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Student {
    #[sea_orm(iden = "students")]
    Table,
    Id,
    #[sea_orm(iden = "first_name")]
    FirstName,
    #[sea_orm(iden = "last_name")]
    LastName,
    #[sea_orm(iden = "full_name")]
    FullName,
    Level,
    #[sea_orm(iden = "person_id")]
    PersonId,
    #[sea_orm(iden = "level_id")]
    LevelId,
}
