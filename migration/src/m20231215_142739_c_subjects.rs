use sea_orm_migration::prelude::*;

use crate::m20231211_172237_c_levels::Level;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Subject::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Subject::Id)
                            .uuid()
                            .not_null()
                            .default(Expr::cust("gen_random_uuid()"))
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Subject::Name).string().not_null())
                    .col(ColumnDef::new(Subject::Description).string())
                    .col(ColumnDef::new(Subject::LevelId).uuid())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_subject_level_id")
                            .from(Subject::Table, Subject::LevelId)
                            .to(Level::Table, Level::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Subject::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Subject {
    #[sea_orm(iden = "subjects")]
    Table,
    Id,
    #[sea_orm(iden = "subject_name")]
    Name,
    #[sea_orm(iden = "subject_description")]
    Description,
    #[sea_orm(iden = "level_id")]
    LevelId,
    #[sea_orm(iden = "created_at")]
    CreatedAt,
}
