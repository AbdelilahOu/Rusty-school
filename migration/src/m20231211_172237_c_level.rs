use sea_orm_migration::prelude::*;

use crate::m20231109_190937_c_student::Student;

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
                    .table(Student::Table)
                    .add_column(ColumnDef::new(Student::LevelId).uuid())
                    .add_foreign_key(
                        TableForeignKey::new()
                            .name("fk_student_level_id")
                            .from_tbl(Student::Table)
                            .from_col(Student::LevelId)
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
            .drop_table(Table::drop().table(Level::Table).to_owned())
            .await
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
}
