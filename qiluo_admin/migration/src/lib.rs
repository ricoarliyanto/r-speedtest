pub use sea_orm_migration::prelude::*;

mod m20220101_000001_create_table;
mod m20220101_000002_create_codegen_config;
mod m20260822_000001_create_net_tables;
pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_create_table::Migration),
            Box::new(m20220101_000002_create_codegen_config::Migration),
            Box::new(m20260822_000001_create_net_tables::Migration),
        ]
    }
}
