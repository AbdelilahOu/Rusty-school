use sea_orm_migration::{prelude::*, sea_query::extension::postgres::PgExpr};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Teachers::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Teachers::Id)
                            .uuid()
                            .not_null()
                            .default(Expr::cust("gen_random_uuid()"))
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Teachers::FirstName).string().not_null())
                    .col(ColumnDef::new(Teachers::LastName).string().not_null())
                    .col(
                        ColumnDef::new(Teachers::FullName).string().generated(
                            Expr::col(Teachers::FirstName)
                                .concat(" ")
                                .concat(Expr::col(Teachers::LastName)),
                            true,
                        ),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Teachers::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Teachers {
    #[sea_orm(iden = "teachers")]
    Table,
    Id,
    #[sea_orm(iden = "first_name")]
    FirstName,
    #[sea_orm(iden = "last_name")]
    LastName,
    #[sea_orm(iden = "full_name")]
    FullName,
    #[sea_orm(iden = "person_id")]
    PersonId,
}
