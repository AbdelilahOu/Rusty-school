use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.create_table(
            Table::create()
                .table(TimeTable::Table)
                .if_not_exists()
                .col(
                    ColumnDef::new(TimeTable::Id)
                        .uuid()
                        .not_null()
                        .default(Expr::cust("gen_random_uuid()"))
                        .primary_key(),
                )
                .col(ColumnDef::new(TimeTable::DayOfWeek).time())
                .col(ColumnDef::new(TimeTable::StartTime).time())
                .col(ColumnDef::new(TimeTable::EndTime).text()),
        );
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(TimeTable::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum TimeTable {
    #[sea_orm(iden = "time_table")]
    Table,
    Id,
    #[sea_orm(iden = "type")]
    Type,
}

#[derive(DeriveIden)]
enum Activity {
    #[sea_orm(iden = "activities")]
    Table,
    Id,
    #[sea_orm(iden = "description")]
    Description,
    #[sea_orm(iden = "activity_type")]
    ActivityType,
    #[sea_orm(iden = "start_time")]
    StartTime,
    #[sea_orm(iden = "end_time")]
    EndTime,
    #[sea_orm(iden = "location")]
    Location,
    #[sea_orm(iden = "time_table_id")]
    TimeTableId,
}

#[derive(DeriveIden)]
enum Course {
    #[sea_orm(iden = "courses")]
    Table,
    Id,
    #[sea_orm(iden = "day_of_week")]
    DayOfWeek,
    #[sea_orm(iden = "start_time")]
    StartTime,
    #[sea_orm(iden = "end_time")]
    EndTime,
    #[sea_orm(iden = "class_id")]
    ClassId,
    #[sea_orm(iden = "time_table_id")]
    TimeTableId,
}

#[derive(DeriveIden)]
enum Event {
    #[sea_orm(iden = "events")]
    Table,
    Id,
    #[sea_orm(iden = "title")]
    Title,
    #[sea_orm(iden = "description")]
    Description,
    #[sea_orm(iden = "start_time")]
    StartTime,
    #[sea_orm(iden = "end_time")]
    EndTime,
    #[sea_orm(iden = "class_id")]
    ClassId,
    #[sea_orm(iden = "time_table_id")]
    TimeTableId,
}
