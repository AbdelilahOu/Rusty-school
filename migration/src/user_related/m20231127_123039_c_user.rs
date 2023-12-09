use sea_orm_migration::{prelude::*, sea_query::extension::postgres::PgExpr};

use crate::m20231118_162555_c_person::Person;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Users::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Users::Id)
                            .uuid()
                            .not_null()
                            .default(Expr::cust("gen_random_uuid()"))
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(Users::Email)
                            .string()
                            .unique_key()
                            .not_null(),
                    )
                    .col(ColumnDef::new(Users::FirstName).string().not_null())
                    .col(ColumnDef::new(Users::LastName).string().not_null())
                    .col(
                        ColumnDef::new(Users::FullName).string().generated(
                            Expr::col(Users::FirstName)
                                .concat(" ")
                                .concat(Expr::col(Users::LastName)),
                            true,
                        ),
                    )
                    .col(ColumnDef::new(Users::PersonId).uuid().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-person-id")
                            .from(Users::Table, Users::PersonId)
                            .to(Person::Table, Person::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .col(ColumnDef::new(Users::Picture).string())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Users::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Users {
    #[sea_orm(iden = "users")]
    Table,
    Id,
    Email,
    #[sea_orm(iden = "first_name")]
    FirstName,
    #[sea_orm(iden = "last_name")]
    LastName,
    #[sea_orm(iden = "full_name")]
    FullName,
    #[sea_orm(iden = "person_id")]
    PersonId,
    Picture,
}
