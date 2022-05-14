use migration::Migrator;
use sea_schema::migration::*;

#[async_std::main]
async fn main() {
    cli::run_cli(Migrator).await;
}

#[cfg(test)]
mod tests {

    use std::env;

    use migration::Migrator;
    use sea_schema::migration::*;
    use testcontainers::{
        images::postgres::Postgres,
        *,
    };

    #[tokio::test]
    async fn test_migrations() {
        let docker = clients::Cli::default();
        let postgres = docker.run(Postgres::default().with_version(14));
        env::set_var("DATABASE_URL", format!("postgres://postgres@localhost:{}/postgres", postgres.get_host_port(5432).unwrap_or_default()));
        cli::run_cli(Migrator).await;
    }
}
