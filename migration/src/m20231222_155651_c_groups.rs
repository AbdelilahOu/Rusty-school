use sea_orm_migration::prelude::*;

use crate::{m20231109_190937_c_students::Student, m20231211_172237_c_levels::Level};

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

        manager
            .alter_table(
                sea_query::Table::alter()
                    .table(Student::Table)
                    .add_column(ColumnDef::new(Student::GroupId).uuid())
                    .add_foreign_key(
                        TableForeignKey::new()
                            .name("fk_student_group_id")
                            .from_tbl(Student::Table)
                            .from_col(Student::GroupId)
                            .to_tbl(Group::Table)
                            .to_col(Group::Id),
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
                    .name("fk_student_group_id")
                    .table(Student::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                sea_query::Table::alter()
                    .table(Student::Table)
                    .drop_column(Student::GroupId)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_table(Table::drop().table(Group::Table).to_owned())
            .await?;
        Ok(())
    }
}

#[derive(DeriveIden)]
pub enum Group {
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
