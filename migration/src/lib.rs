pub use sea_orm_migration::prelude::*;

mod m20240723_000001_connection_config_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![Box::new(
            m20240723_000001_connection_config_table::Migration,
        )]
    }
}
