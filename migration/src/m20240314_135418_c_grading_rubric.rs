use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(GradingRubrics::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(GradingRubrics::Id)
                            .uuid()
                            .not_null()
                            .default(Expr::cust("gen_random_uuid()"))
                            .primary_key(),
                    )
                    .col(ColumnDef::new(GradingRubrics::Title).string().not_null())
                    .col(ColumnDef::new(GradingRubrics::Description).string())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(GradingCriteria::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(GradingCriteria::Id)
                            .uuid()
                            .not_null()
                            .default(Expr::cust("gen_random_uuid()"))
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(GradingCriteria::GradingRubricId)
                            .uuid()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_grading_criteria_rubrics_id")
                            .from(GradingCriteria::Table, GradingCriteria::GradingRubricId)
                            .to(GradingRubrics::Table, GradingRubrics::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .col(ColumnDef::new(GradingCriteria::Description).string())
                    .col(ColumnDef::new(GradingCriteria::Points).integer())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(PerformanceLevel::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(PerformanceLevel::Id)
                            .uuid()
                            .not_null()
                            .default(Expr::cust("gen_random_uuid()"))
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(PerformanceLevel::LevelName)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(PerformanceLevel::GradingCriteriaId)
                            .uuid()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_performance_criteria_id")
                            .from(PerformanceLevel::Table, PerformanceLevel::GradingCriteriaId)
                            .to(GradingCriteria::Table, GradingCriteria::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .col(ColumnDef::new(PerformanceLevel::Description).string())
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(GradingRubrics::Table).to_owned())
            .await?;

        manager
            .drop_foreign_key(
                sea_query::ForeignKey::drop()
                    .name("fk_grading_criteria_rubrics_id")
                    .table(GradingCriteria::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_foreign_key(
                sea_query::ForeignKey::drop()
                    .name("fk_performance_criteria_id")
                    .table(PerformanceLevel::Table)
                    .to_owned(),
            )
            .await?;

        Ok(())
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
