use sea_orm_migration::{prelude::*, sea_query::extension::postgres::PgExpr};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Parents::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Parents::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(Parents::FirstName).string().not_null())
                    .col(ColumnDef::new(Parents::LastName).string().not_null())
                    .col(
                        ColumnDef::new(Parents::FullName).string().generated(
                            Expr::col(Parents::FirstName)
                                .concat(" ")
                                .concat(Expr::col(Parents::LastName)),
                            true,
                        ),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Parents::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Parents {
    #[sea_orm(iden = "parents")]
    Table,
    Id,
    #[sea_orm(iden = "first_name")]
    FirstName,
    #[sea_orm(iden = "last_name")]
    LastName,
    #[sea_orm(iden = "full_name")]
    FullName,
}
