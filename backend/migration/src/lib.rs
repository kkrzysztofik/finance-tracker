pub use sea_orm_migration::prelude::*;

mod m20260621_000001_create_schema;
mod m20260621_000002_seed_categories;

/// Central migration registry used by the backend and migration CLI.
pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20260621_000001_create_schema::Migration),
            Box::new(m20260621_000002_seed_categories::Migration),
        ]
    }
}
