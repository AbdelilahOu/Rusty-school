use sea_orm_migration::prelude::*;

use crate::m20231127_123039_c_users::User;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Session::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Session::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(Session::UserId).uuid().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_sessions_user_id")
                            .from(Session::Table, Session::UserId)
                            .to(User::Table, User::Id)
                            .on_delete(ForeignKeyAction::Restrict),
                    )
                    .col(ColumnDef::new(Session::UserAgent).string().not_null())
                    .col(ColumnDef::new(Session::ClientIp).string().not_null())
                    .col(
                        ColumnDef::new(Session::IsBlocked)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(ColumnDef::new(Session::RefreshToken).string().not_null())
                    .col(ColumnDef::new(Session::ExpiresAt).timestamp().not_null())
                    .col(
                        ColumnDef::new(Session::CreatedAt)
                            .timestamp()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_foreign_key(
                sea_query::ForeignKey::drop()
                    .name("fk_sessions_user_id")
                    .table(Session::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_table(Table::drop().table(Session::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Session {
    #[sea_orm(iden = "sessions")]
    Table,
    Id,
    UserId,
    ClientIp,
    UserAgent,
    IsBlocked,
    RefreshToken,
    ExpiresAt,
    CreatedAt,
}
