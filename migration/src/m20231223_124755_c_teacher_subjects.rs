use sea_orm_migration::prelude::*;

use crate::{m20231113_170500_c_teachers::Teacher, m20231215_142739_c_subjects::Subject};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(TeacherSubjects::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(TeacherSubjects::Id)
                            .uuid()
                            .not_null()
                            .default(Expr::cust("gen_random_uuid()"))
                            .primary_key(),
                    )
                    .col(ColumnDef::new(TeacherSubjects::SubjectId).uuid())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_teacher_subjects_subject_id")
                            .from(TeacherSubjects::Table, TeacherSubjects::SubjectId)
                            .to(Subject::Table, Subject::Id)
                            .on_delete(ForeignKeyAction::SetNull),
                    )
                    .col(ColumnDef::new(TeacherSubjects::TeacherId).uuid())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_teacher_subjects_teacher_id")
                            .from(TeacherSubjects::Table, TeacherSubjects::TeacherId)
                            .to(Teacher::Table, Teacher::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .col(ColumnDef::new(TeacherSubjects::MdIdx).string().extra("GENERATED ALWAYS AS (MD5(CAST(subject_id AS VARCHAR) || '-' || CAST(teacher_id AS VARCHAR))) STORED"))
                    .to_owned(),
            )
            .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(TeacherSubjects::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum TeacherSubjects {
    #[sea_orm(iden = "teacher_subjects")]
    Table,
    Id,
    TeacherId,
    SubjectId,
    #[sea_orm(iden = "md_idx")]
    MdIdx,
}
