use sea_orm_migration::prelude::*;

use super::m20220101_000001_create_table::Job;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .alter_table(
                Table::alter()
                    .table(Job::Table)
                    .add_column(ColumnDef::new(Alias::new("published_at"))
                        .timestamp_with_time_zone()
                        .not_null()
                        .default(Expr::current_timestamp())
                    )
                    .add_column(ColumnDef::new(Alias::new("updated_at"))
                        .timestamp_with_time_zone()
                        .not_null()
                        .default(Expr::current_timestamp())
                    )
                    .to_owned()
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .alter_table(
                Table::alter()
                    .table(Job::Table)
                    .drop_column(Alias::new("published_at"))
                    .to_owned()
            )
            .await
    }
}
