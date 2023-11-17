use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Subjects::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Subjects::Id).uuid().primary_key())
                    .col(ColumnDef::new(Subjects::Name).string().not_null())
                    .col(ColumnDef::new(Subjects::Description).string())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Subjects::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Subjects {
    #[sea_orm(iden = "subjects")]
    Table,
    Id,
    #[sea_orm(iden = "subject_name")]
    Name,
    #[sea_orm(iden = "description")]
    Description,
}
