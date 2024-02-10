use sea_orm_migration::prelude::*;

use crate::m20240126_020426_create_users::User;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Membership::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Membership::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Membership::UserId).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("FK-membership_user_foreign_key")
                            .from(Membership::Table, Membership::UserId)
                            .to(User::Table, User::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade)
                    )
                    .col(ColumnDef::new(Membership::IsLifetime).boolean().not_null())
                    .col(ColumnDef::new(Membership::ExpiresAt).timestamp_with_time_zone().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Membership::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Membership {
    Table,
    Id,
    UserId,
    IsLifetime,
    ExpiresAt
}
