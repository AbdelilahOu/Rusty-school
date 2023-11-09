use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Teacher::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Teacher::Id)
                            .integer()
                            .not_null()
                            .uuid()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Teacher::Name).string().not_null())
                    .col(ColumnDef::new(Teacher::Email).string().null())
                    .col(ColumnDef::new(Teacher::Phone).string().null())
                    .col(ColumnDef::new(Teacher::Address).string().null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Teacher::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Teacher {
    Table,
    Id,
    Name,
    Email,
    Phone,
    Address,
}
