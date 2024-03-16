use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        todo!();

        manager
            .create_table(
                Table::create()
                    .table(GradingRubrics::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(GradingRubrics::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(GradingRubrics::Title).string().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        todo!();

        manager
            .drop_table(Table::drop().table(GradingRubrics::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum GradingRubrics {
    Table,
    Id,
    Title,
    Description,
}

#[derive(DeriveIden)]
enum GradingCriteria {
    Table,
    Id,
    GradingRubricId,
    Description,
    Points,
}

#[derive(DeriveIden)]
enum PerformanceLevel {
    Table,
    Id,
    GradingCriteriaId,
    LevelName,
    Description,
}
