use sea_orm_migration::prelude::*;

use crate::m20240126_020426_create_users::User;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .create_table(
                Table::create()
                    .table(ConfirmationRequest::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(ConfirmationRequest::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(ConfirmationRequest::UserId).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("FK-confirmation-request_user_foreign_key")
                            .from(ConfirmationRequest::Table, ConfirmationRequest::UserId)
                            .to(User::Table, User::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade)
                    )
                    .col(ColumnDef::new(ConfirmationRequest::ConfirmCode).string().not_null())
                    .col(ColumnDef::new(ConfirmationRequest::ExpiresAt).timestamp_with_time_zone().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .drop_table(Table::drop().table(ConfirmationRequest::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum ConfirmationRequest {
    Table,
    Id,
    UserId,
    ConfirmCode,
    ExpiresAt
}

