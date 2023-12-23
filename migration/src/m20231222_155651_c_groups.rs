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
                    .table(Group::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Group::Id)
                            .uuid()
                            .not_null()
                            .default(Expr::cust("gen_random_uuid()"))
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Group::Name).string().not_null())
                    .col(ColumnDef::new(Group::Description).string())
                    .col(ColumnDef::new(Group::LevelId).uuid())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_details_person_id")
                            .from(Group::Table, Group::LevelId)
                            .to(Level::Table, Level::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Group::Table).to_owned())
            .await?;
        Ok(())
    }
}

#[derive(DeriveIden)]
enum Group {
    #[sea_orm(iden = "groups")]
    Table,
    Id,
    #[sea_orm(iden = "group_name")]
    Name,
    #[sea_orm(iden = "level_id")]
    LevelId,
    #[sea_orm(iden = "group_description")]
    Description,
}
