pub use sea_schema::migration::*;

mod m20220302_000001_create_user_table;
mod m20220430_000001_create_resource_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220302_000001_create_user_table::Migration),
            Box::new(m20220430_000001_create_resource_table::Migration),
        ]
    }
}


#[cfg(test)]
mod tests {

    use std::env;

    use super::Migrator;
    use sea_schema::migration::*;
    use testcontainers::{images::postgres::Postgres, *};

    #[tokio::test]
    async fn test_migrations() {
        let docker = clients::Cli::default();
        let postgres = docker.run(Postgres::default().with_version(14));
        env::set_var(
            "DATABASE_URL",
            format!(
                "postgres://postgres@localhost:{}/postgres",
                postgres.get_host_port(5432).unwrap_or_default()
            ),
        );
        cli::run_cli(Migrator).await;
    }
}