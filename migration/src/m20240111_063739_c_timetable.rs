use sea_orm::{sea_query::extension::postgres::Type, EnumIter, Iterable};
use sea_orm_migration::prelude::*;

use crate::m20231223_094755_c_classes::Class;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // create week days enum
        manager
            .create_type(
                Type::create()
                    .as_enum(DayOfWeekEnum::Table)
                    .values(DayOfWeekEnum::iter().skip(1))
                    .to_owned(),
            )
            .await?;

        // create timetable item type enum
        manager
            .create_type(
                Type::create()
                    .as_enum(TimeTableItemType::Table)
                    .values(TimeTableItemType::iter().skip(1))
                    .to_owned(),
            )
            .await?;

        // create timetable table
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
                    .col(
                        ColumnDef::new(TimeTable::Type)
                            .enumeration(
                                TimeTableItemType::Table,
                                TimeTableItemType::iter().skip(1),
                            )
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(TimeTable::DayOfWeek)
                            .enumeration(DayOfWeekEnum::Table, DayOfWeekEnum::iter().skip(1)),
                    )
                    .col(ColumnDef::new(TimeTable::FullDate).date())
                    .col(ColumnDef::new(TimeTable::StartTime).time())
                    .col(ColumnDef::new(TimeTable::EndTime).time())
                    .to_owned(),
            )
            .await?;

        // create activities table
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

        // create lectures table
        manager
            .create_table(
                Table::create()
                    .table(Lecture::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Lecture::Id)
                            .uuid()
                            .not_null()
                            .default(Expr::cust("gen_random_uuid()"))
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Lecture::ClassId).uuid())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_lectures_class_id")
                            .from(Lecture::Table, Lecture::ClassId)
                            .to(Class::Table, Class::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .col(ColumnDef::new(Lecture::TimeTableId).uuid())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_lectures_time_table_id")
                            .from(Lecture::Table, Lecture::TimeTableId)
                            .to(TimeTable::Table, TimeTable::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // create events table
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
            .drop_foreign_key(
                sea_query::ForeignKey::drop()
                    .name("fk_events_time_table_id")
                    .table(Event::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_table(Table::drop().table(Event::Table).to_owned())
            .await?;

        manager
            .drop_foreign_key(
                sea_query::ForeignKey::drop()
                    .name("fk_lectures_time_table_id")
                    .table(Lecture::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_table(Table::drop().table(Lecture::Table).to_owned())
            .await?;

        manager
            .drop_foreign_key(
                sea_query::ForeignKey::drop()
                    .name("fk_activities_time_table_id")
                    .table(Activity::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_table(Table::drop().table(Activity::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(TimeTable::Table).to_owned())
            .await?;

        manager
            .drop_type(
                Type::drop()
                    .if_exists()
                    .name(DayOfWeekEnum::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_type(
                Type::drop()
                    .if_exists()
                    .name(TimeTableItemType::Table)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }
}

#[derive(Iden, EnumIter)]
enum DayOfWeekEnum {
    #[iden = "day_of_week_enum"]
    Table,
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

#[derive(Iden, EnumIter)]
enum TimeTableItemType {
    Table,
    Activity,
    Lecture,
    Event,
}

#[derive(DeriveIden)]
pub enum TimeTable {
    #[sea_orm(iden = "time_table")]
    Table,
    Id,
    #[sea_orm(iden = "item_type")]
    Type,
    #[sea_orm(iden = "day_of_week")]
    DayOfWeek,
    #[sea_orm(iden = "full_date")]
    FullDate,
    #[sea_orm(iden = "start_time")]
    StartTime,
    #[sea_orm(iden = "end_time")]
    EndTime,
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
pub enum Lecture {
    #[sea_orm(iden = "lectures")]
    Table,
    Id,
    #[sea_orm(iden = "class_id")]
    ClassId,
    #[sea_orm(iden = "time_table_id")]
    TimeTableId,
}

#[derive(DeriveIden)]
pub enum Event {
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
