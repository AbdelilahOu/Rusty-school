use sea_orm_migration::prelude::*;

use super::m20231109_190937_c_student::Student;
use super::m20231116_165911_c_parents::Parent;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Pickup::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Pickup::Id)
                            .uuid()
                            .not_null()
                            .default(Expr::cust("gen_random_uuid()"))
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Pickup::StudentId).uuid().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-pickup-student_id")
                            .from(Pickup::Table, Pickup::StudentId)
                            .to(Student::Table, Student::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .col(ColumnDef::new(Pickup::ParentId).uuid().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-pickup-parent_id")
                            .from(Pickup::Table, Pickup::ParentId)
                            .to(Parent::Table, Parent::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .col(
                        ColumnDef::new(Pickup::CreatedAt)
                            .timestamp()
                            .not_null()
                            .extra("DEFAULT CURRENT_TIMESTAMP"),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Pickup::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Pickup {
    #[sea_orm(iden = "pickups")]
    Table,
    Id,
    #[sea_orm(iden = "student_id")]
    StudentId,
    #[sea_orm(iden = "parent_id")]
    ParentId,
    #[sea_orm(iden = "created_at")]
    CreatedAt,
}
