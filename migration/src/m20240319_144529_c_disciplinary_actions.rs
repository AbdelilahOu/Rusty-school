use sea_orm_migration::prelude::*;

use crate::m20231109_190937_c_students::Student;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(DisciplinaryActions::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(DisciplinaryActions::Id)
                            .uuid()
                            .not_null()
                            .default(Expr::cust("gen_random_uuid()"))
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(DisciplinaryActions::StudentId)
                            .uuid()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_disciplinary_actions_student_id")
                            .from(DisciplinaryActions::Table, DisciplinaryActions::StudentId)
                            .to(Student::Table, Student::Id)
                            .on_delete(ForeignKeyAction::Restrict),
                    )
                    .col(
                        ColumnDef::new(DisciplinaryActions::CreatedAt)
                            .timestamp()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(DisciplinaryActions::IssuedAt)
                            .timestamp()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(DisciplinaryActions::Description)
                            .text()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(DisciplinaryActions::Consequences)
                            .text()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_foreign_key(
                sea_query::ForeignKey::drop()
                    .name("fk_disciplinary_actions_student_id")
                    .table(DisciplinaryActions::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_table(Table::drop().table(DisciplinaryActions::Table).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum DisciplinaryActions {
    Table,
    Id,
    StudentId,
    CreatedAt,
    IssuedAt,
    Description,
    Consequences,
}
