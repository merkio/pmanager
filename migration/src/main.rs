use std::env;

use migration::{sea_orm::Database, Migrator};
use sea_orm_migration::prelude::*;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::TRACE)
        .with_test_writer()
        .init();
    dbg!("Tracing initialized.");

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    let db = Database::connect(db_url)
        .await
        .expect("Failed to connect to database");

    Migrator::up(&db, None).await.expect("Failed to migrate");
    Ok(())
}
