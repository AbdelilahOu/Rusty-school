use sea_orm_migration::{prelude::*, sea_query::extension::postgres::PgExpr};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Students::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Students::Id)
                            .uuid()
                            .not_null()
                            .default(Expr::cust("gen_random_uuid()"))
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Students::FirstName).string().not_null())
                    .col(ColumnDef::new(Students::LastName).string().not_null())
                    .col(ColumnDef::new(Students::Level).string().not_null())
                    .col(
                        ColumnDef::new(Students::FullName).string().generated(
                            Expr::col(Students::FirstName)
                                .concat(" ")
                                .concat(Expr::col(Students::LastName)),
                            true,
                        ),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Students::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Students {
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
}
