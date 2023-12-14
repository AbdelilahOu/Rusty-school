use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Level::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Level::Id)
                            .uuid()
                            .not_null()
                            .default(Expr::cust("gen_random_uuid()"))
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Level::Name).string().not_null())
                    .col(ColumnDef::new(Level::Description).string())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Level::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Level {
    #[sea_orm(iden = "levels")]
    Table,
    Id,
    #[sea_orm(iden = "level_name")]
    Name,
    #[sea_orm(iden = "level_description")]
    Description,
}
