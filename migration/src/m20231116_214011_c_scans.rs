use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Scan::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Scan::Id)
                            .uuid()
                            .not_null()
                            .default(Expr::cust("gen_random_uuid()"))
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(Scan::ScanDate)
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
            .drop_table(Table::drop().table(Scan::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Scan {
    #[sea_orm(iden = "scans")]
    Table,
    Id,
    #[sea_orm(iden = "person_id")]
    PersonId,
    #[sea_orm(iden = "scan_date")]
    ScanDate,
}
