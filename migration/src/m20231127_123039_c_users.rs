use sea_orm::{sea_query::extension::postgres::Type, EnumIter, Iterable};
use sea_orm_migration::prelude::*;

use crate::m20231118_162555_c_persons::Person;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_type(
                Type::create()
                    .as_enum(Roles::Table)
                    .values(Roles::iter().skip(1))
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(User::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(User::Id)
                            .uuid()
                            .not_null()
                            .default(Expr::cust("gen_random_uuid()"))
                            .primary_key(),
                    )
                    .col(ColumnDef::new(User::Email).string().unique_key().not_null())
                    .col(ColumnDef::new(User::Name).string().not_null())
                    .col(ColumnDef::new(User::GivenName).string().not_null())
                    .col(ColumnDef::new(User::FamilyName).string().not_null())
                    .col(ColumnDef::new(User::PersonId).uuid().not_null())
                    .col(
                        ColumnDef::new(User::Role)
                            .enumeration(Roles::Table, Roles::iter().skip(1))
                            .not_null()
                            .default(Expr::cust("'not_defined'::roles")),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_user_person_id")
                            .from(User::Table, User::PersonId)
                            .to(Person::Table, Person::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .col(ColumnDef::new(User::Picture).string())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(User::Table).to_owned())
            .await?;
        manager
            .drop_type(Type::drop().if_exists().name(Roles::Table).to_owned())
            .await?;
        Ok(())
    }
}

#[derive(Iden, EnumIter)]
enum Roles {
    #[iden = "roles"]
    Table,
    Admin,
    Assistant,
    Teacher,
    Parent,
    Student,
    NotDefined,
}

#[derive(DeriveIden)]
pub enum User {
    #[sea_orm(iden = "users")]
    Table,
    Id,
    Email,
    #[sea_orm(iden = "name")]
    Name,
    #[sea_orm(iden = "given_name")]
    GivenName,
    #[sea_orm(iden = "family_name")]
    FamilyName,
    #[sea_orm(iden = "person_id")]
    PersonId,
    Picture,
    Role,
}
