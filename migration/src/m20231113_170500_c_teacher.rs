use sea_orm_migration::prelude::*;

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
                    .col(ColumnDef::new(Teachers::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(Teachers::FirstName).string().not_null())
                    .col(ColumnDef::new(Teachers::LastName).string().not_null())
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
enum Teachers {
    #[sea_orm(iden = "teachers")]
    Table,
    Id,
    #[sea_orm(iden = "first_name")]
    FirstName,
    #[sea_orm(iden = "last_name")]
    LastName,
}
