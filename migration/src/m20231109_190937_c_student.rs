use sea_orm_migration::prelude::*;

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
                            .integer()
                            .not_null()
                            .uuid()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Student::FirstName).string().not_null())
                    .col(ColumnDef::new(Student::LastName).string().not_null())
                    .col(ColumnDef::new(Student::DateOfBirth).string().not_null())
                    .col(ColumnDef::new(Student::ContactDetails).string().not_null())
                    .col(ColumnDef::new(Student::Address).string().not_null())
                    .col(ColumnDef::new(Student::Level).string().not_null())
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
enum Student {
    Table,
    Id,
    FirstName,
    LastName,
    DateOfBirth,
    ContactDetails,
    Address,
    Level,
}
