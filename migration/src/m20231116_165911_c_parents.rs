use sea_orm_migration::{prelude::*, sea_query::extension::postgres::PgExpr};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Parent::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Parent::Id)
                            .uuid()
                            .not_null()
                            .default(Expr::cust("gen_random_uuid()"))
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Parent::FirstName).string().not_null())
                    .col(ColumnDef::new(Parent::LastName).string().not_null())
                    .col(
                        ColumnDef::new(Parent::FullName).string().generated(
                            Expr::col(Parent::FirstName)
                                .concat(" ")
                                .concat(Expr::col(Parent::LastName)),
                            true,
                        ),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                sea_query::Index::create()
                    .table(Parent::Table)
                    .col(Parent::FullName)
                    .name("idx_parents_full_name")
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Parent::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Parent {
    #[sea_orm(iden = "parents")]
    Table,
    Id,
    #[sea_orm(iden = "first_name")]
    FirstName,
    #[sea_orm(iden = "last_name")]
    LastName,
    #[sea_orm(iden = "full_name")]
    FullName,
    #[sea_orm(iden = "person_id")]
    PersonId,
}
