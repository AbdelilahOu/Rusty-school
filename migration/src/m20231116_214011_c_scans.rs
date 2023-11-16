use sea_orm_migration::prelude::*;

use super::m20231109_190937_c_student::Students;
use super::m20231113_170500_c_teacher::Teachers;
use super::m20231116_165911_c_parents::Parents;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Scans::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Scans::Id).uuid().primary_key())
                    .col(ColumnDef::new(Scans::StudentId).uuid())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-scan-student_id")
                            .from(Scans::Table, Scans::StudentId)
                            .to(Students::Table, Students::Id),
                    )
                    .col(ColumnDef::new(Scans::ParentId).uuid())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-scan-parent_id")
                            .from(Scans::Table, Scans::ParentId)
                            .to(Parents::Table, Parents::Id),
                    )
                    .col(ColumnDef::new(Scans::TeacherId).uuid())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-scan-teacher_id")
                            .from(Scans::Table, Scans::TeacherId)
                            .to(Teachers::Table, Teachers::Id),
                    )
                    .col(
                        ColumnDef::new(Scans::ScanDate)
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
            .drop_table(Table::drop().table(Scans::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Scans {
    #[sea_orm(iden = "scans")]
    Table,
    Id,
    #[sea_orm(iden = "student_id")]
    StudentId,
    #[sea_orm(iden = "parent_id")]
    ParentId,
    #[sea_orm(iden = "teacher_id")]
    TeacherId,
    #[sea_orm(iden = "scan_date")]
    ScanDate,
}
