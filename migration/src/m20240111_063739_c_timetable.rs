use sea_orm::{sea_query::extension::postgres::Type, EnumIter, Iterable};
use sea_orm_migration::prelude::*;

use crate::m20231223_094755_c_classes::Class;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_type(
                Type::create()
                    .as_enum(DayOfWeek::Table)
                    .values(DayOfWeek::iter().skip(1))
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
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
                    .col(ColumnDef::new(TimeTable::Type).text())
                    .col(
                        ColumnDef::new(TimeTable::DayOfWeek)
                            .enumeration(DayOfWeek::Table, DayOfWeek::iter().skip(1)),
                    )
                    .col(ColumnDef::new(TimeTable::StartTime).date_time())
                    .col(ColumnDef::new(TimeTable::EndTime).date_time())
                    .col(ColumnDef::new(TimeTable::Duration).float().generated(
                        Expr::cust(
                            "EXTRACT(EPOCH from (end_time::TIMESTAMP - start_time::TIMESTAMP))",
                        ),
                        true,
                    ))
                    .col(ColumnDef::new(TimeTable::Location).text())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Activity::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Activity::Id)
                            .uuid()
                            .not_null()
                            .default(Expr::cust("gen_random_uuid()"))
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Activity::Title).text())
                    .col(ColumnDef::new(Activity::Description).text())
                    .col(ColumnDef::new(Activity::ActivityType).text())
                    .col(ColumnDef::new(Activity::TimeTableId).uuid())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_activities_time_table_id")
                            .from(Activity::Table, Activity::TimeTableId)
                            .to(TimeTable::Table, TimeTable::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Course::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Course::Id)
                            .uuid()
                            .not_null()
                            .default(Expr::cust("gen_random_uuid()"))
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Course::ClassId).uuid())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_courses_class_id")
                            .from(Course::Table, Course::ClassId)
                            .to(Class::Table, Class::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .col(ColumnDef::new(Course::TimeTableId).uuid())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_courses_time_table_id")
                            .from(Course::Table, Course::TimeTableId)
                            .to(TimeTable::Table, TimeTable::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Event::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Event::Id)
                            .uuid()
                            .not_null()
                            .default(Expr::cust("gen_random_uuid()"))
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Event::Title).text())
                    .col(ColumnDef::new(Event::Description).text())
                    .col(ColumnDef::new(Event::TimeTableId).uuid())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_events_time_table_id")
                            .from(Event::Table, Event::TimeTableId)
                            .to(TimeTable::Table, TimeTable::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(TimeTable::Table).to_owned())
            .await
    }
}

#[derive(Iden, EnumIter)]
enum DayOfWeek {
    Table,
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

#[derive(DeriveIden)]
enum TimeTable {
    #[sea_orm(iden = "time_table")]
    Table,
    Id,
    #[sea_orm(iden = "type")]
    Type,
    #[sea_orm(iden = "day_of_week")]
    DayOfWeek,
    #[sea_orm(iden = "start_time")]
    StartTime,
    #[sea_orm(iden = "end_time")]
    EndTime,
    #[sea_orm(iden = "duration")]
    Duration,
    #[sea_orm(iden = "location")]
    Location,
}

#[derive(DeriveIden)]
enum Activity {
    #[sea_orm(iden = "activities")]
    Table,
    Id,
    #[sea_orm(iden = "activity_title")]
    Title,
    #[sea_orm(iden = "activity_description")]
    Description,
    #[sea_orm(iden = "activity_type")]
    ActivityType,
    #[sea_orm(iden = "time_table_id")]
    TimeTableId,
}

#[derive(DeriveIden)]
enum Course {
    #[sea_orm(iden = "courses")]
    Table,
    Id,
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
    #[sea_orm(iden = "event_title")]
    Title,
    #[sea_orm(iden = "event_description")]
    Description,
    #[sea_orm(iden = "time_table_id")]
    TimeTableId,
}
