use sea_orm_migration::{prelude::*, sea_query::extension::postgres::PgExpr};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Student::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Student::Id)
                            .uuid()
                            .not_null()
                            .default(Expr::cust("gen_random_uuid()"))
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Student::FirstName).string().not_null())
                    .col(ColumnDef::new(Student::LastName).string().not_null())
                    .col(
                        ColumnDef::new(Student::CreatedAt)
                            .timestamp()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(Student::FullName).string().generated(
                            Expr::col(Student::FirstName)
                                .concat(" ")
                                .concat(Expr::col(Student::LastName)),
                            true,
                        ),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                sea_query::Index::create()
                    .table(Student::Table)
                    .col(Student::FullName)
                    .name("idx_students_full_name")
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Student::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Student {
    #[sea_orm(iden = "students")]
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
    #[sea_orm(iden = "group_id")]
    GroupId,
    #[sea_orm(iden = "created_at")]
    CreatedAt,
}
