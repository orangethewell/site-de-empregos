use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .create_table(
                Table::create()
                    .table(Job::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Job::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Job::Position).string_len(100).not_null())
                    .col(ColumnDef::new(Job::Company).string_len(100).not_null())
                    .col(ColumnDef::new(Job::Description).string())
                    .col(ColumnDef::new(Job::Requirements).string().not_null())
                    .to_owned(),
            )
        .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .drop_table(Table::drop().table(Job::Table).to_owned())
        .await
    }
}

#[derive(DeriveIden)]
pub enum Job {
    Table,
    Id,
    Position,
    Company,
    Description,
    Requirements,
}
