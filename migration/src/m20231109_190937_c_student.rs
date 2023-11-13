use sea_orm_migration::prelude::*;

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
                            .integer()
                            .not_null()
                            .uuid()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Students::FirstName).string().not_null())
                    .col(ColumnDef::new(Students::LastName).string().not_null())
                    .col(ColumnDef::new(Students::Address).string().not_null())
                    .col(ColumnDef::new(Students::Level).string().not_null())
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
enum Students {
    Table,
    Id,
    FirstName,
    LastName,
    Address,
    Level,
}
