use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let table = Table::create()
            .table(SysCodegenConfig::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(SysCodegenConfig::Id)
                    .big_integer()
                    .not_null()
                    .primary_key(),
            )
            .col(
                ColumnDef::new(SysCodegenConfig::ModuleName)
                    .string_len(100)
                    .not_null(),
            )
            .col(
                ColumnDef::new(SysCodegenConfig::TableName)
                    .string_len(100)
                    .not_null(),
            )
            .col(
                ColumnDef::new(SysCodegenConfig::ConfigJson)
                    .text()
                    .not_null(),
            )
            .col(
                ColumnDef::new(SysCodegenConfig::CreatedAt)
                    .date_time()
                    .not_null(),
            )
            .col(
                ColumnDef::new(SysCodegenConfig::UpdatedAt)
                    .date_time()
                    .not_null(),
            )
            .index(
                Index::create()
                    .unique()
                    .name("uk_codegen_module_table")
                    .col(SysCodegenConfig::ModuleName)
                    .col(SysCodegenConfig::TableName),
            )
            .to_owned();
        manager.create_table(table).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(SysCodegenConfig::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum SysCodegenConfig {
    Table,
    Id,
    ModuleName,
    TableName,
    ConfigJson,
    CreatedAt,
    UpdatedAt,
}
