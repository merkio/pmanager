pub use sea_schema::migration::*;

mod m20220302_000001_create_user_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220302_000001_create_user_table::Migration),
        ]
    }
}