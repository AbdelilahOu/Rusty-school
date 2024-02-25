use sea_orm_migration::prelude::*;

use crate::m20231113_170500_c_teachers::Teacher;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Level::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Level::Id)
                            .uuid()
                            .not_null()
                            .default(Expr::cust("gen_random_uuid()"))
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Level::Name).string().not_null())
                    .col(ColumnDef::new(Level::Description).string())
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                sea_query::Table::alter()
                    .table(Teacher::Table)
                    .add_column(ColumnDef::new(Teacher::LevelId).uuid())
                    .add_foreign_key(
                        TableForeignKey::new()
                            .name("fk_teacher_level_id")
                            .from_tbl(Teacher::Table)
                            .from_col(Teacher::LevelId)
                            .to_tbl(Level::Table)
                            .to_col(Level::Id),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_foreign_key(
                sea_query::ForeignKey::drop()
                    .name("fk_teacher_level_id")
                    .table(Teacher::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                sea_query::Table::alter()
                    .table(Teacher::Table)
                    .drop_column(Teacher::LevelId)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_table(Table::drop().table(Level::Table).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
pub enum Level {
    #[sea_orm(iden = "levels")]
    Table,
    Id,
    #[sea_orm(iden = "level_name")]
    Name,
    #[sea_orm(iden = "level_description")]
    Description,
    #[sea_orm(iden = "created_at")]
    CreatedAt,
}
