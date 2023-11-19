use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Levels::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Levels::Id)
                            .uuid()
                            .not_null()
                            .default(Expr::cust("gen_random_uuid()"))
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Levels::Name).string().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Levels::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Levels {
    #[sea_orm(iden = "levels")]
    Table,
    Id,
    #[sea_orm(iden = "level_name")]
    Name,
}
