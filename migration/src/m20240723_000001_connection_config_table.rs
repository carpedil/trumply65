use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(ConnectionConfig::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(ConnectionConfig::Id)
                            .string()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(ConnectionConfig::Env).string().not_null())
                    .col(ColumnDef::new(ConnectionConfig::DbType).string().not_null())
                    .col(ColumnDef::new(ConnectionConfig::Url).string().not_null())
                    .col(
                        ColumnDef::new(ConnectionConfig::Username)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(ConnectionConfig::Password)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(ConnectionConfig::IsActive)
                            .boolean()
                            .not_null(),
                    )
                    .col(ColumnDef::new(ConnectionConfig::AbandonedTableList).string())
                    .to_owned()
                    .col(ColumnDef::new(ConnectionConfig::CreatedAt).timestamp())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(ConnectionConfig::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum ConnectionConfig {
    Table,
    Id,
    Env,
    DbType,
    Url,
    Username,
    Password,
    IsActive,
    AbandonedTableList,
    CreatedAt,
}
