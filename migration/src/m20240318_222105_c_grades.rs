use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Grade::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Grade::Id)
                            .uuid()
                            .not_null()
                            .default(Expr::cust("gen_random_uuid()"))
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Grade::StudentId).uuid().not_null())
                    .col(ColumnDef::new(Grade::AssignmentId).uuid().not_null())
                    .col(ColumnDef::new(Grade::Score).float().not_null())
                    .col(ColumnDef::new(Grade::Feedback).string().not_null())
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Grade::Table).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum Grade {
    Table,
    Id,
    StudentId,
    AssignmentId,
    Score,
    Feedback,
}
