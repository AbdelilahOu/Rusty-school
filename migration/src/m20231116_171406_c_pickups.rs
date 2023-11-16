use sea_orm_migration::prelude::*;

use super::m20231109_190937_c_student::Students;
use super::m20231116_165911_c_parents::Parents;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Pickups::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Pickups::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(Pickups::StudentId).uuid().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-pickup-student_id")
                            .from(Pickups::Table, Pickups::StudentId)
                            .to(Students::Table, Students::Id),
                    )
                    .col(ColumnDef::new(Pickups::ParentId).string().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-pickup-parent_id")
                            .from(Pickups::Table, Pickups::ParentId)
                            .to(Parents::Table, Parents::Id),
                    )
                    .col(ColumnDef::new(Pickups::PickupDate).timestamp().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Pickups::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Pickups {
    #[sea_orm(iden = "pickups")]
    Table,
    Id,
    #[sea_orm(iden = "student_id")]
    StudentId,
    #[sea_orm(iden = "parent_id")]
    ParentId,
    #[sea_orm(iden = "pickup_date")]
    PickupDate,
}
