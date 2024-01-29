use sea_orm_migration::prelude::*;

use crate::{
    m20231113_170500_c_teachers::Teacher, m20231215_142739_c_subjects::Subject,
    m20231222_155651_c_groups::Group, m20231223_093909_c_rooms::Room,
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Class::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Class::Id)
                            .uuid()
                            .not_null()
                            .default(Expr::cust("gen_random_uuid()"))
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Class::SubjectId).uuid())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_classes_subject_id")
                            .from(Class::Table, Class::SubjectId)
                            .to(Subject::Table, Subject::Id)
                            .on_delete(ForeignKeyAction::SetNull),
                    )
                    .col(ColumnDef::new(Class::TeacherId).uuid())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_classes_teacher_id")
                            .from(Class::Table, Class::TeacherId)
                            .to(Teacher::Table, Teacher::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .col(ColumnDef::new(Class::GroupId).uuid())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_classes_group_id")
                            .from(Class::Table, Class::GroupId)
                            .to(Group::Table, Group::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .col(ColumnDef::new(Class::RoomId).uuid())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_classes_room_id")
                            .from(Class::Table, Class::RoomId)
                            .to(Room::Table, Room::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .col(ColumnDef::new(Class::MdIdx).string().extra("GENERATED ALWAYS AS (MD5(CAST(subject_id AS VARCHAR) || '-' || CAST(teacher_id AS VARCHAR) || '-' || CAST(group_id AS VARCHAR) || '-' || CAST(room_id AS VARCHAR))) STORED"))
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Class::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Class {
    #[sea_orm(iden = "classes")]
    Table,
    Id,
    #[sea_orm(iden = "subject_id")]
    SubjectId,
    #[sea_orm(iden = "teacher_id")]
    TeacherId,
    #[sea_orm(iden = "group_id")]
    GroupId,
    #[sea_orm(iden = "room_id")]
    RoomId,
    #[sea_orm(iden = "md_idx")]
    MdIdx,
}
