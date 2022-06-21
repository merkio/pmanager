use anyhow::Result;
use api::files::router;
use app_config::ApplicationConfig;
use axum::{Router, Server, Extension};
use log::info;
use std::net::SocketAddr;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use sea_orm::Database;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<()> {
    let config = ApplicationConfig::default();
    let db = Database::connect(config.db.url.clone()).await.expect("Failed to connect to database");

    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "DEBUG".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let app = Router::new()
        .merge(router())
        .layer(Extension(Arc::new(config)))
        .layer(Extension(Arc::new(db)))
        .layer(tower_http::trace::TraceLayer::new_for_http());

    info!("Starting server...");
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);
    Server::bind(&addr).serve(app.into_make_service()).await?;

    Ok(())
}
