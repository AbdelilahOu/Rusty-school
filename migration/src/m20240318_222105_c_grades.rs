use sea_orm_migration::prelude::*;

use crate::{m20231109_190937_c_students::Student, m20240315_134424_c_assignments::Assignment};

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
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_grade_student_id")
                            .from(Grade::Table, Grade::StudentId)
                            .to(Student::Table, Student::Id)
                            .on_delete(ForeignKeyAction::Restrict),
                    )
                    .col(ColumnDef::new(Grade::AssignmentId).uuid().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_grade_assignment_id")
                            .from(Grade::Table, Grade::AssignmentId)
                            .to(Assignment::Table, Assignment::Id)
                            .on_delete(ForeignKeyAction::Restrict),
                    )
                    .col(ColumnDef::new(Grade::Score).float().not_null())
                    .col(ColumnDef::new(Grade::Feedback).string())
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_foreign_key(
                sea_query::ForeignKey::drop()
                    .name("fk_grade_student_id")
                    .table(Grade::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_foreign_key(
                sea_query::ForeignKey::drop()
                    .name("fk_grade_assignment_id")
                    .table(Grade::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_table(Table::drop().table(Grade::Table).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum Grade {
    #[sea_orm(iden = "grades")]
    Table,
    Id,
    StudentId,
    AssignmentId,
    Score,
    Feedback,
}
