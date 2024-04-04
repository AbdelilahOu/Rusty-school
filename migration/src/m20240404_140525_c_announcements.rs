use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
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
                    .col(
                        ColumnDef::new(Announcement::CreatedAt)
                            .timestamp()
                            .default(Expr::current_timestamp()),
                    )
                    .col(ColumnDef::new(Announcement::StartDate).timestamp())
                    .col(ColumnDef::new(Announcement::EndDate).timestamp())
                    .col(ColumnDef::new(Announcement::Category).text())
                    .col(ColumnDef::new(Announcement::Description).text())
                    .col(ColumnDef::new(Announcement::Attachements).timestamp())
                    .col(ColumnDef::new(Announcement::Important).boolean())
                    .col(ColumnDef::new(Announcement::Audience).string())
                    .col(ColumnDef::new(Announcement::Alert).timestamp())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Announcement::Table).to_owned())
            .await
    }
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
    Alert,
}
