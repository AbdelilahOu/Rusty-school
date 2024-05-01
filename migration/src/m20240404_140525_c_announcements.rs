use sea_orm::{sea_query::extension::postgres::Type, EnumIter, Iterable};
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_type(
                Type::create()
                    .as_enum(AnnouncementCategories::Table)
                    .values(AnnouncementCategories::iter().skip(1))
                    .to_owned(),
            )
            .await?;

        manager
            .create_type(Type::create().as_enum(Audiences::Table).values(Audiences::iter().skip(1)).to_owned())
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Announcement::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Announcement::Id)
                            .uuid()
                            .not_null()
                            .default(Expr::cust("gen_random_uuid()"))
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Announcement::Title).string().not_null())
                    .col(ColumnDef::new(Announcement::Description).text())
                    .col(ColumnDef::new(Announcement::CreatedAt).timestamp().default(Expr::current_timestamp()))
                    .col(ColumnDef::new(Announcement::StartDate).timestamp())
                    .col(ColumnDef::new(Announcement::EndDate).timestamp())
                    .col(
                        ColumnDef::new(Announcement::Category)
                            .enumeration(AnnouncementCategories::Table, AnnouncementCategories::iter().skip(1))
                            .not_null(),
                    )
                    .col(ColumnDef::new(Announcement::Targets).array(ColumnType::Uuid))
                    .col(ColumnDef::new(Announcement::Attachements).timestamp())
                    .col(ColumnDef::new(Announcement::Important).boolean())
                    .col(ColumnDef::new(Announcement::Audience).enumeration(Audiences::Table, Audiences::iter().skip(1)))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(Announcement::Table).to_owned()).await?;

        manager
            .drop_type(Type::drop().if_exists().name(AnnouncementCategories::Table).to_owned())
            .await?;

        manager.drop_type(Type::drop().if_exists().name(Audiences::Table).to_owned()).await?;

        Ok(())
    }
}

#[derive(Iden, EnumIter)]
enum AnnouncementCategories {
    #[iden = "announcement_categories"]
    Table,
    Event,
    General,
    Academic,
}

#[derive(Iden, EnumIter)]
enum Audiences {
    #[iden = "audiences"]
    Table,
    Teachers,
    Parents,
    Students,
    Groups,
}

#[derive(DeriveIden)]
enum Announcement {
    #[sea_orm(iden = "announcements")]
    Table,
    Id,
    Title,
    CreatedAt,
    StartDate,
    EndDate,
    Category,
    Description,
    Attachements,
    Important,
    Audience,
    Targets,
}
