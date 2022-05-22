use migration::Migrator;
use sea_schema::migration::*;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    cli::run_cli(Migrator).await;
}
