use sea_orm_migration::{prelude::*, sea_query::extension::postgres::PgExpr};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Teacher::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Teacher::Id)
                            .uuid()
                            .not_null()
                            .default(Expr::cust("gen_random_uuid()"))
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Teacher::FirstName).string().not_null())
                    .col(ColumnDef::new(Teacher::LastName).string().not_null())
                    .col(
                        ColumnDef::new(Teacher::FullName).string().generated(
                            Expr::col(Teacher::FirstName)
                                .concat(" ")
                                .concat(Expr::col(Teacher::LastName)),
                            true,
                        ),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                sea_query::Index::create()
                    .table(Teacher::Table)
                    .col(Teacher::FullName)
                    .name("teachers_full_name_idx")
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Teacher::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Teacher {
    #[sea_orm(iden = "teachers")]
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
    #[sea_orm(iden = "level_id")]
    LevelId,
}
