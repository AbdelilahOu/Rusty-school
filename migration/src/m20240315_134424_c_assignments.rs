use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Assignment::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Assignment::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Assignment::Title).string().not_null())
                    .col(ColumnDef::new(Assignment::Description).string().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Assignment::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Assignment {
    #[sea_orm(iden = "assignments")]
    Table,
    Id,
    Title,
    Description,
    DueDate,
    SubmissionType,
    GradinRubric,
    Files,
    ClassId,
    TeacherId,
}
